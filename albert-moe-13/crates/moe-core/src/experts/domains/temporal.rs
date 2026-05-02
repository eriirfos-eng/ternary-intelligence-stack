use crate::experts::ExpertLogic;
use super::domain_score;

pub struct TemporalExpert;

impl ExpertLogic for TemporalExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        if score > 0.05 { 1 } else if score < -0.05 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        // World knowledge (event timelines) + reasoning (sequence logic) + tool_use (calendars/clocks)
        [0.2, 0.8, 0.7, 0.6, 0.0, 0.3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_positive() { assert_eq!(TemporalExpert.evaluate(&vec![1.0f32; 64]), 1); }

    #[test]
    fn test_temporal_negative() { assert_eq!(TemporalExpert.evaluate(&vec![-1.0f32; 64]), -1); }

    #[test]
    fn test_temporal_competence() {
        let p = TemporalExpert.competence();
        let max = p.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        assert_eq!(p[1], max, "world_knowledge must dominate temporal expert");
    }
}
