use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct AmbiguityResAgent;

impl TernaryAgent for AmbiguityResAgent {
    fn deliberate(&self, _query: &str, ev: &[f32]) -> ExpertVerdict {
        ExpertVerdict {
            trit: {
                let avg: f32 = ev.iter().take(6).sum::<f32>() / 6.0_f32.max(ev.len() as f32);
                if avg > 0.1 {
                    1
                } else if avg < -0.1 {
                    -1
                } else {
                    0
                }
            },
            confidence: 0.73,
            reasoning: "Ambiguity resolved via averaging.".into(),
            expert_id: 9,
            expert_name: "AmbiguityRes".into(),
        }
    }
}
