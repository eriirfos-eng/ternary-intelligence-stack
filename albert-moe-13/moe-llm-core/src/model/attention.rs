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
    // KV-cache for autoregressive decode: (k, v) shaped [1, heads, seq, head_dim].
    // Populated by forward_and_cache() (prefill); grown by forward_decode() each step.
    kv_cache: RefCell<Option<(Tensor, Tensor)>>,
}

impl Attention {
    pub fn new(hidden_size: usize, num_heads: usize, vb: VarBuilder, threshold: f32) -> Result<Self> {
        let head_dim = hidden_size / num_heads;
        let q_proj = TernaryLinear::new(hidden_size, hidden_size, false, threshold, vb.pp("q_proj"))?;
        let k_proj = TernaryLinear::new(hidden_size, hidden_size, false, threshold, vb.pp("k_proj"))?;
        let v_proj = TernaryLinear::new(hidden_size, hidden_size, false, threshold, vb.pp("v_proj"))?;
        let o_proj = TernaryLinear::new(hidden_size, hidden_size, false, threshold, vb.pp("o_proj"))?;
        Ok(Self {
            q_proj, k_proj, v_proj, o_proj,
            num_heads, head_dim,
            mask_cache: RefCell::new(None),
            kv_cache: RefCell::new(None),
        })
    }

    pub fn prepare_inference(&self) -> Result<()> {
        self.q_proj.prepare_inference()?;
        self.k_proj.prepare_inference()?;
        self.v_proj.prepare_inference()?;
        self.o_proj.prepare_inference()
    }

    pub fn clear_kv_cache(&self) {
        *self.kv_cache.borrow_mut() = None;
    }

    // Training / full-sequence forward (no KV-cache side effects).
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

    // Prefill: full prompt, populates KV-cache for subsequent decode steps.
    pub fn forward_and_cache(&self, x: &Tensor) -> Result<Tensor> {
        let (b_sz, seq_len, h_sz) = x.dims3()?;

        let q = self.q_proj.forward(x)?;
        let k = self.k_proj.forward(x)?;
        let v = self.v_proj.forward(x)?;

        let q = q.reshape((b_sz, seq_len, self.num_heads, self.head_dim))?.transpose(1, 2)?;
        let k = k.reshape((b_sz, seq_len, self.num_heads, self.head_dim))?.transpose(1, 2)?;
        let v = v.reshape((b_sz, seq_len, self.num_heads, self.head_dim))?.transpose(1, 2)?;

        *self.kv_cache.borrow_mut() = Some((k.clone(), v.clone()));

        let mask = self.get_mask(seq_len, x.device())?;
        let mut attn_weights = (q.matmul(&k.transpose(2, 3)?)? / (self.head_dim as f64).sqrt())?;
        attn_weights = attn_weights.broadcast_add(&mask)?;
        let attn_weights = candle_nn::ops::softmax(&attn_weights, D::Minus1)?;

        let attn_output = attn_weights.matmul(&v)?;
        let attn_output = attn_output.transpose(1, 2)?.reshape((b_sz, seq_len, h_sz))?;
        self.o_proj.forward(&attn_output)
    }

    // Decode: single new token. Only computes Q for the new token; K/V come from cache + new.
    // No causal mask needed — single Q token attends over all cached (past) positions.
    pub fn forward_decode(&self, x: &Tensor) -> Result<Tensor> {
        let (b_sz, seq_len, h_sz) = x.dims3()?;

        let q     = self.q_proj.forward(x)?;
        let k_new = self.k_proj.forward(x)?;
        let v_new = self.v_proj.forward(x)?;

        let q     = q    .reshape((b_sz, seq_len, self.num_heads, self.head_dim))?.transpose(1, 2)?;
        let k_new = k_new.reshape((b_sz, seq_len, self.num_heads, self.head_dim))?.transpose(1, 2)?;
        let v_new = v_new.reshape((b_sz, seq_len, self.num_heads, self.head_dim))?.transpose(1, 2)?;

        let (k, v) = {
            let cache = self.kv_cache.borrow();
            match *cache {
                Some((ref k_c, ref v_c)) => (
                    Tensor::cat(&[k_c, &k_new], 2)?,
                    Tensor::cat(&[v_c, &v_new], 2)?,
                ),
                None => (k_new.clone(), v_new.clone()),
            }
        };
        *self.kv_cache.borrow_mut() = Some((k.clone(), v.clone()));

        let scale = (self.head_dim as f64).sqrt();
        let attn_weights = candle_nn::ops::softmax(
            &(q.matmul(&k.transpose(2, 3)?)? / scale)?,
            D::Minus1,
        )?;

        let attn_output = attn_weights.matmul(&v)?;
        let attn_output = attn_output.transpose(1, 2)?.reshape((b_sz, seq_len, h_sz))?;
        self.o_proj.forward(&attn_output)
    }

    fn get_mask(&self, seq_len: usize, device: &candle_core::Device) -> Result<Tensor> {
        let mut cache = self.mask_cache.borrow_mut();
        if let Some((cached_len, ref mask)) = *cache {
            if cached_len == seq_len { return Ok(mask.clone()); }
        }
        let mask_data: Vec<f32> = (0..seq_len)
            .flat_map(|i| (0..seq_len).map(move |j| if j > i { f32::NEG_INFINITY } else { 0f32 }))
            .collect();
        let mask = Tensor::from_slice(&mask_data, (seq_len, seq_len), device)?
            .unsqueeze(0)?.unsqueeze(0)?;
        *cache = Some((seq_len, mask.clone()));
        Ok(mask)
    }
}
