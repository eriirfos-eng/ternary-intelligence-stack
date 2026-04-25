use std::collections::VecDeque;
use std::io::{self, Write as _};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::Terminal;

use commands::slash_command_specs;
use runtime::AssistantEvent;

// ── Colors ────────────────────────────────────────────────────────────────────

const BG: Color = Color::Rgb(11, 11, 11);
const FG: Color = Color::Rgb(220, 220, 220);
const DIM: Color = Color::Rgb(80, 80, 80);       // separators / borders only
const GREY: Color = Color::Rgb(145, 145, 145);   // completed tool text — visible
const GREEN: Color = Color::Rgb(0, 220, 120);
const CYAN: Color = Color::Rgb(0, 200, 255);
const USER_BOX_BG: Color = Color::Rgb(28, 28, 28);
const STATUS_BG: Color = Color::Rgb(18, 18, 18);
const POPUP_BG: Color = Color::Rgb(26, 26, 26);
const POPUP_BORDER: Color = Color::Rgb(55, 55, 55);
const POPUP_MATCH: Color = Color::Rgb(0, 180, 100);
const POPUP_SEL_BG: Color = Color::Rgb(42, 42, 42);

// Permission modes (in display/cycle order)
const PERM_MODES: &[(&str, &str)] = &[
    ("read-only",           "no writes · no shell"),
    ("workspace-write",     "files only · no shell"),
    ("danger-full-access",  "unrestricted · full shell"),
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
    pub session_start: Instant,
    pub turn_start: Option<Instant>,
    pub working: bool,
    /// Rows scrolled up from the bottom (0 = follow latest)
    pub scroll: u16,
    /// Selected index in the active popup
    pub popup_selected: usize,
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
            session_start: Instant::now(),
            turn_start: None,
            working: false,
            scroll: 0,
            popup_selected: 0,
        }
    }
}

impl TuiState {
    pub fn new(model: String, cwd: String) -> Self {
        Self { model, cwd, ..Default::default() }
    }

