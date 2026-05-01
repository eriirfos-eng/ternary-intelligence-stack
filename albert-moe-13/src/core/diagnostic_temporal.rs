use crate::core::mock_layer::{ternary_inference_engine, InferenceConfig, compute_similarity, compute_variance};

pub fn run_temporal_stability_diagnostic() {
    let config = InferenceConfig { batch_size: 1, depth: 4, width: 16, ternary_threshold: 0.3, residual_strength: 0.8 };
    let input = vec![1.0; 16];
    let n = 50;
    
    println!("\n[ALBERT::TEMPORAL_STABILITY_DIAGNOSTIC]");
    println!("Iteration | Var | Similarity(t, t-1)");

    let mut prev_output = ternary_inference_engine(&input, &config);
    let mut similarities = Vec::new();

    for t in 0..n {
        let output = ternary_inference_engine(&input, &config);
        let sim = compute_similarity(&output, &prev_output);
        similarities.push(sim);
        
        let var = compute_variance(&output);
        println!("{:>8} | {:>3.4} | {:>3.4}", t, var, sim);
        
        prev_output = output;
    }

    let avg_sim: f32 = similarities.iter().sum::<f32>() / n as f32;
    println!("\nAnalysis Summary:");
    println!("Average Convergence (Similarity): {:.4}", avg_sim);
    if avg_sim > 0.95 {
        println!("Classification: SINGLE-BASIN (Strict Contraction)");
    } else {
        println!("Classification: MULTI-STABLE (Potential Attractors)");
    }
}
