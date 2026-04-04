use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct FactCheckAgent;

impl TernaryAgent for FactCheckAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();

        // Verifiable signals: claims that can be checked against a source
        let verifiable = ["is true", "is false", "is it true", "fact:", "confirmed", "verified",
                           "according to", "studies show", "research shows", "published",
                           "source:", "citation", "as of ", "data shows"];
        let verifiable_count = verifiable.iter().filter(|&&v| q.contains(v)).count();

        // Temporal/numeric anchors increase verifiability
        let has_date   = query.chars().any(|c| c.is_ascii_digit())
                         && (q.contains("19") || q.contains("20") || q.contains("year ")
                              || q.contains("date") || q.contains("ago"));
        let has_number = query.chars().any(|c| c.is_ascii_digit());

        // Hallucination risk markers: confident-sounding but unverifiable claims
        let hallucination_risk = ["everyone knows", "it is well known", "obviously",
                                    "of course", "always has been", "as we all know",
                                    "definitely true", "100% certain"];
        let risk_count = hallucination_risk.iter().filter(|&&h| q.contains(h)).count();

        let ev_signal = ev.get(1).copied().unwrap_or(0.0);
        let verif_score = verifiable_count as f32 + (if has_date { 0.8 } else { 0.0 })
                          + (if has_number { 0.4 } else { 0.0 });
        let raw = verif_score * 0.8 - risk_count as f32 * 2.0 + ev_signal * 1.5;

        let trit: i8 = if risk_count > 0 { -1 }
                       else if raw > 0.8 { 1 }
                       else { 0 };

        let confidence = (0.68 + (verif_score * 0.06).min(0.28)).clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!(
                "Verifiable anchors: {:.1} (date={}, numeric={}), hallucination markers: {}. Factuality: {}.",
                verif_score, has_date, has_number, risk_count,
                match trit {
                    1  => "grounded — verifiable anchors present",
                    0  => "partially verifiable — some factual content, insufficient anchoring",
                    _  => "at risk — overconfident phrasing without verifiable basis",
                }
            ),
            expert_id: 7,
            expert_name: "FactCheck".into(),
        }
    }
}
