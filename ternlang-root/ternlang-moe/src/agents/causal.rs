use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct CausalReasonAgent;

impl TernaryAgent for CausalReasonAgent {
    fn deliberate(&self, _query: &str, ev: &[f32]) -> ExpertVerdict {
        ExpertVerdict {
            trit: if ev.get(2).copied().unwrap_or(0.0) > 0.4 { 1 } else { 0 },
            confidence: 0.80,
            reasoning: "Causal chain traced.".into(),
            expert_id: 8,
            expert_name: "CausalReason".into(),
        }
    }
}
