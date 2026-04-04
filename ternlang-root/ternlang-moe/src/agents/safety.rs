use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct SafetyAgent;

impl TernaryAgent for SafetyAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();
        let dangerous = ["harm", "exploit", "steal", "hack", "kill", "bomb", "bypass", "illegal"];
        let risk_count = dangerous.iter().filter(|&&k| q.contains(k)).count();
        
        let ev_signal = ev.get(5).copied().unwrap_or(0.0);
        let trit = if risk_count > 0 || ev_signal < -0.3 { -1 } else if ev_signal < 0.2 { 0 } else { 1 };
        let confidence = 0.99;

        ExpertVerdict {
            trit,
            confidence,
            reasoning: if trit == -1 {
                format!("CRITICAL: Found {} sensitive keywords. Risk detected.", risk_count)
            } else {
                "No immediate safety risks detected in query string.".into()
            },
            expert_id: 6,
            expert_name: "Safety".into(),
        }
    }
}
