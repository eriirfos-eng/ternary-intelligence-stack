use anyhow::Result;
use std::sync::mpsc;
use std::time::Duration;
use ternlang_moe::{AxisMemory, VetoEntry};

pub struct PluginSandbox {
    pub memory_limit_mb: u64,
    pub cpu_time_budget_ms: u64,
}

impl PluginSandbox {
    pub fn new(memory_limit_mb: u64, cpu_time_budget_ms: u64) -> Self {
        Self { memory_limit_mb, cpu_time_budget_ms }
    }

    /// Execute `f` inside the sandbox and log results to AxisMesh.
    pub fn enforce<T, F>(&self, axis: &mut AxisMemory, f: F) -> Result<T>
    where
        T: Send + 'static,
        F: FnOnce() -> Result<T> + Send + 'static,
    {
        let budget = self.cpu_time_budget_ms;
        let (tx, rx) = mpsc::channel::<Result<T>>();
        std::thread::spawn(move || {
            let _ = tx.send(f());
        });
        
        match rx.recv_timeout(Duration::from_millis(budget)) {
            Ok(result) => result,
            Err(_) => {
                // Bridge to AxisMesh: Log the VetoEntry
                axis.veto_log.push(VetoEntry {
                    timestamp: 123456789, // Mock or actual timestamp
                    reason: format!("Plugin exceeded CPU time budget of {}ms", budget),
                    severity: 1, // High severity
                });
                
                anyhow::bail!("Plugin exceeded CPU time budget of {}ms (VetoEntry logged in AxisMesh)", budget)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_allows_fast_work() {
        let sandbox = PluginSandbox::new(64, 500);
        let result = sandbox.enforce(|| Ok(42u32));
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_sandbox_enforces_timeout() {
        let sandbox = PluginSandbox::new(64, 50);
        let result = sandbox.enforce::<(), _>(|| {
            std::thread::sleep(Duration::from_millis(200));
            Ok(())
        });
        assert!(result.is_err(), "sandbox must reject work that exceeds time budget");
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("CPU time budget"), "error must mention time budget");
    }

    #[test]
    fn test_sandbox_propagates_inner_error() {
        let sandbox = PluginSandbox::new(64, 500);
        let result = sandbox.enforce::<(), _>(|| {
            anyhow::bail!("plugin internal failure")
        });
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("plugin internal failure"));
    }
}
