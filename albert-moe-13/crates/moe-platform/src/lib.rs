pub mod file_loader;

use anyhow::Result;
use moe_core::core::inference::InferenceEngine;
use file_loader::{load_expert_bank, EXPERT_INPUT_DIM, EXPERT_OUTPUT_DIM};

/// A loaded, ready-to-run model instance.
pub struct LoadedModel {
    pub(crate) engine: InferenceEngine,
    pub model_id: String,
    pub input_dim: usize,
    pub output_dim: usize,
}

/// The ternary verdict and metadata from a single inference pass.
pub struct InferenceResult {
    /// Ternary decision: -1 (reject), 0 (hold), +1 (affirm)
    pub trit_verdict: i8,
    /// Strength of signal driving the verdict (0.0 – 1.0)
    pub confidence: f32,
    /// Raw output activation vector from the expert bank
    pub output_vec: Vec<f32>,
    /// Human-readable routing summary
    pub routing_summary: String,
}

pub struct Platform;

impl Platform {
    pub fn new() -> Self {
        Self
    }

    /// Instantiate a synthetic InferenceEngine with default dimensions.
    /// Use this when no pre-trained weights are available.
    pub fn load_model(&self, model_id: &str) -> Result<LoadedModel> {
        Ok(LoadedModel {
            engine: InferenceEngine::new(
                format!("epis-v1.0/{}", model_id),
                EXPERT_INPUT_DIM,
                EXPERT_OUTPUT_DIM,
            ),
            model_id: model_id.to_string(),
            input_dim: EXPERT_INPUT_DIM,
            output_dim: EXPERT_OUTPUT_DIM,
        })
    }

    /// Load a real ternarized model from a `.tern.bin` file.
    ///
    /// The file must be a `ModelCoherence` binary produced by `scripts/transmute_llama.py`.
    /// Layers are mapped round-robin to the 13 EPIS experts.
    ///
    /// ```no_run
    /// use moe_platform::Platform;
    /// let platform = Platform::new();
    /// let model = platform.load_model_from_file("/path/to/llama32-1b.tern.bin").unwrap();
    /// let result = platform.run_inference(&model, "Should we proceed?").unwrap();
    /// println!("Verdict: {}", result.trit_verdict);
    /// ```
    pub fn load_model_from_file(&self, path: &str) -> Result<LoadedModel> {
        let (expert_bank, info) = load_expert_bank(path)?;

        log::info!(
            "Loaded '{}' — {} layers → 13 experts | sparsity {:.1}% | ᾱ={:.4}",
            info.source_model,
            info.num_layers,
            info.sparsity * 100.0,
            info.mean_alpha,
        );

        let mut engine = InferenceEngine::new(
            format!("epis-v1.0/{}", info.source_model),
            EXPERT_INPUT_DIM,
            EXPERT_OUTPUT_DIM,
        );
        // Swap in real weights from file — overwrites the randomly-initialised bank
        engine.expert_bank = expert_bank;

        Ok(LoadedModel {
            engine,
            model_id: info.source_model,
            input_dim: EXPERT_INPUT_DIM,
            output_dim: EXPERT_OUTPUT_DIM,
        })
    }

    /// Run a forward pass and return a structured ternary result.
    pub fn run_inference(&self, model: &LoadedModel, prompt: &str) -> Result<InferenceResult> {
        let mut input = encode_prompt(prompt, model.input_dim);
        let output = model.engine.forward(&mut input)?;
        Ok(decode_result(output, model))
    }
}

impl Default for Platform {
    fn default() -> Self {
        Self::new()
    }
}

/// Encode a text prompt into a normalised float activation vector.
fn encode_prompt(prompt: &str, dim: usize) -> Vec<f32> {
    let mut vec = vec![0.0f32; dim];
    for (i, b) in prompt.bytes().enumerate() {
        vec[i % dim] += (b as f32 - 128.0) / 128.0;
    }
    let norm = vec.iter().map(|x| x * x).sum::<f32>().sqrt().max(1e-9);
    vec.iter_mut().for_each(|x| *x /= norm);
    vec
}

/// Map raw output activations to a ternary verdict + metadata.
fn decode_result(output: Vec<f32>, model: &LoadedModel) -> InferenceResult {
    let mean = output.iter().sum::<f32>() / output.len() as f32;
    let trit_verdict: i8 = if mean > 0.05 { 1 } else if mean < -0.05 { -1 } else { 0 };
    let confidence = mean.abs().min(1.0);

    let verdict_label = match trit_verdict {
        1  => "affirm (+1)",
        -1 => "reject (-1)",
        _  => "hold   ( 0)",
    };

    let routing_summary = format!(
        "model={} | kernel={} | dims={}→{} | verdict={} | confidence={:.3}",
        model.model_id,
        model.engine.kernel_version,
        model.input_dim,
        model.output_dim,
        verdict_label,
        confidence,
    );

    InferenceResult { trit_verdict, confidence, output_vec: output, routing_summary }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_and_infer() {
        let platform = Platform::new();
        let model = platform.load_model("test-epis").unwrap();
        let result = platform.run_inference(&model, "Should we proceed?").unwrap();
        assert!([-1i8, 0, 1].contains(&result.trit_verdict));
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
        assert_eq!(result.output_vec.len(), model.output_dim);
    }

    #[test]
    fn test_epis_determinism() {
        let platform = Platform::new();
        let model = platform.load_model("test-epis").unwrap();
        let prompt = "Is this action safe?";
        let a = platform.run_inference(&model, prompt).unwrap();
        let b = platform.run_inference(&model, prompt).unwrap();
        assert_eq!(a.trit_verdict, b.trit_verdict,
            "EPIS must produce identical verdicts for identical input");
        assert_eq!(a.output_vec, b.output_vec,
            "EPIS must produce identical activations for identical input");
    }

    #[test]
    fn test_different_prompts_may_differ() {
        let platform = Platform::new();
        let model = platform.load_model("test-epis").unwrap();
        let a = platform.run_inference(&model, "proceed").unwrap();
        let b = platform.run_inference(&model, "abort").unwrap();
        assert_ne!(a.output_vec, b.output_vec,
            "Different prompts must produce different activations");
    }

    /// Smoke test for file loading — skipped if no .tern.bin is present.
    #[test]
    fn test_load_from_file_if_available() {
        let candidates = [
            "/home/eri-irfos/llama32-1b.tern.bin",
            "/home/eri-irfos/Desktop/llama32-1b.tern.bin",
        ];
        let path = candidates.iter().find(|p| std::path::Path::new(p).exists());

        if let Some(p) = path {
            let platform = Platform::new();
            let model = platform.load_model_from_file(p).unwrap();
            let result = platform.run_inference(&model, "What is ternary logic?").unwrap();
            assert!([-1i8, 0, 1].contains(&result.trit_verdict));
            println!("✓ Real model loaded: {}", result.routing_summary);
        } else {
            println!("⚠  No .tern.bin found — skipping file-load smoke test");
            println!("   Run: python3 scripts/transmute_llama.py to generate one");
        }
    }
}
