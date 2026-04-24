import os

path = "crates/rusty-ternlang-cli/src/main.rs"
with open(path, "r") as f:
    content = f.read()

content = content.replace(
"""impl ApiClient for RfiIrfosRuntimeClient {
    fn stream(
        &mut self,
        request: ApiRequest,
    ) -> Result<Vec<AssistantEvent>, Box<dyn std::error::Error + Send + Sync>> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        runtime.block_on(self.stream_async(request))
    }
}""",
"""impl ApiClient for RfiIrfosRuntimeClient {
    fn stream(
        &mut self,
        request: ApiRequest,
    ) -> Result<Vec<AssistantEvent>, RuntimeError> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| RuntimeError::new(e.to_string()))?;
        runtime.block_on(self.stream_async(request)).map_err(|e| RuntimeError::new(e.to_string()))
    }
}""")

content = content.replace(
"""impl ToolExecutor for CliToolExecutor {
    fn execute(
        &mut self,
        tool_name: &str,
        input: serde_json::Value,
    ) -> Result<String, ToolError> {
        execute_tool(tool_name, &input).map_err(|e| ToolError::ExecutionError(e))
    }
}""",
"""impl ToolExecutor for CliToolExecutor {
    fn execute(
        &mut self,
        tool_name: &str,
        input: &str,
    ) -> Result<String, ToolError> {
        let input_val: serde_json::Value = serde_json::from_str(input).unwrap_or(serde_json::Value::Null);
        execute_tool(tool_name, &input_val).map_err(|e| ToolError::new(e))
    }
}""")

content = content.replace("map_usage(payload.message.usage)", "map_usage(payload.message.usage.clone())")
content = content.replace("map_usage(payload.usage)", "map_usage(payload.usage.clone())")

content = content.replace("fn resolve_export_path(requested_path: Option<&str>, session: &Session)", "fn resolve_export_path(requested_path: Option<&str>, _session: &Session)")
content = content.replace("fn render_export_text(session: &Session)", "fn render_export_text(_session: &Session)")
content = content.replace("fn render_teleport_report(target: &str)", "fn render_teleport_report(_target: &str)")
content = content.replace("fn render_last_tool_debug_report(session: &Session)", "fn render_last_tool_debug_report(_session: &Session)")

# Also fix the unused config warning if we want:
content = content.replace("let config = ConfigLoader::default_for(&cwd).load()?;", "let _config = ConfigLoader::default_for(&cwd).load()?;")

with open(path, "w") as f:
    f.write(content)
