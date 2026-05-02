use crate::experts::ExpertLogic;
use super::domain_score;

pub struct EcologicalExpert;

impl ExpertLogic for EcologicalExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        // Ecological: conservative like ethics/medical — environmental harm is hard to reverse
        if score > 0.08 { 1 } else if score < -0.04 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        // World knowledge (environmental science) + safety (harm prevention) + reasoning
        [0.0, 0.9, 0.7, 0.3, 0.2, 0.9]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecological_positive() { assert_eq!(EcologicalExpert.evaluate(&vec![1.0f32; 64]), 1); }

    #[test]
    fn test_ecological_negative() { assert_eq!(EcologicalExpert.evaluate(&vec![-1.0f32; 64]), -1); }

    #[test]
    fn test_ecological_conservative() {
        let query = vec![-0.1f32; 64];
        let v = EcologicalExpert.evaluate(&query);
        assert!(v == -1 || v == 0, "ecological must be conservative on negative signals");
    }

    #[test]
    fn test_ecological_competence() {
        let p = EcologicalExpert.competence();
        assert!(p[1] >= 0.8 && p[5] >= 0.8, "world_knowledge and safety must be co-dominant");
    }
}
