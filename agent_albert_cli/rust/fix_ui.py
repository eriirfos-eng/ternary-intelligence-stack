import re

with open("crates/albert-cli/src/main.rs", "r") as f:
    content = f.read()

# 1. Add event_tx to LiveCli
content = content.replace(
    "    mcp_manager: Arc<std::sync::Mutex<McpServerManager>>,\n}",
    "    mcp_manager: Arc<std::sync::Mutex<McpServerManager>>,\n    event_tx: Option<tokio::sync::broadcast::Sender<AssistantEvent>>,\n}"
)

# 2. Update LiveCli::new
content = content.replace(
    "        let runtime = build_runtime_with_mcp(\n            Session::new(),\n            model.clone(),\n            system_prompt.clone(),\n            enable_tools,\n            true,\n            allowed_tools.clone(),\n            permission_mode,\n            Arc::clone(&mcp_manager),\n        )?;",
    """        let (tx, _rx) = tokio::sync::broadcast::channel(128);
        let event_tx = Some(tx);
        
        let runtime = build_runtime_with_mcp(
            Session::new(),
            model.clone(),
            system_prompt.clone(),
            enable_tools,
            true,
            allowed_tools.clone(),
            permission_mode,
            Arc::clone(&mcp_manager),
            event_tx.clone(),
        )?;"""
)

content = content.replace(
    "            mcp_manager,\n        };\n        cli.persist_session()?;\n        Ok(cli)",
    "            mcp_manager,\n            event_tx,\n        };\n        cli.persist_session()?;\n        Ok(cli)"
)

# 3. Update build_runtime_with_mcp and build_runtime
content = content.replace(
    "    mcp_manager: Arc<std::sync::Mutex<McpServerManager>>,\n) -> Result<ConversationRuntime<TernlangRuntimeClient, CliToolExecutor>, Box<dyn std::error::Error>>\n{\n    build_runtime_inner(session, model, system_prompt, enable_tools, enable_stream_events, allowed_tools, permission_mode, Some(mcp_manager))\n}",
    "    mcp_manager: Arc<std::sync::Mutex<McpServerManager>>,\n    event_tx: Option<tokio::sync::broadcast::Sender<AssistantEvent>>,\n) -> Result<ConversationRuntime<TernlangRuntimeClient, CliToolExecutor>, Box<dyn std::error::Error>>\n{\n    build_runtime_inner(session, model, system_prompt, enable_tools, enable_stream_events, allowed_tools, permission_mode, Some(mcp_manager), event_tx)\n}"
)

content = content.replace(
    "    permission_mode: PermissionMode,\n) -> Result<ConversationRuntime<TernlangRuntimeClient, CliToolExecutor>, Box<dyn std::error::Error>>\n{\n    build_runtime_inner(session, model, system_prompt, enable_tools, enable_stream_events, allowed_tools, permission_mode, None)\n}",
    "    permission_mode: PermissionMode,\n) -> Result<ConversationRuntime<TernlangRuntimeClient, CliToolExecutor>, Box<dyn std::error::Error>>\n{\n    build_runtime_inner(session, model, system_prompt, enable_tools, enable_stream_events, allowed_tools, permission_mode, None, None)\n}"
)

content = content.replace(
    "    permission_mode: PermissionMode,\n    mcp_manager: Option<Arc<std::sync::Mutex<McpServerManager>>>,\n) -> Result<ConversationRuntime<TernlangRuntimeClient, CliToolExecutor>, Box<dyn std::error::Error>>",
    "    permission_mode: PermissionMode,\n    mcp_manager: Option<Arc<std::sync::Mutex<McpServerManager>>>,\n    event_tx: Option<tokio::sync::broadcast::Sender<AssistantEvent>>,\n) -> Result<ConversationRuntime<TernlangRuntimeClient, CliToolExecutor>, Box<dyn std::error::Error>>"
)

