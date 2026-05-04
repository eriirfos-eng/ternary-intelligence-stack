use tokenizers::Tokenizer;

pub struct BpeTokenizer {
    tokenizer: Tokenizer,
}

impl BpeTokenizer {
    pub fn new(vocab_path: &str) -> Self {
        // Simple fallback: if file doesn't exist, try to initialize manually
        // For minimal implementation, we'll assume a path or fail gracefully
        let tokenizer = Tokenizer::from_file(vocab_path)
            .expect("Tokenizer file not found at provided path");
        Self { tokenizer }
    }

    pub fn encode(&self, text: &str) -> Vec<u32> {
        self.tokenizer.encode(text, true).unwrap().get_ids().to_vec()
    }

    pub fn decode(&self, ids: &[u32]) -> String {
        self.tokenizer.decode(ids, true).unwrap()
    }

    pub fn vocab_size(&self) -> usize {
        self.tokenizer.get_vocab_size(true)
    }
}
