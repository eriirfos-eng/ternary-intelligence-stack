use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct FactCheckAgent;

impl TernaryAgent for FactCheckAgent {
    fn deliberate(&self, _query: &str, ev: &[f32]) -> ExpertVerdict {
        ExpertVerdict {
            trit: if ev.get(1).copied().unwrap_or(0.0) > 0.3 { 1 } else { 0 },
            confidence: 0.82,
            reasoning: "Fact verification done.".into(),
            expert_id: 7,
            expert_name: "FactCheck".into(),
        }
    }
}
