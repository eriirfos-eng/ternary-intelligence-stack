// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Example: RuVector Enterprise Bridge Demo

use ternlang_ruvector::RuVectorDB;

fn main() -> anyhow::Result<()> {
    println!("=== RuVector Enterprise Bridge Demo ===");
    
    // Simulated embeddings from a Vector DB (e.g. OpenAI text-embedding-3-small)
    let docs = vec![
        "The Ternary Intelligence Stack uses balanced ternary logic (-1, 0, +1).",
        "Binary systems are limited to 0 and 1, missing the deliberative hold state.",
        "RAG pipelines benefit from Sparse GEMV acceleration in high-latency environments.",
        "Albert is the primary agent in the Ternary Intelligence Stack.",
    ];
    
    // Mock f32 embeddings (128-dim)
    let mut doc_embeddings = Vec::new();
    for i in 0..docs.len() {
        let mut v = vec![0.0f32; 128];
        // Create some distinct patterns
        for j in 0..128 {
            v[j] = ((i + j) % 7) as f32 / 3.0 - 1.0;
        }
        doc_embeddings.push(v);
    }
    
    println!("Indexing {} documents into RuVectorDB...", docs.len());
    let db = RuVectorDB::from_f32(&doc_embeddings, docs.iter().map(|s| s.to_string()).collect())?;
    
    // Mock query embedding
    let mut query = vec![0.0f32; 128];
    for j in 0..128 {
        query[j] = (j % 5) as f32 / 2.0 - 1.0;
    }
    
    println!("Searching for top 2 matches using ternary Sparse GEMV...");
    let results = db.search(&query, 2);
    
    for (i, res) in results.iter().enumerate() {
        println!("{}. [Score: {:.2}] {}", i + 1, res.score, res.metadata);
    }
    
    println!("\nRuVector Bridge: Acceleration COMPLETE.");
    Ok(())
}
