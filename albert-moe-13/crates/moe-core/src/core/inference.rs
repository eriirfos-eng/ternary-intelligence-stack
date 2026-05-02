// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
//! # Ternary Inference Engine
//!
//! High-performance execution core optimized for sparsity-aware kernels.
//! In EPIS mode: fully deterministic, no entropy injection.
//! In HYBRID mode: entropy-injected, AEDL-biased adaptive routing.

use anyhow::Result;
use crate::core::entropy_injector::EntropyInjector;
use crate::core::routing::{MoERouter13, ExpertBank13};
use crate::core::rdl::RepresentationDivergenceLayer;
use crate::core::policy::{HybridRouterPolicy, MoEMode};

pub struct InferenceEngine {
    pub kernel_version: String,
    pub entropy_injector: EntropyInjector,
    pub router: MoERouter13,
    pub expert_bank: ExpertBank13,
    pub rdl: RepresentationDivergenceLayer,
    pub policy: HybridRouterPolicy,
    output_dim: usize,
}

impl InferenceEngine {
    pub fn new(kernel_version: String, input_dim: usize, output_dim: usize) -> Self {
        Self {
            kernel_version,
            entropy_injector: EntropyInjector::new(0.01, 0.05, 0.02),
            router: MoERouter13::new(input_dim),
            expert_bank: ExpertBank13::new(input_dim, output_dim),
            rdl: RepresentationDivergenceLayer::new(input_dim),
            policy: HybridRouterPolicy::default(),
            output_dim,
        }
    }

    /// Execute a forward pass through the MoE-13 EPIS pipeline.
    ///
    /// EPIS mode skips entropy injection — guaranteed deterministic for identical input.
    /// HYBRID mode enables entropy + AEDL-biased routing for adaptive behavior.
    pub fn forward(&self, input: &mut [f32]) -> Result<Vec<f32>> {
        // Entropy injection only in HYBRID mode — EPIS must be fully deterministic
        if self.policy.mode == MoEMode::HYBRID {
            self.entropy_injector.inject_entropy(input);
        }

        let domain_bias = self.expert_bank.domain_scores(input);
        let task_embedding = vec![0.1f32; 16.min(input.len())];
        let selected_experts = self.router.route(input, 2, &domain_bias);

        let mut final_output = vec![0.0f32; self.output_dim];
        for (idx, weight) in &selected_experts {
            let divergent_input = self.rdl.diverge(*idx, input, &task_embedding);
            let expert_out = self.expert_bank.execute_expert(*idx, &divergent_input);
            for (o, e) in final_output.iter_mut().zip(expert_out.iter()) {
                *o += e * weight;
            }
        }

        Ok(final_output)
    }

    /// Return which experts would be activated for this input without executing.
    /// Used for audit trails and routing diagnostics.
    pub fn route_only(&self, input: &[f32]) -> Vec<(usize, f32)> {
        let domain_bias = self.expert_bank.domain_scores(input);
        self.router.route(input, 2, &domain_bias)
    }

    pub fn mode(&self) -> MoEMode {
        self.policy.mode
    }
}
