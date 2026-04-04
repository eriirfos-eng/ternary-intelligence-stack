use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct MathReasonAgent;

impl TernaryAgent for MathReasonAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();

        // Mathematical operation markers
        let math_ops = ["+", "-", "*", "/", "=", "^", "%", "√", "∫", "∑", "∂", "≤", "≥", "≠", "∞"];
        let op_count: usize = math_ops.iter().map(|s| query.matches(s).count()).sum();

        // Mathematical vocabulary
        let math_vocab = ["sum", "integral", "equation", "derivative", "matrix", "vector",
                           "factorial", "prime", "modulo", "logarithm", "probability",
                           "coefficient", "polynomial", "theorem", "proof", "calculate",
                           "solve", "formula", "divide", "multiply", "squared"];
        let vocab_count = math_vocab.iter().filter(|&&w| q.contains(w)).count();

        // Numeric content
        let has_digits = query.chars().any(|c| c.is_ascii_digit());

        // Impossible operation signals
        let impossible = ["divide by zero", "divided by 0", "1/0", "sqrt(-", "log(-",
                           "undefined operation", "cannot compute"];
        let impossible_count = impossible.iter().filter(|&&i| q.contains(i)).count();

        let ev_signal = ev.get(2).copied().unwrap_or(0.0);
        let math_score = op_count as f32 * 1.2 + vocab_count as f32 * 0.8
                         + (if has_digits { 0.5 } else { 0.0 });
        let raw = math_score - impossible_count as f32 * 3.0 + ev_signal * 1.5;

        let trit: i8 = if impossible_count > 0 { -1 }
                       else if raw > 1.5 { 1 }
                       else if raw > 0.3 { 0 }
                       else { -1 };

        let confidence = (0.80 + (op_count.min(5) as f32 * 0.03) + vocab_count.min(3) as f32 * 0.02)
            .clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!(
                "Math operators: {}, vocabulary terms: {}, numeric content: {}, impossible ops: {}. \
                 Computation: {}.",
                op_count, vocab_count, has_digits, impossible_count,
                match trit {
                    1  => "valid — mathematical content with computable operations",
                    0  => "partial — numeric context without clear operation",
                    _  => "rejected — impossible or undefined operation",
                }
            ),
            expert_id: 10,
            expert_name: "MathReason".into(),
        }
    }
}
