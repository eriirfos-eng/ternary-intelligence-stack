use crate::experts::ExpertLogic;
use super::domain_score;

pub struct MedicalExpert;

impl ExpertLogic for MedicalExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        // Medical: asymmetric like ethics — safety-critical domain, conservative threshold
        if score > 0.09 { 1 } else if score < -0.03 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        // Safety is co-dominant with world_knowledge (clinical evidence base)
        [0.1, 1.0, 0.7, 0.3, 0.2, 1.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_medical_positive() { assert_eq!(MedicalExpert.evaluate(&vec![1.0f32; 64]), 1); }

    #[test]
    fn test_medical_negative() { assert_eq!(MedicalExpert.evaluate(&vec![-1.0f32; 64]), -1); }

    #[test]
    fn test_medical_conservative_threshold() {
        // Mild negative should reject (safety-critical)
        let query = vec![-0.1f32; 64];
        let v = MedicalExpert.evaluate(&query);
        assert!(v == -1 || v == 0, "medical must be conservative on negative signals");
    }

    #[test]
    fn test_medical_competence() {
        let p = MedicalExpert.competence();
        // Safety and world_knowledge are co-dominant
        assert!(p[1] >= 0.9 && p[5] >= 0.9, "world_knowledge and safety must be co-dominant");
    }
}
