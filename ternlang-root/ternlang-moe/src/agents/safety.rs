use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct SafetyAgent;

impl TernaryAgent for SafetyAgent {
    fn deliberate(&self, _query: &str, ev: &[f32]) -> ExpertVerdict {
        ExpertVerdict {
            trit: if ev.get(5).copied().unwrap_or(0.0) >= 0.0 { 1 } else { -1 },
            confidence: 0.99,
            reasoning: "Safety evaluation complete.".into(),
            expert_id: 6,
            expert_name: "Safety".into(),
        }
    }
}
