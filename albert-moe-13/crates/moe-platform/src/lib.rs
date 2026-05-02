use anyhow::Result;
use moe_core::core::inference::InferenceEngine;

/// A loaded, ready-to-run model instance.
pub struct LoadedModel {
    engine: InferenceEngine,
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
    /// Human-readable summary of expert routing
    pub routing_summary: String,
}

pub struct Platform;

impl Platform {
    pub fn new() -> Self {
        Self
    }

    /// Instantiate an InferenceEngine for the given model identifier.
    /// Dimensions default to 64; future versions will parse model_id for config.
    pub fn load_model(&self, model_id: &str) -> Result<LoadedModel> {
        let input_dim = 64;
        let output_dim = 64;
        Ok(LoadedModel {
            engine: InferenceEngine::new(
                format!("epis-v1.0/{}", model_id),
                input_dim,
                output_dim,
            ),
            model_id: model_id.to_string(),
            input_dim,
            output_dim,
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
/// Each byte maps to a dimension via modular indexing; result is L2-normalised.
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

    // Ternary thresholding: ±0.05 dead-band becomes HOLD
    let trit_verdict: i8 = if mean > 0.05 { 1 } else if mean < -0.05 { -1 } else { 0 };
    let confidence = mean.abs().min(1.0);

    let verdict_label = match trit_verdict {
        1  => "affirm (+1)",
        -1 => "reject (-1)",
        _  => "hold   ( 0)",
    };

    let routing_summary = format!(
        "model={} | kernel={} | dims={}→{} | verdict={}| confidence={:.3}",
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
            "EPIS mode must produce identical verdicts for identical input");
        assert_eq!(a.output_vec, b.output_vec,
            "EPIS mode must produce identical output vectors for identical input");
    }

    #[test]
    fn test_different_prompts_may_differ() {
        let platform = Platform::new();
        let model = platform.load_model("test-epis").unwrap();
        let a = platform.run_inference(&model, "proceed").unwrap();
        let b = platform.run_inference(&model, "abort").unwrap();
        // Different inputs should produce different activations (routing sanity check)
        assert_ne!(a.output_vec, b.output_vec, "Different prompts must produce different activations");
    }
}
