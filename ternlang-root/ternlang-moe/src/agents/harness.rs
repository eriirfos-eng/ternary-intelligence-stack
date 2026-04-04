use crate::ExpertVerdict;
use crate::agents::*;

pub trait TernaryAgent: Send + Sync {
    fn deliberate(&self, query: &str, context: &[f32]) -> ExpertVerdict;
}

// ---------------------------------------------------------------------------
// Aggregate Verdict — result of the full 13-agent introspective pass
// ---------------------------------------------------------------------------

/// The collective output of all agents running together.
///
/// `is_stable_hold` is the introspective hold condition: when affirm and conflict
/// signals are balanced across enough agents, trit=0 is a **stable attractor**,
/// not a transient. The system does not force a tiebreaker — it holds deliberately.
#[derive(Debug, Clone)]
pub struct AggregateVerdict {
    /// Collective trit: -1 / 0 / +1
    pub trit: i8,
    /// Aggregate confidence ∈ [0, 1]
    pub confidence: f32,
    /// All individual agent verdicts
    pub verdicts: Vec<ExpertVerdict>,
    /// True when trit=0 is a stable attractor (balanced signals), not low evidence
    pub is_stable_hold: bool,
    /// Human-readable reason for hold state, if held
    pub hold_reason: Option<String>,
    /// Count of agents that returned +1
    pub affirm_count: usize,
    /// Count of agents that returned -1
    pub conflict_count: usize,
    /// Count of agents that returned 0
    pub hold_count: usize,
}

// ---------------------------------------------------------------------------
// Agent Harness
// ---------------------------------------------------------------------------

pub struct AgentHarness {
    pub agents: Vec<Box<dyn TernaryAgent>>,
}

impl AgentHarness {
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    pub fn with_standard_agents() -> Self {
        Self {
            agents: vec![
                Box::new(SyntaxAgent),
                Box::new(WorldKnowledgeAgent),
                Box::new(DeductiveReasonAgent),
                Box::new(InductiveReasonAgent),
                Box::new(ToolUseAgent),
                Box::new(PersonaAgent),
                Box::new(SafetyAgent),
                Box::new(FactCheckAgent),
                Box::new(CausalReasonAgent),
                Box::new(AmbiguityResAgent),
                Box::new(MathReasonAgent),
                Box::new(ContextMemAgent),
                Box::new(MetaSafetyAgent),
            ],
        }
    }

    /// Run all agents and return individual verdicts.
    pub fn run(&self, query: &str, context: &[f32]) -> Vec<ExpertVerdict> {
        self.agents
            .iter()
            .map(|a| a.deliberate(query, context))
            .collect()
    }

    // -----------------------------------------------------------------------
    // Introspective Hold
    // -----------------------------------------------------------------------