    pub fn push_exec(&mut self, block: ExecBlock) {
        if matches!(&block, ExecBlock::ToolUse { .. }) {
            self.deactivate_last_tool();
        }
        self.exec_log.push_back(block);
        while self.exec_log.len() > 1000 {
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
        std::mem::take(&mut self.input)
    }
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
}

// ── Popup items ───────────────────────────────────────────────────────────────

#[derive(Clone)]
struct PopupItem {
    /// What the popup row displays (without leading /)
    display: String,
    /// What to write into state.input when selected
    complete: String,
    /// Right-hand description
    desc: String,
}

/// Returns popup items for the current input:
///   /           → all slash commands
///   /partial    → matching slash commands
///   /permissions, /permissions → mode picker
fn popup_items(input: &str) -> Vec<PopupItem> {
    if !input.starts_with('/') {
        return vec![];
    }

    // Permission mode picker: `/permissions` (bare) or `/permissions <partial>`
    let perm_prefix = "/permissions";
    if input == perm_prefix
        || input.starts_with("/permissions ")
        || perm_prefix.starts_with(input.trim_end_matches(' '))
            && input.len() > 1
            && perm_prefix.starts_with(input)
    {
        let partial = if input.starts_with("/permissions ") {
            input["/permissions ".len()..].trim()
        } else {
            ""
        };
        return PERM_MODES
            .iter()
            .filter(|(mode, _)| mode.starts_with(partial))
            .map(|(mode, desc)| PopupItem {
                display: format!("permissions {mode}"),
                complete: format!("/permissions {mode}"),
                desc: desc.to_string(),
            })
            .collect();
    }

    // Regular slash command matching
    let prefix = &input[1..];
    slash_command_specs()
        .iter()
        .filter(|s| s.name.starts_with(prefix))
        .take(8)
        .map(|s| {
            let hint = s.argument_hint.map(|h| format!(" {h}")).unwrap_or_default();
            PopupItem {
                display: format!("{}{hint}", s.name),
                complete: format!("/{}{hint}", s.name),
                desc: s.summary.to_string(),
            }
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

// ── Rendering ─────────────────────────────────────────────────────────────────

pub fn render(f: &mut ratatui::Frame, state: &TuiState) {
    let area = f.area();
    let items = popup_items(&state.input);
    let popup_h = if items.is_empty() { 0u16 } else { items.len() as u16 + 1 };
    let working_h = if state.working { 1u16 } else { 0u16 };

    // Layout (bottom→top): status(2) | input(2) | [working?] | [popup?] | content(rest)
    let mut constraints = vec![Constraint::Min(3)];
    if popup_h > 0 { constraints.push(Constraint::Length(popup_h)); }
    if working_h > 0 { constraints.push(Constraint::Length(working_h)); }
    constraints.push(Constraint::Length(2)); // input
    constraints.push(Constraint::Length(2)); // status

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    let mut idx = 0usize;
    render_content(f, layout[idx], state);
    idx += 1;
    if popup_h > 0 {
        render_popup(f, layout[idx], &items, state.popup_selected.min(items.len().saturating_sub(1)));
        idx += 1;
    }
    if working_h > 0 {
        render_working(f, layout[idx], state);
        idx += 1;
    }
    render_input(f, layout[idx], state);
    idx += 1;
    render_status(f, layout[idx], state);
}

fn build_exec_lines(state: &TuiState, _width: u16) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    for block in &state.exec_log {
        match block {
            ExecBlock::UserMessage(msg) => {
                lines.push(Line::default());
                lines.push(Line::from(vec![
                    Span::styled(
                        " > ",
                        Style::default().fg(CYAN).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(msg.clone(), Style::default().fg(FG).bg(USER_BOX_BG)),
                ]));
            }

            ExecBlock::ToolUse { name, args, active } => {
                let (dot_col, name_col, args_col) = if *active {
                    (GREEN, FG, CYAN)
                } else {
                    (GREY, GREY, GREY)
                };
                let mut spans = vec![
                    Span::styled(
                        "● ",
                        Style::default().fg(dot_col).add_modifier(Modifier::BOLD),
                    ),
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
                    let connector = if i == 0 { "⎿  " } else { "   " };
                    lines.push(Line::from(vec![
                        Span::styled(connector, Style::default().fg(DIM)),
                        Span::styled(line.clone(), Style::default().fg(GREY)),
                    ]));
                }
                if *total > out.len() {
                    lines.push(Line::from(vec![
                        Span::styled("   ", Style::default()),
                        Span::styled(
                            format!("… +{} lines", total - out.len()),
                            Style::default().fg(DIM).add_modifier(Modifier::ITALIC),
                        ),
                    ]));
                }
            }

            ExecBlock::AgentText(text) => {
                lines.push(Line::default());
                for line in text.lines() {
                    lines.push(Line::from(Span::styled(
                        line.to_string(),
                        Style::default().fg(FG),
                    )));
                }
            }

            ExecBlock::SystemMsg(msg) => {
                lines.push(Line::default());
                for line in msg.lines() {
                    lines.push(Line::from(vec![
                        Span::styled("* ", Style::default().fg(DIM)),
                        Span::styled(line.to_string(), Style::default().fg(GREY)),
                    ]));
                }
            }
        }
    }

    lines
}

fn render_content(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let lines = build_exec_lines(state, area.width);
    let total = lines.len() as u16;
    let visible = area.height;
    let raw_offset = total.saturating_sub(visible);
    let scroll = raw_offset.saturating_sub(state.scroll);

    let para = Paragraph::new(Text::from(lines))
        .style(Style::default().bg(BG).fg(FG))
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    f.render_widget(para, area);
}

fn render_popup(
    f: &mut ratatui::Frame,
    area: Rect,
    items: &[PopupItem],
    selected: usize,
) {
    let mut lines: Vec<Line<'static>> = Vec::new();
    // thin top border
    lines.push(Line::from(Span::styled(
        "─".repeat(area.width as usize),
        Style::default().fg(POPUP_BORDER),
    )));
    for (i, item) in items.iter().enumerate() {
        let is_sel = i == selected;
        let bg = if is_sel { POPUP_SEL_BG } else { POPUP_BG };
        let name_col = if is_sel { GREEN } else { POPUP_MATCH };
        let desc_col = if is_sel { FG } else { GREY };
        lines.push(Line::from(vec![
            Span::styled("  ", Style::default().bg(bg)),
            Span::styled(
                format!("/{}", item.display),
                Style::default().fg(name_col).bg(bg).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  ", Style::default().bg(bg)),
            Span::styled(item.desc.clone(), Style::default().fg(desc_col).bg(bg)),
        ]));
    }
    let para = Paragraph::new(Text::from(lines)).style(Style::default().bg(POPUP_BG));
    f.render_widget(para, area);
}

fn render_input(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let before: String = state.input.chars().take(state.cursor).collect();
    let cursor_ch: String = state
        .input
        .chars()
        .nth(state.cursor)
        .map(|c| c.to_string())
        .unwrap_or_else(|| " ".to_string());
    let after: String = state.input.chars().skip(state.cursor + 1).collect();

    let line = Line::from(vec![
        Span::styled(" > ", Style::default().fg(CYAN).add_modifier(Modifier::BOLD)),
        Span::styled(before, Style::default().fg(FG)),
        Span::styled(cursor_ch, Style::default().fg(BG).bg(FG)),
        Span::styled(after, Style::default().fg(FG)),
    ]);

    let para = Paragraph::new(line)
        .block(Block::default().style(Style::default().bg(USER_BOX_BG)))
        .wrap(Wrap { trim: false });
    f.render_widget(para, area);
}

/// Dedicated 1-row working indicator between content and input.
fn render_working(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let secs = state.turn_start.map(|t| t.elapsed().as_secs()).unwrap_or(0);
    let timer = if secs >= 60 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{secs}s")
    };
    let tok = if state.tokens_out > 0 {
        format!(" · ↓ {} tokens", state.tokens_out)
    } else {
        String::new()
    };
    let line = Line::from(vec![
        Span::styled(" ● ", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
        Span::styled(
            format!("Working ({timer}{tok} · esc to interrupt)"),
            Style::default().fg(FG),
        ),
    ]);
    let para = Paragraph::new(line).style(Style::default().bg(STATUS_BG));
    f.render_widget(para, area);
}

/// 2-row telemetry bar:
///   row 1: model · branch · tokens_in↑ tokens_out↓ · session_time
///   row 2: albert vX.X.X · cwd · permission · turns
fn render_status(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    // git branch (best-effort, non-blocking)
    let branch = git_branch_cached();

    let elapsed = state.session_start.elapsed();
    let h = elapsed.as_secs() / 3600;
    let m = (elapsed.as_secs() % 3600) / 60;
    let s = elapsed.as_secs() % 60;
    let time_str = if h > 0 {
        format!("{h}h {m}m")
    } else if m > 0 {
        format!("{m}m {s}s")
    } else {
        format!("{s}s")
    };

    let tokens_in = fmt_tokens(state.tokens_in);
    let tokens_out = fmt_tokens(state.tokens_out);
    let branch_str = branch.map(|b| format!(" · {b}")).unwrap_or_default();

    let row1 = format!(
        " {}{}  ·  {}↑ {}↓  ·  {}",
        state.model, branch_str, tokens_in, tokens_out, time_str
    );

    let dir = std::path::Path::new(&state.cwd)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&state.cwd);
    let row2 = format!(
        " albert v{}  ·  {}  ·  session {}",
        env!("CARGO_PKG_VERSION"),
        dir,
        time_str,
    );

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(area);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(row1, Style::default().fg(FG))))
            .style(Style::default().bg(STATUS_BG)),
        layout[0],
    );
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(row2, Style::default().fg(GREY))))
            .style(Style::default().bg(STATUS_BG)),
        layout[1],
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

