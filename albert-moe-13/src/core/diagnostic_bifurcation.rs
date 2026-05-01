use crate::core::mock_layer::{ternary_inference_engine, InferenceConfig, compute_similarity, compute_variance};
use crate::core::entropy_injector::EntropyInjector;

pub fn run_bifurcation_analysis() {
    let base_config = InferenceConfig { batch_size: 1, depth: 4, width: 16, ternary_threshold: 0.3, residual_strength: 0.8 };
    let input = vec![1.0; 16];
    let n = 50;
    
    // Scaling pressure: (noise_scale, beta_conflict)
    let pressure_levels = vec![(0.1, 0.1), (0.5, 0.5), (1.0, 1.0)];
    
    println!("\n[ALBERT::BIFURCATION_ANALYSIS]");
    println!("Pressure | Stable Clusters | Persistence | Similarity");

    for (ns, beta) in pressure_levels {
        let injector = EntropyInjector::new(ns, 0.05, 0.02);
        let mut outputs = Vec::new();
        let mut prev_output = ternary_inference_engine(&input, &base_config);
        
        let mut cluster_count = 1;
        let mut persistence = 0;
        
        for _t in 0..n {
            let mut output = ternary_inference_engine(&input, &base_config);
            // Apply intensified perturbation logic here as per objectives
            // (e.g., using the injector instance locally if needed)
            
            let sim = compute_similarity(&output, &prev_output);
            if sim < 0.7 {
                cluster_count += 1;
            }
            if sim > 0.9 {
                persistence += 1;
            }
            outputs.push(output.clone());
            prev_output = output;
        }

        let avg_sim: f32 = outputs.iter().take(outputs.len()-1).zip(outputs.iter().skip(1))
            .map(|(a, b)| compute_similarity(a, b)).sum::<f32>() / n as f32;
        
        println!("{:<8} | {:<15} | {:<11} | {:<10.4}", ns, cluster_count, persistence, avg_sim);
    }
}
