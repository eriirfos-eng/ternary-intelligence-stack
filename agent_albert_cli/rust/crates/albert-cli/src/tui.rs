use std::collections::VecDeque;
use std::io::{self, Write as _};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crossterm::event::{
    self, DisableBracketedPaste, EnableBracketedPaste, EnableMouseCapture, DisableMouseCapture,
    Event, KeyCode, KeyEvent, KeyModifiers, MouseEventKind,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Terminal;

use pulldown_cmark::{
    Event as MdEvent, HeadingLevel, Options as MdOptions, Parser as MdParser, Tag, TagEnd,
};

use commands::slash_command_specs;
use runtime::AssistantEvent;

// ── Colors ────────────────────────────────────────────────────────────────────

const BG: Color = Color::Rgb(13, 17, 33);
const FG: Color = Color::Rgb(220, 220, 220);
const DIM: Color = Color::Rgb(80, 80, 80);
const GREY: Color = Color::Rgb(145, 145, 145);
const GREEN: Color = Color::Rgb(0, 220, 120);
const CYAN: Color = Color::Rgb(0, 200, 255);
const ORANGE: Color = Color::Rgb(255, 140, 50);   // working `*` indicator
const USER_BOX_BG: Color = Color::Rgb(20, 26, 48);
const STATUS_BG: Color = Color::Rgb(16, 21, 40);
const BRANCH_BG: Color = Color::Rgb(35, 55, 35);  // git branch pill background
const POPUP_BG: Color = Color::Rgb(20, 26, 48);
const POPUP_BORDER: Color = Color::Rgb(55, 55, 55);
const POPUP_MATCH: Color = Color::Rgb(0, 180, 100);
const POPUP_SEL_BG: Color = Color::Rgb(42, 42, 42);
const CODE_FG: Color = Color::Rgb(100, 210, 255);    // inline code / code blocks
const CODE_BG: Color = Color::Rgb(14, 22, 30);       // code block row background
const CODE_BAR: Color = Color::Rgb(0, 100, 160);     // code block left border bar
const CHAT_BORDER: Color = Color::Rgb(0, 65, 75);    // chat area frame
const INPUT_BORDER: Color = Color::Rgb(0, 175, 160); // input box turquoise frame
const ERROR_FG: Color = Color::Rgb(230, 80, 50);     // error lines in tool output
const POPUP_WINDOW: usize = 16;                       // max items visible at once in popup
const CATEGORY_FG: Color = Color::Rgb(60, 60, 60);   // greyed-out category headers in popup

// Tip lines shown below the working indicator — cycle by elapsed seconds
const TIPS: &[&str] = &[
    "Use /compress to free context space mid-session",
    "Use /model to switch providers without losing session history",
    "Use /permissions danger-full-access for unrestricted shell access",
    "PageUp / PageDown to scroll through conversation history",
    "Type /help to see all available slash commands",
    "Press esc to interrupt a running turn at any time",
    "Use /session list to see and switch between saved sessions",
    "Use /init to scaffold an ALBERT.md for project context",
    "Use /memory to view the agent's persistent long-term memory",
    "Use /commit to have AI draft a perfect git commit message",
    "Use /diff to review current changes before committing",
    "Use /tdd to enter a test-driven development loop",
    "Use /bughunter to scan the codebase for potential issues",
    "Use /refactor to improve the structure of the current file",
    "Ctrl+L scrolls the conversation back to the bottom",
    "Ctrl+Space toggles voice recording (STT) for hands-free input",
    "Shift+Enter adds a newline to your message",
    "Tab completes slash commands and opens sub-menus",
    "Use /aside to take temporary notes during a deep session",
    "Use /export to save the current conversation to a file",
    "Use /checkpoint to save a snapshot of the current workspace",
];

// Permission modes (in display/cycle order)
const PERM_MODES: &[(&str, &str)] = &[
    ("read-only",           "no writes · no shell"),
    ("workspace-write",     "files only · no shell"),
    ("danger-full-access",  "unrestricted · full shell"),
];

// Known models for the in-popup model picker — (id, provider, description)
const MODEL_ENTRIES: &[(&str, &str, &str)] = &[
    // Google
    ("gemini-2.5-pro",                              "Google",       "Most capable Gemini"),
    ("gemini-2.5-flash",                            "Google",       "Fast & capable — recommended"),
    ("gemini-2.5-flash-lite",                       "Google",       "Lightest Gemini"),
    // Anthropic
    ("claude-opus-4-7",                             "Anthropic",    "Most capable Claude"),
    ("claude-sonnet-4-6",                           "Anthropic",    "Best balance"),
    ("claude-haiku-4-5-20251001",                   "Anthropic",    "Fastest Claude"),
    // OpenAI
    ("gpt-4o",                                      "OpenAI",       "GPT-4o flagship"),
    ("gpt-4o-mini",                                 "OpenAI",       "Efficient GPT-4o"),
    ("gpt-5",                                       "OpenAI",       "GPT-5 frontier"),
    ("o3",                                          "OpenAI",       "Full o3 reasoning"),
    ("o3-mini",                                     "OpenAI",       "o3 reasoning — efficient"),
    // xAI
    ("grok-3",                                      "xAI",          "Grok 3 flagship"),
    ("grok-3-mini",                                 "xAI",          "Efficient Grok"),
    // Groq LPU
    ("llama-3.3-70b-versatile",                     "Groq",         "Llama 3.3 70B — ultra-fast LPU"),
    ("llama-3.1-8b-instant",                        "Groq",         "Llama 3.1 8B — fastest/cheapest"),
    ("gemma2-9b-it",                                "Groq",         "Gemma2 9B on Groq"),
    // Mistral
    ("mistral-large-latest",                        "Mistral",      "Mistral Large 2"),
    ("mistral-small-latest",                        "Mistral",      "Mistral Small — fast"),
    ("codestral-latest",                            "Mistral",      "Code specialist"),
    ("pixtral-large-latest",                        "Mistral",      "Pixtral multimodal"),
    // DeepSeek
    ("deepseek-chat",                               "DeepSeek",     "DeepSeek V3 flagship"),
    ("deepseek-reasoner",                           "DeepSeek",     "DeepSeek R1 chain-of-thought"),
    // OpenRouter
    ("openai/gpt-4o",                               "OpenRouter",   "GPT-4o via OpenRouter"),
    ("anthropic/claude-sonnet-4-6",                 "OpenRouter",   "Claude Sonnet 4.6"),
    ("google/gemini-2.5-flash",                     "OpenRouter",   "Gemini Flash"),
    ("x-ai/grok-3-mini",                            "OpenRouter",   "Grok 3 Mini"),
    // Perplexity
    ("sonar-pro",                                   "Perplexity",   "Search-grounded Pro"),
    ("sonar",                                       "Perplexity",   "Search-grounded Fast"),
    // Cohere
    ("command-r-plus",                              "Cohere",       "Command R+ RAG flagship"),
    ("command-r",                                   "Cohere",       "Command R — efficient"),
    // Cerebras
    ("llama3.3-70b",                                "Cerebras",     "Llama 3.3 70B on WSE"),
    // Qwen
    ("qwen-max",                                    "Qwen",         "Qwen Max flagship"),
    ("qwq-32b",                                     "Qwen",         "QwQ 32B chain-of-thought"),
    // NVIDIA NIM
    ("nvidia/llama-3.1-nemotron-70b-instruct",      "NVIDIA NIM",   "Nemotron 70B"),
    // Together AI
    ("meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo","Together",     "Llama 3.1 70B Turbo"),
    // Local
    ("llama3.2",                                    "Ollama",       "Llama 3.2 local"),
    ("phi4",                                        "Ollama",       "Phi-4 local"),
    ("qwen2.5-coder:14b",                           "Ollama",       "Qwen2.5 Coder local"),
    ("local-model",                                 "LM Studio",    "Active LM Studio model"),
];

// ── Data model ────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum ExecBlock {
    /// User message:  > text  on slightly dark background
    UserMessage(String),
    /// Tool call — green dot while active, grey when done
    ToolUse { name: String, args: String, active: bool },
    /// L-shaped output under a ToolUse
    ToolOutput { lines: Vec<String>, total: usize },
    /// Streaming agent text
    AgentText(String),
    /// System / info note
    SystemMsg(String),
    /// Post-turn elapsed time: "Worked for Xm Ys"
    WorkedFor(u64),
    /// Pre-formatted verbatim text — bypasses markdown, renders each line as-is.
    RawText(String),
}

#[derive(Clone, Debug)]
pub struct TuiState {
    pub exec_log: VecDeque<ExecBlock>,
    pub input: String,
    pub cursor: usize,
    pub tokens_in: u32,
    pub tokens_out: u32,
    pub model: String,
    pub cwd: String,
    pub permission_mode: String,
    pub session_start: Instant,
    /// Set when a turn starts, cleared when it ends — drives the working timer.
    pub turn_start: Option<Instant>,
    pub working: bool,
    /// Rows scrolled up from the bottom (0 = follow latest)
    pub scroll: u16,
    /// Selected index in the active popup
    pub popup_selected: usize,
    /// True while voice recording is active (Ctrl+Space toggle)
    pub is_recording: bool,
    /// Set while waiting for an API key — input is masked + submitted as the key.
    pub auth_flow: Option<String>,
    /// Set when the current input arrived via a large paste (>= 3 lines).
    /// Drives the compact "pasted text · N lines" badge in render_input.
    pub paste_line_count: Option<usize>,
    /// Show the full help popup overlay (opened by /help, closed by Esc).
    pub help_open: bool,
    /// Scroll offset inside the help popup.
    pub help_scroll: u16,
    /// Ordered list of previously submitted messages (max 200).
    pub input_history: Vec<String>,
    /// Index into input_history while browsing (None = not browsing).
    pub history_idx: Option<usize>,
    /// Stashed live input while browsing history — restored on Down past end.
    pub input_saved: String,

    // ── Session Metrics ──────────────────────────────────────────────────────
    pub tool_calls: usize,
    pub tool_success: usize,
    pub tool_failure: usize,
    pub agent_active_ms: u64,
    pub api_time_ms: u64,
    pub tool_time_ms: u64,
    pub session_id: String,

    /// Set when Ctrl+C is pressed once.
    pub quit_confirm: bool,
    /// Whether the user has explicitly trusted this directory for this session.
    pub trusted: bool,
    /// Whether the agent is currently blocked waiting for a permission prompt response.
    pub is_prompting: Arc<AtomicBool>,
}

