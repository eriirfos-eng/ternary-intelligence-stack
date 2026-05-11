use candle_core::{Result, Tensor, IndexOp};
use candle_nn::{Module, VarBuilder};
use super::attention::Attention;
use super::moe::MoeBlock;
use super::mlp::Mlp;
use super::config::TransformerConfig;
use super::ternary_linear::TernaryLinear;

pub struct Block {
    attention: Attention,
    moe: Option<MoeBlock>,
    mlp: Option<Mlp>,
    ln1: candle_nn::LayerNorm,
    ln2: candle_nn::LayerNorm,
}

impl Block {
    pub fn new(config: &TransformerConfig, vb: VarBuilder, threshold: f32) -> Result<Self> {
        let attention = Attention::new(config.hidden_size, config.num_heads, vb.pp("attn"), threshold)?;

        let (moe, mlp) = if config.num_experts > 0 {
            let moe = MoeBlock::new(config.hidden_size, config.num_experts, vb.pp("moe"), threshold)?;
            (Some(moe), None)
        } else {
            let mlp = Mlp::new(config.hidden_size, config.hidden_size * 4, vb.pp("mlp"), threshold)?;
            (None, Some(mlp))
        };

        let ln1 = candle_nn::layer_norm(config.hidden_size, 1e-5, vb.pp("ln1"))?;
        let ln2 = candle_nn::layer_norm(config.hidden_size, 1e-5, vb.pp("ln2"))?;
        Ok(Self { attention, moe, mlp, ln1, ln2 })
    }

    pub fn prepare_inference(&self) -> Result<()> {
        self.attention.prepare_inference()?;
        if let Some(moe) = &self.moe { moe.prepare_inference()?; }
        if let Some(mlp) = &self.mlp { mlp.prepare_inference()?; }
        Ok(())
    }

    pub fn freeze_ttl(&self, steps: usize) {
        if let Some(moe) = &self.moe { moe.freeze_ttl(steps); }
    }

    pub fn clear_kv_cache(&self) {
        self.attention.clear_kv_cache();
    }

    // Training / full-sequence forward.
    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x = (x + self.attention.forward(&self.ln1.forward(x)?)?)?;
        self.feed_forward(x)
    }

    // Prefill: process full prompt, populate KV-cache.
    pub fn forward_and_cache(&self, x: &Tensor) -> Result<Tensor> {
        let x = (x + self.attention.forward_and_cache(&self.ln1.forward(x)?)?)?;
        self.feed_forward(x)
    }

    // Decode: single new token, reads from KV-cache.
    pub fn forward_decode(&self, x: &Tensor) -> Result<Tensor> {
        let x = (x + self.attention.forward_decode(&self.ln1.forward(x)?)?)?;
        self.feed_forward(x)
    }

    fn feed_forward(&self, x: Tensor) -> Result<Tensor> {
        if let Some(moe) = &self.moe {
            Ok((&x + moe.forward(&self.ln2.forward(&x)?)?)?)
        } else if let Some(mlp) = &self.mlp {
            Ok((&x + mlp.forward(&self.ln2.forward(&x)?)?)?)
        } else {
            Ok(x)
        }
    }
}

pub struct Transformer {
    embedding: candle_nn::Embedding,
    pos_embedding: candle_nn::Embedding,
    blocks: Vec<Block>,
    ln_f: candle_nn::LayerNorm,
    lm_head: TernaryLinear,
    config: TransformerConfig,
}

impl Transformer {
    pub fn new(config: &TransformerConfig, vb: VarBuilder) -> Result<Self> {
        let embedding = candle_nn::embedding(config.vocab_size, config.hidden_size, vb.pp("embed"))?;
        let pos_embedding = candle_nn::embedding(config.max_seq_len, config.hidden_size, vb.pp("pos_embed"))?;

        let mut blocks = Vec::new();
        let vb_blocks = vb.pp("blocks");
        for i in 0..config.num_layers {
            let layer_threshold = config.layer_threshold(i);
            blocks.push(Block::new(config, vb_blocks.pp(i), layer_threshold)?);
        }

        let ln_f = candle_nn::layer_norm(config.hidden_size, 1e-5, vb.pp("ln_f"))?;
        let threshold = config.threshold;
        let lm_head = TernaryLinear::new(config.hidden_size, config.vocab_size, false, threshold, vb.pp("lm_head"))?;

        Ok(Self { embedding, pos_embedding, blocks, ln_f, lm_head, config: config.clone() })
    }

