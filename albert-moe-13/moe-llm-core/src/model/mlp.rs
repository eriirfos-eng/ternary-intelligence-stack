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

    /// Average mean-abs of F32 shadow weights across both linear layers.
    /// Tracks expert divergence in weight space for H3 (STE swallowing) diagnostic.
    pub fn f32_weight_signal(&self) -> Result<f32> {
        Ok((self.c_fc.weight_mean_abs()? + self.c_proj.weight_mean_abs()?) / 2.0)
    }

    /// Differentiable version: returns (c_fc_mean_abs + c_proj_mean_abs) / 2 as a Tensor.
    /// Gradient flows directly to F32 shadow weights without passing through STE.
    pub fn f32_weight_signal_tensor(&self) -> Result<Tensor> {
        let fc   = self.c_fc.weight_mean_abs_tensor()?;
        let proj = self.c_proj.weight_mean_abs_tensor()?;
        (fc + proj)? * 0.5_f64
    }

pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x = self.c_fc.forward(x)?;
        let x = x.gelu()?;
        self.c_proj.forward(&x)
    }
}
