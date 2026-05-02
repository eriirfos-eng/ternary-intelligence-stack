use crate::core::inference::InferenceEngine;
use crate::core::mock_layer::compute_similarity;

pub fn run_moe_route_test() {
    let engine = InferenceEngine::new("v1.0".to_string(), 16, 16);
    let input = vec![1.0; 16];
    let domain_bias = engine.expert_bank.domain_scores(&input);
    let routes = engine.router.route(&input, 2, &domain_bias);
    
    println!("\n[ALBERT::MOE_ROUTE_TEST]");
    for (idx, score) in routes {
        println!("Expert {}: Score {:.4}", idx, score);
    }
}

pub fn run_moe_expert_divergence_test() {
    let engine = InferenceEngine::new("v1.0".to_string(), 16, 16);
    let input = vec![1.0; 16];
    
    println!("\n[ALBERT::MOE_DIVERGENCE_TEST]");
    let out1 = engine.expert_bank.execute_expert(0, &input);
    let out2 = engine.expert_bank.execute_expert(1, &input);
    
    let sim = compute_similarity(&out1, &out2);
    println!("Similarity Expert 0 vs 1: {:.4}", sim);
}

pub fn run_moe_load_report() {
    let engine = InferenceEngine::new("v1.0".to_string(), 16, 16);
    // Simulate multiple passes to report distribution
    println!("\n[ALBERT::MOE_LOAD_REPORT]");
    println!("Load distribution metrics initialized.");
}
