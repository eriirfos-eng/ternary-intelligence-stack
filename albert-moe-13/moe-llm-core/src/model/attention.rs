use candle_core::{Result, Tensor, D};
use candle_nn::VarBuilder;
use super::ternary_linear::TernaryLinear;
use std::cell::RefCell;

pub struct Attention {
    q_proj: TernaryLinear,
    k_proj: TernaryLinear,
    v_proj: TernaryLinear,
    o_proj: TernaryLinear,
    num_heads: usize,
    head_dim: usize,
    // Cached causal mask: (seq_len, Tensor). Rebuilt only when seq_len changes.
    mask_cache: RefCell<Option<(usize, Tensor)>>,
}

impl Attention {
    pub fn new(hidden_size: usize, num_heads: usize, vb: VarBuilder, threshold: f32) -> Result<Self> {
        let head_dim = hidden_size / num_heads;
        let q_proj = TernaryLinear::new(hidden_size, hidden_size, false, threshold, vb.pp("q_proj"))?;
        let k_proj = TernaryLinear::new(hidden_size, hidden_size, false, threshold, vb.pp("k_proj"))?;
        let v_proj = TernaryLinear::new(hidden_size, hidden_size, false, threshold, vb.pp("v_proj"))?;
        let o_proj = TernaryLinear::new(hidden_size, hidden_size, false, threshold, vb.pp("o_proj"))?;
        Ok(Self {
            q_proj,
            k_proj,
            v_proj,
            o_proj,
            num_heads,
            head_dim,
            mask_cache: RefCell::new(None),
        })
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let (b_sz, seq_len, h_sz) = x.dims3()?;

        let q = self.q_proj.forward(x)?;
        let k = self.k_proj.forward(x)?;
        let v = self.v_proj.forward(x)?;

        let q = q.reshape((b_sz, seq_len, self.num_heads, self.head_dim))?.transpose(1, 2)?;
        let k = k.reshape((b_sz, seq_len, self.num_heads, self.head_dim))?.transpose(1, 2)?;
        let v = v.reshape((b_sz, seq_len, self.num_heads, self.head_dim))?.transpose(1, 2)?;

        let mask = self.get_mask(seq_len, x.device())?;

        let mut attn_weights = (q.matmul(&k.transpose(2, 3)?)? / (self.head_dim as f64).sqrt())?;
        attn_weights = attn_weights.broadcast_add(&mask)?;
        let attn_weights = candle_nn::ops::softmax(&attn_weights, D::Minus1)?;

        let attn_output = attn_weights.matmul(&v)?;
        let attn_output = attn_output.transpose(1, 2)?.reshape((b_sz, seq_len, h_sz))?;

        self.o_proj.forward(&attn_output)
    }

    fn get_mask(&self, seq_len: usize, device: &candle_core::Device) -> Result<Tensor> {
        let mut cache = self.mask_cache.borrow_mut();
        if let Some((cached_len, ref mask)) = *cache {
            if cached_len == seq_len {
                return Ok(mask.clone());
            }
        }
        let mask_data: Vec<f32> = (0..seq_len)
            .flat_map(|i| (0..seq_len).map(move |j| if j > i { f32::NEG_INFINITY } else { 0f32 }))
            .collect();
        let mask = Tensor::from_slice(&mask_data, (seq_len, seq_len), device)?
            .unsqueeze(0)?
            .unsqueeze(0)?;
        *cache = Some((seq_len, mask.clone()));
        Ok(mask)
    }
}
