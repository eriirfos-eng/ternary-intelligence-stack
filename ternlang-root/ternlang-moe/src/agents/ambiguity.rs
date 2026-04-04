use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct AmbiguityResAgent;

impl TernaryAgent for AmbiguityResAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();
        let vague = ["maybe", "something", "perhaps", "kind of", "sort of", "unclear", "possible"];
        let count = vague.iter().filter(|&&k| q.contains(k)).count();
        
        let _ev_avg: f32 = ev.iter().sum::<f32>() / ev.len().max(1) as f32;
        let trit = if count > 2 { -1 } else if count > 0 { 0 } else { 1 };
        let confidence = (0.8 - (count as f32 * 0.1)).clamp(0.1, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!("Detected {} vague semantic markers. Clarity score: {:.2}.", count, 1.0 - (count as f32 / 10.0).min(1.0)),
            expert_id: 9,
            expert_name: "AmbiguityRes".into(),
        }
    }
}
