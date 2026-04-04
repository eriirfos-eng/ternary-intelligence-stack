use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct ContextMemAgent;

impl TernaryAgent for ContextMemAgent {
    fn deliberate(&self, _query: &str, ev: &[f32]) -> ExpertVerdict {
        ExpertVerdict {
            trit: if ev.get(0).copied().unwrap_or(0.0) > -0.5 { 1 } else { 0 },
            confidence: 0.77,
            reasoning: "Context retrieved from memory.".into(),
            expert_id: 11,
            expert_name: "ContextMem".into(),
        }
    }
}
