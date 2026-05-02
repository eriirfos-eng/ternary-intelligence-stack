use anyhow::{Result, bail};
use moe_sdk::Platform;

pub struct DeterminismHarness {
    iterations: usize,
}

impl DeterminismHarness {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }

    /// Run the same prompt N times and verify every verdict is identical.
    /// Returns all verdict strings if determinism holds, errors on first violation.
    pub fn run_test(&self, input: &str, platform: &Platform) -> Result<Vec<String>> {
        let model = platform.load_model("determinism-harness")?;
        let mut verdicts: Vec<String> = Vec::new();

        for i in 0..self.iterations {
            let result = platform.run_inference(&model, input)?;
            let summary = result.routing_summary.clone();
            verdicts.push(summary);

            if i > 0 && verdicts[i] != verdicts[0] {
                bail!(
                    "Determinism violation at iteration {}: got\n  {}\nexpected\n  {}",
                    i, verdicts[i], verdicts[0]
                );
            }
        }

        Ok(verdicts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harness_determinism_epis() {
        let platform = Platform::new();
        let harness = DeterminismHarness::new(10);
        let results = harness.run_test("Should we proceed with the deployment?", &platform).unwrap();
        assert_eq!(results.len(), 10);
        // All 10 must be identical
        assert!(results.windows(2).all(|w| w[0] == w[1]),
            "EPIS must be deterministic across all iterations");
        println!("✓ EPIS determinism verified over 10 iterations");
        println!("  verdict: {}", results[0]);
    }

    #[test]
    fn test_trit_verdict_is_valid() {
        let platform = Platform::new();
        let model = platform.load_model("validation-suite").unwrap();
        let result = platform.run_inference(&model, "Is this action safe?").unwrap();
        assert!([-1i8, 0, 1].contains(&result.trit_verdict),
            "Verdict must be a valid trit, got: {}", result.trit_verdict);
        println!("✓ Trit verdict: {} | confidence: {:.3}", result.trit_verdict, result.confidence);
        println!("  {}", result.routing_summary);
    }

    #[test]
    fn test_output_vec_has_correct_dims() {
        let platform = Platform::new();
        let model = platform.load_model("validation-suite").unwrap();
        let result = platform.run_inference(&model, "test").unwrap();
        assert_eq!(result.output_vec.len(), model.output_dim,
            "Output vector must match declared output_dim");
    }

    #[test]
    fn test_empty_prompt_handled() {
        let platform = Platform::new();
        let model = platform.load_model("validation-suite").unwrap();
        let result = platform.run_inference(&model, "").unwrap();
        assert!([-1i8, 0, 1].contains(&result.trit_verdict));
    }
}
