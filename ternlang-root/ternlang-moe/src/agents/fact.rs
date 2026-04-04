use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct FactCheckAgent;

impl TernaryAgent for FactCheckAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();
        let patterns = ["is true", "fact", "date", "who is", "what is", "when", "how many"];
        let count = patterns.iter().filter(|&&k| q.contains(k)).count();
        let has_digits = query.chars().any(|c| c.is_ascii_digit());
        
        let ev_signal = ev.get(1).copied().unwrap_or(0.0);
        let trit = if count > 0 || has_digits || ev_signal > 0.3 { 1 } else { 0 };
        let confidence = (0.7f32 + (if has_digits { 0.15f32 } else { 0.0f32 })).clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!("Query contains {} fact-seeking patterns. Verifiability check suggested.", count + (has_digits as usize)),
            expert_id: 7,
            expert_name: "FactCheck".into(),
        }
    }
}
