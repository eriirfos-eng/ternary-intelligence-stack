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
    
    // Generate deterministic mock weights for sweep
    let mut weights = Vec::with_capacity(input_dim * output_dim);
    for i in 0..(input_dim * output_dim) {
        let val = (i as f32 * 0.1).sin() * (i as f32 * 0.2).cos();
        weights.push(val);
    }
    
    let bias = vec![0.0; output_dim];
    let mut input = Vec::with_capacity(input_dim);
    for i in 0..input_dim {
        input.push((i as f32 * 0.5).cos());
    }

    let float_layer = LinearLayer {
        weights,
        bias,
        input_dim,
        output_dim,
    };

    let thresholds = vec![0.05, 0.1, 0.2, 0.3, 0.5];
    let float_out = float_layer.forward(&input);

    println!("\n[ALBERT::SWEEP]");
    println!("{:<10} | {:<10} | {:<10}", "Threshold", "Sparsity", "MSE");
    println!("--------------------------------");

    let mut csv_data = String::from("threshold,sparsity,mse\n");

    for &t in &thresholds {
        let ternary_layer = convert_to_ternary(&float_layer, t);
        let ternary_out = ternary_layer.forward_ternary(&input);
        let mse = compute_mse(&float_out, &ternary_out);
        let zero_count = ternary_layer.weights.iter().filter(|&&w| w == 0).count();
        let sparsity = (zero_count as f32 / ternary_layer.weights.len() as f32) * 100.0;

        println!("{:<10.2} | {:<10.1}% | {:<10.6}", t, sparsity, mse);
        csv_data.push_str(&format!("{:.2},{:.2},{:.6}\n", t, sparsity, mse));
    }

    // Save results to docs (optional/best-effort)
    let _ = std::fs::create_dir_all("docs");
    let _ = std::fs::write("docs/ternary_sweep.csv", csv_data);
    println!("\nResults saved to docs/ternary_sweep.csv");
    println!();
}
