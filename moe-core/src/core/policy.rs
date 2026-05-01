//! # Hybrid Routing Policy
//! 
//! Controls the operational regime of the MoE-13 system, allowing transitions
//! between deterministic EPIS execution and adaptive HYBRID routing.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoEMode { EPIS, HYBRID }

pub struct HybridRouterPolicy {
    pub mode: MoEMode,
    pub lambda: f32,
    pub freeze_aedl: bool,
}

impl Default for HybridRouterPolicy {
    fn default() -> Self {
        Self {
            mode: MoEMode::EPIS,
            lambda: 0.25,
            freeze_aedl: true,
        }
    }
}
