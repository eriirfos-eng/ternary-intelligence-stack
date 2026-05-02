use crate::experts::ExpertLogic;
use super::domain_score;

pub struct SpatialExpert;

impl ExpertLogic for SpatialExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        if score > 0.05 { 1 } else if score < -0.05 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        // Tool use (geometry/mapping APIs) + reasoning (spatial inference) + world knowledge (geography)
        [0.1, 0.7, 0.8, 0.9, 0.0, 0.2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_positive() { assert_eq!(SpatialExpert.evaluate(&vec![1.0f32; 64]), 1); }

    #[test]
    fn test_spatial_negative() { assert_eq!(SpatialExpert.evaluate(&vec![-1.0f32; 64]), -1); }

    #[test]
    fn test_spatial_competence() {
        let p = SpatialExpert.competence();
        let max = p.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        assert_eq!(p[3], max, "tool_use must dominate spatial expert");
    }
}