impl Default for TuiState {
    fn default() -> Self {
        Self {
            exec_log: VecDeque::new(),
            input: String::new(),
            cursor: 0,
            tokens_in: 0,
            tokens_out: 0,
            model: String::new(),
            cwd: String::new(),
            permission_mode: String::new(),
            session_start: Instant::now(),
            turn_start: None,
            working: false,
            scroll: 0,
            popup_selected: 0,
            is_recording: false,
            auth_flow: None,
            paste_line_count: None,
            help_open: false,
            help_scroll: 0,
            input_history: Vec::new(),
            history_idx: None,
            input_saved: String::new(),
            tool_calls: 0,
            tool_success: 0,
            tool_failure: 0,
            agent_active_ms: 0,
            api_time_ms: 0,
            tool_time_ms: 0,
            session_id: String::new(),
            quit_confirm: false,
            trusted: false,
            is_prompting: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl TuiState {
    pub fn new(model: String, cwd: String, permission_mode: String, session_id: String) -> Self {
        Self { model, cwd, permission_mode, session_id, ..Default::default() }
    }

    pub fn push_exec(&mut self, block: ExecBlock) {
        if matches!(&block, ExecBlock::ToolUse { .. }) {
            self.deactivate_last_tool();
        }

        // Bug fix: Remove ASCII splash and startup system message when the first user message arrives
        if matches!(&block, ExecBlock::UserMessage(_)) {
            let has_splash = self.exec_log.iter().any(|b| {
                if let ExecBlock::RawText(s) = b {
                    s.contains("██████╗")
                } else {
                    false
                }
            });
            if has_splash {
                self.exec_log.retain(|b| {
                    match b {
                        ExecBlock::RawText(s) if s.contains("██████╗") => false,
                        ExecBlock::SystemMsg(s) if s.contains("v") && s.contains("type /help") => false,
                        _ => true,
                    }
                });
            }
        }

        // Filter out meta-tools from "Ran [tool]" display
        if let ExecBlock::ToolUse { ref name, .. } = block {
            if name == "SendUserMessage" || name == "Brief" {
                return;
            }
        }

        // Suppress consecutive identical SystemMsg entries (e.g. repeated voice errors).
        if let ExecBlock::SystemMsg(ref msg) = block {
            if let Some(ExecBlock::SystemMsg(ref last)) = self.exec_log.back() {
                if last == msg {
                    return;
                }
            }
        }
        self.exec_log.push_back(block);
        // Keep the log bounded so rendering stays fast — older blocks are trimmed.
        while self.exec_log.len() > 120 {
            self.exec_log.pop_front();
        }
    }

    /// Mark the most-recent active ToolUse as completed (grey dot).
    pub fn deactivate_last_tool(&mut self) {
        for block in self.exec_log.iter_mut().rev() {
            if let ExecBlock::ToolUse { active, .. } = block {
                if *active {
                    *active = false;
                    break;
                }
            }
        }
    }

    pub fn input_insert(&mut self, ch: char) {
        let pos = self
            .input
            .char_indices()
            .nth(self.cursor)
            .map(|(i, _)| i)
            .unwrap_or(self.input.len());
        self.input.insert(pos, ch);
        self.cursor += 1;
    }

    pub fn input_backspace(&mut self) {
        if self.cursor > 0 {
            let pos = self
                .input
                .char_indices()
                .nth(self.cursor - 1)
                .map(|(i, _)| i)
                .unwrap();
            self.input.remove(pos);
            self.cursor -= 1;
        }
    }

    pub fn input_delete(&mut self) {
        let len = self.input.chars().count();
        if self.cursor < len {
            let pos = self
                .input
                .char_indices()
                .nth(self.cursor)
                .map(|(i, _)| i)
                .unwrap();
            self.input.remove(pos);
        }
    }

    pub fn input_take(&mut self) -> String {
        self.cursor = 0;
        self.paste_line_count = None;
        std::mem::take(&mut self.input)
    }

    pub fn history_push(&mut self, text: &str) {
        let text = text.trim().to_string();
        if text.is_empty() { return; }
        if self.input_history.last().map(|s| s == &text).unwrap_or(false) { return; }
        self.input_history.push(text);
        if self.input_history.len() > 200 { self.input_history.remove(0); }
        self.history_idx = None;
    }

    pub fn history_prev(&mut self) {
        if self.input_history.is_empty() { return; }
        match self.history_idx {
            None => {
                self.input_saved = self.input.clone();
                let idx = self.input_history.len() - 1;
                self.history_idx = Some(idx);
                self.input = self.input_history[idx].clone();
                self.cursor = self.input.chars().count();
            }
            Some(0) => {}
            Some(idx) => {
                let new = idx - 1;
                self.history_idx = Some(new);
                self.input = self.input_history[new].clone();
                self.cursor = self.input.chars().count();
            }
        }
    }

    pub fn history_next(&mut self) {
        match self.history_idx {
            None => {}
            Some(idx) if idx + 1 >= self.input_history.len() => {
                self.history_idx = None;
                self.input = std::mem::take(&mut self.input_saved);
                self.cursor = self.input.chars().count();
            }
            Some(idx) => {
                let new = idx + 1;
                self.history_idx = Some(new);
                self.input = self.input_history[new].clone();
                self.cursor = self.input.chars().count();
            }
        }
    }
}

// ── Word-boundary helpers ─────────────────────────────────────────────────────

fn word_left(input: &str, cursor: usize) -> usize {
    if cursor == 0 { return 0; }
    let chars: Vec<char> = input.chars().collect();
    let mut pos = cursor - 1;
    while pos > 0 && chars[pos].is_whitespace() { pos -= 1; }
    while pos > 0 && !chars[pos - 1].is_whitespace() { pos -= 1; }
    pos
}

fn word_right(input: &str, cursor: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    if cursor >= len { return len; }
    let mut pos = cursor;
    while pos < len && !chars[pos].is_whitespace() { pos += 1; }
    while pos < len && chars[pos].is_whitespace() { pos += 1; }
    pos
}

// ── Events ────────────────────────────────────────────────────────────────────

pub enum TuiEvent {
    Key(KeyEvent),
    AgentEvent(AssistantEvent),
    Tick,
    /// Main thread needs terminal for a slash command — TUI yields and waits.
    Suspend { ack: std::sync::mpsc::SyncSender<()> },
    Resume,
    Quit,
    /// Exit and show the session report card.
    QuitWithReport,
    /// Voice transcription result — insert this text at the cursor.
    VoiceText(String),
    /// Voice transcription failed — show the error message.
    VoiceError(String),
    /// Bracketed paste — insert without triggering submit on newlines.
    PasteText(String),
}

// ── Popup items ───────────────────────────────────────────────────────────────

#[derive(Clone)]
struct PopupItem {
    display: String,
    complete: String,
    desc: String,
    /// Category header row — not selectable, rendered differently.
    is_header: bool,
}

impl PopupItem {
    fn cmd(display: &str, complete: &str, desc: &str) -> Self {
        Self { display: display.to_string(), complete: complete.to_string(), desc: desc.to_string(), is_header: false }
    }
    fn header(label: &str) -> Self {
        Self { display: label.to_string(), complete: String::new(), desc: String::new(), is_header: true }
    }
}

// Command groups for the categorised root view (shown when input == "/")
const CMD_GROUPS: &[(&str, &[&str])] = &[
    ("CONFIG",    &["model", "permissions", "auth"]),
    ("SESSION",   &["status", "compact", "compress", "clear", "cost", "export", "session", "resume"]),
    ("GIT",       &["commit", "pr", "issue", "diff"]),
    ("AGENT",     &["plan", "loop", "tdd", "verify", "code-review", "build-fix", "bughunter", "ultraplan", "refactor"]),
    ("WORKSPACE", &["init", "memory", "config", "docs", "learn", "checkpoint", "aside", "teleport", "debug-tool-call"]),
    ("INFO",      &["help", "version"]),
];

/// Commands that open a sub-menu when Enter is pressed (rather than submitting directly).
/// Enter → fills input with "/cmd " → popup re-renders the sub-options.
fn is_drilldown(complete: &str) -> bool {
    matches!(complete, "/model" | "/permissions" | "/auth")
}

// All supported auth providers (id, description)
const AUTH_PROVIDERS: &[(&str, &str)] = &[
    ("anthropic",  "Claude opus-4-7 · sonnet-4-6 · haiku-4-5"),
    ("openai",     "GPT-4o · GPT-4o-mini · o3 · o3-mini"),
    ("google",     "Gemini 2.5 Pro · Flash · Flash-Lite"),
    ("xai",        "Grok 3 · Grok 3-mini"),
    ("groq",       "Llama 3.3 70B · 8B — ultra-fast LPU"),
    ("mistral",    "Mistral Large · Small · Codestral"),
    ("deepseek",   "DeepSeek V3 · R1 chain-of-thought"),
    ("openrouter", "100+ models via unified API"),
    ("perplexity", "Sonar Pro · Sonar — search-grounded"),
    ("cohere",     "Command R+ · Command R — RAG"),
    ("cerebras",   "Llama 3.3 70B on WSE accelerator"),
    ("together",   "Open source models at scale"),
    ("fireworks",  "Fast inference — Llama, Mistral, …"),
    ("novita",     "Cost-efficient open model hosting"),
    ("ollama",     "Local models — no API key required"),
];

/// Returns popup items for the current input:
///   /              → full categorised command list  (Enter drills into sub-commands)
///   /partial       → flat filtered list
///   /permissions   → permission mode picker  (Enter applies mode)
///   /model         → model picker            (Enter switches model)
///   /auth          → provider picker         (Enter starts auth flow)
fn popup_items(input: &str) -> Vec<PopupItem> {
    if !input.starts_with('/') {
        return vec![];
    }

    // ── Permission mode picker ─────────────────────────────────────────────
    if input.starts_with("/permissions") {
        let partial = input.strip_prefix("/permissions").unwrap_or("").trim();
        return PERM_MODES
            .iter()
            .filter(|(mode, _)| partial.is_empty() || mode.starts_with(partial))
            .map(|(mode, desc)| PopupItem::cmd(
                &format!("permissions  {mode}"),
                &format!("/permissions {mode}"),
                desc,
            ))
            .collect();
    }

    // ── Model picker ──────────────────────────────────────────────────────
    if input.starts_with("/model") {
        let partial = input.strip_prefix("/model").unwrap_or("").trim();
        let mut items: Vec<PopupItem> = Vec::new();
        let mut cur_provider = "";
        for (id, provider, desc) in MODEL_ENTRIES {
            if !partial.is_empty() && !id.contains(partial) && !provider.to_lowercase().contains(partial) {
                continue;
            }
            if *provider != cur_provider {
                items.push(PopupItem::header(provider));
                cur_provider = provider;
            }
            items.push(PopupItem::cmd(
                id,
                &format!("/model {id}"),
                desc,
            ));
        }
        return items;
    }

    // ── Auth provider picker ───────────────────────────────────────────────
    if input.starts_with("/auth") {
        let partial = input.strip_prefix("/auth").unwrap_or("").trim();
        return AUTH_PROVIDERS
            .iter()
            .filter(|(p, _)| partial.is_empty() || p.starts_with(partial))
            .map(|(provider, desc)| PopupItem::cmd(
                &format!("auth  {provider}"),
                &format!("/auth {provider}"),
                desc,
            ))
            .collect();
    }

    let prefix = &input[1..]; // text after the "/"

    // ── Root view: type "/" alone → categorised full list ─────────────────
    if prefix.is_empty() {
        let specs = slash_command_specs();
        let mut items: Vec<PopupItem> = Vec::new();
        for (label, names) in CMD_GROUPS {
            let group_items: Vec<PopupItem> = names
                .iter()
                .filter_map(|&n| specs.iter().find(|s| s.name == n))
                .map(|s| {
                    // Drill-down commands show "›" hint; others show their argument hint.
                    let hint = if is_drilldown(&format!("/{}", s.name)) {
                        "  ›".to_string()
                    } else {
                        s.argument_hint.map(|h| format!(" {h}")).unwrap_or_default()
                    };
                    PopupItem::cmd(
                        &format!("{}{hint}", s.name),
                        &format!("/{}", s.name),
                        s.summary,
                    )
                })
                .collect();
            if !group_items.is_empty() {
                items.push(PopupItem::header(label));
                items.extend(group_items);
            }
        }
        return items;
    }

    // ── Prefix search: flat filtered list ─────────────────────────────────
    slash_command_specs()
        .iter()
        .filter(|s| s.name.starts_with(prefix))
        .map(|s| {
            let hint = if is_drilldown(&format!("/{}", s.name)) {
                "  ›".to_string()
            } else {
                s.argument_hint.map(|h| format!(" {h}")).unwrap_or_default()
            };
            PopupItem::cmd(
                &format!("{}{hint}", s.name),
                &format!("/{}", s.name),
                s.summary,
            )
        })
        .collect()
}

// ── Tool preview ──────────────────────────────────────────────────────────────

/// Extract a clean human-readable preview from a tool's JSON input.
pub fn tool_input_preview(input: &str) -> String {
    const MAX: usize = 90;

    if let Ok(val) = serde_json::from_str::<serde_json::Value>(input) {
        // Ordered priority: first matching non-empty string key wins
        let priority = [
            "command", "path", "file_path", "pattern", "query",
            "url", "prompt", "text", "content",
        ];
        for key in &priority {
            if let Some(s) = val.get(key).and_then(|v| v.as_str()) {
                let s = s.trim();
                if !s.is_empty() {
                    return truncate(s, MAX);
                }
            }
        }
        // Fallback: first string value in the object
        if let Some(obj) = val.as_object() {
            for (_, v) in obj {
                if let Some(s) = v.as_str() {
                    let s = s.trim();
                    if !s.is_empty() {
                        return truncate(s, MAX);
                    }
                }
            }
        }
    }

    truncate(input.trim(), MAX)
}

fn truncate(s: &str, max_chars: usize) -> String {
    let count = s.chars().count();
    if count <= max_chars {
        return s.to_string();
    }
    let end = s.char_indices().nth(max_chars).map(|(i, _)| i).unwrap_or(s.len());
    format!("{}…", &s[..end])
}

// ── Markdown rendering ────────────────────────────────────────────────────────

fn md_flush(spans: &mut Vec<Span<'static>>, lines: &mut Vec<Line<'static>>) {
    if !spans.is_empty() {
        lines.push(Line::from(std::mem::take(spans)));
    }
}

/// Convert a markdown string to styled ratatui Lines.
fn markdown_to_lines(text: &str) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut bold = false;
    let mut italic = false;
    let mut in_code_block = false;
    let mut in_heading = false;
    let mut heading_color = FG;
    let mut list_depth: usize = 0;
    let mut item_needs_bullet = false;

    let opts = MdOptions::ENABLE_STRIKETHROUGH;
    let parser = MdParser::new_ext(text, opts);

    for event in parser {
        match event {
            MdEvent::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                heading_color = match level {
                    HeadingLevel::H1 => GREEN,
                    HeadingLevel::H2 => CYAN,
                    _ => FG,
                };
            }
            MdEvent::End(TagEnd::Heading(_)) => {
                md_flush(&mut spans, &mut lines);
                in_heading = false;
            }
            MdEvent::Start(Tag::Strong) => bold = true,
            MdEvent::End(TagEnd::Strong) => bold = false,
            MdEvent::Start(Tag::Emphasis) => italic = true,
            MdEvent::End(TagEnd::Emphasis) => italic = false,
            MdEvent::Start(Tag::CodeBlock(_)) => in_code_block = true,
            MdEvent::End(TagEnd::CodeBlock) => {
                md_flush(&mut spans, &mut lines);
                lines.push(Line::default());
                in_code_block = false;
            }
            MdEvent::Start(Tag::List(_)) => list_depth += 1,
            MdEvent::End(TagEnd::List(_)) => list_depth = list_depth.saturating_sub(1),
            MdEvent::Start(Tag::Item) => item_needs_bullet = true,
            MdEvent::End(TagEnd::Item) => md_flush(&mut spans, &mut lines),
            MdEvent::Start(Tag::Paragraph) => {}
            MdEvent::End(TagEnd::Paragraph) => {
                md_flush(&mut spans, &mut lines);
                lines.push(Line::default());
            }
            MdEvent::Text(t) => {
                if item_needs_bullet {
                    item_needs_bullet = false;
                    let indent = "  ".repeat(list_depth.saturating_sub(1));
                    spans.push(Span::styled(
                        format!("{indent}• "),
                        Style::default().fg(DIM),
                    ));
                }
                if in_code_block {
                    for line in t.lines() {
                        lines.push(Line::from(vec![
                            Span::styled("▌", Style::default().fg(CODE_BAR).bg(CODE_BG)),
                            Span::styled(format!(" {line}"), Style::default().fg(CODE_FG).bg(CODE_BG)),
                        ]));
                    }
                } else {
                    let mut style = Style::default().fg(FG);
                    if in_heading {
                        style = style.fg(heading_color).add_modifier(Modifier::BOLD);
                    } else {
                        if bold { style = style.add_modifier(Modifier::BOLD); }
                        if italic { style = style.add_modifier(Modifier::ITALIC); }
                    }
                    spans.push(Span::styled(t.to_string(), style));
                }
            }
            MdEvent::Code(c) => {
                spans.push(Span::styled(
                    format!("`{c}`"),
                    Style::default().fg(CODE_FG),
                ));
            }
            MdEvent::SoftBreak => {
                spans.push(Span::styled(" ".to_string(), Style::default().fg(FG)));
            }
            MdEvent::HardBreak => md_flush(&mut spans, &mut lines),
            MdEvent::Rule => {
                md_flush(&mut spans, &mut lines);
                lines.push(Line::from(Span::styled(
                    "─".repeat(60),
                    Style::default().fg(DIM),
                )));
            }
            _ => {}
        }
    }

    md_flush(&mut spans, &mut lines);
    lines
}

// ── Rendering ─────────────────────────────────────────────────────────────────

pub fn render(f: &mut ratatui::Frame, state: &TuiState) {
    let area = f.area();
    // No popup while waiting for an API key.
    let items = if state.auth_flow.is_some() { vec![] } else { popup_items(&state.input) };
    let n_items = items.len();
    // Popup: up to POPUP_WINDOW items + 1 nav footer; placed BELOW input (Gemini-style)
    let popup_h = if n_items == 0 { 0u16 } else { (n_items.min(POPUP_WINDOW) + 1) as u16 };

    // Input height: text rows + 2 border rows. Text expands 1..3 rows with content.
    // Usable width = total - " > " prefix (3) - badge (≈15) - margin (1) - borders (2).
    let input_h = {
        let badge_approx = git_branch_cached()
            .map(|b| b.chars().count() as u16 + 2)
            .unwrap_or(0);
        let usable = area.width.saturating_sub(3 + badge_approx + 3).max(10) as usize;
        let n = if state.paste_line_count.is_some() { 1 } else { state.input.chars().count() };
        let text_rows = if n == 0 { 1u16 } else if state.paste_line_count.is_some() { 1u16 } else { ((n + usable - 1) / usable).max(1).min(10) as u16 };
        text_rows + 2 // top + bottom border rows
    };

    // Layout top→bottom: content(flex) | status(1r) | input(dynamic) | [popup?] | tips(1r) | footer(1r)
    let mut constraints = vec![
        Constraint::Min(3),
        Constraint::Length(1),        // status strip (always visible)
        Constraint::Length(input_h),  // expanding input
    ];
    if popup_h > 0 { constraints.push(Constraint::Length(popup_h)); }
    constraints.push(Constraint::Length(1)); // rotating tip row
    constraints.push(Constraint::Length(1)); // footer

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    let mut idx = 0usize;
    render_content(f, layout[idx], state);
    idx += 1;
    render_status(f, layout[idx], state);
    idx += 1;
    render_input(f, layout[idx], state);
    idx += 1;
    if popup_h > 0 {
        let sel = state.popup_selected.min(n_items.saturating_sub(1));
        render_popup(f, layout[idx], &items, sel);
        idx += 1;
    }
    render_tips(f, layout[idx], state);
    idx += 1;
    render_footer(f, layout[idx], state);

    // Help overlay floats on top of everything — rendered last so it covers all other widgets.
    if state.help_open {
        render_help_overlay(f, area, state.help_scroll);
    }
}

fn build_exec_lines(state: &TuiState, _width: u16) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    for block in &state.exec_log {
        match block {
            ExecBlock::UserMessage(msg) => {
                lines.push(Line::default());
                lines.push(Line::from(vec![
                    Span::styled(
                        " ≻ ",
                        Style::default().fg(CYAN).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(msg.clone(), Style::default().fg(FG).bg(USER_BOX_BG)),
                ]));
            }

            ExecBlock::ToolUse { name, args, active } => {
                let dot_style = if *active {
                    Style::default().fg(GREEN).add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
                } else {
                    Style::default().fg(GREY)
                };
                let (name_col, args_col) = if *active { (FG, CYAN) } else { (GREY, GREY) };
                let mut spans = vec![
                    Span::styled("● ", dot_style),
                    Span::styled(
                        format!("Ran {name}"),
                        Style::default().fg(name_col).add_modifier(Modifier::BOLD),
                    ),
                ];
                if !args.is_empty() {
                    spans.push(Span::styled(
                        format!("  {args}"),
                        Style::default().fg(args_col),
                    ));
                }
                lines.push(Line::from(spans));
            }

            ExecBlock::ToolOutput { lines: out, total } => {
                for (i, line) in out.iter().enumerate() {
                    let connector = if i == 0 { "  └ " } else { "    " };
                    let lower = line.to_ascii_lowercase();
                    let is_err = lower.contains("error")
                        || lower.contains("not found")
                        || lower.contains("permission denied")
                        || lower.contains("failed:")
                        || lower.starts_with("error")
                        || lower.starts_with("fatal");
                    let line_col = if is_err { ERROR_FG } else { Color::Rgb(110, 120, 120) };
                    lines.push(Line::from(vec![
                        Span::styled(connector, Style::default().fg(DIM)),
                        Span::styled(line.clone(), Style::default().fg(line_col)),
                    ]));
                }
                if *total > out.len() {
                    lines.push(Line::from(vec![
                        Span::styled("    ", Style::default()),
                        Span::styled(
                            format!("… +{} lines", total - out.len()),
                            Style::default().fg(DIM).add_modifier(Modifier::ITALIC),
                        ),
                    ]));
                }
            }

            ExecBlock::AgentText(text) => {
                lines.push(Line::default());
                lines.push(Line::from(vec![
                    Span::styled("albert", Style::default().fg(Color::Rgb(0, 170, 120)).add_modifier(Modifier::BOLD)),
                    Span::styled(" ─────────────────────", Style::default().fg(Color::Rgb(25, 45, 45))),
                ]));
                lines.extend(markdown_to_lines(text));
            }

            // WorkedFor is shown in the status bar — not duplicated in the chat log.
            ExecBlock::WorkedFor(_) => {}

            ExecBlock::SystemMsg(msg) => {
                lines.push(Line::default());
                for line in msg.lines() {
                    lines.push(Line::from(vec![
                        Span::styled("* ", Style::default().fg(DIM)),
                        Span::styled(line.to_string(), Style::default().fg(GREY)),
                    ]));
                }
            }

            ExecBlock::RawText(text) => {
                lines.push(Line::default());
                for line in text.lines() {
                    lines.push(Line::from(Span::styled(
                        line.to_string(),
                        Style::default().fg(FG),
                    )));
                }
            }
        }
    }

    lines
}

fn render_content(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let scroll_indicator = if state.scroll > 0 {
        format!(" ↑ {}  ctrl+l → bottom ", state.scroll)
    } else {
        String::new()
    };

    let title_line = if scroll_indicator.is_empty() {
        Line::from(vec![
            Span::styled(" albert ", Style::default().fg(CHAT_BORDER).add_modifier(Modifier::BOLD))
        ])
    } else {
        Line::from(vec![
            Span::styled(" albert ", Style::default().fg(CHAT_BORDER).add_modifier(Modifier::BOLD)),
            Span::styled(scroll_indicator, Style::default().fg(GREY)),
        ])
    };

    let block = Block::default()
        .title(title_line)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(CHAT_BORDER))
        .style(Style::default().bg(BG));

