pub struct PluginSandbox {
    pub memory_limit_mb: u64,
    pub cpu_time_budget_ms: u64,
}

impl PluginSandbox {
    pub fn new(memory_limit_mb: u64, cpu_time_budget_ms: u64) -> Self {
        Self {
            memory_limit_mb,
            cpu_time_budget_ms,
        }
    }
}
