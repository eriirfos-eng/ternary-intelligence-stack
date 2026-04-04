use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct SyntaxAgent;

impl TernaryAgent for SyntaxAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let markers = ["(", ")", "[", "]", "{", "}", ";", ".", ",", "```"];
        let count: usize = markers.iter().map(|m| query.matches(m).count()).sum();
        
        let ev_signal = ev.get(0).copied().unwrap_or(0.0);
        let trit = if count > 5 || ev_signal > 0.5 { 1 } else if count > 0 { 0 } else { -1 };
        let confidence = (0.7 + (count as f32 * 0.05).min(0.25) + ev_signal.abs() * 0.05).clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!("Found {} structural markers. Structural complexity is {}.", count, if count > 10 { "high" } else if count > 3 { "moderate" } else { "low" }),
            expert_id: 0,
            expert_name: "Syntax".into(),
        }
    }
}
