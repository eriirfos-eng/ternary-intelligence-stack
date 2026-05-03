use candle_core::{Result, Tensor, IndexOp};
use candle_nn::{Module, VarBuilder};

pub struct Transformer {
    embedding: candle_nn::Embedding,
    linear: candle_nn::Linear,
}

impl Transformer {
    pub fn new(vocab_size: usize, hidden_size: usize, vb: VarBuilder) -> Result<Self> {
        let embedding = candle_nn::embedding(vocab_size, hidden_size, vb.pp("embed"))?;
        let linear = candle_nn::linear(hidden_size, vocab_size, vb.pp("out"))?;
        Ok(Self { embedding, linear })
    }

    pub fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x = self.embedding.forward(x)?;
        self.linear.forward(&x)
    }

    pub fn generate(&self, tokenizer: &super::super::tokenizer::BpeTokenizer, prompt: &str) -> String {
        let tokens = tokenizer.encode(prompt);
        if tokens.is_empty() { 
            return "".to_string(); 
        }
        
        let input = Tensor::new(tokens.as_slice(), &candle_core::Device::Cpu).unwrap().unsqueeze(0).unwrap();
        let logits = self.forward(&input).unwrap();
        
        // logits shape: [1, seq_len, vocab_size]
        let dims = logits.dims();
        let seq_len = dims[1];
        
        // Extract the logits for the last token in the sequence
        let last_logits = logits.i((0, seq_len - 1)).unwrap();
        let next_token_idx = last_logits.argmax(0).unwrap().to_scalar::<u32>().unwrap();
        
        tokenizer.decode(&[next_token_idx])
    }
}