# 4. Update TernlangRuntimeClient creation
client_creation = """        tools: if enable_tools {
            filter_tool_specs(allowed_tools.as_ref())
        } else {
            Vec::new()
        },
        event_tx: if enable_stream_events {
            Some(
                tokio::runtime::Runtime::new()?
                    .block_on(async {
                        let (tx, _) = tokio::sync::broadcast::channel(128);
                        tx
                    }),
            )
        } else {
            None
        },
    };"""
new_client_creation = """        tools: if enable_tools {
            filter_tool_specs(allowed_tools.as_ref())
        } else {
            Vec::new()
        },
        event_tx,
    };"""
content = content.replace(client_creation, new_client_creation)

# 5. Update CliPermissionPrompter
content = content.replace(
    "struct CliPermissionPrompter {\n    permission_mode: PermissionMode,\n    interactive: bool,\n}",
    "struct CliPermissionPrompter {\n    permission_mode: PermissionMode,\n    interactive: bool,\n    is_prompting: Option<Arc<AtomicBool>>,\n}"
)
content = content.replace(
    "    fn new(permission_mode: PermissionMode, interactive: bool) -> Self {\n        Self {\n            permission_mode,\n            interactive,\n        }\n    }",
    "    fn new(permission_mode: PermissionMode, interactive: bool) -> Self {\n        Self {\n            permission_mode,\n            interactive,\n            is_prompting: None,\n        }\n    }"
)

decide_start = """impl runtime::PermissionPrompter for CliPermissionPrompter {
    fn decide(
        &mut self,
        request: &runtime::PermissionRequest,
    ) -> runtime::PermissionPromptDecision {"""

decide_new = """impl runtime::PermissionPrompter for CliPermissionPrompter {
    fn decide(
        &mut self,
        request: &runtime::PermissionRequest,
    ) -> runtime::PermissionPromptDecision {
        struct Guard(Option<Arc<AtomicBool>>);
        impl Drop for Guard {
            fn drop(&mut self) {
                if let Some(f) = &self.0 {
                    f.store(false, std::sync::atomic::Ordering::Relaxed);
                }
            }
        }
        let _guard = Guard(self.is_prompting.clone());
        if let Some(flag) = &self.is_prompting {
            flag.store(true, std::sync::atomic::Ordering::Relaxed);
        }"""
content = content.replace(decide_start, decide_new)


# 6. Replace run_turn
run_turn_old = """    fn run_turn(&mut self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::with_template("{spinner:.cyan.bold} {msg}")
                .unwrap()
                .tick_strings(&["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏","⠿"]),
        );
        pb.set_message("thinking...");
        pb.enable_steady_tick(Duration::from_millis(80));

        let mut permission_prompter = CliPermissionPrompter::new(self.permission_mode, true);
        let result = self
            .runtime
            .run_turn(input.to_string(), Some(&mut permission_prompter));
        match result {
            Ok(summary) => {
                pb.finish_and_clear();
                let response_text = final_assistant_text(&summary);
                if !response_text.is_empty() {
                    typewriter_print(&TerminalRenderer::new().render_markdown(&response_text));
                }
                if let Some(event) = summary.auto_compaction {
                    println!(
                        "{}",
                        format_auto_compaction_notice(event.removed_message_count)
                    );
                }
                self.persist_session()?;

                // Sprint C: LLM-powered self-reflection (async, best-effort)"""

