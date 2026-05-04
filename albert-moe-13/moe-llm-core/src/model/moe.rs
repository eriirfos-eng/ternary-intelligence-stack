use candle_core::{Result, Tensor};
use candle_nn::{Module, VarBuilder};
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
        
        // 1. Gate logits
        let gate_logits = self.gate.forward(x)?; // [B, S, E]
        
        // 2. Manual Top-k routing (k=2) using argmax twice
        let max1_indices = gate_logits.argmax(candle_core::D::Minus1)?;
        let max1_values = gate_logits.max(candle_core::D::Minus1)?;
        
        // Create mask for max1 to find the second best
        let dev = gate_logits.device();
        let range = Tensor::arange(0u32, self.num_experts as u32, dev)?
            .reshape((1, 1, self.num_experts))?;
        let mask1 = range.broadcast_eq(&max1_indices.unsqueeze(candle_core::D::Minus1)?)?;
        
        let large_neg = Tensor::new(&[-1e9f32], dev)?.broadcast_as(gate_logits.shape())?;
        let gate_logits_masked = mask1.where_cond(&large_neg, &gate_logits)?;
        
        let max2_indices = gate_logits_masked.argmax(candle_core::D::Minus1)?;
        let max2_values = gate_logits_masked.max(candle_core::D::Minus1)?;
        
        let top_k_indices = Tensor::stack(&[max1_indices, max2_indices], candle_core::D::Minus1)?;
        let top_k_values = Tensor::stack(&[max1_values, max2_values], candle_core::D::Minus1)?;
        
        // 3. Softmax over top-k values
        let top_k_probs = candle_nn::ops::softmax(&top_k_values, candle_core::D::Minus1)?;
        
        // 4. Vectorized expert execution and combining
        let mut final_output = Tensor::zeros((b, s, h), x.dtype(), x.device())?;
        
        let k = 2;
        for j in 0..k {
            let expert_indices = top_k_indices.narrow(candle_core::D::Minus1, j, 1)?;
            let expert_weights = top_k_probs.narrow(candle_core::D::Minus1, j, 1)?;
            
            for expert_idx in 0..self.num_experts {
                // mask: [B, S, 1]
                let mask = expert_indices.eq(expert_idx as u32)?.to_dtype(x.dtype())?;
                
                let expert_out = self.experts[expert_idx].forward(x)?;
                let weighted_expert_out = expert_out.broadcast_mul(&mask)?.broadcast_mul(&expert_weights)?;
                
                final_output = (final_output + weighted_expert_out)?;
            }
        }
        
        Ok(final_output)
    }
}
