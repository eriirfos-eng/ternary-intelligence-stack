use crate::experts::ExpertLogic;
use super::domain_score;

pub struct TechnicalExpert;

impl ExpertLogic for TechnicalExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        if score > 0.05 { 1 } else if score < -0.05 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        // Tool use dominant (APIs, systems, execution) + syntax (code structure) + reasoning
        [0.8, 0.5, 0.7, 1.0, 0.0, 0.4]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_technical_positive() { assert_eq!(TechnicalExpert.evaluate(&vec![1.0f32; 64]), 1); }

    #[test]
    fn test_technical_negative() { assert_eq!(TechnicalExpert.evaluate(&vec![-1.0f32; 64]), -1); }

    #[test]
    fn test_technical_competence() {
        let p = TechnicalExpert.competence();
        let max = p.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        assert_eq!(p[3], max, "tool_use must dominate technical expert");
    }
}