    let inner = block.inner(area);
    let lines = build_exec_lines(state, inner.width);
    let w = inner.width.max(1) as usize;

    // Compute total rendered height in rows, accounting for text wrapping.
    // Using usize to avoid u16 overflow with large logs.
    let total_wrapped: usize = lines
        .iter()
        .map(|line| {
            let chars: usize = line.spans.iter().map(|s| s.content.chars().count()).sum();
            if chars == 0 { 1 } else { (chars + w - 1) / w }
        })
        .sum();

    let visible = inner.height as usize;
    let total_wrapped_count = total_wrapped;
    // max_scroll is how many rows we can scroll up from the bottom
    let max_scroll = total_wrapped_count.saturating_sub(visible);
    
    // Clamp state.scroll so we never over-scroll past the top
    let effective_scroll = (state.scroll as usize).min(max_scroll);
    
    // scroll_row is the top row to pass to Paragraph (0 = top of content)
    let scroll_row = max_scroll.saturating_sub(effective_scroll).min(u16::MAX as usize) as u16;

    let para = Paragraph::new(Text::from(lines))
        .style(Style::default().bg(BG).fg(FG))
        .wrap(Wrap { trim: false })
        .scroll((scroll_row, 0))
        .block(block);
    f.render_widget(para, area);
}