    /// Pre-ternarize all layer weights + pre-warm the lm_head.
    /// Call once after loading a checkpoint for inference; eliminates per-step re-quantization.
    pub fn prepare_inference(&self) -> Result<()> {
        for block in &self.blocks { block.prepare_inference()?; }
        self.lm_head.prepare_inference()
    }

    /// Freeze TTL logit modifiers on a specific transformer layer for `steps` update() calls.
    /// Called by train_bible on gradient-norm burst detection to allow gate learning.
    pub fn freeze_ttl_layer(&self, layer_idx: usize, steps: usize) {
        if let Some(block) = self.blocks.get(layer_idx) {
            block.freeze_ttl(steps);
        }
    }

    /// Reset all per-layer KV-caches. Call before each new generation sequence.
    pub fn clear_kv_cache(&self) {
        for block in &self.blocks { block.clear_kv_cache(); }
    }

    // Training / evaluation forward — no KV-cache side effects.
    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let seq_len = x.dims()[1];
        let dev = x.device();

        let x_embed = self.embedding.forward(x)?;
        let pos = Tensor::arange(0u32, seq_len as u32, dev)?
            .unsqueeze(0)?.to_dtype(candle_core::DType::U32)?;
        let pos_embed = self.pos_embedding.forward(&pos)?;

        let mut x = x_embed.broadcast_add(&pos_embed)?;
        for block in &self.blocks { x = block.forward(&x)?; }
        x = self.ln_f.forward(&x)?;
        self.lm_head.forward(&x)
    }

    /// Prefill: process full prompt through all layers, populate KV-cache in each attention block.
    /// Returns full logit tensor [1, seq_len, vocab]; caller takes the last position.
    pub fn forward_prefill(&self, x: &Tensor) -> Result<Tensor> {
        let seq_len = x.dims()[1];
        let dev = x.device();

        let x_embed = self.embedding.forward(x)?;
        let pos = Tensor::arange(0u32, seq_len as u32, dev)?
            .unsqueeze(0)?.to_dtype(candle_core::DType::U32)?;
        let pos_embed = self.pos_embedding.forward(&pos)?;

        let mut h = x_embed.broadcast_add(&pos_embed)?;
        for block in &self.blocks { h = block.forward_and_cache(&h)?; }
        h = self.ln_f.forward(&h)?;
        self.lm_head.forward(&h)
    }

    /// Decode: single new token. Uses KV-cache — O(1) per layer instead of O(seq_len).
    /// `token` is the last generated token; `seq_pos` is its absolute position index.
    /// Returns logits [1, 1, vocab]; caller takes [0, 0].
    pub fn forward_decode(&self, token: u32, seq_pos: usize, dev: &candle_core::Device) -> Result<Tensor> {
        let pos = seq_pos.min(self.config.max_seq_len - 1) as u32;

        let x = Tensor::new(&[token], dev)?
            .unsqueeze(0)?.to_dtype(candle_core::DType::U32)?;
        let x_embed = self.embedding.forward(&x)?;

        let pos_t = Tensor::new(&[pos], dev)?
            .unsqueeze(0)?.to_dtype(candle_core::DType::U32)?;
        let pos_embed = self.pos_embedding.forward(&pos_t)?;

        let mut h = x_embed.broadcast_add(&pos_embed)?;
        for block in &self.blocks { h = block.forward_decode(&h)?; }
        h = self.ln_f.forward(&h)?;
        self.lm_head.forward(&h)
    }

    pub fn generate(&self, tokenizer: &super::super::tokenizer::BpeTokenizer, prompt: &str, max_new_tokens: usize) -> String {
        let mut tokens = tokenizer.encode(prompt);
        if tokens.is_empty() { return "".to_string(); }

        let dev = candle_core::Device::Cpu;
        let mut rng = rand::thread_rng();

        for _ in 0..max_new_tokens {
            let start_idx = tokens.len().saturating_sub(self.config.max_seq_len);
            let context = &tokens[start_idx..];

            let input = Tensor::new(&context[..], &dev).unwrap()
                .unsqueeze(0).unwrap().to_dtype(candle_core::DType::U32).unwrap();
            let logits = self.forward(&input).unwrap();

            let dims = logits.dims();
            let last_logits = logits.i((0, dims[1] - 1)).unwrap();

            let probs = candle_nn::ops::softmax(&last_logits, 0).unwrap();
            let pr = probs.to_vec1::<f32>().unwrap();

            use rand::distributions::{Distribution, WeightedIndex};
            let dist = WeightedIndex::new(&pr).unwrap();
            let next_token_idx = dist.sample(&mut rng) as u32;
            tokens.push(next_token_idx);
        }

        tokenizer.decode(&tokens)
    }
}
