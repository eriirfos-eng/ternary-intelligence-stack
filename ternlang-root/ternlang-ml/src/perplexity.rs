// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Phase 12C: Perplexity Validation
//
// Pseudo-perplexity for ternary MLPs:
//   1. Treat each test sample as a classification problem.
//   2. The "correct class" is argmax(target).
//   3. Compute softmax over f32 logits from forward_logits().
//   4. CE = -log P(correct_class) per sample.
//   5. PPL = exp(mean CE over all samples).
//
// Additionally reports top-1 accuracy and average ternary output entropy.

use crate::TernaryMLP;

// ─── Softmax / entropy helpers ───────────────────────────────────────────────

fn softmax(logits: &[f32]) -> Vec<f32> {
    let max = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = logits.iter().map(|&x| (x - max).exp()).collect();
    let sum: f32 = exps.iter().sum();
    exps.iter().map(|e| e / sum.max(1e-9)).collect()
}

fn cross_entropy(probs: &[f32], target_class: usize) -> f32 {
    -probs[target_class].max(1e-9).ln()
}

fn entropy(probs: &[f32]) -> f32 {
    probs.iter().map(|&p| if p > 0.0 { -p * p.ln() } else { 0.0 }).sum()
}

// ─── Evaluation results ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PerplexityReport {
    pub pseudo_perplexity: f32,
    pub mean_cross_entropy: f32,
    pub top1_accuracy: f32,
    pub mean_output_entropy: f32,
    pub n_samples: usize,
}

impl PerplexityReport {
    pub fn print(&self, label: &str) {
        println!("  [{}]", label);
        println!("    Pseudo-PPL:         {:.4}", self.pseudo_perplexity);
        println!("    Mean cross-entropy: {:.4}", self.mean_cross_entropy);
        println!("    Top-1 accuracy:     {:.1}%", self.top1_accuracy * 100.0);
        println!("    Output entropy:     {:.4} nats", self.mean_output_entropy);
        println!("    Samples:            {}", self.n_samples);
    }
}

// ─── Evaluator ───────────────────────────────────────────────────────────────

pub struct PerplexityEvaluator;

impl PerplexityEvaluator {
    /// Evaluate `model` on `test_samples`.
    ///
    /// Each sample is `(input: Vec<f32>, target: Vec<f32>)`.
    /// The correct class for cross-entropy is `argmax(target)`.
    pub fn evaluate(model: &TernaryMLP, test_samples: &[(Vec<f32>, Vec<f32>)]) -> PerplexityReport {
        assert!(!test_samples.is_empty(), "need at least one test sample");

        let mut total_ce = 0.0f32;
        let mut total_entropy = 0.0f32;
        let mut correct = 0usize;

        for (input, target) in test_samples {
            let logits = model.forward_logits(input);
            let probs  = softmax(&logits);

            let target_class = argmax(target);
            let pred_class   = argmax(&logits);

            total_ce      += cross_entropy(&probs, target_class);
            total_entropy += entropy(&probs);
            if pred_class == target_class { correct += 1; }
        }

        let n = test_samples.len() as f32;
        let mean_ce = total_ce / n;

        PerplexityReport {
            pseudo_perplexity:   mean_ce.exp(),
            mean_cross_entropy:  mean_ce,
            top1_accuracy:       correct as f32 / n,
            mean_output_entropy: total_entropy / n,
            n_samples:           test_samples.len(),
        }
    }
}

fn argmax(v: &[f32]) -> usize {
    v.iter().enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0)
}

// ─── Comparison utility ───────────────────────────────────────────────────────

pub struct ComparisonResult {
    pub before: PerplexityReport,
    pub after:  PerplexityReport,
}

