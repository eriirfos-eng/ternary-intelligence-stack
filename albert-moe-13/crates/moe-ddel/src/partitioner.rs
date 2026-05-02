use crate::node::ExecutionNode;

/// Maps a single expert index to a specific compute node.
#[derive(Debug, Clone)]
pub struct ExpertPartition {
    pub expert_idx: usize,
    pub node_id: String,
}

/// Assign experts to nodes using core-weighted round-robin.
///
/// Each node receives slots proportional to `cores / 4` (minimum 1).
/// If no nodes are provided, all experts fall back to `"local"`.
pub fn partition_experts(nodes: &[ExecutionNode], num_experts: usize) -> Vec<ExpertPartition> {
    if nodes.is_empty() {
        return (0..num_experts)
            .map(|i| ExpertPartition { expert_idx: i, node_id: "local".to_string() })
            .collect();
    }

    // Build weighted node slot list: more cores → more expert slots
    let weighted: Vec<&ExecutionNode> = nodes
        .iter()
        .flat_map(|n| {
            let slots = (n.compute_capacity.cores / 4).max(1) as usize;
            std::iter::repeat(n).take(slots)
        })
        .collect();

    (0..num_experts)
        .map(|i| ExpertPartition {
            expert_idx: i,
            node_id: weighted[i % weighted.len()].node_id.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::{ComputeProfile, ExecutionNode};

    fn make_node(id: &str, cores: u32) -> ExecutionNode {
        ExecutionNode {
            node_id: id.to_string(),
            compute_capacity: ComputeProfile { memory_gb: 32, cores },
        }
    }

    #[test]
    fn test_partition_no_nodes_falls_back_to_local() {
        let partitions = partition_experts(&[], 13);
        assert_eq!(partitions.len(), 13);
        assert!(partitions.iter().all(|p| p.node_id == "local"));
    }

    #[test]
    fn test_partition_single_node_gets_all_experts() {
        let nodes = vec![make_node("node-0", 8)];
        let partitions = partition_experts(&nodes, 13);
        assert_eq!(partitions.len(), 13);
        assert!(partitions.iter().all(|p| p.node_id == "node-0"));
    }

    #[test]
    fn test_partition_round_robin_three_nodes() {
        let nodes = vec![
            make_node("node-0", 4),
            make_node("node-1", 4),
            make_node("node-2", 4),
        ];
        let partitions = partition_experts(&nodes, 13);
        assert_eq!(partitions.len(), 13);
        // All three nodes must be represented
        let ids: std::collections::HashSet<&str> =
            partitions.iter().map(|p| p.node_id.as_str()).collect();
        assert!(ids.contains("node-0") && ids.contains("node-1") && ids.contains("node-2"));
    }

    #[test]
    fn test_partition_high_core_node_gets_more_experts() {
        let nodes = vec![
            make_node("fat", 16),  // 4 slots
            make_node("thin", 4),  // 1 slot
        ];
        let partitions = partition_experts(&nodes, 13);
        let fat_count = partitions.iter().filter(|p| p.node_id == "fat").count();
        let thin_count = partitions.iter().filter(|p| p.node_id == "thin").count();
        assert!(fat_count > thin_count, "high-core node must receive more experts");
    }
}
