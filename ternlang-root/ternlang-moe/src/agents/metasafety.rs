use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct MetaSafetyAgent;

impl TernaryAgent for MetaSafetyAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();
        let patterns = ["ignore previous", "system prompt", "as a game", "you are now", " DAN ", " jailbreak "];
        let count = patterns.iter().filter(|&&p| q.contains(p)).count();
        
        let ev_signal = ev.get(5).copied().unwrap_or(0.0);
        let trit = if count > 0 || ev_signal < -0.5 { -1 } else { 1 };
        let confidence = 0.98;

        ExpertVerdict {
            trit,
            confidence,
            reasoning: if count > 0 {
                format!("ALERT: Detected {} meta-adversarial patterns. Potential injection attempt.", count)
            } else {
                "Meta-safety audit found no adversarial patterns.".into()
            },
            expert_id: 12,
            expert_name: "MetaSafety".into(),
        }
    }
}