// ── TuiApp ────────────────────────────────────────────────────────────────────

pub struct TuiApp {
    pub state: Arc<Mutex<TuiState>>,
    pub event_tx: tokio::sync::mpsc::UnboundedSender<TuiEvent>,
    event_rx: tokio::sync::mpsc::UnboundedReceiver<TuiEvent>,
    submit_tx: std::sync::mpsc::Sender<String>,
    key_paused: Arc<AtomicBool>,
    /// Set by ESC during a running turn — main thread exits the event loop.
    pub cancel_flag: Arc<AtomicBool>,
}

impl TuiApp {
    pub fn new(model: String, cwd: String) -> (Self, std::sync::mpsc::Receiver<String>) {
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        let (submit_tx, submit_rx) = std::sync::mpsc::channel();
        let app = Self {
            state: Arc::new(Mutex::new(TuiState::new(model, cwd))),
            event_tx,
            event_rx,
            submit_tx,
            key_paused: Arc::new(AtomicBool::new(false)),
            cancel_flag: Arc::new(AtomicBool::new(false)),
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
                if let Ok(Event::Key(k)) = event::read() {
                    let _ = ktx.send(TuiEvent::Key(k));
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
            loop {
                {
                    let state = self.state.lock().unwrap();
                    terminal.draw(|f| render(f, &state))?;
                }

                match self.event_rx.recv().await {
                    // ── keyboard ──────────────────────────────────────────────
                    Some(TuiEvent::Key(key)) => {
                        let mut do_quit = false;
                        let mut submit_text: Option<String> = None;
                        {
                            let mut state = self.state.lock().unwrap();
                            let items = popup_items(&state.input);
                            let has_popup = !items.is_empty();

                            match (key.code, key.modifiers) {
                                (KeyCode::Char('c'), KeyModifiers::CONTROL)
                                | (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                                    do_quit = true;
                                }

                                // ESC: interrupt running turn, dismiss popup, or reset scroll
                                (KeyCode::Esc, _) => {
                                    if state.working {
                                        cancel_flag.store(true, Ordering::Relaxed);
                                    } else if has_popup {
                                        state.input.clear();
                                        state.cursor = 0;
                                        state.popup_selected = 0;
                                    } else {
                                        state.scroll = 0;
                                    }
                                }

                                // Up / Down / Left / Right all navigate the popup when open
                                (KeyCode::Up, KeyModifiers::NONE)
                                | (KeyCode::Left, KeyModifiers::NONE)
                                    if has_popup =>
                                {
                                    state.popup_selected =
                                        state.popup_selected.saturating_sub(1);
                                }
                                (KeyCode::Down, KeyModifiers::NONE)
                                | (KeyCode::Right, KeyModifiers::NONE)
                                    if has_popup =>
                                {
                                    let max = items.len().saturating_sub(1);
                                    state.popup_selected =
                                        (state.popup_selected + 1).min(max);
                                }

                                // Scroll when no popup
                                (KeyCode::Up, _) | (KeyCode::PageUp, _) => {
                                    let step = if key.code == KeyCode::PageUp { 10 } else { 3 };
                                    state.scroll = state.scroll.saturating_add(step);
                                }
                                (KeyCode::Down, _) | (KeyCode::PageDown, _) => {
                                    let step = if key.code == KeyCode::PageDown { 10 } else { 3 };
                                    state.scroll = state.scroll.saturating_sub(step);
                                }

                                // Tab / Right-at-EOL: autocomplete selected popup item
                                (KeyCode::Tab, _)
                                    if has_popup =>
                                {
                                    let sel = state.popup_selected.min(items.len().saturating_sub(1));
                                    let complete = items[sel].complete.clone();
                                    // For permission mode: no trailing space, just complete + space
                                    let with_space = if complete.starts_with("/permissions ") {
                                        complete
                                    } else {
                                        format!("{complete} ")
                                    };
                                    state.input = with_space;
                                    state.cursor = state.input.chars().count();
                                    state.popup_selected = 0;
                                }

                                // Enter with popup: for `/permissions X` auto-submit, else just complete
                                (KeyCode::Enter, KeyModifiers::NONE) if has_popup => {
                                    let sel = state.popup_selected.min(items.len().saturating_sub(1));
                                    let complete = items[sel].complete.clone();
                                    // Permission mode: auto-submit immediately
                                    if complete.starts_with("/permissions ") {
                                        state.input = complete;
                                        state.cursor = state.input.chars().count();
                                        state.popup_selected = 0;
                                        let text = state.input_take();
                                        submit_text = Some(text);
                                    } else {
                                        // Regular command: put in input box, let user see it
                                        state.input = format!("{complete} ");
                                        state.cursor = state.input.chars().count();
                                        state.popup_selected = 0;
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
                                    state.input_insert(c);
                                    state.popup_selected = 0;
                                }
                                (KeyCode::Backspace, _) => {
                                    state.input_backspace();
                                    state.popup_selected = 0;
                                }
                                (KeyCode::Delete, _) => state.input_delete(),
                                (KeyCode::Left, _) => {
                                    if state.cursor > 0 {
                                        state.cursor -= 1;
                                    }
                                }
                                (KeyCode::Right, _) => {
                                    if state.cursor < state.input.chars().count() {
                                        state.cursor += 1;
                                    }
                                }
                                (KeyCode::Home, _) => state.cursor = 0,
                                (KeyCode::End, _) => {
                                    state.cursor = state.input.chars().count();
                                }
                                _ => {}
                            }
                        }
                        if do_quit {
                            break;
                        }
                        if let Some(text) = submit_text {
                            let _ = self.submit_tx.send(text);
                        }
                    }

                    // ── agent events ──────────────────────────────────────────
                    Some(TuiEvent::AgentEvent(ev)) => {
                        let mut state = self.state.lock().unwrap();
                        match ev {
                            AssistantEvent::TextDelta(delta) => {
                                match state.exec_log.back_mut() {
                                    Some(ExecBlock::AgentText(ref mut s)) => {
                                        s.push_str(&delta);
                                    }
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
                                // Mark last tool done; run_turn may issue more.
                                // Main thread clears `working` after the full turn.
                                state.deactivate_last_tool();
                            }
                        }
                    }

                    // ── terminal handoff for slash commands ───────────────────
                    Some(TuiEvent::Suspend { ack }) => {
                        self.key_paused.store(true, Ordering::Relaxed);
                        disable_raw_mode().ok();
                        io::stdout().execute(LeaveAlternateScreen).ok();
                        io::stdout().flush().ok();
                        let _ = ack.send(());
                        loop {
                            match self.event_rx.recv().await {
                                Some(TuiEvent::Resume) => break,
                                Some(TuiEvent::Quit) | None => return Ok(()),
                                _ => {}
                            }
                        }
                        enable_raw_mode().ok();
                        io::stdout().execute(EnterAlternateScreen).ok();
                        terminal.clear().ok();
                        self.key_paused.store(false, Ordering::Relaxed);
                    }

                    Some(TuiEvent::Tick) | Some(TuiEvent::Resume) => {}

                    Some(TuiEvent::Quit) | None => break,
                }
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        })?;

        disable_raw_mode().ok();
        io::stdout().execute(LeaveAlternateScreen).ok();
        Ok(())
    }
}
