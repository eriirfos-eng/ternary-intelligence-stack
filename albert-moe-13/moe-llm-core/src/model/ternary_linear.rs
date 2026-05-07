// TernaryLinear: weight-scaled ternary matmul with cached gamma — whitepaper §5.1
use candle_core::{Result, Tensor};
use candle_nn::VarBuilder;
use super::ste::ternarize_ste_with_gamma;
use std::cell::RefCell;

pub struct TernaryLinear {
    weight: Tensor,
    bias: Option<Tensor>,
    threshold: f32,
    // Cached gamma: (call_count, cached_scalar_tensor).
    // Recomputed every GAMMA_REFRESH calls; stable enough since weights change slowly.
    gamma_cache: RefCell<(u32, Option<Tensor>)>,
    // Inference-mode pre-ternarized weight cache (None = training mode).
    // Populated once by prepare_inference(); avoids re-quantizing on every forward pass.
    inference_cache: RefCell<Option<Tensor>>,
}

const GAMMA_REFRESH: u32 = 20;

impl TernaryLinear {
    pub fn new(in_dim: usize, out_dim: usize, bias: bool, threshold: f32, vb: VarBuilder) -> Result<Self> {
        let weight = vb.get_with_hints(
            (out_dim, in_dim), "weight",
            candle_nn::Init::Uniform { lo: -0.05, up: 0.05 }
        )?;
        let bias = if bias {
            Some(vb.get_with_hints(out_dim, "bias", candle_nn::Init::Const(0.0))?)
        } else {
            None
        };
        Ok(Self {
            weight,
            bias,
            threshold,
            gamma_cache: RefCell::new((0, None)),
            inference_cache: RefCell::new(None),
        })
    }

    /// Pre-ternarize weights once for inference (no backward pass, no STE graph).
    /// Call on all layers after model load; avoids re-quantizing on every forward pass.
    pub fn prepare_inference(&self) -> Result<()> {
        let gamma = self.weight.abs()?.mean_all()?;
        let w_ternary = ternarize_ste_with_gamma(&self.weight, self.threshold, &gamma)?;
        *self.inference_cache.borrow_mut() = Some(w_ternary.detach());
        Ok(())
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let w_ternary = {
            let cache = self.inference_cache.borrow();
            match *cache {
                Some(ref w) => w.clone(),
                None => {
                    let gamma = self.get_gamma()?;
                    ternarize_ste_with_gamma(&self.weight, self.threshold, &gamma)?
                }
            }
        };

        let dims = x.dims();
        let out = if dims.len() == 3 {
            let (b, s, h) = (dims[0], dims[1], dims[2]);
            let x2 = x.reshape((b * s, h))?;
            let x2 = x2.matmul(&w_ternary.t()?)?;
            x2.reshape((b, s, x2.dims()[1]))?
        } else {
            x.matmul(&w_ternary.t()?)?
        };

        match &self.bias {
            None       => Ok(out),
            Some(bias) => out.broadcast_add(bias),
        }
    }

    fn get_gamma(&self) -> Result<Tensor> {
        let mut cache = self.gamma_cache.borrow_mut();
        let (count, ref mut stored) = *cache;
        if count % GAMMA_REFRESH == 0 || stored.is_none() {
            // γ = E[|W|] — the per-layer scaling factor from §5.1 Eq. (1).
            let g = self.weight.abs()?.mean_all()?;
            *stored = Some(g.detach()); // detach: gamma is a stat, not part of the graph
        }
        cache.0 = cache.0.wrapping_add(1);
        Ok(cache.1.as_ref().unwrap().clone())
    }
}
