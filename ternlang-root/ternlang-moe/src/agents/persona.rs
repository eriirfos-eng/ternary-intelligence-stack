use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct PersonaAgent;

impl TernaryAgent for PersonaAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();

        // Strong persona engagement: first/second person, subjective requests
        let personal_markers = ["i think", "i feel", "i believe", "my opinion", "in my view",
                                  "what do you think", "how do you feel", "do you agree",
                                  "your opinion", "tell me about yourself", "who are you",
                                  "you are ", "act as", "roleplay", "be a "];
        let persona_count = personal_markers.iter().filter(|&&p| q.contains(p)).count();

        // First/second person pronouns (rough heuristic)
        let pronoun_count = ["i ", " me ", " my ", " you ", " your ", " we ", " our "]
            .iter().filter(|&&p| q.contains(p)).count();

        // Depersonalisation: explicit requests for objectivity or third-person output
        let depersonal_markers = ["objectively", "impersonally", "without bias",
                                    "be neutral", "no opinions", "just facts",
                                    "don't use i", "third person"];
        let depersonal_count = depersonal_markers.iter().filter(|&&d| q.contains(d)).count();

        let ev_signal = ev.get(4).copied().unwrap_or(0.0);
        let raw = persona_count as f32 * 1.8 + pronoun_count as f32 * 0.3
                  - depersonal_count as f32 * 2.0 + ev_signal * 1.5;

        let trit: i8 = if raw > 1.2 { 1 } else if raw < -0.8 { -1 } else { 0 };
        let confidence = (0.60 + ((persona_count + pronoun_count).min(6) as f32 * 0.04))
            .clamp(0.0, 1.0);

        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!(
                "Persona markers: {}, pronoun signals: {}, depersonalisation: {}. Identity engagement: {}.",
                persona_count, pronoun_count, depersonal_count,
                match trit {
                    1  => "strong — subjective, identity-level interaction required",
                    0  => "mixed — some personal framing, tone adaption may apply",
                    _  => "suppressed — explicit depersonalisation or purely objective query",
                }
            ),
            expert_id: 5,
            expert_name: "Persona".into(),
        }
    }
}
