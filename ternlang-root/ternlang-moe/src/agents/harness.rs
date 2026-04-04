use crate::ExpertVerdict;
use crate::agents::*;

pub trait TernaryAgent: Send + Sync {
    fn deliberate(&self, query: &str, context: &[f32]) -> ExpertVerdict;
}

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

    pub fn run(&self, query: &str, context: &[f32]) -> Vec<ExpertVerdict> {
        self.agents
            .iter()
            .map(|a| a.deliberate(query, context))
            .collect()
    }
}