impl ComparisonResult {
    pub fn print(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!(  "║     Phase 12C — QAT Perplexity Comparison (RFI-IRFOS TIS)  ║");
        println!(  "╠══════════════════════════════════════════════════════════════╣");
        self.before.print("Pre-QAT  (baseline)");
        println!(  "  ──────────────────────────────────────────────────────────");
        self.after.print("Post-QAT (STE fine-tuned)");
        println!(  "╠══════════════════════════════════════════════════════════════╣");

        let ppl_delta  = self.after.pseudo_perplexity - self.before.pseudo_perplexity;
        let acc_delta  = self.after.top1_accuracy     - self.before.top1_accuracy;
        let improved   = ppl_delta < 0.0 || acc_delta > 0.0;

        println!("  PPL delta:  {:+.4}  ({})",
            ppl_delta, if ppl_delta < 0.0 { "IMPROVED" } else { "regressed" });
        println!("  Acc delta:  {:+.1}%  ({})",
            acc_delta * 100.0, if acc_delta > 0.0 { "IMPROVED" } else if acc_delta < 0.0 { "regressed" } else { "unchanged" });
        println!("  Verdict:    {}", if improved { "[OK] QAT improved model quality" } else { "[WARN] No improvement — check lr/epochs" });
        println!("╚══════════════════════════════════════════════════════════════╝");
    }
}

/// Run a full pre/post QAT comparison and return the result.
pub fn compare_perplexity(
    pre_qat:  &TernaryMLP,
    post_qat: &TernaryMLP,
    test_samples: &[(Vec<f32>, Vec<f32>)],
) -> ComparisonResult {
    ComparisonResult {
        before: PerplexityEvaluator::evaluate(pre_qat,  test_samples),
        after:  PerplexityEvaluator::evaluate(post_qat, test_samples),
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TernaryMLP;

    fn lcg(n: usize, seed: u64) -> Vec<f32> {
        let mut s = seed;
        (0..n).map(|_| {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((s >> 33) as f32) / (u32::MAX as f32) * 2.0 - 1.0
        }).collect()
    }

    fn make_mlp(inf: usize, hs: usize, outf: usize, seed: u64) -> TernaryMLP {
        let w1 = lcg(inf * hs, seed);
        let w2 = lcg(hs * outf, seed + 1);
        TernaryMLP::from_f32(inf, hs, outf, &w1, &w2)
    }

    #[test]
    fn perplexity_report_is_finite() {
        let mlp = make_mlp(8, 16, 4, 0xabc);
        let samples: Vec<(Vec<f32>, Vec<f32>)> = (0u64..10).map(|i| {
            let input  = lcg(8, i * 7 + 1);
            let mut target = vec![0.0f32; 4];
            target[i as usize % 4] = 1.0;
            (input, target)
        }).collect();

        let report = PerplexityEvaluator::evaluate(&mlp, &samples);
        assert!(report.pseudo_perplexity.is_finite());
        assert!(report.pseudo_perplexity > 0.0);
        assert!(report.mean_cross_entropy >= 0.0);
        assert!(report.top1_accuracy >= 0.0 && report.top1_accuracy <= 1.0);
    }

    #[test]
    fn qat_comparison_runs() {
        use crate::qat::{QatConfig, SteTrainer};

        let (inf, hs, outf) = (8, 16, 4);
        let w1 = lcg(inf * hs, 0x1234);
        let w2 = lcg(hs * outf, 0x5678);

        let pre_qat = TernaryMLP::from_f32(inf, hs, outf, &w1, &w2);

        let config = QatConfig { lr: 0.05, epochs: 30, clip_threshold: 1.0, log_every: 0 };
        let mut trainer = SteTrainer::from_f32(inf, hs, outf, w1.clone(), w2.clone(), config);

        let train_samples: Vec<(Vec<f32>, Vec<f32>)> = (0u64..8).map(|i| {
            let input  = lcg(inf, i * 13 + 5);
            let mut target = vec![-1.0f32; outf];
            target[i as usize % outf] = 1.0;
            (input, target)
        }).collect();

        trainer.train(&train_samples);
        let post_qat = trainer.finalize();

        let test_samples: Vec<(Vec<f32>, Vec<f32>)> = (0u64..8).map(|i| {
            let input  = lcg(inf, i * 19 + 3);
            let mut target = vec![-1.0f32; outf];
            target[i as usize % outf] = 1.0;
            (input, target)
        }).collect();

        let result = compare_perplexity(&pre_qat, &post_qat, &test_samples);
        // Both reports must be well-formed
        assert!(result.before.pseudo_perplexity.is_finite());
        assert!(result.after.pseudo_perplexity.is_finite());
    }
}
