use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct ToolUseAgent;

impl TernaryAgent for ToolUseAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();
        let imperatives = ["calculate", "find", "search", "run", "execute", "compute", "fetch", "get", "plot"];
        let count = imperatives.iter().filter(|&&k| q.contains(k)).count();
        
        let ev_signal = ev.get(3).copied().unwrap_or(0.0);
        let trit = if count >= 1 || ev_signal > 0.5 { 1 } else if ev_signal > 0.0 { 0 } else { -1 };
        let confidence = (0.8 + (count as f32 * 0.1).min(0.2)).clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!("Found {} imperative verbs signaling tool requirement. High actionability detected.", count),
            expert_id: 4,
            expert_name: "ToolUse".into(),
        }
    }
}
