use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct SafetyAgent;

impl TernaryAgent for SafetyAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();

        // Hard violation keywords — unambiguous risk signal
        let hard_risk = ["harm ", "exploit", "steal", "hack ", "kill ", "bomb", "bypass security",
                          "illegal", "weapon", "malware", "ransomware", "phish", "poison",
                          "synthesize drug", "doxxing", "swatting"];
        let hard_count = hard_risk.iter().filter(|&&k| q.contains(k)).count();

        // Soft/borderline risk — context-dependent, needs review
        let soft_risk = ["dangerous", "risk", "unsafe", "vulnerability", "attack", "pentest",
                          "offensive", "bypass", "sensitive data", "private key", "credentials",
                          "without permission", "unauthorised"];
        let soft_count = soft_risk.iter().filter(|&&k| q.contains(k)).count();

        // Positive safety signals — explicitly framed as safe/authorised context
        let safe_signals = ["for safety", "in a controlled", "authorised", "with permission",
                              "for research", "defensive", "ctf ", "capture the flag",
                              "security audit", "pentest engagement"];
        let safe_count = safe_signals.iter().filter(|&&s| q.contains(s)).count();

        let ev_signal = ev.get(5).copied().unwrap_or(0.0);

        // Hard gate: any hard risk keyword overrides everything
        let trit: i8 = if hard_count > 0 || ev_signal < -0.5 {
            -1
        } else if soft_count > 0 && safe_count == 0 {
            // Soft risk without authorisation context → hold for review
            0
        } else if soft_count > 0 && safe_count > 0 {
            // Soft risk with explicit safe context → marginal hold
            0
        } else {
            1
        };

        let confidence = if hard_count > 0 { 0.99 } else { 0.85 };

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!(
                "Hard risk signals: {}, soft risk signals: {}, safe context markers: {}. Safety verdict: {}.",
                hard_count, soft_count, safe_count,
                match trit {
                    1  => "clear — no risk signals detected",
                    0  => "borderline — soft risk present, context-dependent review needed",
                    _  => "BLOCKED — hard risk signal detected or evidence veto",
                }
            ),
            expert_id: 6,
            expert_name: "Safety".into(),
        }
    }
}
