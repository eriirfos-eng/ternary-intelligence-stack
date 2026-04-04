use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct ContextMemAgent;

impl TernaryAgent for ContextMemAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();

        // Strong context references: anaphora and discourse continuity markers
        let strong_ref = [" it ", " they ", " this ", " that ", " these ", " those ",
                           "previously", "as mentioned", "as stated", "as discussed",
                           "above", "earlier", "you said", "we said", "the previous"];
        let strong_count = strong_ref.iter().filter(|&&r| q.contains(r)).count();

        // Topic continuation markers
        let continuation = ["also", "furthermore", "additionally", "moreover", "continuing",
                              "following up", "back to", "regarding that", "on that note",
                              "in that case", "given that context"];
        let cont_count = continuation.iter().filter(|&&c| q.contains(c)).count();

        // Fresh-start signals: explicit indication there is no prior context
        let fresh_start = ["to start", "first of all", "from the beginning", "brand new",
                             "starting fresh", "clean slate", "initial query",
                             "no context", "without context"];
        let fresh_count = fresh_start.iter().filter(|&&f| q.contains(f)).count();

        // Contradiction with likely prior: signals inconsistent continuation
        let contradiction_ref = ["no, i said", "that's not what", "i didn't mean",
                                    "you misunderstood", "not that, but"];
        let contradict_count = contradiction_ref.iter().filter(|&&c| q.contains(c)).count();

        let ev_signal = ev.get(0).copied().unwrap_or(0.0);
        let raw = strong_count as f32 * 1.5 + cont_count as f32 * 0.6
                  - fresh_count as f32 * 1.5 - contradict_count as f32 * 1.0
                  + ev_signal * 1.5;

        let trit: i8 = if raw > 1.0 { 1 }
                       else if raw < -0.8 { -1 }
                       else { 0 };

        let confidence = (0.65 + ((strong_count + cont_count).min(5) as f32 * 0.04))
            .clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!(
                "Strong references: {}, continuation: {}, fresh-start: {}, contradictions: {}. \
                 Context continuity: {}.",
                strong_count, cont_count, fresh_count, contradict_count,
                match trit {
                    1  => "strong — clear anaphoric references to prior context",
                    0  => "weak — marginal context dependency, may be self-contained",
                    _  => "absent or contradicted — no prior context, or conflict with it",
                }
            ),
            expert_id: 11,
            expert_name: "ContextMem".into(),
        }
    }
}
