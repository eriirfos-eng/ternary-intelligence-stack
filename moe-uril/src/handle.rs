use anyhow::Result;
use moe_runtime::{ExecutionGraph, GraphExecutor};
use moe_core::model_adapter::umil::cmir::CMIR;

pub struct RuntimeHandle {
    pub graph: ExecutionGraph,
}

pub struct MoEPlatform;

impl MoEPlatform {
    pub fn load(config: serde_json::Value) -> Result<RuntimeHandle> {
        // In a real implementation, this would resolve providers and build the graph.
        // For URIL, we assume valid configuration maps to a pre-validated ExecutionGraph.
        let graph = ExecutionGraph {
            nodes: vec![],
            edges: vec![],
        };
        Ok(RuntimeHandle { graph })
    }

    pub fn run(handle: RuntimeHandle, input: CMIR) -> Result<CMIR> {
        GraphExecutor::execute(handle.graph, input)
    }
}