fn render_popup(f: &mut ratatui::Frame, area: Rect, items: &[PopupItem], selected: usize) {
    let total = items.len();
    let win_size = total.min(POPUP_WINDOW);

    // Center the visible window around the selected item
    let win_start = selected
        .saturating_sub(win_size / 2)
        .min(total.saturating_sub(win_size));
    let win_end = (win_start + win_size).min(total);

    let selectable_total = items.iter().filter(|i| !i.is_header).count();
    let selectable_idx = items[..selected.min(total.saturating_sub(1))]
        .iter()
        .filter(|i| !i.is_header)
        .count();

    // Is the currently selected item a drill-down parent?
    let sel_item = items.get(selected.min(total.saturating_sub(1)));
    let sel_is_drilldown = sel_item
        .map(|it| !it.is_header && is_drilldown(&it.complete))
        .unwrap_or(false);

    let mut lines: Vec<Line<'static>> = Vec::new();

    for (abs_i, item) in items[win_start..win_end].iter().enumerate() {
        let i = win_start + abs_i;
        if item.is_header {
            let label = format!("  {} ", item.display);
            lines.push(Line::from(Span::styled(label, Style::default().fg(CATEGORY_FG).bg(POPUP_BG))));
        } else {
            let is_sel = i == selected;
            let bg = if is_sel { POPUP_SEL_BG } else { POPUP_BG };
            let name_col = if is_sel { GREEN } else { POPUP_MATCH };
            let desc_col = if is_sel { FG } else { GREY };
            // Drill-down items get a "›" right-hand indicator
            let drilldown_hint = if is_sel && is_drilldown(&item.complete) {
                Span::styled("  ›", Style::default().fg(CYAN).bg(bg).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("", Style::default().bg(bg))
            };
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default().bg(bg)),
                Span::styled(
                    format!("/{}", item.display),
                    Style::default().fg(name_col).bg(bg).add_modifier(Modifier::BOLD),
                ),
                drilldown_hint,
                Span::styled("  ", Style::default().bg(bg)),
                Span::styled(item.desc.clone(), Style::default().fg(desc_col).bg(bg)),
            ]));
        }
    }

    // Nav footer — contextual based on selected item type
    let action_hint = if sel_is_drilldown { "enter → open  ·  " } else { "enter → select  ·  " };
    let nav = if selectable_total > 0 {
        format!(
            "  ({}/{})  ↑↓ navigate  ·  {action_hint}tab → complete  ·  esc dismiss",
            selectable_idx + 1,
            selectable_total,
        )
    } else {
        "  ↑↓ navigate  ·  esc dismiss".to_string()
    };
    lines.push(Line::from(Span::styled(nav, Style::default().fg(DIM).bg(POPUP_BG))));

    let para = Paragraph::new(Text::from(lines)).style(Style::default().bg(POPUP_BG));
    f.render_widget(para, area);
}

