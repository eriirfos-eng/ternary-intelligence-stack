use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct CausalReasonAgent;

impl TernaryAgent for CausalReasonAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();
        let causal_markers = ["because", "due to", "leads to", "result", "since", "effect", "consequence"];
        let count = causal_markers.iter().filter(|&&k| q.contains(k)).count();
        
        let ev_signal = ev.get(2).copied().unwrap_or(0.0);
        let trit = if count >= 1 || ev_signal > 0.4 { 1 } else { 0 };
        let confidence = (0.75 + (count as f32 * 0.1).min(0.2)).clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!("Causal linkages detected: {}. Analyzing cause-effect chains.", count),
            expert_id: 8,
            expert_name: "CausalReason".into(),
        }
    }
}
