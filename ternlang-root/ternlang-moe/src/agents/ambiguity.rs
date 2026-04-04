use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct AmbiguityResAgent;

impl TernaryAgent for AmbiguityResAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();

        // Fundamental ambiguity: cannot be resolved without clarification
        let hard_vague = ["something", "stuff", "things", "whatever", "idk",
                           "i don't know what", "not sure what", "unclear what"];
        let hard_count = hard_vague.iter().filter(|&&v| q.contains(v)).count();

        // Resolvable ambiguity: context can narrow interpretation
        let soft_vague = ["maybe", "perhaps", "possibly", "kind of", "sort of",
                           "might be", "could be", "i think", "not certain",
                           "roughly", "approximately", "about "];
        let soft_count = soft_vague.iter().filter(|&&v| q.contains(v)).count();

        // Clarity signals: specificity markers reduce ambiguity
        let clear_markers = ["specifically", "exactly", "precisely", "in particular",
                               "to be clear", "by that i mean", "that is to say",
                               "defined as", "means that"];
        let clear_count = clear_markers.iter().filter(|&&c| q.contains(c)).count();

        // Sentence length heuristic: very short queries are often ambiguous
        let word_count = query.split_whitespace().count();
        let too_short = word_count < 3;

        let ev_avg: f32 = if ev.is_empty() { 0.0 } else { ev.iter().sum::<f32>() / ev.len() as f32 };
        let raw = -(hard_count as f32 * 2.0) - (soft_count as f32 * 0.6)
                  + (clear_count as f32 * 1.5) - (if too_short { 1.0 } else { 0.0 })
                  + ev_avg * 1.0;

        // Note: for AmbiguityRes, +1 = CLEAR (unambiguous), -1 = AMBIGUOUS (needs clarification)
        let trit: i8 = if hard_count >= 2 || raw < -1.5 { -1 }
                       else if raw < -0.3 { 0 }
                       else { 1 };

        let confidence = (0.75 - (hard_count as f32 * 0.1) + (clear_count as f32 * 0.05))
            .clamp(0.3, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!(
                "Hard-vague markers: {}, soft-vague: {}, clarity signals: {}, word count: {}. Clarity: {}.",
                hard_count, soft_count, clear_count, word_count,
                match trit {
                    1  => "clear — sufficient specificity to proceed",
                    0  => "resolvable ambiguity — interpretation is possible but imprecise",
                    _  => "unresolvable — clarification required before proceeding",
                }
            ),
            expert_id: 9,
            expert_name: "AmbiguityRes".into(),
        }
    }
}
