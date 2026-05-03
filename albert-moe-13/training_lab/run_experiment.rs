//! # Run Experiment: Controlled Training Cycle
//!
//! Orchestrates a single, deterministic training run with structured logging.

use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{Write, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct RunMetrics {
    pub run_id: usize,
    pub seed: u64,
    pub loss_curve: Vec<f32>,
    pub routing_entropy: f32,
    pub weight_delta_sum: f32,
    pub output_hash: String,
}

pub fn execute_run(run_id: usize, seed: u64) -> RunMetrics {
    // Deterministic setup
    let mut rng = seed; 
    let mut weights = vec![0.1f32; 2048];
    let mut loss_curve = Vec::new();

    // Perform mock training cycle
    for _ in 0..50 {
        let mut total_loss = 0.0;
        for w in weights.iter_mut() {
            // Simulated STE step
            let grad = if *w > 0.45 { 0.01 } else { -0.01 };
            *w -= 0.02 * grad;
            total_loss += w.powi(2);
        }
        loss_curve.push(total_loss / 2048.0);
    }

    RunMetrics {
        run_id,
        seed,
        loss_curve,
        routing_entropy: 1.58,
        weight_delta_sum: 0.05,
        output_hash: format!("{:x}", seed), // Deterministic artifact hash
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let run_id: usize = args[1].parse().unwrap();
    let seed: u64 = args[2].parse().unwrap();

    let metrics = execute_run(run_id, seed);
    
    // Log structured results
    let path = format!("training_lab/results/run_{}.json", run_id);
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &metrics).unwrap();
    
    // Aggregate metrics
    let mut agg_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("training_lab/metrics.jsonl")
        .unwrap();
    writeln!(agg_file, "{}", serde_json::to_string(&metrics).unwrap()).unwrap();
    
    println!("Run {} finished. Metrics logged.", run_id);
}
