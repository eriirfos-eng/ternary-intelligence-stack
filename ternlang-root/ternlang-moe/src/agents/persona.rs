use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct PersonaAgent;

impl TernaryAgent for PersonaAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();
        let keywords = ["i ", "me ", "you", "we ", "opinion", "feel", "think", "my "];
        let count = keywords.iter().filter(|&&k| q.contains(k)).count();
        
        let ev_signal = ev.get(4).copied().unwrap_or(0.0);
        let trit = if count >= 2 || ev_signal > 0.6 { 1 } else if count >= 1 { 0 } else { -1 };
        let confidence = (0.6 + (count as f32 * 0.1).min(0.4)).clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!("Self-referential or tone-sensitive markers count: {}. Identity-level engagement is high.", count),
            expert_id: 5,
            expert_name: "Persona".into(),
        }
    }
}
