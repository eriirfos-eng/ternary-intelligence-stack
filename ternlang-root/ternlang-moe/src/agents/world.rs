use crate::ExpertVerdict;
use crate::agents::TernaryAgent;

pub struct WorldKnowledgeAgent;

impl TernaryAgent for WorldKnowledgeAgent {
    fn deliberate(&self, query: &str, ev: &[f32]) -> ExpertVerdict {
        let q = query.to_lowercase();

        // Positive: named entities, proper nouns, recognised domain vocabulary
        let domains = ["science", "history", "biology", "physics", "chemistry", "geography",
                        "medicine", "economics", "law", "technology", "astronomy", "climate"];
        let domain_hits = domains.iter().filter(|&&d| q.contains(d)).count();

        // Count capitalised tokens (proper nouns heuristic)
        let proper_nouns = query.split_whitespace()
            .filter(|w| w.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
                     && w.len() > 1)
            .count();

        // Verifiable specifics: dates, numeric facts, geo markers
        let specifics = ["in ", " was ", " is ", " are ", "km", "°", "percent", "million", "billion",
                          "founded", "discovered", "invented", "born", "died"];
        let specifics_hits = specifics.iter().filter(|&&s| q.contains(s)).count();

        // Negative: purely abstract/hypothetical queries that carry no world knowledge signal
        let abstract_markers = ["hypothetically", "imagine", "suppose", "what if", "let's say",
                                  "pretend", "fictional", "purely theoretical"];
        let abstract_count = abstract_markers.iter().filter(|&&a| q.contains(a)).count();

        let ev_signal = ev.get(1).copied().unwrap_or(0.0);
        let pos = domain_hits + proper_nouns + specifics_hits;
        let raw = pos as f32 * 0.5 - abstract_count as f32 * 1.5 + ev_signal * 2.0;

        let trit: i8 = if raw > 1.5 { 1 } else if raw < -0.5 { -1 } else { 0 };
        let confidence = (0.55 + (pos.min(8) as f32 * 0.05) + ev_signal.abs() * 0.1)
            .clamp(0.0, 1.0);

        let words = query.split_whitespace().count().max(1);
        ExpertVerdict {
            trit,
            confidence,
            reasoning: format!(
                "Domain hits: {}, proper nouns: {}, factual specifics: {}, abstract markers: {}. \
                 Entity density: {:.2}/word. World knowledge is {}.",
                domain_hits, proper_nouns, specifics_hits, abstract_count,
                pos as f32 / words as f32,
                match trit {
                    1  => "strongly grounded",
                    0  => "marginal — holding for domain confirmation",
                    _  => "displaced by abstract/hypothetical framing",
                }
            ),
            expert_id: 1,
            expert_name: "WorldKnowledge".into(),
        }
    }
}