/// Full-screen help overlay — floats over the whole terminal, closed with Esc.
fn render_help_overlay(f: &mut ratatui::Frame, area: Rect, scroll: u16) {
    use ratatui::widgets::Clear;

    // Semi-transparent frame: clear the background first, then draw the box
    let overlay = Rect {
        x: area.x + 2,
        y: area.y + 1,
        width: area.width.saturating_sub(4),
        height: area.height.saturating_sub(2),
    };
    f.render_widget(Clear, overlay);

    const SECTION: Color = Color::Rgb(0, 200, 120);
    const CMD_C:   Color = Color::Rgb(0, 200, 255);
    const HINT_C:  Color = Color::Rgb(120, 120, 120);

    let mut lines: Vec<Line<'static>> = Vec::new();
    let h = |s: &'static str| Line::from(Span::styled(s, Style::default().fg(SECTION).add_modifier(Modifier::BOLD)));
    let c = |cmd: &'static str, desc: &'static str| Line::from(vec![
        Span::styled(format!("  {cmd:<22}"), Style::default().fg(CMD_C).add_modifier(Modifier::BOLD)),
        Span::styled(desc, Style::default().fg(FG)),
    ]);
    let hint_line = |s: &'static str| Line::from(Span::styled(format!("  {s}"), Style::default().fg(HINT_C)));
    let blank = || Line::from("");

    lines.push(blank());
    lines.push(h("  MODELS & PROVIDERS"));
    lines.push(c("/model <id>",          "switch model — opens picker when blank"));
    lines.push(c("/auth <provider>",      "set API key for a provider"));
    lines.push(c("/auth browser",         "OAuth browser login (Google / GitHub)"));
    lines.push(hint_line("Providers: anthropic · openai · google · xai · groq · mistral · deepseek"));
    lines.push(hint_line("           together · openrouter · perplexity · cohere · cerebras · qwen"));
    lines.push(hint_line("           nvidia · fireworks · deepinfra · novita · sambanova · ollama"));
    lines.push(blank());

    lines.push(h("  TIPS & INTERACTION"));
    lines.push(hint_line("Selection: Click & Drag to select and copy text."));
    lines.push(hint_line("Scrolling: Mouse wheel or Shift+Up/Down arrows to scroll chat history."));
    lines.push(hint_line("Override:  Hold SHIFT to force native terminal selection/scrolling."));
    lines.push(blank());
    lines.push(h("  SESSION"));
    lines.push(c("/compact",              "summarise old context to free tokens"));
    lines.push(c("/compress",             "aggressive compression — strip tool outputs"));
    lines.push(c("/status",              "show token usage and session info"));
    lines.push(c("/cost",                "show estimated API cost for this session"));
    lines.push(c("/clear",               "wipe conversation history"));
    lines.push(c("/export",              "save conversation to markdown file"));
    lines.push(c("/session",             "list saved sessions"));
    lines.push(c("/resume <id>",          "restore a previous session"));
    lines.push(blank());
    lines.push(h("  AGENT MODES"));
    lines.push(c("/plan <task>",          "decompose task into numbered steps"));
    lines.push(c("/loop <mission>",       "autonomous mode — runs until MISSION COMPLETE"));
    lines.push(c("/tdd <spec>",           "test-driven development loop"));
    lines.push(c("/code-review",          "review staged diff"));
    lines.push(c("/bughunter",            "scan codebase for bugs"));
    lines.push(c("/refactor",             "refactor current file"));
    lines.push(blank());
    lines.push(h("  WORKSPACE & GIT"));
    lines.push(c("/commit",              "commit staged changes with AI message"));
    lines.push(c("/pr",                  "create pull request"));
    lines.push(c("/diff",                "show current git diff"));
    lines.push(c("/init",                "scaffold ALBERT.md in current directory"));
    lines.push(c("/memory",              "view/edit persistent memory"));
    lines.push(blank());
    lines.push(h("  PERMISSIONS"));
    lines.push(c("/permissions",          "show current permission mode — picker when blank"));
    lines.push(hint_line("Modes: read-only · workspace-write · danger-full-access"));
    lines.push(blank());
    lines.push(h("  KEYBOARD"));
    lines.push(hint_line("Enter       send message"));
    lines.push(hint_line("Tab         autocomplete command from popup"));
    lines.push(hint_line("↑ ↓         navigate input history"));
    lines.push(hint_line("PageUp/Dn   scroll conversation (or Shift + ↑/↓)"));
    lines.push(hint_line("MouseWheel  scroll conversation"));
    lines.push(hint_line("Shift+Click select text / right-click (native)"));
    lines.push(hint_line("Esc         interrupt · dismiss popup · close this overlay"));
    lines.push(hint_line("Ctrl+Space  toggle voice recording (whisper STT)"));
    lines.push(hint_line("Ctrl+V      paste from clipboard"));
    lines.push(hint_line("Ctrl+C      quit"));
    lines.push(blank());
    lines.push(Line::from(Span::styled("  Esc to close", Style::default().fg(DIM))));

    let total = lines.len() as u16;
    let visible = overlay.height.saturating_sub(2);
    let max_scroll = total.saturating_sub(visible);
    let scroll = scroll.min(max_scroll);

    let block = Block::default()
        .title(" Albert — Command Reference ")
        .title_style(Style::default().fg(GREEN).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(50, 50, 50)))
        .style(Style::default().bg(Color::Rgb(8, 8, 8)));

    let para = Paragraph::new(Text::from(lines))
        .block(block)
        .scroll((scroll, 0));
    f.render_widget(para, overlay);
}

/// Multi-row expanding input bar wrapped in a turquoise border.
/// Text wraps automatically; the real terminal cursor is placed via set_cursor_position.
fn render_input(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    // Outer turquoise border — rendered on the full area.
    let border_col = if state.auth_flow.is_some() {
        ORANGE // orange border while in API-key entry mode
    } else {
        INPUT_BORDER
    };
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_col))
        .style(Style::default().bg(USER_BOX_BG));
    f.render_widget(input_block.clone(), area);

    // Inner area = area minus the 1-row border on each side.
    let inner = input_block.inner(area);

    let branch = git_branch_cached();
    let badge_text = branch.as_deref().map(|b| format!(" {b} ")).unwrap_or_default();
    let badge_w = badge_text.chars().count() as u16;

    let h_layout = if badge_w > 0 && inner.width > badge_w + 4 {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(4), Constraint::Length(badge_w)])
            .split(inner)
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1)])
            .split(inner)
    };

    let text_area = h_layout[0];

    // Render input text with wrapping (or dim placeholder when empty).
    // In auth_flow mode: show provider prompt and mask the typed key.
    let para = if let Some(n) = state.paste_line_count {
        Paragraph::new(Line::from(vec![
            Span::styled(" ≻ ", Style::default().fg(CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("[Pasted Text: {} lines]", n),
                Style::default()
                    .fg(Color::Rgb(198, 120, 221))
                    .bg(Color::Rgb(40, 44, 52)),
            ),
            Span::styled("  Esc to clear", Style::default().fg(DIM).add_modifier(Modifier::ITALIC)),
        ]))
    } else if let Some(ref provider) = state.auth_flow {
        if state.input.is_empty() {
            Paragraph::new(Line::from(vec![
                Span::styled(" 🔑 ", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("API key for {provider}:"),
                    Style::default().fg(ORANGE),
                ),
                Span::styled("  (press Enter to save)", Style::default().fg(DIM)),
            ]))
        } else {
            let masked: String = "*".repeat(state.input.chars().count());
            Paragraph::new(Line::from(vec![
                Span::styled(" 🔑 ", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::styled(masked, Style::default().fg(ORANGE)),
            ]))
        }
    } else if state.input.is_empty() {
        Paragraph::new(Line::from(vec![
            Span::styled(" > ", Style::default().fg(CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("Type your message or @path/to/file", Style::default().fg(DIM)),
        ]))
    } else {
        let (prompt_txt, prompt_col) = if state.history_idx.is_some() {
            (" ↑ ", ORANGE)
        } else {
            (" > ", CYAN)
        };
        Paragraph::new(Line::from(vec![
            Span::styled(prompt_txt, Style::default().fg(prompt_col).add_modifier(Modifier::BOLD)),
            Span::styled(state.input.clone(), Style::default().fg(FG)),
        ]))
        .wrap(Wrap { trim: false })
    };
    f.render_widget(para.style(Style::default().bg(USER_BOX_BG)), text_area);

    // Place the real blinking terminal cursor inside the inner text area.
    {
        const PREFIX: u16 = 3; // " > " or " ≻ "
        let (cx, cy) = if state.paste_line_count.is_some() {
            // Position cursor at the very end of the badge line
            (text_area.x + PREFIX + 1, text_area.y) // Just put it after the "≻"
        } else {
            let text_w = text_area.width.saturating_sub(PREFIX).max(1) as usize;
            let visual_row = state.cursor / text_w;
            let visual_col = state.cursor % text_w;
            let cx = (text_area.x + PREFIX + visual_col as u16)
                .min(text_area.x + text_area.width.saturating_sub(1));
            let cy = (text_area.y + visual_row as u16)
                .min(text_area.y + text_area.height.saturating_sub(1));
            (cx, cy)
        };
        f.set_cursor_position((cx, cy));
    }

    // Branch badge (inside the border, right side)
    if h_layout.len() == 2 {
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                badge_text,
                Style::default().fg(GREEN).bg(BRANCH_BG).add_modifier(Modifier::BOLD),
            )))
            .style(Style::default().bg(BRANCH_BG)),
            h_layout[1],
        );
    }
}

