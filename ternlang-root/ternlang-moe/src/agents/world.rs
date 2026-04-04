use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct WorldKnowledgeAgent;

impl TernaryAgent for WorldKnowledgeAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let entities = ["Earth", "Mars", "Paris", "London", "WWII", "Einstein", "Gravity", "Python"];
        let mut entity_count = 0;
        for word in query.split_whitespace() {
            if !word.is_empty() && word.chars().next().unwrap().is_uppercase() {
                entity_count += 1;
            }
        }
        for ent in entities {
            if query.contains(ent) { entity_count += 1; }
        }
        
        let ev_signal = ev.get(1).copied().unwrap_or(0.0);
        let trit = if entity_count > 2 || ev_signal > 0.4 { 1 } else if entity_count > 0 { 0 } else { -1 };
        let confidence = (0.6 + (entity_count as f32 * 0.08).min(0.3) + ev_signal.abs() * 0.1).clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!("Detected {} potential entities/proper nouns. Entity density: {:.2} entities/word.", entity_count, entity_count as f32 / query.split_whitespace().count().max(1) as f32),
            expert_id: 1,
            expert_name: "WorldKnowledge".into(),
        }
    }
}
