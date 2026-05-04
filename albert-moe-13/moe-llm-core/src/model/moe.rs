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
        // x: [B, S, H]
        let gate_logits = self.gate.forward(x)?; // [B, S, E]
        let weights = candle_nn::ops::softmax(&gate_logits, candle_core::D::Minus1)?;
        
        // Dense MoE for initial prototype: weighted sum of all experts
        let mut output: Option<Tensor> = None;
        for i in 0..self.num_experts {
            let expert_out = self.experts[i].forward(x)?;
            let weight = weights.narrow(2, i, 1)?;
            let weighted_out = expert_out.broadcast_mul(&weight)?;
            
            output = match output {
                None => Some(weighted_out),
                Some(prev) => Some((prev + weighted_out)?),
            };
        }
        
        Ok(output.unwrap())
    }
}
