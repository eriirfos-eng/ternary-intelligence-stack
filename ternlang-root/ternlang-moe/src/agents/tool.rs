use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct ToolUseAgent;

impl TernaryAgent for ToolUseAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();

        // Strong imperative verbs — clear tool invocation intent
        let strong_imperatives = ["calculate", "compute", "run ", "execute", "fetch", "search ",
                                    "find ", "plot ", "generate", "build ", "compile", "deploy",
                                    "install", "query", "call ", "invoke", "send ", "download",
                                    "upload", "parse ", "format "];
        let strong_count = strong_imperatives.iter().filter(|&&k| q.contains(k)).count();

        // Weak/ambiguous action verbs — might want a tool, might be discussion
        let weak_imperatives = ["show ", "tell ", "explain", "describe", "what is", "how does",
                                  "can you", "help me", "give me", "list "];
        let weak_count = weak_imperatives.iter().filter(|&&k| q.contains(k)).count();

        // Negative: passive/declarative forms with no action intent
        let passive_markers = ["was done", "has been", "it is known", "i know", "i understand",
                                 "no need to", "don't ", "do not "];
        let passive_count = passive_markers.iter().filter(|&&p| q.contains(p)).count();

        let ev_signal = ev.get(3).copied().unwrap_or(0.0);
        let raw = strong_count as f32 * 1.5 + weak_count as f32 * 0.4
                  - passive_count as f32 * 1.2 + ev_signal * 1.5;

        let trit: i8 = if raw > 1.0 { 1 } else if raw < -0.5 { -1 } else { 0 };
        let confidence = (0.75 + (strong_count.min(4) as f32 * 0.05) + ev_signal.abs() * 0.05)
            .clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!(
                "Strong imperatives: {}, weak/ambiguous: {}, passive disavowals: {}. Tool intent: {}.",
                strong_count, weak_count, passive_count,
                match trit {
                    1  => "clear — explicit tool invocation required",
                    0  => "ambiguous — could be tool or conversational",
                    _  => "absent — declarative or passive, no action requested",
                }
            ),
            expert_id: 4,
            expert_name: "ToolUse".into(),
        }
    }
}
