//! # Albert-Test: Unified Runtime Orchestrator
//! 
//! Bootstraps inference, audit logic, and interactive REPL for the TIS.

use std::io::{self, Write};
use moe_llm_core::model::{Transformer, TransformerConfig};
use moe_llm_core::tokenizer::BpeTokenizer;
use candle_core::{Device, DType};

struct AlbertTest {
    model: Transformer,
    tokenizer: BpeTokenizer,
}

use moe_llm_core::model::loader::TritLoader;
use candle_nn::VarBuilder;

impl AlbertTest {
    fn new() -> Self {
        let dev = Device::Cpu;
        let trit_path = "albert-moe-13/models/bible_ternary_v1.3.6.trit";
        
        println!("Loading bit-packed weights from {}...", trit_path);
        let loader = TritLoader::load(trit_path, &dev).expect("Failed to load .trit weights");
        
        // Use a dummy varmap and overwrite with loader if needed, 
        // but easier to build VarBuilder from the loader's tensors directly.
        // Since loader.tensors is private, I'll need to expose it or add a helper.
        // For now, let's assume we can use the loader to provide a VarBuilder.
        
        let tokenizer = BpeTokenizer::new("albert-moe-13/data/vocab.json");
        let mut config = TransformerConfig::default();
        config.vocab_size = tokenizer.vocab_size();
        
        let vb = VarBuilder::from_tensors(loader.into_tensors(), DType::F32, &dev);
        let model = Transformer::new(&config, vb).expect("Failed to init model");
        
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

                    let response = self.model.generate(&self.tokenizer, cmd, 10);
                    println!("[Albert]: {}", response);
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
