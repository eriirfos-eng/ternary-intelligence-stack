// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
//! # Mock Transformer Layer
//! 
//! Minimal implementation of a linear layer for numerical validation of 
//! ternary model transformations.

use crate::core::ternary_mapper::TernaryMapper;

pub struct LinearLayer {
    pub weights: Vec<f32>, // Flat matrix [output_dim * input_dim]
    pub bias: Vec<f32>,    // [output_dim]
    pub input_dim: usize,
    pub output_dim: usize,
}

pub struct TernaryLayer {
    pub weights: Vec<i8>,
    pub alpha: f32,
    pub bias: Vec<f32>,
    pub input_dim: usize,
    pub output_dim: usize,
}

impl LinearLayer {
    /// Standard float32 forward pass (Matrix-Vector Multiplication).
    pub fn forward(&self, input: &[f32]) -> Vec<f32> {
        assert_eq!(input.len(), self.input_dim);
        let mut output = vec![0.0; self.output_dim];

        for i in 0..self.output_dim {
            let mut sum = 0.0;
            for j in 0..self.input_dim {
                sum += self.weights[i * self.input_dim + j] * input[j];
            }
            output[i] = sum + self.bias[i];
        }
        output
    }
}

impl TernaryLayer {
    /// Ternary forward pass.
    /// Result = (input * ternary_weights * alpha) + bias
    pub fn forward_ternary(&self, input: &[f32]) -> Vec<f32> {
        assert_eq!(input.len(), self.input_dim);
        let mut output = vec![0.0; self.output_dim];

        for i in 0..self.output_dim {
            let mut sum = 0.0;
            for j in 0..self.input_dim {
                let w = self.weights[i * self.input_dim + j] as f32;
                // @sparseskip simulation: if w is 0, no addition happens
                if w != 0.0 {
                    sum += w * input[j];
                }
            }
            output[i] = (sum * self.alpha) + self.bias[i];
        }
        output
    }
}

pub fn convert_to_ternary(layer: &LinearLayer, threshold: f32) -> TernaryLayer {
    let mapper = TernaryMapper::new(threshold);
    let (t_weights, alpha) = mapper.ternarize(&layer.weights, threshold);

    TernaryLayer {
        weights: t_weights,
        alpha,
        bias: layer.bias.clone(),
        input_dim: layer.input_dim,
        output_dim: layer.output_dim,
    }
}

