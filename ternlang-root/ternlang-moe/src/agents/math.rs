use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct MathReasonAgent;

impl TernaryAgent for MathReasonAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let math_syms = ["+", "-", "*", "/", "=", "^", "%", "√", "∫"];
        let math_words = ["sum", "integral", "equation", "multiply", "divide", "plus", "minus", "derivative"];
        let sym_count = math_syms.iter().map(|s| query.matches(s).count()).sum::<usize>();
        let word_count = math_words.iter().filter(|&&w| query.to_lowercase().contains(w)).count();
        let has_digits = query.chars().any(|c| c.is_ascii_digit());
        
        let ev_signal = ev.get(2).copied().unwrap_or(0.0);
        let trit = if sym_count > 0 || word_count > 0 || has_digits || ev_signal > 0.5 { 1 } else { -1 };
        let confidence = (0.85 + (sym_count as f32 * 0.05).min(0.15)).clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!("Found {} math symbols and {} math keywords. Numerical precision context identified.", sym_count, word_count),
            expert_id: 10,
            expert_name: "MathReason".into(),
        }
    }
}
