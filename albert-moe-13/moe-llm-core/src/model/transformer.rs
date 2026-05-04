use candle_core::{Result, Tensor, IndexOp};
use candle_nn::{Module, VarBuilder};
use super::attention::Attention;
use super::moe::MoeBlock;
use super::config::TransformerConfig;
use super::ternary_linear::TernaryLinear;

pub struct Block {
    attention: Attention,
    moe: MoeBlock,
    ln1: candle_nn::LayerNorm,
    ln2: candle_nn::LayerNorm,
}

impl Block {
    pub fn new(config: &TransformerConfig, vb: VarBuilder) -> Result<Self> {
        let attention = Attention::new(config.hidden_size, config.num_heads, vb.pp("attn"), config.threshold)?;
        let moe = MoeBlock::new(config.hidden_size, config.num_experts, vb.pp("moe"), config.threshold)?;
        let ln1 = candle_nn::layer_norm(config.hidden_size, 1e-5, vb.pp("ln1"))?;
        let ln2 = candle_nn::layer_norm(config.hidden_size, 1e-5, vb.pp("ln2"))?;
        Ok(Self { attention, moe, ln1, ln2 })
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x = (x + self.attention.forward(&self.ln1.forward(x)?)?)?;
        let x = (&x + self.moe.forward(&self.ln2.forward(&x)?)?)?;
        Ok(x)
    }
}

pub struct Transformer {
    embedding: candle_nn::Embedding,
    pos_embedding: candle_nn::Embedding,
    blocks: Vec<Block>,
    ln_f: candle_nn::LayerNorm,
    lm_head: TernaryLinear,
}

impl Transformer {
    pub fn new(config: &TransformerConfig, vb: VarBuilder) -> Result<Self> {
        let embedding = candle_nn::embedding(config.vocab_size, config.hidden_size, vb.pp("embed"))?;
        let pos_embedding = candle_nn::embedding(config.max_seq_len, config.hidden_size, vb.pp("pos_embed"))?;
        
        let mut blocks = Vec::new();
        let vb_blocks = vb.pp("blocks");
        for i in 0..config.num_layers {
            blocks.push(Block::new(config, vb_blocks.pp(i))?);
        }

        let ln_f = candle_nn::layer_norm(config.hidden_size, 1e-5, vb.pp("ln_f"))?;
        let threshold = config.threshold;
        let lm_head = TernaryLinear::new(config.hidden_size, config.vocab_size, false, threshold, vb.pp("lm_head"))?;
        
        Ok(Self { 
            embedding, 
            pos_embedding,
            blocks, 
            ln_f, 
            lm_head 
        })
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let seq_len = x.dims()[1];
        let dev = x.device();
        
        let x_embed = self.embedding.forward(x)?;
        
        let pos = Tensor::arange(0u32, seq_len as u32, dev)?.unsqueeze(0)?;
        let pos_embed = self.pos_embedding.forward(&pos)?;
        
        let mut x = x_embed.broadcast_add(&pos_embed)?;
        
        for block in &self.blocks {
            x = block.forward(&x)?;
        }
        x = self.ln_f.forward(&x)?;
        self.lm_head.forward(&x)
    }

    pub fn generate(&self, tokenizer: &super::super::tokenizer::BpeTokenizer, prompt: &str, max_new_tokens: usize) -> String {
        let mut tokens = tokenizer.encode(prompt);
        if tokens.is_empty() { 
            return "".to_string(); 
        }

        let dev = candle_core::Device::Cpu;
        let mut rng = rand::thread_rng();

        for _ in 0..max_new_tokens {
            // Only use the last max_seq_len tokens
            let start_idx = tokens.len().saturating_sub(64);
            let context = &tokens[start_idx..];
            
            let input = Tensor::new(context, &dev).unwrap().unsqueeze(0).unwrap();
            let logits = self.forward(&input).unwrap();
            
            let dims = logits.dims();
            let seq_len = dims[1];
            
            // Get logits for the last token
            let last_logits = logits.i((0, seq_len - 1)).unwrap();
            
            // Temperature sampling (simple version: add some noise or just use softmax)
            // For now, let's just do top-k or simple random sample if we want "conversations"
            // But greedy argmax is safer for tiny models.
            // Let's at least avoid index 0 ([UNK]) if possible
            let probs = candle_nn::ops::softmax(&last_logits, 0).unwrap();
            let pr = probs.to_vec1::<f32>().unwrap();
            
            // Sample from pr
            use rand::distributions::{Distribution, WeightedIndex};
            let dist = WeightedIndex::new(&pr).unwrap();
            let next_token_idx = dist.sample(&mut rng) as u32;
            
            tokens.push(next_token_idx);
            
            // Stop if we hit a stop token (if we had one)
        }
        
        tokenizer.decode(&tokens)
    }
}
