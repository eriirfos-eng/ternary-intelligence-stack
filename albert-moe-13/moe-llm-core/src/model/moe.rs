use candle_core::{Result, Tensor};
use candle_nn::VarBuilder;
use super::ternary_linear::TernaryLinear;
use super::mlp::Mlp;

pub struct MoeBlock {
    gate: TernaryLinear,
    experts: Vec<Mlp>,
    num_experts: usize,
}

impl MoeBlock {
    pub fn new(hidden_size: usize, num_experts: usize, vb: VarBuilder, threshold: f32) -> Result<Self> {
        let gate = TernaryLinear::new(hidden_size, num_experts, false, threshold, vb.pp("gate"))?;
        let mut experts = Vec::new();
        let vb_experts = vb.pp("experts");
        for i in 0..num_experts {
            experts.push(Mlp::new(hidden_size, hidden_size * 4, vb_experts.pp(i), threshold)?);
        }
        Ok(Self { gate, experts, num_experts })
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let (b, s, h) = x.dims3()?;
        let dev = x.device();
        let x_flat = x.reshape((b * s, h))?;
        
        // 1. Gate logits
        let mut gate_logits = self.gate.forward(x)?; // [B, S, E]
        
        // Add Gating Jitter (Noise) for exploration
        let noise = Tensor::rand(0.98f32, 1.02f32, gate_logits.shape(), dev)?;
        gate_logits = gate_logits.broadcast_mul(&noise)?;

        // 2. Top-3 Routing (v2.0 Evolution)
        let large_neg_val = Tensor::new(&[-1e9f32], dev)?;
        
        // Max 1
        let max1_indices = gate_logits.argmax(candle_core::D::Minus1)?.to_dtype(candle_core::DType::U32)?;
        let mask1 = Tensor::arange(0u32, self.num_experts as u32, dev)?
            .reshape((1, 1, self.num_experts))?.to_dtype(candle_core::DType::U32)?
            .broadcast_eq(&max1_indices.unsqueeze(candle_core::D::Minus1)?)?;
        
        // Max 2
        let gate_logits_m1 = mask1.where_cond(&large_neg_val.broadcast_as(gate_logits.shape())?, &gate_logits)?;
        let max2_indices = gate_logits_m1.argmax(candle_core::D::Minus1)?.to_dtype(candle_core::DType::U32)?;
        let mask2 = Tensor::arange(0u32, self.num_experts as u32, dev)?
            .reshape((1, 1, self.num_experts))?.to_dtype(candle_core::DType::U32)?
            .broadcast_eq(&max2_indices.unsqueeze(candle_core::D::Minus1)?)?;
            
        // Max 3
        let gate_logits_m2 = mask2.where_cond(&large_neg_val.broadcast_as(gate_logits.shape())?, &gate_logits_m1)?;
        let max3_indices = gate_logits_m2.argmax(candle_core::D::Minus1)?.to_dtype(candle_core::DType::U32)?;

        let max1_values = gate_logits.max(candle_core::D::Minus1)?;
        let max2_values = gate_logits_m1.max(candle_core::D::Minus1)?;
        let max3_values = gate_logits_m2.max(candle_core::D::Minus1)?;
        
        // 3. ASYMMETRIC SAFETY LOGIC (v2.0)
        let safety_threshold = 0.05f32;
        
        let apply_safety = |idx_tensor: &Tensor, val_tensor: &Tensor| -> Result<Tensor> {
            let is_safety = idx_tensor.lt(4u32)?.to_dtype(candle_core::DType::F32)?;
            let is_low_conf = val_tensor.lt(safety_threshold)?.to_dtype(candle_core::DType::F32)?;
            let should_hold = (is_safety * is_low_conf)?;
            let multiplier = (should_hold.neg()? + 1.0)?;
            val_tensor.broadcast_mul(&multiplier)
        };

        let max1_values = apply_safety(&max1_indices, &max1_values)?;
        let max2_values = apply_safety(&max2_indices, &max2_values)?;
        let max3_values = apply_safety(&max3_indices, &max3_values)?;

        // Softmax across Top-3
        let top3_logits = Tensor::stack(&[max1_values.flatten_all()?, max2_values.flatten_all()?, max3_values.flatten_all()?], 1)?;
        let top3_probs = candle_nn::ops::softmax(&top3_logits, 1)?;

        let mut final_output = Tensor::zeros((b * s, h), x.dtype(), dev)?;

        // 4. Sequential Expert Execution (Stable)
        let m1_flat = max1_indices.flatten_all()?;
        let m2_flat = max2_indices.flatten_all()?;
        let m3_flat = max3_indices.flatten_all()?;
        
        let p1 = top3_probs.narrow(1, 0, 1)?.flatten_all()?;
        let p2 = top3_probs.narrow(1, 1, 1)?.flatten_all()?;
        let p3 = top3_probs.narrow(1, 2, 1)?.flatten_all()?;

        for expert_idx in 0..self.num_experts {
            let mask1_bool = m1_flat.eq(expert_idx as u32)?;
            let mask2_bool = m2_flat.eq(expert_idx as u32)?;
            let mask3_bool = m3_flat.eq(expert_idx as u32)?;
            
            let w1 = (mask1_bool.to_dtype(x.dtype())? * &p1)?;
            let w2 = (mask2_bool.to_dtype(x.dtype())? * &p2)?;
            let w3 = (mask3_bool.to_dtype(x.dtype())? * &p3)?;
            let combined_weight = (w1 + w2 + w3)?.unsqueeze(1)?;
            
            // Mask input for efficiency: only process tokens assigned to this expert
            let expert_in = x_flat.broadcast_mul(&combined_weight.gt(0.0)?.to_dtype(x.dtype())?)?;
            let expert_out = self.experts[expert_idx].forward(&expert_in)?;
            
            // Accumulate weighted output
            final_output = (final_output + expert_out.broadcast_mul(&combined_weight)?)?;
        }
        
        final_output.reshape((b, s, h))
    }
}
