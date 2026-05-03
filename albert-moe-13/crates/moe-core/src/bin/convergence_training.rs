//! # Convergence Training Experiment
//! 
//! A functional training experiment that demonstrates real ternary weight 
//! updates using Straight-Through Estimation (STE) and logs the loss.

use moe_core::training::ternarization::{TernarizationPipeline, MockStreamer, DatasetStreamer};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    println!("--- Launching Functional Convergence Training ---");
    
    let n = 2048; // Model dimension
    let mut weights = vec![0.1f32; n];
    let streamer = MockStreamer { input_size: n };
    let learning_rate = 0.02;
    let threshold = 0.45;
    
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("convergence_training.log")
        .expect("Unable to open log file");
    
    writeln!(log_file, "--- New Functional Run: N={}, LR={}, Tau={} ---", n, learning_rate, threshold).unwrap();
    
    for epoch in 0..100 {
        let (_input, target) = streamer.get_next_batch(1);
        let t = target[0]; // Shared target for all weights in this mock task
        
        // 1. Forward Pass (STE)
        let mut total_loss = 0.0;
        let mut mean_weight = 0.0;
        
        for w in weights.iter_mut() {
            let pred = TernarizationPipeline::forward_ste(*w, threshold) as f32;
            let diff = pred - t;
            total_loss += diff.powi(2);
            
            // 3. Backward Pass (Identity STE)
            let grad = 2.0 * diff;
            *w -= learning_rate * grad;
            mean_weight += *w;
        }
        
        mean_weight /= n as f32;
        let avg_loss = total_loss / n as f32;
        
        if epoch % 10 == 0 {
            println!("Epoch {}: avg_loss = {:.4}, mean_w = {:.4}", epoch, avg_loss, mean_weight);
            writeln!(log_file, "Epoch {}: avg_loss={:.6}, mean_w={:.6}", epoch, avg_loss, mean_weight).unwrap();
        }
    }
    
    println!("--- Training Complete. Convergence Logged. ---");
}
