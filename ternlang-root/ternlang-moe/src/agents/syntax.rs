use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct SyntaxAgent;

impl TernaryAgent for SyntaxAgent {
    fn deliberate(&self, _query: &str, ev: &[f32]) -> ExpertVerdict {
        ExpertVerdict {
            trit: if ev.get(0).copied().unwrap_or(0.0) > 0.3 { 1 } else { 0 },
            confidence: 0.85,
            reasoning: "Syntax analysis complete.".into(),
            expert_id: 0,
            expert_name: "Syntax".into(),
        }
    }
}
