use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct DeductiveReasonAgent;

impl TernaryAgent for DeductiveReasonAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();
        let keywords = ["if", "then", "implies", "therefore", "must", "consequently", "it follows"];
        let count = keywords.iter().filter(|&&k| q.contains(k)).count();
        
        let ev_signal = ev.get(2).copied().unwrap_or(0.0);
        let trit = if count >= 2 || (count >= 1 && ev_signal > 0.3) { 1 } else if count >= 1 { 0 } else { -1 };
        let confidence = (0.75 + (count as f32 * 0.05).min(0.2) + ev_signal.abs() * 0.05).clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!("Logical connectors found: {}. Evidence of deductive structure: {}.", count, if count >= 2 { "strong" } else { "weak" }),
            expert_id: 2,
            expert_name: "DeductiveReason".into(),
        }
    }
}
