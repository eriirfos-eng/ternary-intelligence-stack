use candle_core::{Result, Tensor};
use candle_nn::VarBuilder;
use super::ste::ternarize_ste;

pub struct TernaryLinear {
    weight: Tensor,
    bias: Option<Tensor>,
    threshold: f32,
}

impl TernaryLinear {
    pub fn new(in_dim: usize, out_dim: usize, bias: bool, threshold: f32, vb: VarBuilder) -> Result<Self> {
        let weight = vb.get_with_hints((out_dim, in_dim), "weight", candle_nn::Init::Uniform { lo: -0.05, up: 0.05 })?;
        let bias = if bias {
            Some(vb.get_with_hints(out_dim, "bias", candle_nn::Init::Const(0.0))?)
        } else {
            None
        };
        Ok(Self { weight, bias, threshold })
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let w_ternary = ternarize_ste(&self.weight, self.threshold)?;
        let dims = x.dims();
        if dims.len() == 3 {
            let (b, s, h) = (dims[0], dims[1], dims[2]);
            let x = x.reshape((b * s, h))?;
            let x = x.matmul(&w_ternary.t()?)?;
            let x = x.reshape((b, s, x.dims()[1]))?;
            match &self.bias {
                None => Ok(x),
                Some(bias) => x.broadcast_add(bias),
            }
        } else {
            let x = x.matmul(&w_ternary.t()?)?;
            match &self.bias {
                None => Ok(x),
                Some(bias) => x.broadcast_add(bias),
            }
        }
    }
}
