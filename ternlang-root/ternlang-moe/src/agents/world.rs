use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct WorldKnowledgeAgent;

impl TernaryAgent for WorldKnowledgeAgent {
    fn deliberate(&self, _query: &str, ev: &[f32]) -> ExpertVerdict {
        ExpertVerdict {
            trit: if ev.get(1).copied().unwrap_or(0.0) > 0.0 { 1 } else { 0 },
            confidence: 0.78,
            reasoning: "World knowledge retrieved.".into(),
            expert_id: 1,
            expert_name: "WorldKnowledge".into(),
        }
    }
}
