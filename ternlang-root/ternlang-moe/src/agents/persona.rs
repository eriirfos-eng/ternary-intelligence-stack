use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct PersonaAgent;

impl TernaryAgent for PersonaAgent {
    fn deliberate(&self, _query: &str, ev: &[f32]) -> ExpertVerdict {
        ExpertVerdict {
            trit: if ev.get(4).copied().unwrap_or(0.0) >= 0.0 { 1 } else { -1 },
            confidence: 0.70,
            reasoning: "Persona alignment checked.".into(),
            expert_id: 5,
            expert_name: "Persona".into(),
        }
    }
}
