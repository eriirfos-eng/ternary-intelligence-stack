pub struct TransformerConfig {
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub num_layers: usize,
    pub num_heads: usize,
    pub max_seq_len: usize,
}

impl Default for TransformerConfig {
    fn default() -> Self {
        Self {
            vocab_size: 8000,
            hidden_size: 128,
            num_layers: 2,
            num_heads: 4,
            max_seq_len: 128,
        }
    }
}
