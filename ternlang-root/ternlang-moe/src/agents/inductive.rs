use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct InductiveReasonAgent;

impl TernaryAgent for InductiveReasonAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();

        // Example signals — observations that seed induction
        let example_markers = ["for example", "for instance", "such as", "like ", "e.g.", "observe",
                                 "we see that", "in case", "typically", "usually", "often", "tend to",
                                 "in most cases"];
        let example_count = example_markers.iter().filter(|&&e| q.contains(e)).count();

        // Generalisation signals — the inductive leap
        let generalisation_markers = ["in general", "always", "every ", "all ", "never ", "pattern",
                                        "rule ", "law ", "principle", "universally", "across all"];
        let gen_count = generalisation_markers.iter().filter(|&&g| q.contains(g)).count();

        // Counter-example signals — evidence that weakens induction
        let counter_markers = ["except", "but not", "unless", "however", "counter", "anomaly",
                                 "outlier", "does not follow", "exception"];
        let counter_count = counter_markers.iter().filter(|&&c| q.contains(c)).count();

        let ev_signal = ev.get(2).copied().unwrap_or(0.0);

        // Induction requires examples AND generalisation leap
        let has_examples    = example_count >= 1;
        let has_gen_leap    = gen_count >= 1;
        let has_counterexam = counter_count >= 1;

        let raw = match (has_examples, has_gen_leap) {
            (true, true)   =>  2.0,
            (true, false)  =>  0.3,
            (false, true)  => -0.3,
            (false, false) => -1.5,
        } + ev_signal * 1.5 - counter_count as f32 * 1.2;

        let trit: i8 = if raw > 1.2 && !has_counterexam { 1 }
                       else if raw < -1.0 { -1 }
                       else { 0 };

        let confidence = (0.62 + ((example_count + gen_count).min(6) as f32 * 0.04))
            .clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!(
                "Example markers: {}, generalisation markers: {}, counter-examples: {}. Inductive quality: {}.",
                example_count, gen_count, counter_count,
                match trit {
                    1  => "strong — examples and generalisation present",
                    0  => "developing — examples or generalisation incomplete",
                    _  => "undermined — counter-examples or no observable base",
                }
            ),
            expert_id: 3,
            expert_name: "InductiveReason".into(),
        }
    }
}
