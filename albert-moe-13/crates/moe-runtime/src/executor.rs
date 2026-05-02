use crate::graph::{ExecutionGraph, CMIRNode};
use moe_core::model_adapter::umil::cmir::CMIR;
use moe_core::core::rdl::RepresentationDivergenceLayer;
use anyhow::Result;

pub struct GraphExecutor;

impl GraphExecutor {
    /// Traverse the execution graph, route the CMIR through active expert nodes,
    /// and return an updated CMIR with routing metadata attached.
    pub fn execute(graph: ExecutionGraph, input: CMIR) -> Result<CMIR> {
        // Find the first Router and Model nodes in the graph
        let mut router_idx: Option<usize> = None;
        let mut bank_idx: Option<usize> = None;

        for (i, node) in graph.nodes.iter().enumerate() {
            match node {
                CMIRNode::Router(_) if router_idx.is_none() => router_idx = Some(i),
                CMIRNode::Model(_)  if bank_idx.is_none()   => bank_idx  = Some(i),
                _ => {}
            }
        }

        match (router_idx, bank_idx) {
            (Some(ri), Some(bi)) => {
                let CMIRNode::Router(router) = &graph.nodes[ri] else {
                    return Ok(input);
                };
                let CMIRNode::Model(bank) = &graph.nodes[bi] else {
                    return Ok(input);
                };

                // Build float activation from ternary weight buffer
                let activation: Vec<f32> = input.ternary_weights
                    .iter()
                    .map(|&t| t as f32)
                    .collect();

                if activation.is_empty() {
                    return Ok(input);
                }

                let task_embedding = vec![0.1f32; 16.min(activation.len())];
                let rdl = RepresentationDivergenceLayer::new(activation.len());
                let selected = router.route(&activation, 2);

                let output_dim = bank.experts.first().map(|e| e.output_dim).unwrap_or(16);
                let mut combined = vec![0.0f32; output_dim];
                let mut activated_ids: Vec<usize> = Vec::new();

                for (idx, weight) in &selected {
                    let divergent = rdl.diverge(*idx, &activation, &task_embedding);
                    let expert_out = bank.execute_expert(*idx, &divergent);
                    for (o, e) in combined.iter_mut().zip(expert_out.iter()) {
                        *o += e * weight;
                    }
                    activated_ids.push(*idx);
                }

                let routing_map = serde_json::json!({
                    "activated_experts": activated_ids,
                    "num_active": activated_ids.len(),
                    "mode": "EPIS",
                    "graph_nodes": graph.nodes.len(),
                });

                Ok(CMIR {
                    expert_bank: input.expert_bank,
                    ternary_weights: input.ternary_weights,
                    routing_map,
                    metadata: input.metadata,
                })
            }

            // Graph has no router/bank pair — passthrough (plugins-only or empty graph)
            _ => Ok(input),
        }
    }
}
