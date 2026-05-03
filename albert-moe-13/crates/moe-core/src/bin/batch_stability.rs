//! # Batch Stability Training
//! 
//! Automates 18 training iterations to reach our 30-run statistical stability goal.

use moe_core::training::ternarization::{TernarizationPipeline, MockStreamer, DatasetStreamer};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let runs_needed = 18;
    let n = 2048;
    let learning_rate = 0.02;
    let threshold = 0.45;
    
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("crates/moe-core/training.log")
        .expect("Unable to open log file");
    
    for run in 0..runs_needed {
        println!("--- Starting Run {}/{} ---", run + 1, runs_needed);
        let mut weights = vec![0.1f32; n];
        let streamer = MockStreamer { input_size: n };
        
        writeln!(log_file, "--- New Run: N={}, LR={}, Tau={}, Seed={} ---", n, learning_rate, threshold, run).unwrap();
        
        for epoch in 0..50 {
            let (_, target) = streamer.get_next_batch(1);
            let t = target[0];
            
            let mut total_loss = 0.0;
            for w in weights.iter_mut() {
                let pred = TernarizationPipeline::forward_ste(*w, threshold) as f32;
                let diff = pred - t;
                total_loss += diff.powi(2);
                let grad = 2.0 * diff;
                *w -= learning_rate * grad;
            }
            
            if epoch % 10 == 0 {
                writeln!(log_file, "Epoch {}: loss={:.6}", epoch, total_loss / n as f32).unwrap();
            }
        }
    }
    println!("--- Batch training complete. Stats logged. ---");
}
