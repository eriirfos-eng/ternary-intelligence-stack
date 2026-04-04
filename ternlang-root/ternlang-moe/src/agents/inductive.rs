use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct InductiveReasonAgent;

impl TernaryAgent for InductiveReasonAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();
        let keywords = ["example", "likely", "usually", "pattern", "observed", "tend to", "often", "probability"];
        let count = keywords.iter().filter(|&&k| q.contains(k)).count();
        
        let ev_signal = ev.get(2).copied().unwrap_or(0.0);
        let trit = if count >= 2 || ev_signal > 0.2 { 1 } else if count >= 1 { 0 } else { -1 };
        let confidence = (0.65 + (count as f32 * 0.07).min(0.25) + ev_signal.abs() * 0.1).clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!("Inductive markers: {}. Probabilistic reasoning detected via statistical keywords.", count),
            expert_id: 3,
            expert_name: "InductiveReason".into(),
        }
    }
}
