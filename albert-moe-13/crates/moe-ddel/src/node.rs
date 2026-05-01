pub type NodeId = String;

pub struct ComputeProfile {
    pub memory_gb: u64,
    pub cores: u32,
}

pub struct ExecutionNode {
    pub node_id: NodeId,
    pub compute_capacity: ComputeProfile,
}
