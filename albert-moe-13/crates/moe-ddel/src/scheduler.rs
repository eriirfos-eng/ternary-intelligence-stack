use anyhow::Result;
use crate::partitioner::ExpertPartition;

/// A unit of expert work dispatched to a specific compute node.
#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub expert_idx: usize,
    pub node_id: String,
    /// Routing weight from the MoE gate for this expert.
    pub weight: f32,
}

/// Maps active experts from a routing decision onto their assigned nodes.
pub struct ExpertScheduler {
    partitions: Vec<ExpertPartition>,
}

impl ExpertScheduler {
    pub fn new(partitions: Vec<ExpertPartition>) -> Self {
        Self { partitions }
    }

    /// Convert `(expert_idx, gate_weight)` pairs into `ScheduledTask`s.
    ///
    /// Experts with no partition entry fall back to `"local"`.
    pub fn schedule(&self, active_experts: &[(usize, f32)]) -> Result<Vec<ScheduledTask>> {
        active_experts
            .iter()
            .map(|&(idx, weight)| {
                let node_id = self.partitions
                    .iter()
                    .find(|p| p.expert_idx == idx)
                    .map(|p| p.node_id.clone())
                    .unwrap_or_else(|| "local".to_string());
                Ok(ScheduledTask { expert_idx: idx, node_id, weight })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::partitioner::ExpertPartition;

    fn make_partitions(n: usize) -> Vec<ExpertPartition> {
        (0..n).map(|i| ExpertPartition {
            expert_idx: i,
            node_id: format!("node-{}", i % 3),
        }).collect()
    }

    #[test]
    fn test_schedule_produces_correct_tasks() {
        let scheduler = ExpertScheduler::new(make_partitions(13));
        let active = vec![(0usize, 0.7f32), (5, 0.3)];
        let tasks = scheduler.schedule(&active).unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].expert_idx, 0);
        assert_eq!(tasks[1].expert_idx, 5);
        assert!((tasks[0].weight - 0.7).abs() < 1e-6);
    }

    #[test]
    fn test_schedule_fallback_to_local_for_unknown_expert() {
        let scheduler = ExpertScheduler::new(vec![]);
        let tasks = scheduler.schedule(&[(7, 1.0)]).unwrap();
        assert_eq!(tasks[0].node_id, "local");
    }

    #[test]
    fn test_schedule_empty_active_experts() {
        let scheduler = ExpertScheduler::new(make_partitions(13));
        let tasks = scheduler.schedule(&[]).unwrap();
        assert!(tasks.is_empty());
    }
}
