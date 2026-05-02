use crate::experts::ExpertLogic;
use super::domain_score;

pub struct CausalExpert;

impl ExpertLogic for CausalExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        // Causal reasoning uses standard symmetric thresholds
        if score > 0.05 { 1 } else if score < -0.05 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        // Reasoning is dominant — causal chains require deep structural inference
        [0.1, 0.6, 1.0, 0.4, 0.0, 0.4]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_causal_positive() { assert_eq!(CausalExpert.evaluate(&vec![1.0f32; 64]), 1); }

    #[test]
    fn test_causal_negative() { assert_eq!(CausalExpert.evaluate(&vec![-1.0f32; 64]), -1); }

    #[test]
    fn test_causal_competence() {
        let p = CausalExpert.competence();
        let max = p.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        assert_eq!(p[2], max, "reasoning must dominate causal expert");
    }
}
