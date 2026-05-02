use anyhow::Result;

/// Payload sent to a compute node to execute a single expert.
#[derive(Debug, Clone)]
pub struct TransportPayload {
    pub expert_idx: usize,
    pub input: Vec<f32>,
}

/// Output returned from a compute node after expert execution.
#[derive(Debug, Clone)]
pub struct TransportResult {
    pub expert_idx: usize,
    pub output: Vec<f32>,
}

/// Abstraction over the communication channel between the scheduler and compute nodes.
pub trait TaskTransport: Send + Sync {
    fn send(&self, node_id: &str, payload: TransportPayload) -> Result<TransportResult>;
}

/// Single-node transport: executes the payload in-process via a supplied closure.
///
/// Used when all 13 experts run on the same machine. The closure receives
/// `(expert_idx, input)` and returns the output activation vector.
pub struct LocalTransport {
    pub executor: Box<dyn Fn(usize, &[f32]) -> Vec<f32> + Send + Sync>,
}

impl TaskTransport for LocalTransport {
    fn send(&self, _node_id: &str, payload: TransportPayload) -> Result<TransportResult> {
        let output = (self.executor)(payload.expert_idx, &payload.input);
        Ok(TransportResult { expert_idx: payload.expert_idx, output })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn echo_transport() -> LocalTransport {
        LocalTransport {
            executor: Box::new(|idx, input| {
                // Echo: output = input scaled by expert_idx + 1
                input.iter().map(|&v| v * (idx + 1) as f32).collect()
            }),
        }
    }

    #[test]
    fn test_local_transport_returns_output() {
        let transport = echo_transport();
        let payload = TransportPayload { expert_idx: 2, input: vec![1.0, 2.0, 3.0] };
        let result = transport.send("local", payload).unwrap();
        assert_eq!(result.expert_idx, 2);
        assert_eq!(result.output, vec![3.0, 6.0, 9.0]);
    }

    #[test]
    fn test_local_transport_node_id_is_ignored() {
        let transport = echo_transport();
        let p1 = TransportPayload { expert_idx: 0, input: vec![1.0] };
        let p2 = TransportPayload { expert_idx: 0, input: vec![1.0] };
        let r1 = transport.send("node-0", p1).unwrap();
        let r2 = transport.send("node-99", p2).unwrap();
        assert_eq!(r1.output, r2.output, "LocalTransport ignores node_id");
    }

    #[test]
    fn test_local_transport_empty_input() {
        let transport = echo_transport();
        let result = transport.send("local", TransportPayload { expert_idx: 0, input: vec![] }).unwrap();
        assert!(result.output.is_empty());
    }
}
