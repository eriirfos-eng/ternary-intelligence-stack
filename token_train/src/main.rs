use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    // Relative to workspace root if run from there, or ../ if run from here
    let corpus_path = Path::new("../albert-moe-13/data/corpus/bible.txt");
    
    if !corpus_path.exists() {
        eprintln!("Error: Bible corpus not found at {:?}", corpus_path);
        std::process::exit(1);
    }

    println!("Generating word-frequency vocabulary from Bible corpus...");
    let content = fs::read_to_string(corpus_path).expect("Failed to read corpus");
    
    let mut counts = HashMap::new();
    for word in content.split_whitespace() {
        let word = word.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "");
        if !word.is_empty() {
            *counts.entry(word).or_insert(0) += 1;
        }
    }
    
    let mut words: Vec<_> = counts.into_iter().collect();
    words.sort_by(|a, b| b.1.cmp(&a.1));
    
    let top_words: Vec<_> = words.into_iter().take(7995).map(|(w, _)| w).collect();
    
    let mut vocab = HashMap::new();
    vocab.insert("[UNK]".to_string(), 0);
    vocab.insert("[CLS]".to_string(), 1);
    vocab.insert("[SEP]".to_string(), 2);
    vocab.insert("[PAD]".to_string(), 3);
    vocab.insert("[MASK]".to_string(), 4);
    
    for (i, word) in top_words.into_iter().enumerate() {
        vocab.insert(word, (i + 5) as u32);
    }
    
    let data = serde_json::json!({
        "version": "1.0",
        "truncation": null,
        "padding": null,
        "added_tokens": [],
        "normalizer": null,
        "pre_tokenizer": { "type": "Whitespace" },
        "post_processor": null,
        "decoder": null,
        "model": {
            "type": "WordLevel",
            "vocab": vocab,
            "unk_token": "[UNK]"
        }
    });
    
    let out_dir = Path::new("data");
    fs::create_dir_all(out_dir).expect("Failed to create data directory");
    let out_path = out_dir.join("vocab.json");
    fs::write(&out_path, serde_json::to_string_pretty(&data).unwrap()).expect("Failed to write vocab.json");
    
    println!("Vocabulary successfully generated and saved to {:?}", out_path);
}
