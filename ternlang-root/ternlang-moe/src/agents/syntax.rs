use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct SyntaxAgent;

impl TernaryAgent for SyntaxAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        // Positive: rich, well-formed structural tokens
        let structure_tokens = ["fn ", "let ", "match ", "->", "::", "impl ", "struct ", "```",
                                 "pub ", "use ", "return ", "if ", "else ", "for ", "while "];
        let rich_punct = ["(", ")", "[", "]", "{", "}", ";", ":"];
        let pos = structure_tokens.iter().map(|s| query.matches(s).count()).sum::<usize>()
                + rich_punct.iter().map(|s| query.matches(s).count()).sum::<usize>();

        // Negative: bracket imbalance signals malformed structure
        let opens  = query.chars().filter(|&c| c == '(' || c == '[' || c == '{').count();
        let closes = query.chars().filter(|&c| c == ')' || c == ']' || c == '}').count();
        let imbalance = (opens as i32 - closes as i32).unsigned_abs() as usize;

        // Hold detector: some structure present but ambiguous (single token, no context)
        let words = query.split_whitespace().count();
        let density = if words > 0 { pos as f32 / words as f32 } else { 0.0 };

        let ev_signal = ev.get(0).copied().unwrap_or(0.0);
        let raw = pos as f32 * 0.4 - imbalance as f32 * 1.8 + ev_signal * 1.5;

        let trit: i8 = if raw > 1.2 { 1 } else if raw < -1.0 || imbalance > 3 { -1 } else { 0 };

        let confidence = (0.60 + (density * 0.25).min(0.35) + ev_signal.abs() * 0.05)
            .clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!(
                "Structural tokens: {}, bracket imbalance: {}, token density: {:.2}. Structure is {}.",
                pos, imbalance, density,
                match trit {
                    1  => "well-formed and rich",
                    0  => "partial — holding for structural completeness",
                    _  => "malformed or critically unbalanced",
                }
            ),
            expert_id: 0,
            expert_name: "Syntax".into(),
        }
    }
}
