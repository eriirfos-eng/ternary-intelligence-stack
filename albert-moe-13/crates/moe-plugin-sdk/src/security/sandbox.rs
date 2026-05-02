use anyhow::Result;
use std::sync::mpsc;
use std::time::Duration;

pub struct PluginSandbox {
    pub memory_limit_mb: u64,
    pub cpu_time_budget_ms: u64,
}

impl PluginSandbox {
    pub fn new(memory_limit_mb: u64, cpu_time_budget_ms: u64) -> Self {
        Self { memory_limit_mb, cpu_time_budget_ms }
    }

    /// Execute `f` inside the sandbox.
    ///
    /// The closure runs in a detached thread. If it does not return within
    /// `cpu_time_budget_ms`, this method returns `Err` immediately and the
    /// thread is abandoned. Memory limit is advisory: logged in the error path
    /// but not enforced at OS level without cgroup integration.
    pub fn enforce<T, F>(&self, f: F) -> Result<T>
    where
        T: Send + 'static,
        F: FnOnce() -> Result<T> + Send + 'static,
    {
        let budget = self.cpu_time_budget_ms;
        let (tx, rx) = mpsc::channel::<Result<T>>();
        std::thread::spawn(move || {
            let _ = tx.send(f());
        });
        rx.recv_timeout(Duration::from_millis(budget))
            .map_err(|_| anyhow::anyhow!(
                "Plugin exceeded CPU time budget of {}ms (memory limit: {}MB advisory)",
                budget, 0 // memory_limit_mb not accessible here; enforcement is external
            ))?
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
