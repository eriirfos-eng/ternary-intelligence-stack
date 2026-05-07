use candle_core::{Result, Tensor};
use candle_nn::VarBuilder;
use super::ternary_linear::TernaryLinear;

pub struct Mlp {
    c_fc: TernaryLinear,
    c_proj: TernaryLinear,
}

impl Mlp {
    pub fn new(hidden_size: usize, intermediate_size: usize, vb: VarBuilder, threshold: f32) -> Result<Self> {
        let c_fc = TernaryLinear::new(hidden_size, intermediate_size, true, threshold, vb.pp("c_fc"))?;
        let c_proj = TernaryLinear::new(intermediate_size, hidden_size, true, threshold, vb.pp("c_proj"))?;
        Ok(Self { c_fc, c_proj })
    }

    pub fn prepare_inference(&self) -> Result<()> {
        self.c_fc.prepare_inference()?;
        self.c_proj.prepare_inference()
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x = self.c_fc.forward(x)?;
        let x = x.gelu()?;
        self.c_proj.forward(&x)
    }
}
