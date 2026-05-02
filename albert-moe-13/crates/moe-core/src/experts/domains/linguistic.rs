use crate::experts::ExpertLogic;
use super::domain_score;

pub struct LinguisticExpert;

impl ExpertLogic for LinguisticExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        if score > 0.05 { 1 } else if score < -0.05 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        // Syntax is dominant (grammar, tokenisation, structure) + persona (register/tone) + world knowledge
        [1.0, 0.6, 0.5, 0.2, 0.7, 0.2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linguistic_positive() { assert_eq!(LinguisticExpert.evaluate(&vec![1.0f32; 64]), 1); }

    #[test]
    fn test_linguistic_negative() { assert_eq!(LinguisticExpert.evaluate(&vec![-1.0f32; 64]), -1); }

    #[test]
    fn test_linguistic_competence() {
        let p = LinguisticExpert.competence();
        let max = p.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        assert_eq!(p[0], max, "syntax must dominate linguistic expert");
    }
}
