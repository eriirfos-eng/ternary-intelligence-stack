use crate::experts::ExpertLogic;
use super::domain_score;

pub struct ScienceExpert;

impl ExpertLogic for ScienceExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        if score > 0.05 { 1 } else if score < -0.05 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        // World knowledge (axis 1) and reasoning (axis 2) drive factual verification
        [0.1, 1.0, 0.8, 0.2, 0.0, 0.5]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_science_positive_query() {
        let expert = ScienceExpert;
        let query = vec![1.0f32; 64];
        assert_eq!(expert.evaluate(&query), 1);
    }

    #[test]
    fn test_science_negative_query() {
        let expert = ScienceExpert;
        let query = vec![-1.0f32; 64];
        assert_eq!(expert.evaluate(&query), -1);
    }

    #[test]
    fn test_science_competence_profile() {
        let expert = ScienceExpert;
        let profile = expert.competence();
        // World knowledge (axis 1) must be highest weight for science expert
        let max = profile.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        assert_eq!(profile[1], max, "world_knowledge axis must dominate science expert");
    }

    #[test]
    fn test_experts_produce_different_verdicts_for_mixed_input() {
        use crate::experts::domains::{ethics::EthicsExpert, logic::LogicExpert};
        // Input tilted toward reasoning axis (every 3rd element positive, others negative)
        // This should favour LogicExpert over EthicsExpert
        let query: Vec<f32> = (0..60).map(|i| if i % 6 == 2 { 1.0 } else { -0.5 }).collect();
        let ethics_verdict = EthicsExpert.evaluate(&query);
        let logic_verdict = LogicExpert.evaluate(&query);
        let science_verdict = ScienceExpert.evaluate(&query);
        // They should not all agree — proves competence profiles differentiate
        let all_same = ethics_verdict == logic_verdict && logic_verdict == science_verdict;
        assert!(!all_same, "domain experts must diverge on axis-skewed input: ethics={} logic={} science={}", ethics_verdict, logic_verdict, science_verdict);
    }
}
