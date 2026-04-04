use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct MetaSafetyAgent;

impl TernaryAgent for MetaSafetyAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();

        // Hard adversarial injection patterns — immediate -1
        let hard_injection = ["ignore previous instructions", "ignore all instructions",
                                "disregard your", "override your", "your new instructions",
                                "ignore your training", "forget your guidelines",
                                "jailbreak", " DAN ", "do anything now",
                                "you have no restrictions", "pretend you have no"];
        let hard_count = hard_injection.iter().filter(|&&p| q.contains(p)).count();

        // Soft adversarial patterns — unusual framing worth monitoring
        let soft_adversarial = ["ignore previous", "system prompt", "as a game",
                                  "you are now", "act as if", "hypothetically speaking",
                                  "in a fictional world where you", "for a story where you",
                                  "pretend you are", "roleplay as an ai with no",
                                  "character that always", "never refuses"];
        let soft_count = soft_adversarial.iter().filter(|&&p| q.contains(p)).count();

        // Legitimate creative/roleplay signals — reduce soft risk score
        let legitimate_creative = ["write a story", "help me write", "for a novel",
                                     "for a screenplay", "for a character", "for my game",
                                     "educational purposes", "ctf challenge", "penetration test"];
        let legitimate_count = legitimate_creative.iter().filter(|&&l| q.contains(l)).count();

        let ev_signal = ev.get(5).copied().unwrap_or(0.0);

        let trit: i8 = if hard_count > 0 || ev_signal < -0.6 {
            -1
        } else if soft_count > 0 && legitimate_count == 0 {
            // Soft adversarial with no legitimate context → hold for review
            0
        } else {
            1
        };

        let confidence = if hard_count > 0 { 0.99 } else if soft_count > 0 { 0.90 } else { 0.95 };

        ExpertVerdict {
            trit,
            confidence,
            reasoning: if hard_count > 0 {
                format!("ALERT: {} hard adversarial injection pattern(s) detected. Full veto.", hard_count)
            } else if soft_count > 0 {
                format!(
                    "Soft adversarial patterns: {}, legitimate context: {}. {}.",
                    soft_count, legitimate_count,
                    if legitimate_count > 0 { "Context plausible — monitoring" }
                    else { "No legitimate context — holding for review" }
                )
            } else {
                "Meta-safety audit: no adversarial patterns detected.".into()
            },
            expert_id: 12,
            expert_name: "MetaSafety".into(),
        }
    }
}