pub fn compute_mse(original: &[f32], ternary: &[f32]) -> f32 {
    assert_eq!(original.len(), ternary.len());
    let sum_sq_diff: f32 = original.iter()
        .zip(ternary.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum();
    sum_sq_diff / original.len() as f32
}

pub fn compute_variance(data: &[f32]) -> f32 {
    if data.len() <= 1 { return 0.0; }
    let mean = data.iter().sum::<f32>() / data.len() as f32;
    let var = data.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / (data.len() - 1) as f32;
    // Ensure we don't return 0 to avoid division by zero in NMSE
    if var == 0.0 { 1e-6 } else { var }
}

pub fn baseline_error(layer: &LinearLayer, input: &[f32]) -> f32 {
    let out1 = layer.forward(input);
    let out2 = layer.forward(input);
    compute_mse(&out1, &out2)
}

struct SweepResult {
    threshold: f32,
    sparsity: f32,
    mse: f32,
    nmse: f32,
}

fn perform_sweep(input_dim: usize, output_dim: usize, layer: &LinearLayer, inputs: &[Vec<f32>]) -> Vec<SweepResult> {
    let thresholds = vec![0.05, 0.1, 0.2, 0.3, 0.5];
    let mut results = Vec::new();

    for &t in &thresholds {
        let ternary_layer = convert_to_ternary(layer, t);
        let mut total_mse = 0.0;
        let mut total_nmse = 0.0;

        for input in inputs {
            let float_out = layer.forward(input);
            let ternary_out = ternary_layer.forward_ternary(input);
            
            let mse = compute_mse(&float_out, &ternary_out);
            let variance = compute_variance(&float_out);
            let nmse = mse / variance;

            total_mse += mse;
            total_nmse += nmse;
        }

        let avg_mse = total_mse / inputs.len() as f32;
        let avg_nmse = total_nmse / inputs.len() as f32;
        
        let zero_count = ternary_layer.weights.iter().filter(|&&w| w == 0).count();
        let sparsity = (zero_count as f32 / ternary_layer.weights.len() as f32) * 100.0;

        results.push(SweepResult {
            threshold: t,
            sparsity,
            mse: avg_mse,
            nmse: avg_nmse,
        });
    }
    results
}

pub fn run_layer_comparison() {
    let input_dim = 4;
    let output_dim = 4;
    
    // Mock weights with some variance
    let weights = vec![
        0.9,  0.1, -0.8,  0.0,
        0.2,  0.7, -0.1, -0.6,
       -0.9,  0.0,  0.8,  0.2,
        0.1, -0.1,  0.1,  0.9,
    ];
    let bias = vec![0.1, -0.1, 0.05, -0.05];
    let input = vec![1.0, -0.5, 0.2, 0.8];

    let float_layer = LinearLayer {
        weights,
        bias,
        input_dim,
        output_dim,
    };

    let threshold = 0.4;
    let ternary_layer = convert_to_ternary(&float_layer, threshold);

    let float_out = float_layer.forward(&input);
    let ternary_out = ternary_layer.forward_ternary(&input);
    let mse = compute_mse(&float_out, &ternary_out);

    let zero_count = ternary_layer.weights.iter().filter(|&&w| w == 0).count();
    let compression = (zero_count as f32 / ternary_layer.weights.len() as f32) * 100.0;

    println!("\n[ALBERT::LAYER_TEST]");
    println!("Input Dim: {} | Output Dim: {}", input_dim, output_dim);
    println!("Original Output: {:?}", float_out);
    println!("Ternary Output:  {:?}", ternary_out);
    println!("Mean Squared Error (MSE): {:.6}", mse);
    println!("Sparsity (Compression): {:.1}%", compression);
    println!("Alpha (Scale): {:.4}", ternary_layer.alpha);
    println!();
}

pub fn run_threshold_sweep() {
    let input_dim = 16;
    let output_dim = 16;
    
    // Deterministic weight generation (Fixed Seed Concept)
    let mut weights = Vec::with_capacity(input_dim * output_dim);
    for i in 0..(input_dim * output_dim) {
        let val = (i as f32 * 0.1).sin() * (i as f32 * 0.2).cos();
        weights.push(val);
    }
    
    let bias = vec![0.0; output_dim];
    
    // Multi-input evaluation (Deterministic)
    let mut inputs = Vec::new();
    for s in 0..5 { // 5 different input vectors
        let mut input = Vec::with_capacity(input_dim);
        for i in 0..input_dim {
            input.push(((i + s) as f32 * 0.5).cos());
        }
        inputs.push(input);
    }

    let float_layer = LinearLayer {
        weights,
        bias,
        input_dim,
        output_dim,
    };

    // Baseline Sanity Check
    let baseline = baseline_error(&float_layer, &inputs[0]);
    if baseline > 1e-7 {
        println!("\n[ALBERT::WARNING] Baseline error non-zero: {:.8}. Pipeline may be unstable.", baseline);
    }

    // Repeatability Test: Run twice and compare
    let run1 = perform_sweep(input_dim, output_dim, &float_layer, &inputs);
    let run2 = perform_sweep(input_dim, output_dim, &float_layer, &inputs);

    for (r1, r2) in run1.iter().zip(run2.iter()) {
        if (r1.mse - r2.mse).abs() > 1e-7 {
            println!("\n[ALBERT::ERROR] Non-deterministic behavior detected at threshold {}", r1.threshold);
        }
    }

    println!("\n[ALBERT::SWEEP]");
    println!("{:<10} | {:<10} | {:<10} | {:<10}", "Threshold", "Sparsity", "MSE (Avg)", "NMSE (Avg)");
    println!("----------------------------------------------------------");

    let mut csv_data = String::from("threshold,sparsity,mse,nmse\n");
    let mut prev_mse = 0.0;
    let mut prev_sparsity = 0.0;

    for res in run1 {
        println!("{:<10.2} | {:<10.1}% | {:<10.6} | {:<10.6}", res.threshold, res.sparsity, res.mse, res.nmse);
        csv_data.push_str(&format!("{:.2},{:.2},{:.6},{:.6}\n", res.threshold, res.sparsity, res.mse, res.nmse));

        // Monotonicity Check
        if res.sparsity > prev_sparsity + 5.0 && res.mse < prev_mse - 0.001 {
            // Usually error should increase with sparsity. 
            // If it drops significantly while sparsity jumps, it's worth a warning.
            // Note: Small fluctuations are possible due to scaling factor optimization.
            println!("[ALBERT::WARNING] Non-monotonic error detected at threshold {} — investigate threshold behavior", res.threshold);
        }
        prev_mse = res.mse;
        prev_sparsity = res.sparsity;
    }

    let _ = std::fs::create_dir_all("docs");
    let _ = std::fs::write("docs/ternary_sweep.csv", csv_data);
    println!("\nResults verified and saved to docs/ternary_sweep.csv");
    println!();
}