run_turn_new = """    fn run_turn(&mut self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        let is_prompting = Arc::new(AtomicBool::new(false));
        let mut permission_prompter = CliPermissionPrompter::new(self.permission_mode, true);
        permission_prompter.is_prompting = Some(is_prompting.clone());
        
        let mut rx = self.event_tx.as_ref().unwrap().subscribe();
        let input_string = input.to_string();
        
        let runtime = &mut self.runtime;
        
        let result = std::thread::scope(|s| {
            let is_done = Arc::new(AtomicBool::new(false));
            let is_done_clone = is_done.clone();
            
            let handle = s.spawn(move || {
                let res = runtime.run_turn(input_string, Some(&mut permission_prompter));
                is_done_clone.store(true, std::sync::atomic::Ordering::Relaxed);
                res
            });
            
            let mut stream_spinner = crate::render::Spinner::new();
            let mut tool_spinner = crate::render::Spinner::new();
            let mut md_state = crate::render::MarkdownStreamState::default();
            let mut saw_text = false;
            let renderer = crate::render::TerminalRenderer::new();
            let mut out = std::io::stdout();
            
            let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async {
                let mut interval = tokio::time::interval(Duration::from_millis(80));
                let mut state = 0; // 0 = waiting, 1 = streaming text, 2 = tool requested
                let mut tool_desc = String::new();
                let mut last_prompt_state = false;
                
                loop {
                    if is_done.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }
                    
                    let currently_prompting = is_prompting.load(std::sync::atomic::Ordering::Relaxed);
                    if currently_prompting && !last_prompt_state {
                        let _ = crossterm::execute!(out, crossterm::cursor::MoveToColumn(0), crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine));
                    }
                    last_prompt_state = currently_prompting;

                    tokio::select! {
                        _ = interval.tick() => {
                            if !currently_prompting {
                                if state == 0 {
                                    let _ = stream_spinner.tick("thinking...", renderer.color_theme(), &mut out);
                                } else if state == 2 {
                                    let _ = tool_spinner.tick(&tool_desc, renderer.color_theme(), &mut out);
                                }
                            }
                        }
                        event = rx.recv() => {
                            match event {
                                Ok(runtime::AssistantEvent::TextDelta(delta)) => {
                                    if state == 0 || state == 2 {
                                        if !saw_text {
                                            let _ = stream_spinner.finish("Streaming response", renderer.color_theme(), &mut out);
                                            saw_text = true;
                                        }
                                        state = 1;
                                    }
                                    if let Some(rendered) = md_state.push(&renderer, &delta) {
                                        let _ = write!(out, "{}", rendered);
                                        let _ = out.flush();
                                    }
                                }
                                Ok(runtime::AssistantEvent::ToolUse { name, input, .. }) => {
                                    if state == 1 || state == 0 {
                                        if saw_text {
                                            let _ = writeln!(out);
                                        }
                                    }
                                    state = 2;
                                    tool_desc = format!("Running tool `{}` with {}", name, input);
                                    let _ = tool_spinner.tick(&tool_desc, renderer.color_theme(), &mut out);
                                }
                                Ok(runtime::AssistantEvent::Usage(_)) => {}
                                Ok(runtime::AssistantEvent::MessageStop) => {
                                    if let Some(rendered) = md_state.flush(&renderer) {
                                        let _ = write!(out, "{}", rendered);
                                        let _ = out.flush();
                                    }
                                    
                                    if state == 0 {
                                        let _ = stream_spinner.finish("Streaming response", renderer.color_theme(), &mut out);
                                        saw_text = true;
                                        state = 1;
                                    } else if state == 2 {
                                        let _ = tool_spinner.finish(&tool_desc, renderer.color_theme(), &mut out);
                                        state = 0;
                                    } else if state == 1 {
                                        state = 0;
                                    }
                                }
                                Err(_) => {
                                    break;
                                }
                            }
                        }
                    }
                }
            });
            
            handle.join().unwrap()
        });

        match result {
            Ok(summary) => {
                if let Some(event) = summary.auto_compaction {
                    println!(
                        "{}",
                        format_auto_compaction_notice(event.removed_message_count)
                    );
                }
                self.persist_session()?;
                
                let response_text = final_assistant_text(&summary);

                // Sprint C: LLM-powered self-reflection (async, best-effort)"""
content = content.replace(run_turn_old, run_turn_new)

# 7. Remove typewriter_print
tw_old = """fn typewriter_print(text: &str) {
    use std::io::Write;
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let delay = if text.len() > 800 { 1 } else if text.len() > 300 { 2 } else { 3 };
    for ch in text.chars() {
        let _ = write!(out, "{ch}");
        let _ = out.flush();
        thread::sleep(Duration::from_millis(delay));
    }
    println!();
}"""
content = content.replace(tw_old, "")

with open("crates/albert-cli/src/main.rs", "w") as f:
    f.write(content)
