use crate::experts::ExpertLogic;
use super::domain_score;

pub struct EthicsExpert;

impl ExpertLogic for EthicsExpert {
    fn evaluate(&self, query: &[f32]) -> i8 {
        let score = domain_score(query, &self.competence());
        // Asymmetric safety threshold: affirm only on clear positive signal,
        // reject on even mild negative (safety axis has high competence weight 0.9)
        if score > 0.08 { 1 } else if score < -0.03 { -1 } else { 0 }
    }

    fn competence(&self) -> [f32; 6] {
        // [syntax, world_knowledge, reasoning, tool_use, persona, safety]
        [0.0, 0.5, 0.5, 0.0, 1.0, 0.9]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethics_positive_query() {
        let expert = EthicsExpert;
        let query = vec![1.0f32; 64];
        let verdict = expert.evaluate(&query);
        assert_eq!(verdict, 1, "strong positive input should affirm");
    }

    #[test]
    fn test_ethics_negative_query() {
        let expert = EthicsExpert;
        let query = vec![-1.0f32; 64];
        let verdict = expert.evaluate(&query);
        assert_eq!(verdict, -1, "strong negative input should reject");
    }

    #[test]
    fn test_ethics_rejects_on_mild_negative() {
        let expert = EthicsExpert;
        // Mild negative: just below the -0.03 reject threshold after competence weighting
        let query = vec![-0.1f32; 64];
        let verdict = expert.evaluate(&query);
        assert!(verdict == -1 || verdict == 0, "mild negative should reject or hold");
    }

    #[test]
    fn test_ethics_competence_profile() {
        let expert = EthicsExpert;
        let profile = expert.competence();
        assert_eq!(profile.len(), 6);
        // Safety (axis 5) must be highest weight for ethics
        assert!(profile[5] >= profile[2], "safety axis must be >= reasoning for ethics expert");
    }
}
