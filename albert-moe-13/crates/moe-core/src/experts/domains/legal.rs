use crate::experts::ExpertLogic;
use super::domain_score;

pub struct LegalExpert;

impl ExpertLogic for LegalExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        // Legal is conservative: requires stronger positive signal to affirm,
        // rejects on any meaningful negative (compliance risk)
        if score > 0.09 { 1 } else if score < -0.04 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        // Heavy on world_knowledge (law corpus) + reasoning (statutory interpretation) + safety
        [0.3, 0.9, 0.8, 0.1, 0.2, 0.8]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_positive() { assert_eq!(LegalExpert.evaluate(&vec![1.0f32; 64]), 1); }

    #[test]
    fn test_legal_negative() { assert_eq!(LegalExpert.evaluate(&vec![-1.0f32; 64]), -1); }

    #[test]
    fn test_legal_competence() {
        let p = LegalExpert.competence();
        // World knowledge must be dominant for legal
        let max = p.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        assert_eq!(p[1], max);
    }
}
