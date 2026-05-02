// Phase 12B demo: QAT/STE fine-tuning on a synthetic dataset.
//
// Run with: cargo run --bin qat_train -p ternlang-ml

use ternlang_ml::qat::{QatConfig, SteTrainer};

fn lcg(n: usize, seed: u64) -> Vec<f32> {
    let mut s = seed;
    (0..n).map(|_| {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((s >> 33) as f32) / (u32::MAX as f32) * 2.0 - 1.0
    }).collect()
}

fn main() {
    println!("=== RFI-IRFOS TIS — Phase 12B: QAT / Straight-Through Estimator ===\n");

    let (inf, hs, outf) = (32, 64, 8);
    println!("Architecture: {}→{}→{}", inf, hs, outf);

    // Initialise latent weights from random f32
    let w1 = lcg(inf * hs, 0xdead_beef);
    let w2 = lcg(hs  * outf, 0xcafe_f00d);

    let config = QatConfig {
        lr: 0.02,
        epochs: 200,
        clip_threshold: 1.0,
        log_every: 20,
    };

    let mut trainer = SteTrainer::from_f32(inf, hs, outf, w1, w2, config);

    // Synthetic dataset: 16 samples, target = first class always affirm
    let samples: Vec<(Vec<f32>, Vec<f32>)> = (0u64..16).map(|i| {
        let input = lcg(inf, i * 31 + 7);
        let mut target = vec![-1.0f32; outf];
        target[(i as usize) % outf] = 1.0;
        (input, target)
    }).collect();

    println!("Training on {} synthetic samples…\n", samples.len());
    let result = trainer.train(&samples);

    println!("\n--- QAT Result ---");
    println!("  Final MSE loss:              {:.6}", result.final_loss);
    println!("  Active gradient fraction:    {:.1}%", result.active_gradient_fraction * 100.0);

    let mlp = trainer.finalize();
    println!("  W1 sparsity after QAT:       {:.1}%", mlp.layer1_sparsity() * 100.0);
    println!("  W2 sparsity after QAT:       {:.1}%", mlp.layer2_sparsity() * 100.0);
    println!("\n[OK] Phase 12B complete — QAT/STE model ready for perplexity validation (Phase 12C).");
}
