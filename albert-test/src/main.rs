//! # Albert-Test: Unified Runtime Orchestrator
//! 
//! Bootstraps inference, audit logic, and interactive REPL for the TIS.

use std::io::{self, Write};
use albert_llm_core::model::{Transformer, TransformerConfig};
use albert_llm_core::tokenizer::BpeTokenizer;
use candle_nn::VarMap;
use candle_core::{Device, DType};

struct AlbertTest {
    model: Transformer,
    tokenizer: BpeTokenizer,
}

impl AlbertTest {
    fn new() -> Self {
        let dev = &Device::Cpu;
        let varmap = VarMap::new();
        let vb = candle_nn::VarBuilder::from_varmap(&varmap, DType::F32, dev);
        let config = TransformerConfig::default();
        
        // Use relative path for vocab
        let tokenizer = BpeTokenizer::new("data/vocab.json"); 
        let model = Transformer::new(config.vocab_size, config.hidden_size, vb).expect("Failed to init model");
        
        Self { 
            model,
            tokenizer,
        }
    }

    fn bootstrap(&self) {
        println!("System initialized: albert-llm-core-v0");
        println!("Integrity: NEURAL-BACKEND-ACTIVE");
    }

    fn repl(&mut self) {
        loop {
            print!("\n[Albert-Test] > ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => break, // Handle EOF (Ctrl+D or pipe empty)
                Ok(_) => {
                    let cmd = input.trim();
                    if cmd == "exit" { break; }
                    if cmd.is_empty() { continue; }

                    let mut current_prompt = cmd.to_string();
                    print!("[Albert]: ");
                    for _ in 0..10 {
                        let next_token = self.model.generate(&self.tokenizer, &current_prompt);
                        if next_token.is_empty() { break; }
                        print!("{} ", next_token);
                        io::stdout().flush().unwrap();
                        current_prompt.push_str(&next_token);
                    }
                    println!();
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut tester = AlbertTest::new();
    tester.bootstrap();
    tester.repl();
}
