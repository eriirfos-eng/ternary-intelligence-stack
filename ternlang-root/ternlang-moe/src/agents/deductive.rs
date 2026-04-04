use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct DeductiveReasonAgent;

impl TernaryAgent for DeductiveReasonAgent {
    fn deliberate(&self, _query: &str, ev: &[f32]) -> ExpertVerdict {
        ExpertVerdict {
            trit: {
                let r = ev.get(2).copied().unwrap_or(0.0);
                if r > 0.5 {
                    1
                } else if r < -0.3 {
                    -1
                } else {
                    0
                }
            },
            confidence: 0.90,
            reasoning: "Deductive chain evaluated.".into(),
            expert_id: 2,
            expert_name: "DeductiveReason".into(),
        }
    }
}
