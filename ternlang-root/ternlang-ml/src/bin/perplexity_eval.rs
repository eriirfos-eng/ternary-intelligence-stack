// Phase 12C: Perplexity validation — compare pre-QAT vs post-QAT TernaryMLP.
//
// Run with: cargo run --bin perplexity_eval -p ternlang-ml

use ternlang_ml::{TernaryMLP, perplexity::compare_perplexity};
use ternlang_ml::qat::{QatConfig, SteTrainer};

fn lcg(n: usize, seed: u64) -> Vec<f32> {
    let mut s = seed;
    (0..n).map(|_| {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((s >> 33) as f32) / (u32::MAX as f32) * 2.0 - 1.0
    }).collect()
}

fn main() {
    println!("=== RFI-IRFOS TIS — Phase 12C: Perplexity Validation ===\n");

    let (inf, hs, outf) = (32, 64, 8);
    println!("Architecture: {}→{}→{}", inf, hs, outf);

    // ── Build baseline model ──────────────────────────────────────────────────
    let w1 = lcg(inf * hs, 0xdead_beef);
    let w2 = lcg(hs  * outf, 0xcafe_f00d);
    let pre_qat = TernaryMLP::from_f32(inf, hs, outf, &w1, &w2);
    println!("Pre-QAT  W1 sparsity: {:.1}%", pre_qat.layer1_sparsity() * 100.0);
    println!("Pre-QAT  W2 sparsity: {:.1}%", pre_qat.layer2_sparsity() * 100.0);

    // ── QAT fine-tuning ───────────────────────────────────────────────────────
    let config = QatConfig {
        lr: 0.02,
        epochs: 300,
        clip_threshold: 1.0,
        log_every: 0,
    };
    let mut trainer = SteTrainer::from_f32(inf, hs, outf, w1.clone(), w2.clone(), config);

    // Training set: 32 synthetic samples, multi-class classification
    let train_samples: Vec<(Vec<f32>, Vec<f32>)> = (0u64..32).map(|i| {
        let input = lcg(inf, i * 31 + 7);
        let mut target = vec![-1.0f32; outf];
        target[i as usize % outf] = 1.0;
        (input, target)
    }).collect();

    print!("Fine-tuning with STE ({} epochs)… ", trainer.config.epochs);
    let qat_result = trainer.train(&train_samples);
    println!("done. Loss: {:.4}", qat_result.final_loss);

    let post_qat = trainer.finalize();
    println!("Post-QAT W1 sparsity: {:.1}%", post_qat.layer1_sparsity() * 100.0);
    println!("Post-QAT W2 sparsity: {:.1}%\n", post_qat.layer2_sparsity() * 100.0);

    // ── Perplexity comparison on held-out test set ────────────────────────────
    // Use samples NOT seen during training (different seeds)
    let test_samples: Vec<(Vec<f32>, Vec<f32>)> = (0u64..32).map(|i| {
        let input = lcg(inf, i * 97 + 13);
        let mut target = vec![-1.0f32; outf];
        target[i as usize % outf] = 1.0;
        (input, target)
    }).collect();

    let cmp = compare_perplexity(&pre_qat, &post_qat, &test_samples);
    cmp.print();
}
