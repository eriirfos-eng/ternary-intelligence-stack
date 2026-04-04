use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct ContextMemAgent;

impl TernaryAgent for ContextMemAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();
        let anaphora = [" it ", " they ", " that ", " previously ", " mentioned ", " before ", " above "];
        let count = anaphora.iter().filter(|&&a| q.contains(a)).count();
        
        let ev_signal = ev.get(0).copied().unwrap_or(0.0);
        let trit = if count >= 1 || ev_signal > 0.3 { 1 } else { 0 };
        let confidence = (0.7 + (count as f32 * 0.1).min(0.3)).clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!("Detected {} anaphoric references. High continuity with prior context suspected.", count),
            expert_id: 11,
            expert_name: "ContextMem".into(),
        }
    }
}