pub fn render_report_card(f: &mut ratatui::Frame, state: &TuiState) {
    let area = f.area();
    let w = 76;
    let h = 20; // Increased from 18 to fit tokens
    let x = (area.width.saturating_sub(w)) / 2;
    let y = (area.height.saturating_sub(h)) / 2;
    let popup_area = Rect::new(x, y, w.min(area.width), h.min(area.height));

    let mut lines = Vec::new();
    lines.push(Line::from(vec![
        Span::styled(" 𒀭 Agent powering down. Goodbye!", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::default());

    let label_style = Style::default().fg(GREY);
    let value_style = Style::default().fg(CYAN).add_modifier(Modifier::BOLD);

    lines.push(Line::from(Span::styled("  Interaction Summary", Style::default().add_modifier(Modifier::BOLD))));
    lines.push(Line::from(vec![
        Span::styled("  Session ID:                 ", label_style),
        Span::styled(&state.session_id, value_style),
    ]));

    let success_rate = if state.tool_calls > 0 {
        (state.tool_success as f32 / state.tool_calls as f32) * 100.0
    } else {
        0.0
    };

    lines.push(Line::from(vec![
        Span::styled("  Tool Calls:                 ", label_style),
        Span::styled(format!("{} ( ✓ {}  ✗ {} )", state.tool_calls, state.tool_success, state.tool_failure), value_style),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Success Rate:               ", label_style),
        Span::styled(format!("{:.1}%", success_rate), value_style),
    ]));
    lines.push(Line::default());

    lines.push(Line::from(Span::styled("  Resources", Style::default().add_modifier(Modifier::BOLD))));
    lines.push(Line::from(vec![
        Span::styled("  Total Tokens:               ", label_style),
        Span::styled(format!("{} in  ·  {} out", fmt_tokens(state.tokens_in), fmt_tokens(state.tokens_out)), value_style),
    ]));
    lines.push(Line::default());

    lines.push(Line::from(Span::styled("  Performance", Style::default().add_modifier(Modifier::BOLD))));
    let wall_secs = state.session_start.elapsed().as_secs();
    let wall_time = if wall_secs >= 60 { format!("{}m {}s", wall_secs / 60, wall_secs % 60) } else { format!("{wall_secs}s") };
    
    let active_secs = state.agent_active_ms / 1000;
    let active_time = if active_secs >= 60 { format!("{}m {}s", active_secs / 60, active_secs % 60) } else { format!("{active_secs}s") };
    
    lines.push(Line::from(vec![
        Span::styled("  Wall Time:                  ", label_style),
        Span::styled(wall_time, value_style),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Agent Active:               ", label_style),
        Span::styled(active_time, value_style),
    ]));

    let api_pct = if state.agent_active_ms > 0 { (state.api_time_ms as f32 / state.agent_active_ms as f32) * 100.0 } else { 0.0 };
    let tool_pct = if state.agent_active_ms > 0 { (state.tool_time_ms as f32 / state.agent_active_ms as f32) * 100.0 } else { 0.0 };

    lines.push(Line::from(vec![
        Span::styled("    » API Time:               ", label_style),
        Span::styled(format!("{}s ({:.1}%)", state.api_time_ms / 1000, api_pct), value_style),
    ]));
    lines.push(Line::from(vec![
        Span::styled("    » Tool Time:              ", label_style),
        Span::styled(format!("{}s ({:.1}%)", state.tool_time_ms / 1000, tool_pct), value_style),
    ]));
    
    lines.push(Line::default());
    lines.push(Line::from(vec![
        Span::styled("  To resume this session: ", label_style),
        Span::styled(format!("albert --resume {}", state.session_id), Style::default().fg(GREEN)),
    ]));
    lines.push(Line::default());
    lines.push(Line::from(vec![
        Span::styled("  ( Press any key to exit )", Style::default().fg(DIM).add_modifier(Modifier::ITALIC)),
    ]));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(CHAT_BORDER))
        .style(Style::default().bg(BG));

    let para = Paragraph::new(lines)
        .block(block);

    f.render_widget(para, popup_area);
}

/// Derive a human-readable activity label from the currently running tool (if any).
fn current_activity(state: &TuiState) -> &'static str {
    for block in state.exec_log.iter().rev() {
        if let ExecBlock::ToolUse { name, active, .. } = block {
            if *active {
                let n = name.as_str();
                return if n.contains("read") {
                    "Reading…"
                } else if n.contains("write") || n.contains("edit") {
                    "Writing…"
                } else if n.contains("bash") || n.contains("execute") {
                    "Running…"
                } else if n.contains("grep") || n.contains("search") {
                    "Searching…"
                } else if n.contains("glob") || n.contains("scan") {
                    "Scanning…"
                } else if n.contains("web") || n.contains("fetch") {
                    "Fetching…"
                } else if n.contains("plan") {
                    "Planning…"
                } else {
                    "On it…"
                };
            }
        }
    }
    match (state.session_start.elapsed().as_secs() / 3) % 3 {
        0 => "Thinking…",
        1 => "Pondering…",
        _ => "Pondering…", // can add more here
    }
}

const MIC_RED: Color = Color::Rgb(255, 60, 60);

/// 1-row status strip — ALWAYS visible.
/// Recording: `@ Recording…  ctrl+space to stop`
/// Working:   `* Reading… (2s · ↓ 42 tokens)`
/// Idle:      `◆ Idle  ·  type / for commands`
fn render_status(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let line = if !state.trusted {
        Line::from(vec![
            Span::styled(" ⚠ ", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
            Span::styled("Untrusted Folder", Style::default().fg(ORANGE)),
            Span::styled("  tools will require manual approval", Style::default().fg(DIM)),
        ])
    } else if state.is_recording {
        Line::from(vec![
            Span::styled(" 𒀭 ", Style::default().fg(MIC_RED).add_modifier(Modifier::BOLD)),
            Span::styled("Recording…", Style::default().fg(MIC_RED)),
            Span::styled("  ctrl+space to stop & transcribe", Style::default().fg(GREY)),
        ])
    } else if state.is_prompting.load(Ordering::Relaxed) {
        Line::from(vec![
            Span::styled(" 𒀭 ", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
            Span::styled("Waiting for you…", Style::default().fg(ORANGE)),
            Span::styled("  check main terminal for approval prompt", Style::default().fg(GREY)),
        ])
    } else if state.working {
        let elapsed_ms = state.turn_start.map(|t| t.elapsed().as_millis()).unwrap_or(0);
        let secs = elapsed_ms / 1000;
        let timer = if secs >= 60 {
            format!("{}m {}s", secs / 60, secs % 60)
        } else {
            format!("{secs}s")
        };
        let tok_str = if state.tokens_out > 0 {
            format!(" · ↓ {} tokens", fmt_tokens(state.tokens_out))
        } else {
            String::new()
        };
        let activity = current_activity(state);

        // Pulse the Dingir symbol when active
        let elapsed = state.session_start.elapsed().as_secs_f32();
        let period = 2.0; 
        let t = (elapsed % period) / period;
        let intensity = ((t * std::f32::consts::PI).sin()).powf(2.0);
        let r = (80.0 + (0.0 - 80.0) * intensity) as u8;
        let g = (80.0 + (200.0 - 80.0) * intensity) as u8;
        let b = (80.0 + (255.0 - 80.0) * intensity) as u8;
        let pulse_color = Color::Rgb(r, g, b);

        Line::from(vec![
            Span::styled(" 𒀭 ", Style::default().fg(pulse_color).add_modifier(Modifier::BOLD)),
            Span::styled(activity, Style::default().fg(pulse_color)),
            Span::styled(format!(" ({timer}{tok_str})"), Style::default().fg(GREY)),
        ])
    } else {
        let last_worked = state.exec_log.iter().rev().find_map(|b| {
            if let ExecBlock::WorkedFor(s) = b { Some(*s) } else { None }
        });
        let worked_part = last_worked.map(|secs| {
            let dur = if secs >= 60 {
                format!("{}m {}s", secs / 60, secs % 60)
            } else {
                format!("{secs}s")
            };
            format!("  ·  Worked for {dur}  ")
        }).unwrap_or_else(|| "  ·  ".to_string());

        Line::from(vec![
            Span::styled(" 𒀭 ", Style::default().fg(DIM).add_modifier(Modifier::BOLD)),
            Span::styled("Idle", Style::default().fg(DIM)),
            Span::styled(worked_part, Style::default().fg(DIM)),
            Span::styled("type / for commands", Style::default().fg(DIM)),
        ])
    };
    f.render_widget(Paragraph::new(line).style(Style::default().bg(STATUS_BG)), area);
}

/// 1-row rotating tip strip — sits between the input and footer.
fn render_tips(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let secs = state.session_start.elapsed().as_secs();
    let tip_idx = (secs / 8) as usize % TIPS.len();
    let tip = TIPS[tip_idx];
    let line = Line::from(vec![
        Span::styled("⎿  ", Style::default().fg(DIM)),
        Span::styled("Tip: ", Style::default().fg(DIM)),
        Span::styled(tip, Style::default().fg(GREY)),
    ]);
    f.render_widget(Paragraph::new(line).style(Style::default().bg(BG)), area);
}

/// 1-row footer.
/// When working : `▶▶  esc to interrupt  ·  ctrl+c to quit`
/// When idle    : `▶▶  model  ·  dir  ·  perm  ·  tokens↑ tokens↓`
fn render_footer(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let line = if state.working {
        Line::from(vec![
            Span::styled(" ▶▶ ", Style::default().fg(CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("esc to interrupt", Style::default().fg(CYAN)),
            Span::styled(
                "  ·  ctrl+c to quit",
                Style::default().fg(Color::Rgb(60, 60, 60)),
            ),
        ])
    } else {
        let dir = std::path::Path::new(&state.cwd)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&state.cwd);
        let perm = if state.permission_mode.is_empty() {
            String::new()
        } else {
            format!("  ·  {}", state.permission_mode)
        };
        let base = format!(" {}  ·  {}{}", state.model, dir, perm);
        // Very subtle colors for the sidenote footer
        let base_style = Style::default().fg(Color::Rgb(50, 50, 50));
        let tok_style = Style::default().fg(Color::Rgb(40, 40, 40));

        if state.tokens_in > 0 {
            let tok_str = format!(
                "  ·  {}↑ {}↓",
                fmt_tokens(state.tokens_in),
                fmt_tokens(state.tokens_out),
            );
            Line::from(vec![
                Span::styled(base, base_style),
                Span::styled(tok_str, tok_style),
            ])
        } else {
            Line::from(Span::styled(base, base_style))
        }
    };
    f.render_widget(
        Paragraph::new(line).style(Style::default().bg(BG)),
        area,
    );
}

fn fmt_tokens(n: u32) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

/// Read current git branch without blocking (uses std::process, fire-and-forget cache).
fn git_branch_cached() -> Option<String> {
    use std::sync::OnceLock;
    use std::time::SystemTime;

    static CACHE: OnceLock<std::sync::Mutex<(Option<String>, SystemTime)>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| std::sync::Mutex::new((None, SystemTime::UNIX_EPOCH)));

    let mut guard = cache.lock().ok()?;
    let (ref mut branch, ref mut updated) = *guard;

    let age = SystemTime::now().duration_since(*updated).unwrap_or_default();
    if age.as_secs() > 10 {
        // refresh in background; show stale value in the meantime
        if let Ok(out) = std::process::Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() && s != "HEAD" {
                *branch = Some(s);
            }
        }
        *updated = SystemTime::now();
    }
    branch.clone()
}

// ── Voice transcription ────────────────────────────────────────────────────────

/// Transcription priority:
///   1. local `whisper` CLI  (openai-whisper or whisper.cpp — free, offline)
///   2. OpenAI Whisper API   (requires OPENAI_API_KEY)
///   3. friendly error with install hint
async fn transcribe(wav_path: &str) -> Result<String, String> {
    // ── 1. local whisper CLI ──────────────────────────────────────────────────
    if let Ok(out) = std::process::Command::new("whisper")
        .args([wav_path, "--model", "tiny", "--language", "en",
               "--output_format", "txt", "--output_dir", "/tmp", "--fp16", "False"])
        .output()
    {
        if out.status.success() {
            // whisper writes <filename>.txt next to the input or in output_dir
            let txt_path = "/tmp/albert-voice.txt";
            if let Ok(text) = std::fs::read_to_string(txt_path) {
                let _ = std::fs::remove_file(txt_path);
                let t = text.trim().to_string();
                if !t.is_empty() { return Ok(t); }
            }
            // also try stdout directly
            let stdout = String::from_utf8_lossy(&out.stdout);
            let t = stdout.trim().to_string();
            if !t.is_empty() { return Ok(t); }
        }
    }

    // ── 2. OpenAI Whisper API ─────────────────────────────────────────────────
    if let Ok(key) = std::env::var("OPENAI_API_KEY") {
        if !key.is_empty() {
            let wav = std::fs::read(wav_path).map_err(|e| e.to_string())?;
            return transcribe_openai(wav, &key).await;
        }
    }

    // ── 3. no STT available ───────────────────────────────────────────────────
    Err("voice: no STT available — install whisper:  pip install openai-whisper".to_string())
}

/// POST a WAV buffer to the OpenAI Whisper API.
async fn transcribe_openai(wav: Vec<u8>, api_key: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let part = reqwest::multipart::Part::bytes(wav)
        .file_name("audio.wav")
        .mime_str("audio/wav")
        .map_err(|e| e.to_string())?;
    let form = reqwest::multipart::Form::new()
        .part("file", part)
        .text("model", "whisper-1");
    let resp = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {api_key}"))
        .multipart(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    json.get("text")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("unexpected response: {json}"))
}

// ── TuiApp ────────────────────────────────────────────────────────────────────

pub struct TuiApp {
    pub state: Arc<Mutex<TuiState>>,
    pub event_tx: tokio::sync::mpsc::UnboundedSender<TuiEvent>,
    event_rx: tokio::sync::mpsc::UnboundedReceiver<TuiEvent>,
    submit_tx: std::sync::mpsc::Sender<String>,
    key_paused: Arc<AtomicBool>,
    /// Set by ESC during a running turn — main thread exits the event loop.
    pub cancel_flag: Arc<AtomicBool>,
    /// Live arecord process while voice recording is active.
    voice_process: Arc<std::sync::Mutex<Option<std::process::Child>>>,
}

impl TuiApp {
    pub fn new(model: String, cwd: String, permission_mode: String, session_id: String) -> (Self, std::sync::mpsc::Receiver<String>) {
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        let (submit_tx, submit_rx) = std::sync::mpsc::channel();
        let app = Self {
            state: Arc::new(Mutex::new(TuiState::new(model, cwd, permission_mode, session_id))),
            event_tx,
            event_rx,
            submit_tx,
            key_paused: Arc::new(AtomicBool::new(false)),
            cancel_flag: Arc::new(AtomicBool::new(false)),
            voice_process: Arc::new(std::sync::Mutex::new(None)),
        };
        (app, submit_rx)
    }

    pub fn run(self) {
        if let Err(e) = self.run_inner() {
            eprintln!("tui: {e}");
        }
    }

    fn run_inner(mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen)?;
        io::stdout().execute(EnableBracketedPaste)?;

        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        let cancel_flag = Arc::clone(&self.cancel_flag);

        // Key-event thread — paused during slash command Suspend
        let ktx = self.event_tx.clone();
        let key_paused = Arc::clone(&self.key_paused);
        std::thread::spawn(move || loop {
            if key_paused.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(30));
                continue;
            }
            if event::poll(Duration::from_millis(50)).unwrap_or(false) {
                match event::read() {
                    Ok(Event::Key(k)) => { let _ = ktx.send(TuiEvent::Key(k)); }
                    Ok(Event::Paste(text)) => { let _ = ktx.send(TuiEvent::PasteText(text)); }
                    Ok(Event::Resize(_, _)) => { let _ = ktx.send(TuiEvent::Tick); }
                    _ => {}
                }
            }
        });

        // Tick thread: 100 ms redraws keep the working timer live
        let ttx = self.event_tx.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(Duration::from_millis(100));
            let _ = ttx.send(TuiEvent::Tick);
        });

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.block_on(async {
            // Rate-limit rendering to ~30fps (33ms between draws).
            // Without this, rapid streaming events cause 1000+ draws/sec which the
            // terminal coalesces into a single visible frame — giving "all at once" appearance.
            let mut last_draw = Instant::now();
            const DRAW_INTERVAL: Duration = Duration::from_millis(33);

            loop {
                // Draw if enough time has passed since the last frame.
                if last_draw.elapsed() >= DRAW_INTERVAL {
                    {
                        let state = self.state.lock().unwrap();
                        terminal.draw(|f| render(f, &state))?;
                    }
                    last_draw = Instant::now();
                }

                // Wait for the next event, but with a deadline so we always redraw
                // at ~30fps even when no events arrive (keeps spinner/timer live).
                let wait = DRAW_INTERVAL.saturating_sub(last_draw.elapsed());
                let ev = match tokio::time::timeout(wait, self.event_rx.recv()).await {
                    Ok(ev) => ev,
                    Err(_) => continue, // timeout — loop back to draw
                };
                match ev {
                    // ── keyboard ──────────────────────────────────────────────
                    Some(TuiEvent::Key(key)) => {
                        let mut quit_event: Option<TuiEvent> = None;
                        let mut submit_text: Option<String> = None;
                        // voice_toggle: true=start recording, false=stop recording, None=no change
                        let mut voice_toggle: Option<bool> = None;
                        {
                            let mut state = self.state.lock().unwrap();
                            let items = popup_items(&state.input);
                            let has_popup = !items.is_empty();

                            match (key.code, key.modifiers) {
                                (KeyCode::Char('c'), KeyModifiers::CONTROL)
                                | (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                                    if state.quit_confirm {
                                        quit_event = Some(TuiEvent::QuitWithReport);
                                    } else {
                                        state.quit_confirm = true;
                                        state.push_exec(ExecBlock::SystemMsg("Press Ctrl+C again to exit session".to_string()));
                                    }
                                }

                                // Ctrl+Space — toggle voice recording
                                (KeyCode::Char(' '), KeyModifiers::CONTROL) => {
                                    state.is_recording = !state.is_recording;
                                    voice_toggle = Some(state.is_recording);
                                }

                                // Ctrl+V — paste from system clipboard
                                (KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                                    if let Ok(mut board) = arboard::Clipboard::new() {
                                        if let Ok(text) = board.get_text() {
                                            for ch in text.chars() {
                                                if ch == '\n' || ch == '\r' {
                                                    state.input_insert(' ');
                                                } else {
                                                    state.input_insert(ch);
                                                }
                                            }
                                        }
                                    }
                                }

                                // ESC: close help overlay → dismiss popup → clear paste → reset scroll
                                (KeyCode::Esc, _) => {
                                    if state.working {
                                        cancel_flag.store(true, Ordering::Relaxed);
                                    } else if state.help_open {
                                        state.help_open = false;
                                        state.help_scroll = 0;
                                    } else if state.paste_line_count.is_some() {
                                        state.input.clear();
                                        state.cursor = 0;
                                        state.paste_line_count = None;
                                    } else if has_popup {
                                        state.input.clear();
                                        state.cursor = 0;
                                        state.popup_selected = 0;
                                    } else {
                                        state.scroll = 0;
                                    }
                                }
                                // PageUp/Down in help popup
                                (KeyCode::Up, _) | (KeyCode::PageUp, _) if state.help_open => {
                                    state.help_scroll = state.help_scroll.saturating_sub(3);
                                }
                                (KeyCode::Down, _) | (KeyCode::PageDown, _) if state.help_open => {
                                    state.help_scroll = state.help_scroll.saturating_add(3);
                                }

                                // Up / Down / Left / Right all navigate the popup when open.
                                // Header rows are skipped automatically.
                                (KeyCode::Up, KeyModifiers::NONE)
                                | (KeyCode::Left, KeyModifiers::NONE)
                                    if has_popup =>
                                {
                                    let mut idx = state.popup_selected.saturating_sub(1);
                                    while idx > 0 && items.get(idx).map(|i| i.is_header).unwrap_or(false) {
                                        idx = idx.saturating_sub(1);
                                    }
                                    if !items.get(idx).map(|i| i.is_header).unwrap_or(true) {
                                        state.popup_selected = idx;
                                    }
                                }
                                (KeyCode::Down, KeyModifiers::NONE)
                                | (KeyCode::Right, KeyModifiers::NONE)
                                    if has_popup =>
                                {
                                    let max = items.len().saturating_sub(1);
                                    let mut idx = (state.popup_selected + 1).min(max);
                                    while idx < max && items.get(idx).map(|i| i.is_header).unwrap_or(false) {
                                        idx = (idx + 1).min(max);
                                    }
                                    if !items.get(idx).map(|i| i.is_header).unwrap_or(true) {
                                        state.popup_selected = idx;
                                    }
                                }

                                // Up/Down — history navigation (bash-style)
                                (KeyCode::Up, KeyModifiers::NONE) => {
                                    state.history_prev();
                                }
                                (KeyCode::Down, KeyModifiers::NONE) => {
                                    state.history_next();
                                }

                                // PageUp/PageDown or Shift+Up/Down — scroll content
                                (KeyCode::PageUp, _) | (KeyCode::Up, KeyModifiers::SHIFT) => {
                                    state.scroll = state.scroll.saturating_add(10);
                                }
                                (KeyCode::PageDown, _) | (KeyCode::Down, KeyModifiers::SHIFT) => {
                                    state.scroll = state.scroll.saturating_sub(10);
                                }

                                // Tab: complete into the input (never submits).
                                // For drill-down parents: opens sub-menu.
                                // For leaf commands: fills full command + space ready to run.
                                (KeyCode::Tab, _) if has_popup => {
                                    let sel = state.popup_selected.min(items.len().saturating_sub(1));
                                    if !items[sel].is_header {
                                        let complete = items[sel].complete.clone();
                                        // Always append space — this opens the sub-menu for parents
                                        // and puts a space after leaf commands for argument entry.
                                        let already_has_space = complete.ends_with(' ');
                                        state.input = if already_has_space {
                                            complete
                                        } else {
                                            format!("{complete} ")
                                        };
                                        state.cursor = state.input.chars().count();
                                        let new_items = popup_items(&state.input);
                                        state.popup_selected = new_items.iter()
                                            .position(|i| !i.is_header)
                                            .unwrap_or(0);
                                    }
                                }

                                // Enter with popup:
                                //   drill-down items  (model / permissions / auth) → navigate into sub-menu
                                //   leaf items        → execute immediately
                                (KeyCode::Enter, KeyModifiers::NONE) if has_popup => {
                                    let sel = state.popup_selected.min(items.len().saturating_sub(1));
                                    if !items[sel].is_header {
                                        let complete = items[sel].complete.clone();
                                        if is_drilldown(&complete) {
                                            // Open sub-menu: append space so popup_items sees the prefix
                                            state.input = format!("{complete} ");
                                            state.cursor = state.input.chars().count();
                                            let new_items = popup_items(&state.input);
                                            state.popup_selected = new_items.iter()
                                                .position(|i| !i.is_header)
                                                .unwrap_or(0);
                                        } else {
                                            // Leaf: execute
                                            state.input = complete;
                                            state.cursor = state.input.chars().count();
                                            state.popup_selected = 0;
                                            let text = state.input_take();
                                            submit_text = Some(text);
                                        }
                                    }
                                }
                                (KeyCode::Enter, KeyModifiers::NONE) => {
                                    let text = state.input_take();
                                    if !text.trim().is_empty() {
                                        submit_text = Some(text);
                                    }
                                }

                                (KeyCode::Char(c), m)
                                    if m == KeyModifiers::NONE || m == KeyModifiers::SHIFT =>
                                {
                                    state.quit_confirm = false;
                                    state.history_idx = None; // exit history browse on new input
                                    state.input_insert(c);
                                    // Reset to first selectable item (skip any header at 0)
                                    let new_items = popup_items(&state.input);
                                    state.popup_selected = new_items.iter()
                                        .position(|i| !i.is_header)
                                        .unwrap_or(0);
                                }
                                (KeyCode::Backspace, _) => {
                                    state.quit_confirm = false;
                                    state.input_backspace();
                                    let new_items = popup_items(&state.input);
                                    state.popup_selected = new_items.iter()
                                        .position(|i| !i.is_header)
                                        .unwrap_or(0);
                                }
                                (KeyCode::Delete, _) => {
                                    state.quit_confirm = false;
                                    state.input_delete();
                                }

                                // ── Readline shortcuts ────────────────────────
                                // Ctrl+A: jump to start of line
                                (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                                    state.quit_confirm = false;
                                    state.cursor = 0;
                                }
                                // Ctrl+E: jump to end of line
                                (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                                    state.quit_confirm = false;
                                    state.cursor = state.input.chars().count();
                                }
                                // Ctrl+K: kill to end of line
                                (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                                    state.quit_confirm = false;
                                    let pos = state.input.char_indices()
                                        .nth(state.cursor)
                                        .map(|(i, _)| i)
                                        .unwrap_or(state.input.len());
                                    state.input.truncate(pos);
                                }
                                // Ctrl+U: kill to start of line
                                (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                                    state.quit_confirm = false;
                                    let pos = state.input.char_indices()
                                        .nth(state.cursor)
                                        .map(|(i, _)| i)
                                        .unwrap_or(state.input.len());
                                    state.input.drain(..pos);
                                    state.cursor = 0;
                                }
                                // Ctrl+W: kill previous word
                                (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                                    state.quit_confirm = false;
                                    let new_cur = word_left(&state.input, state.cursor);
                                    let start = state.input.char_indices()
                                        .nth(new_cur).map(|(i, _)| i).unwrap_or(0);
                                    let end = state.input.char_indices()
                                        .nth(state.cursor).map(|(i, _)| i)
                                        .unwrap_or(state.input.len());
                                    state.input.drain(start..end);
                                    state.cursor = new_cur;
                                }
                                // Ctrl+L: scroll to bottom (show latest)
                                (KeyCode::Char('l'), KeyModifiers::CONTROL) => {
                                    state.quit_confirm = false;
                                    state.scroll = 0;
                                }
                                // Ctrl+Left: word left
                                (KeyCode::Left, KeyModifiers::CONTROL) => {
                                    state.quit_confirm = false;
                                    state.cursor = word_left(&state.input, state.cursor);
                                }
                                // Ctrl+Right: word right
                                (KeyCode::Right, KeyModifiers::CONTROL) => {
                                    state.quit_confirm = false;
                                    state.cursor = word_right(&state.input, state.cursor);
                                }

                                // ── Cursor movement ───────────────────────────
                                (KeyCode::Left, _) => {
                                    state.quit_confirm = false;
                                    if state.cursor > 0 { state.cursor -= 1; }
                                }
                                (KeyCode::Right, _) => {
                                    state.quit_confirm = false;
                                    if state.cursor < state.input.chars().count() {
                                        state.cursor += 1;
                                    }
                                }
                                (KeyCode::Home, _) => {
                                    state.quit_confirm = false;
                                    state.cursor = 0;
                                }
                                (KeyCode::End, _) => {
                                    state.quit_confirm = false;
                                    state.cursor = state.input.chars().count();
                                }
                                _ => {}
                            }
                        }
                        if let Some(ev) = quit_event {
                            let _ = self.event_tx.send(ev);
                        }
                        if let Some(text) = submit_text {
                            let trimmed = text.trim();
                            if trimmed == "/help" || trimmed == "/?" {
                                self.state.lock().unwrap().help_open = true;
                            } else {
                                self.state.lock().unwrap().history_push(&text);
                                let _ = self.submit_tx.send(text);
                            }
                        }
                        // Handle voice recording toggle outside the state lock
                        match voice_toggle {
                            Some(true) => {
                                // Start recording — try arecord (Linux ALSA)
                                let _ = std::fs::remove_file("/tmp/albert-voice.wav");
                                match std::process::Command::new("arecord")
                                    .args(["-q", "-r", "16000", "-c", "1", "-f", "S16_LE",
                                           "/tmp/albert-voice.wav"])
                                    .spawn()
                                {
                                    Ok(child) => {
                                        *self.voice_process.lock().unwrap() = Some(child);
                                    }
                                    Err(_) => {
                                        // arecord not available
                                        let _ = self.event_tx.send(TuiEvent::VoiceError(
                                            "voice: arecord not found (install alsa-utils)".to_string(),
                                        ));
                                        self.state.lock().unwrap().is_recording = false;
                                    }
                                }
                            }
                            Some(false) => {
                                // Stop recording and transcribe
                                if let Some(mut child) = self.voice_process.lock().unwrap().take() {
                                    let _ = child.kill();
                                    let _ = child.wait();
                                }
                                let tx = self.event_tx.clone();
                                tokio::spawn(async move {
                                    const WAV: &str = "/tmp/albert-voice.wav";
                                    let size = std::fs::metadata(WAV).map(|m| m.len()).unwrap_or(0);
                                    if size > 44 {
                                        match transcribe(WAV).await {
                                            Ok(text) => { let _ = tx.send(TuiEvent::VoiceText(text)); }
                                            Err(e)   => { let _ = tx.send(TuiEvent::VoiceError(e)); }
                                        }
                                    } else {
                                        let _ = tx.send(TuiEvent::VoiceError(
                                            "voice: no audio captured".to_string(),
                                        ));
                                    }
                                });
                            }
                            None => {}
                        }
                    }

                    // ── agent events ──────────────────────────────────────────
                    Some(TuiEvent::AgentEvent(ev)) => {
                        let mut state = self.state.lock().unwrap();
                        match ev {
                            AssistantEvent::TextDelta(delta) => {
                                // Append directly to the AgentText block — renders on the next
                                // draw call, giving real-time streaming appearance.
                                match state.exec_log.back_mut() {
                                    Some(ExecBlock::AgentText(ref mut s)) => s.push_str(&delta),
                                    _ => state.exec_log.push_back(ExecBlock::AgentText(delta)),
                                }
                            }
                            AssistantEvent::ToolUse { name, input, .. } => {
                                let preview = tool_input_preview(&input);
                                state.push_exec(ExecBlock::ToolUse {
                                    name,
                                    args: preview,
                                    active: true,
                                });
                            }
                            AssistantEvent::Usage(usage) => {
                                state.tokens_in = state.tokens_in.max(usage.input_tokens);
                                state.tokens_out += usage.output_tokens;
                            }
                            AssistantEvent::MessageStop => {
                                // Do NOT deactivate the tool dot here — the tool may still be
                                // executing. The main thread deactivates after run_turn returns
                                // and ToolOutput blocks have been injected.
                            }
                        }
                    }

                    // ── terminal handoff for slash commands ───────────────────
                    Some(TuiEvent::Suspend { ack }) => {
                        self.key_paused.store(true, Ordering::Relaxed);
                        io::stdout().execute(DisableMouseCapture).ok();
                        io::stdout().execute(DisableBracketedPaste).ok();
                        disable_raw_mode().ok();
                        io::stdout().execute(LeaveAlternateScreen).ok();
                        io::stdout().flush().ok();
                        let _ = ack.send(());
                        loop {
                            match self.event_rx.recv().await {
                                Some(TuiEvent::Resume) => break,
                                Some(TuiEvent::Quit) | Some(TuiEvent::QuitWithReport) | None => return Ok(()),
                                _ => {}
                            }
                        }
                        enable_raw_mode().ok();
                        io::stdout().execute(EnableBracketedPaste).ok();
                        io::stdout().execute(EnterAlternateScreen).ok();
                        terminal.clear().ok();
                        self.key_paused.store(false, Ordering::Relaxed);
                    }

                    // ── voice transcription result ────────────────────────────
                    Some(TuiEvent::VoiceText(text)) => {
                        let mut state = self.state.lock().unwrap();
                        for ch in text.trim().chars() {
                            state.input_insert(ch);
                        }
                    }
                    Some(TuiEvent::VoiceError(msg)) => {
                        let mut state = self.state.lock().unwrap();
                        state.push_exec(ExecBlock::SystemMsg(msg));
                    }

                    // ── bracketed paste ───────────────────────────────────────
                    Some(TuiEvent::PasteText(text)) => {
                        let mut state = self.state.lock().unwrap();
                        let line_count = text.lines().count();
                        if line_count >= 3 || text.chars().count() > 200 {
                            // Large paste: store raw, show compact badge in render_input.
                            state.input = text;
                            state.cursor = 0; // Keep cursor at 0 so it stays on the badge
                            state.paste_line_count = Some(line_count);
                        } else {
                            // Small paste: insert inline, convert newlines to spaces.
                            state.paste_line_count = None;
                            for ch in text.chars() {
                                if ch == '\n' || ch == '\r' {
                                    state.input_insert(' ');
                                } else {
                                    state.input_insert(ch);
                                }
                            }
                        }
                    }

                    Some(TuiEvent::Tick) | Some(TuiEvent::Resume) => {
                        // Tick fires at 100ms — redraws the screen (spinner, timer).
                        // Text is appended directly on each TextDelta, so no work needed here.
                    }
                    Some(TuiEvent::Quit) => {
                        break;
                    }

                    Some(TuiEvent::QuitWithReport) => {
                        // Show report card and wait for any keypress before exiting
                        {
                            let state = self.state.lock().unwrap();
                            terminal.draw(|f| render_report_card(f, &state))?;
                        }
                        // Drain any pending keys then wait for a fresh one
                        while self.event_rx.try_recv().is_ok() {}
                        loop {
                            match self.event_rx.recv().await {
                                Some(TuiEvent::Key(_)) | Some(TuiEvent::Quit) | None => break,
                                _ => {}
                            }
                        }
                        break;
                    }
                    None => break,
                }
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        })?;

        io::stdout().execute(DisableMouseCapture).ok();
        io::stdout().execute(DisableBracketedPaste).ok();
        disable_raw_mode().ok();
        io::stdout().execute(LeaveAlternateScreen).ok();
        Ok(())
    }
}
