use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct ToolUseAgent;

impl TernaryAgent for ToolUseAgent {
    fn deliberate(&self, _query: &str, ev: &[f32]) -> ExpertVerdict {
        ExpertVerdict {
            trit: if ev.get(3).copied().unwrap_or(0.0) > 0.0 { 1 } else { 0 },
            confidence: 0.88,
            reasoning: "Tool invocation planned.".into(),
            expert_id: 4,
            expert_name: "ToolUse".into(),
        }
    }
}
