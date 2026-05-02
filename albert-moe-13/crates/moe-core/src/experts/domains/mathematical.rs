use crate::experts::ExpertLogic;
use super::domain_score;

pub struct MathematicalExpert;

impl ExpertLogic for MathematicalExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        if score > 0.05 { 1 } else if score < -0.05 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        // Reasoning dominant (formal proof) + tool_use (symbolic computation) + syntax (notation)
        [0.6, 0.4, 1.0, 0.8, 0.0, 0.3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mathematical_positive() { assert_eq!(MathematicalExpert.evaluate(&vec![1.0f32; 64]), 1); }

    #[test]
    fn test_mathematical_negative() { assert_eq!(MathematicalExpert.evaluate(&vec![-1.0f32; 64]), -1); }

    #[test]
    fn test_mathematical_competence() {
        let p = MathematicalExpert.competence();
        let max = p.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        assert_eq!(p[2], max, "reasoning must dominate mathematical expert");
    }
}
