// TernaryLinear: weight-scaled ternary matmul with cached gamma — whitepaper §5.1
// @sparseskip element-level: SparseCache skips all zero-weight positions — whitepaper §5.2
use candle_core::{Result, Tensor};
use candle_nn::VarBuilder;
use super::ste::ternarize_ste_with_gamma;
use std::cell::RefCell;

/// Pre-computed sparse index representation of a ternary weight matrix.
/// pos_indices[i] = input positions where weight row i is +gamma (add)
/// neg_indices[i] = input positions where weight row i is -gamma (subtract)
/// Zero-weight positions are absent — they are never touched during matmul.
struct SparseCache {
    pos_indices: Vec<Vec<u16>>,
    neg_indices: Vec<Vec<u16>>,
    gamma: f32,
    out_dim: usize,
    bias: Option<Vec<f32>>,
}

pub struct TernaryLinear {
    weight: Tensor,
    bias: Option<Tensor>,
    threshold: f32,
    // Cached gamma: (call_count, cached_scalar_tensor).
    // Recomputed every GAMMA_REFRESH calls; stable enough since weights change slowly.
    gamma_cache: RefCell<(u32, Option<Tensor>)>,
    // Dense inference cache: pre-ternarized F32 tensor, avoids re-quantizing each step.
    inference_cache: RefCell<Option<Tensor>>,
    // Sparse inference cache: @sparseskip element-level — indices of non-zero weights only.
    // Populated by prepare_inference(); replaces dense path for all inference forwards.
    sparse_cache: RefCell<Option<SparseCache>>,
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
            sparse_cache: RefCell::new(None),
        })
    }

    /// Mean absolute value of F32 shadow weights — used to track expert divergence in
    /// weight space independently of ternary quantization. Diagnostic for H3 (STE swallowing).
    pub fn weight_mean_abs(&self) -> Result<f32> {
        self.weight.abs()?.mean_all()?.to_scalar::<f32>()
    }

    /// Differentiable version: returns mean-abs as a gradient-carrying Tensor.
    /// Used by the F32-direct divergence loss to bypass STE and create differential
    /// gradient per expert based on weight-space position, not ternary output.
    pub fn weight_mean_abs_tensor(&self) -> Result<Tensor> {
        self.weight.abs()?.mean_all()
    }

/// Pre-ternarize weights and build sparse index tables for @sparseskip inference.
    /// The dense cache is kept as fallback; the sparse cache is the primary inference path.
    pub fn prepare_inference(&self) -> Result<()> {
        let gamma_t = self.weight.abs()?.mean_all()?;
        let gamma   = gamma_t.to_scalar::<f32>()?;
        let w_ternary = ternarize_ste_with_gamma(&self.weight, self.threshold, &gamma_t)?;

        // Dense fallback cache (used if sparse build fails)
        *self.inference_cache.borrow_mut() = Some(w_ternary.detach());

        // Build sparse indices — @sparseskip element level
        let w_data = self.weight.to_vec2::<f32>()?;
        let out_dim = w_data.len();
        let thr = self.threshold;
        let mut pos_indices: Vec<Vec<u16>> = Vec::with_capacity(out_dim);
        let mut neg_indices: Vec<Vec<u16>> = Vec::with_capacity(out_dim);
        for row in &w_data {
            pos_indices.push(
                row.iter().enumerate()
                    .filter(|(_, w)| **w > thr)
                    .map(|(j, _)| j as u16)
                    .collect(),
            );
            neg_indices.push(
                row.iter().enumerate()
                    .filter(|(_, w)| **w < -thr)
                    .map(|(j, _)| j as u16)
                    .collect(),
            );
        }

        let bias_vec = if let Some(ref b) = self.bias {
            Some(b.to_vec1::<f32>()?)
        } else {
            None
        };

        let zero_pct = pos_indices.iter().chain(neg_indices.iter())
            .map(|v| v.len()).sum::<usize>();
        let total = w_data.len() * w_data.first().map(|r| r.len()).unwrap_or(0);
        let nonzero_pct = if total > 0 { zero_pct * 100 / total } else { 0 };
        let _ = nonzero_pct; // logged externally via TELE sparsity

        *self.sparse_cache.borrow_mut() = Some(SparseCache {
            pos_indices,
            neg_indices,
            gamma,
            out_dim,
            bias: bias_vec,
        });
        Ok(())
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        // @sparseskip path: element-level zero skip — inference only, no backward pass needed
        if let Some(ref sparse) = *self.sparse_cache.borrow() {
            return self.forward_sparse(x, sparse);
        }

        // Training path: dense ternary matmul with STE (gradient flows through F32 weights)
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

    /// Sparse forward: skip all zero-weight positions — the CPU realisation of @sparseskip.
    /// For a 256×256 weight at 56% sparsity: 44% of multiplications, all cache-hot (x fits L1).
    fn forward_sparse(&self, x: &Tensor, sparse: &SparseCache) -> Result<Tensor> {
        let original_dims = x.dims().to_vec();
        let in_dim  = *original_dims.last().unwrap();
        let bs: usize = original_dims[..original_dims.len() - 1].iter().product();

        // Pull activations into a flat Vec — x is tiny ([1,1,256]) in decode mode, O(µs) copy
        let x_flat = x.reshape((bs, in_dim))?.to_vec2::<f32>()?;

        let out_dim = sparse.out_dim;
        let gamma   = sparse.gamma;
        let mut out = vec![0.0f32; bs * out_dim];

        for b in 0..bs {
            let row = &x_flat[b];
            for i in 0..out_dim {
                let mut acc = 0.0f32;
                // Add: weight = +gamma
                for &j in &sparse.pos_indices[i] { acc += row[j as usize]; }
                // Subtract: weight = -gamma
                for &j in &sparse.neg_indices[i] { acc -= row[j as usize]; }
                out[b * out_dim + i] = acc * gamma;
            }
        }

        // Bias add
        if let Some(ref bias) = sparse.bias {
            for b in 0..bs {
                for i in 0..out_dim {
                    out[b * out_dim + i] += bias[i];
                }
            }
        }

        // Reshape to original leading dims + out_dim
        let mut out_shape = original_dims[..original_dims.len() - 1].to_vec();
        out_shape.push(out_dim);
        Tensor::from_vec(out, out_shape.as_slice(), x.device())
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