    /// Run all 13 agents and synthesise an `AggregateVerdict`.
    ///
    /// The introspective hold mechanism treats trit=0 as a **stable attractor**:
    /// if affirm and conflict counts are balanced across enough agents that took a
    /// position, the system does not try to break the tie — it holds deliberately,
    /// recognising that the signals have reached equilibrium and more evidence is
    /// needed before collapsing to a decision.
    pub fn run_introspective(&self, query: &str, context: &[f32]) -> AggregateVerdict {
        let verdicts = self.run(query, context);

        let affirm_count   = verdicts.iter().filter(|v| v.trit ==  1).count();
        let conflict_count = verdicts.iter().filter(|v| v.trit == -1).count();
        let hold_count     = verdicts.iter().filter(|v| v.trit ==  0).count();

        // Safety hard gate: any high-confidence -1 from Safety or MetaSafety vetos immediately
        let safety_veto = verdicts.iter().any(|v| {
            (v.expert_name == "Safety" || v.expert_name == "MetaSafety")
                && v.trit == -1
                && v.confidence > 0.80
        });

        if safety_veto {
            return AggregateVerdict {
                trit: -1,
                confidence: 0.98,
                is_stable_hold: false,
                hold_reason: None,
                affirm_count,
                conflict_count,
                hold_count,
                verdicts,
            };
        }

        // Weighted trit score across all agents
        let weighted_sum: f32 = verdicts.iter()
            .map(|v| v.trit as f32 * v.confidence)
            .sum();

        // Stable attractor condition:
        //   |affirm - conflict| ≤ 1  (signals are balanced)
        //   affirm + conflict ≥ 4    (enough agents took a strong position)
        let engaged      = affirm_count + conflict_count;
        let balance_gap  = (affirm_count as i32 - conflict_count as i32).unsigned_abs() as usize;
        let is_stable_hold = balance_gap <= 1 && engaged >= 4;

        let (trit, hold_reason) = if is_stable_hold {
            (0i8, Some(format!(
                "Stable attractor: {} affirm vs {} conflict across {} agents — \
                 signals in equilibrium. Deliberation continues; force-breaking would \
                 discard real uncertainty.",
                affirm_count, conflict_count, verdicts.len()
            )))
        } else if weighted_sum > 2.5 {
            (1i8, None)
        } else if weighted_sum < -2.5 {
            (-1i8, None)
        } else {
            (0i8, Some(format!(
                "Insufficient signal mass (weighted sum {:.2}) — collecting more evidence. \
                 {} agents in hold.",
                weighted_sum, hold_count
            )))
        };

        let confidence = (weighted_sum.abs() / verdicts.len().max(1) as f32)
            .clamp(0.30, 0.95);

        AggregateVerdict {
            trit,
            confidence,
            verdicts,
            is_stable_hold,
            hold_reason,
            affirm_count,
            conflict_count,
            hold_count,
        }
    }

    // -----------------------------------------------------------------------
    // Evidence Vector Bridge
    // -----------------------------------------------------------------------

    /// Map 13 agent verdicts to a 6D competence-aligned evidence vector.
    ///
    /// Axis mapping matches `ternlang_moe::axis`:
    ///   [0] SYNTAX          ← SyntaxAgent
    ///   [1] WORLD_KNOWLEDGE ← WorldKnowledgeAgent
    ///   [2] REASONING       ← mean(Deductive, Inductive, Causal, Math, FactCheck)
    ///   [3] TOOL_USE        ← ToolUseAgent
    ///   [4] PERSONA         ← PersonaAgent
    ///   [5] SAFETY          ← min(Safety, MetaSafety)  — hard gate: worst signal wins
    pub fn to_evidence_vector(verdicts: &[ExpertVerdict]) -> [f32; 6] {
        let signal = |name: &str| {
            verdicts.iter()
                .find(|v| v.expert_name == name)
                .map(|v| v.trit as f32 * v.confidence)
                .unwrap_or(0.0)
        };

        let reasoning_avg = {
            let names = ["DeductiveReason", "InductiveReason", "CausalReason",
                          "MathReason", "FactCheck"];
            let sum: f32 = names.iter().map(|n| signal(n)).sum();
            sum / names.len() as f32
        };

        // Safety: take the minimum (most conservative) of the two safety agents
        let safety_signal = signal("Safety").min(signal("MetaSafety"));

        [
            signal("Syntax"),           // ev[0] SYNTAX
            signal("WorldKnowledge"),   // ev[1] WORLD_KNOWLEDGE
            reasoning_avg,              // ev[2] REASONING
            signal("ToolUse"),          // ev[3] TOOL_USE
            signal("Persona"),          // ev[4] PERSONA
            safety_signal,              // ev[5] SAFETY
        ]
    }

    // -----------------------------------------------------------------------
    // Deliberation Temperature
    // -----------------------------------------------------------------------

    /// Compute the "deliberation temperature" from an agent verdict set.
    ///
    /// High hold fraction → high temperature (exploratory, broad beam).
    /// Low hold fraction  → low temperature (consensus reached, focused beam).
    /// Range: [0.30, 0.90].
    pub fn deliberation_temperature(verdicts: &[ExpertVerdict]) -> f32 {
        let n = verdicts.len().max(1);
        let hold_fraction = verdicts.iter().filter(|v| v.trit == 0).count() as f32 / n as f32;
        0.30 + hold_fraction * 0.60
    }
}

impl Default for AgentHarness {
    fn default() -> Self {
        Self::new()
    }
}
