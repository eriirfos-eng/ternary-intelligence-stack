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
        
        // Add Gating Jitter (Noise)
        let noise = Tensor::rand(0.98f32, 1.02f32, gate_logits.shape(), dev)?;
        gate_logits = gate_logits.broadcast_mul(&noise)?;

        // 2. Top-2 Routing
        let max1_indices = gate_logits.argmax(candle_core::D::Minus1)?.to_dtype(candle_core::DType::U32)?;
        let max1_values = gate_logits.max(candle_core::D::Minus1)?;
        
        let range = Tensor::arange(0u32, self.num_experts as u32, dev)?
            .reshape((1, 1, self.num_experts))?.to_dtype(candle_core::DType::U32)?;
        let mask1 = range.broadcast_eq(&max1_indices.unsqueeze(candle_core::D::Minus1)?)?;
        
        let large_neg = Tensor::new(&[-1e9f32], dev)?.broadcast_as(gate_logits.shape())?;
        let gate_logits_masked = mask1.where_cond(&large_neg, &gate_logits)?;
        
        let max2_indices = gate_logits_masked.argmax(candle_core::D::Minus1)?.to_dtype(candle_core::DType::U32)?;
        let max2_values = gate_logits_masked.max(candle_core::D::Minus1)?;
        
        // Stack for Top-2 probs
        let top2_probs_flat = candle_nn::ops::softmax(&Tensor::stack(&[max1_values.flatten_all()?, max2_values.flatten_all()?], 1)?, 1)?;

        let mut final_output = Tensor::zeros((b * s, h), x.dtype(), dev)?;

        // 3. Streamlined Expert Execution
        // We remove the slow to_scalar() calls and use pure tensor masking.
        // Even if an expert isn't used, the matmul on a zero-mask is extremely fast on CPU.
        let max1_flat = max1_indices.flatten_all()?;
        let max2_flat = max2_indices.flatten_all()?;
        
        let prob1 = top2_probs_flat.narrow(1, 0, 1)?.flatten_all()?;
        let prob2 = top2_probs_flat.narrow(1, 1, 1)?.flatten_all()?;

        for expert_idx in 0..self.num_experts {
            let mask1_bool = max1_flat.eq(expert_idx as u32)?;
            let mask2_bool = max2_flat.eq(expert_idx as u32)?;
            
            let w1 = (mask1_bool.to_dtype(x.dtype())? * &prob1)?;
            let w2 = (mask2_bool.to_dtype(x.dtype())? * &prob2)?;
            let combined_weight = (w1 + w2)?.unsqueeze(1)?;
            
            // Mask the input: tokens not for this expert become 0
            // This is efficient because most of the input matrix will be 0.
            let expert_in = x_flat.broadcast_mul(&combined_weight.gt(0.0)?.to_dtype(x.dtype())?)?;
            let expert_out = self.experts[expert_idx].forward(&expert_in)?;
            
            // Accumulate weighted output
            final_output = (final_output + expert_out.broadcast_mul(&combined_weight)?)?;
        }
        
        final_output.reshape((b, s, h))
    }
}
