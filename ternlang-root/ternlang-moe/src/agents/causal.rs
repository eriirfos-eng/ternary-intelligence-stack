use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct CausalReasonAgent;

impl TernaryAgent for CausalReasonAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();

        // Cause markers
        let cause_markers = ["because ", "due to", "caused by", "as a result of",
                               "owing to", "since ", "triggered by", "leads to",
                               "results in", "produces ", "causes "];
        let cause_count = cause_markers.iter().filter(|&&c| q.contains(c)).count();

        // Effect markers
        let effect_markers = ["therefore", "thus", "hence", "consequently", "as a result",
                                "effect of", "outcome", "impact of", "result is", "so that"];
        let effect_count = effect_markers.iter().filter(|&&e| q.contains(e)).count();

        // False causation / correlation-only markers — weak signal
        let correlation_only = ["correlated", "associated with", "related to", "linked to",
                                  "coincides with", "at the same time as"];
        let correl_count = correlation_only.iter().filter(|&&c| q.contains(c)).count();

        // Reverse causation signals — explicit confusion
        let reverse_markers = ["reverse causation", "chicken and egg", "which came first",
                                 "causality unclear", "confounded"];
        let reverse_count = reverse_markers.iter().filter(|&&r| q.contains(r)).count();

        let ev_signal = ev.get(2).copied().unwrap_or(0.0);

        // Strongest signal: both cause AND effect markers present
        let has_mechanism = cause_count >= 1 && effect_count >= 1;
        let raw = if has_mechanism { 2.0 }
                  else if cause_count + effect_count >= 1 { 0.5 }
                  else { -1.0 }
                  - correl_count as f32 * 0.8 - reverse_count as f32 * 1.5
                  + ev_signal * 1.5;

        let trit: i8 = if reverse_count > 0 || raw < -1.0 { -1 }
                       else if raw > 1.0 { 1 }
                       else { 0 };

        let confidence = (0.72 + ((cause_count + effect_count).min(5) as f32 * 0.04))
            .clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!(
                "Cause markers: {}, effect markers: {}, correlation-only: {}, reverse signals: {}. \
                 Causal chain: {}.",
                cause_count, effect_count, correl_count, reverse_count,
                match trit {
                    1  => "strong mechanism — both cause and effect articulated",
                    0  => "incomplete — one side of the causal chain or correlation only",
                    _  => "unreliable — reverse causation or no causal structure",
                }
            ),
            expert_id: 8,
            expert_name: "CausalReason".into(),
        }
    }
}
