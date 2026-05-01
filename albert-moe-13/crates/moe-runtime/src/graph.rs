use moe_core::model_adapter::umil::cmir::CMIR;
use moe_core::core::routing::{ExpertBank13, MoERouter13};
use moe_plugin_sdk::MoePlugin;

pub enum CMIRNode {
    Model(ExpertBank13),
    Plugin(Box<dyn MoePlugin>),
    Router(MoERouter13),
}

pub struct Edge {
    pub from: usize,
    pub to: usize,
}

pub struct ExecutionGraph {
    pub nodes: Vec<CMIRNode>,
    pub edges: Vec<Edge>,
}
