use candle_core::{Result, Tensor};
use candle_nn::VarBuilder;
use super::ternary_linear::TernaryLinear;

pub struct Attention {
    q_proj: TernaryLinear,
    k_proj: TernaryLinear,
    v_proj: TernaryLinear,
    head_dim: usize,
}

impl Attention {
    pub fn new(hidden_size: usize, num_heads: usize, vb: VarBuilder, threshold: f32) -> Result<Self> {
        let head_dim = hidden_size / num_heads;
        let q_proj = TernaryLinear::new(hidden_size, hidden_size, false, threshold, vb.pp("q_proj"))?;
        let k_proj = TernaryLinear::new(hidden_size, hidden_size, false, threshold, vb.pp("k_proj"))?;
        let v_proj = TernaryLinear::new(hidden_size, hidden_size, false, threshold, vb.pp("v_proj"))?;
        Ok(Self { q_proj, k_proj, v_proj, head_dim })
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let q = self.q_proj.forward(x)?;
        let k = self.k_proj.forward(x)?;
        let v = self.v_proj.forward(x)?;
        let attn = q.matmul(&k.transpose(1, 2)?)?;
        let attn = (attn / (self.head_dim as f64).sqrt())?;
        let attn = candle_nn::ops::softmax(&attn, candle_core::D::Minus1)?;
        attn.matmul(&v)
    }
}
