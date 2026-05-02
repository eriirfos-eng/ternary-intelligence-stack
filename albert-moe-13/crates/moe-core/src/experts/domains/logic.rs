use crate::experts::ExpertLogic;
use super::domain_score;

pub struct LogicExpert;

impl ExpertLogic for LogicExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        if score > 0.05 { 1 } else if score < -0.05 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        // Reasoning (axis 2) is dominant for formal consistency checking
        [0.2, 0.4, 1.0, 0.3, 0.0, 0.6]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic_positive_query() {
        let expert = LogicExpert;
        let query = vec![1.0f32; 64];
        assert_eq!(expert.evaluate(&query), 1);
    }

    #[test]
    fn test_logic_negative_query() {
        let expert = LogicExpert;
        let query = vec![-1.0f32; 64];
        assert_eq!(expert.evaluate(&query), -1);
    }

    #[test]
    fn test_logic_competence_profile() {
        let expert = LogicExpert;
        let profile = expert.competence();
        // Reasoning (axis 2) must be highest weight for logic expert
        let max = profile.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        assert_eq!(profile[2], max, "reasoning axis must dominate logic expert");
    }
}
