use crate::experts::ExpertLogic;
use super::domain_score;

pub struct CulturalExpert;

impl ExpertLogic for CulturalExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        if score > 0.05 { 1 } else if score < -0.05 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        // Persona dominant (tone, cultural register) + world_knowledge (context) + safety (bias risk)
        [0.3, 0.8, 0.4, 0.1, 1.0, 0.6]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cultural_positive() { assert_eq!(CulturalExpert.evaluate(&vec![1.0f32; 64]), 1); }

    #[test]
    fn test_cultural_negative() { assert_eq!(CulturalExpert.evaluate(&vec![-1.0f32; 64]), -1); }

    #[test]
    fn test_cultural_competence() {
        let p = CulturalExpert.competence();
        let max = p.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        assert_eq!(p[4], max, "persona must dominate cultural expert");
    }
}
