use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct DeductiveReasonAgent;

impl TernaryAgent for DeductiveReasonAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();

        // Premise markers — the "if" side of an argument
        let premise_markers = ["if ", "since ", "given that", "assuming ", "suppose ",
                                 "because ", "as ", "from the fact"];
        let premise_count = premise_markers.iter().filter(|&&p| q.contains(p)).count();

        // Conclusion markers — the "then" side
        let conclusion_markers = ["therefore", "thus", "hence", "it follows", "consequently",
                                    "we can conclude", "must be", "necessarily", "implies that",
                                    "then ", "so "];
        let conclusion_count = conclusion_markers.iter().filter(|&&c| q.contains(c)).count();

        // Contradiction signals — explicit logical conflicts
        let contradiction_markers = ["but also", "contradicts", "cannot both", "inconsistent",
                                       "yet also", "paradox", "both true and false"];
        let contradiction_count = contradiction_markers.iter().filter(|&&c| q.contains(c)).count();

        let ev_signal = ev.get(2).copied().unwrap_or(0.0);

        // A complete deductive structure needs BOTH premises AND conclusions
        let has_full_chain = premise_count >= 1 && conclusion_count >= 1;
        let has_partial    = premise_count >= 1 || conclusion_count >= 1;

        let raw = if has_full_chain { 2.0 }
                  else if has_partial { 0.0 }
                  else { -1.5 }
                  + ev_signal * 1.5
                  - contradiction_count as f32 * 2.5;

        let trit: i8 = if contradiction_count > 0 { -1 }
                       else if raw > 1.0 { 1 }
                       else if raw < -1.0 { -1 }
                       else { 0 };

        let confidence = (0.70 + ((premise_count + conclusion_count).min(6) as f32 * 0.04))
            .clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!(
                "Premise markers: {}, conclusion markers: {}, contradictions: {}. Chain is {}.",
                premise_count, conclusion_count, contradiction_count,
                match trit {
                    1  => "complete — both premises and conclusion present",
                    0  => "partial — one side of the deductive chain is missing",
                    _  => "broken — contradiction detected or chain absent",
                }
            ),
            expert_id: 2,
            expert_name: "DeductiveReason".into(),
        }
    }
}
