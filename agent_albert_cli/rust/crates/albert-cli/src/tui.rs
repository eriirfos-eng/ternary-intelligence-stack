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
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Terminal;

use pulldown_cmark::{
    Event as MdEvent, HeadingLevel, Options as MdOptions, Parser as MdParser, Tag, TagEnd,
};

use commands::slash_command_specs;
use runtime::AssistantEvent;

// ── Colors ────────────────────────────────────────────────────────────────────

const BG: Color = Color::Rgb(11, 11, 11);
const FG: Color = Color::Rgb(220, 220, 220);
const DIM: Color = Color::Rgb(80, 80, 80);
const GREY: Color = Color::Rgb(145, 145, 145);
const GREEN: Color = Color::Rgb(0, 220, 120);
const CYAN: Color = Color::Rgb(0, 200, 255);
const ORANGE: Color = Color::Rgb(255, 140, 50);   // working `*` indicator
const USER_BOX_BG: Color = Color::Rgb(28, 28, 28);
const STATUS_BG: Color = Color::Rgb(18, 18, 18);
const BRANCH_BG: Color = Color::Rgb(35, 55, 35);  // git branch pill background
const POPUP_BG: Color = Color::Rgb(26, 26, 26);
const POPUP_BORDER: Color = Color::Rgb(55, 55, 55);
const POPUP_MATCH: Color = Color::Rgb(0, 180, 100);
const POPUP_SEL_BG: Color = Color::Rgb(42, 42, 42);
const CODE_FG: Color = Color::Rgb(100, 210, 255);    // inline code / code blocks
const POPUP_WINDOW: usize = 7;                        // max items visible at once in popup

// Tip lines shown below the working indicator — cycle by elapsed seconds
const TIPS: &[&str] = &[
    "Use /compress to free context space mid-session",
    "Use /model to switch providers without losing session history",
    "Use /permissions danger-full-access for unrestricted shell access",
    "PageUp / PageDown to scroll through conversation history",
    "Type /help to see all available slash commands",
    "Press esc to interrupt a running turn at any time",
    "Use /session list to see and switch between saved sessions",
];

// Permission modes (in display/cycle order)
const PERM_MODES: &[(&str, &str)] = &[
    ("read-only",           "no writes · no shell"),
    ("workspace-write",     "files only · no shell"),
    ("danger-full-access",  "unrestricted · full shell"),
];

// Known models for the in-popup model picker
const MODEL_ENTRIES: &[(&str, &str, &str)] = &[
    ("gemini-2.5-pro",            "Google",    "Most capable Gemini — complex reasoning"),
    ("gemini-2.5-flash",          "Google",    "Fast & capable — recommended default"),
    ("gemini-2.5-flash-lite",     "Google",    "Lightest Gemini — maximum speed"),
    ("gemini-2.0-flash",          "Google",    "Previous Flash generation"),
    ("claude-opus-4-7",           "Anthropic", "Most capable Claude"),
    ("claude-sonnet-4-6",         "Anthropic", "Best balance of speed and capability"),
    ("claude-haiku-4-5-20251001", "Anthropic", "Fastest Claude"),
    ("gpt-4o",                    "OpenAI",    "GPT-4o multimodal flagship"),
    ("gpt-4o-mini",               "OpenAI",    "Efficient GPT-4o variant"),
    ("o3-mini",                   "OpenAI",    "o3 reasoning — efficient"),
    ("grok-3",                    "xAI",       "Grok 3 flagship"),
    ("grok-3-mini",               "xAI",       "Efficient Grok variant"),
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
        }
    }
}

impl TuiState {
    pub fn new(model: String, cwd: String, permission_mode: String) -> Self {
        Self { model, cwd, permission_mode, ..Default::default() }
    }

    pub fn push_exec(&mut self, block: ExecBlock) {
        if matches!(&block, ExecBlock::ToolUse { .. }) {
            self.deactivate_last_tool();
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
///   /permissions[…] → mode picker (auto-submit on Enter)
///   /model[…]   → model picker (auto-submit on Enter)
fn popup_items(input: &str) -> Vec<PopupItem> {
    if !input.starts_with('/') {
        return vec![];
    }

    // ── Permission mode picker ─────────────────────────────────────────────
    let perm_prefix = "/permissions";
    if input == perm_prefix
        || input.starts_with("/permissions ")
        || (input.len() > 1 && perm_prefix.starts_with(input))
    {
        let partial = if input.starts_with("/permissions ") {
            input["/permissions ".len()..].trim()
        } else {
            ""
        };
        return PERM_MODES
            .iter()
            .filter(|(mode, _)| partial.is_empty() || mode.starts_with(partial))
            .map(|(mode, desc)| PopupItem {
                display: format!("permissions {mode}"),
                complete: format!("/permissions {mode}"),
                desc: desc.to_string(),
            })
            .collect();
    }

    // ── Model picker ──────────────────────────────────────────────────────
    let model_prefix = "/model";
    if input == model_prefix
        || input.starts_with("/model ")
        || (input.len() > 1 && model_prefix.starts_with(input))
    {
        let partial = if input.starts_with("/model ") {
            input["/model ".len()..].trim()
        } else {
            ""
        };
        return MODEL_ENTRIES
            .iter()
            .filter(|(id, _, _)| partial.is_empty() || id.contains(partial))
            .take(12)
            .map(|(id, provider, desc)| PopupItem {
                display: format!("model  {id}"),
                complete: format!("/model {id}"),
                desc: format!("{provider}  ·  {desc}"),
            })
            .collect();
    }

    // ── Regular slash command matching ────────────────────────────────────
    let prefix = &input[1..];
    slash_command_specs()
        .iter()
        .filter(|s| s.name.starts_with(prefix))
        .take(8)
        .map(|s| {
            // Don't show <arg> placeholder for commands handled in-popup
            let hint = match s.name {
                "model" | "permissions" => String::new(),
                _ => s.argument_hint.map(|h| format!(" {h}")).unwrap_or_default(),
            };
            PopupItem {
                display: format!("{}{hint}", s.name),
                complete: format!("/{}", s.name),
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
                        lines.push(Line::from(Span::styled(
                            format!("  {line}"),
                            Style::default().fg(CODE_FG),
                        )));
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
    let items = popup_items(&state.input);
    let n_items = items.len();
    // Popup: up to POPUP_WINDOW items + 1 nav footer; placed BELOW input (Gemini-style)
    let popup_h = if n_items == 0 { 0u16 } else { (n_items.min(POPUP_WINDOW) + 1) as u16 };
    // Working zone: 2 rows (indicator + tip); absent when idle
    let working_h = if state.working { 2u16 } else { 0u16 };

    // Layout top→bottom: content(flex) | [working 2r?] | input(1r) | [popup?] | footer(1r)
    let mut constraints = vec![Constraint::Min(3)];
    if working_h > 0 { constraints.push(Constraint::Length(working_h)); }
    constraints.push(Constraint::Length(1)); // input
    if popup_h > 0 { constraints.push(Constraint::Length(popup_h)); }
    constraints.push(Constraint::Length(1)); // footer

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    let mut idx = 0usize;
    render_content(f, layout[idx], state);
    idx += 1;
    if working_h > 0 {
        render_working(f, layout[idx], state);
        idx += 1;
    }
    render_input(f, layout[idx], state);
    idx += 1;
    if popup_h > 0 {
        let sel = state.popup_selected.min(n_items.saturating_sub(1));
        render_popup(f, layout[idx], &items, sel);
        idx += 1;
    }
    render_footer(f, layout[idx], state);
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
                lines.extend(markdown_to_lines(text));
            }

            ExecBlock::WorkedFor(secs) => {
                let dur = if *secs >= 60 {
                    format!("{}m {}s", secs / 60, secs % 60)
                } else {
                    format!("{secs}s")
                };
                lines.push(Line::from(Span::styled(
                    format!("  Worked for {dur}"),
                    Style::default().fg(DIM).add_modifier(Modifier::ITALIC),
                )));
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
    let w = area.width.max(1) as usize;

    // Compute total rendered height in rows, accounting for text wrapping.
    // Using usize to avoid u16 overflow with large logs.
    let total_wrapped: usize = lines
        .iter()
        .map(|line| {
            let chars: usize = line.spans.iter().map(|s| s.content.chars().count()).sum();
            if chars == 0 { 1 } else { (chars + w - 1) / w }
        })
        .sum();

    let visible = area.height as usize;
    // max_scroll is how many rows we can scroll up from the bottom
    let max_scroll = total_wrapped.saturating_sub(visible);
    // Clamp state.scroll so we never over-scroll past the top
    let effective_scroll = (state.scroll as usize).min(max_scroll);
    // scroll_row is the top row to pass to Paragraph (0 = top of content)
    let scroll_row = max_scroll.saturating_sub(effective_scroll).min(u16::MAX as usize) as u16;

    let para = Paragraph::new(Text::from(lines))
        .style(Style::default().bg(BG).fg(FG))
        .wrap(Wrap { trim: false })
        .scroll((scroll_row, 0));
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

    let mut lines: Vec<Line<'static>> = Vec::new();

    for (abs_i, item) in items[win_start..win_end].iter().enumerate() {
        let i = win_start + abs_i;
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

    // Nav footer: position counter + key hints
    let nav = format!(
        "  ({}/{})  ↑↓ navigate  ·  tab select  ·  esc dismiss",
        selected + 1,
        total,
    );
    lines.push(Line::from(Span::styled(nav, Style::default().fg(DIM).bg(POPUP_BG))));

    let para = Paragraph::new(Text::from(lines)).style(Style::default().bg(POPUP_BG));
    f.render_widget(para, area);
}

/// 1-row input bar with the git branch badge pinned to the right edge.
fn render_input(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let branch = git_branch_cached();

    // Branch badge width: " branch-name  " = name + 2 spaces padding + separator
    let badge_text = branch
        .as_deref()
        .map(|b| format!(" {b} "))
        .unwrap_or_default();
    let badge_w = badge_text.chars().count() as u16;

    // Split row: [input | branch badge]
    let h_layout = if badge_w > 0 && area.width > badge_w + 4 {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(4),
                Constraint::Length(badge_w),
            ])
            .split(area)
    } else {
        // Terminal too narrow — no badge
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1)])
            .split(area)
    };

    let input_line = if state.input.is_empty() {
        Line::from(vec![
            Span::styled(" > ", Style::default().fg(CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(
                "Type your message or @path/to/file",
                Style::default().fg(DIM),
            ),
        ])
    } else {
        let before: String = state.input.chars().take(state.cursor).collect();
        let cursor_ch: String = state
            .input
            .chars()
            .nth(state.cursor)
            .map(|c| c.to_string())
            .unwrap_or_else(|| " ".to_string());
        let after: String = state.input.chars().skip(state.cursor + 1).collect();
        Line::from(vec![
            Span::styled(" > ", Style::default().fg(CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(before, Style::default().fg(FG)),
            Span::styled(cursor_ch, Style::default().fg(BG).bg(FG)),
            Span::styled(after, Style::default().fg(FG)),
        ])
    };

    f.render_widget(
        Paragraph::new(input_line).style(Style::default().bg(USER_BOX_BG)),
        h_layout[0],
    );

    // Branch pill (only if layout has 2 columns)
    if h_layout.len() == 2 {
        let badge_line = Line::from(Span::styled(
            badge_text,
            Style::default()
                .fg(GREEN)
                .bg(BRANCH_BG)
                .add_modifier(Modifier::BOLD),
        ));
        f.render_widget(
            Paragraph::new(badge_line).style(Style::default().bg(BRANCH_BG)),
            h_layout[1],
        );
    }
}

/// 2-row working zone shown only when a turn is in progress.
/// Row 0: `* Thinking…  (elapsed · ↓ tokens)`  — matches Claude Code style
/// Row 1: `⌐ Tip: …`  — rotating tip from TIPS list
fn render_working(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let secs = state.turn_start.map(|t| t.elapsed().as_secs()).unwrap_or(0);
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

    // Rotating tip — changes every 8 seconds
    let tip_idx = (secs / 8) as usize % TIPS.len();
    let tip = TIPS[tip_idx];

    let working_line = Line::from(vec![
        Span::styled(" * ", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
        Span::styled("Thinking… ", Style::default().fg(ORANGE)),
        Span::styled(
            format!("({timer}{tok_str})"),
            Style::default().fg(GREY),
        ),
    ]);
    let tip_line = Line::from(vec![
        Span::styled("   ⌐ ", Style::default().fg(DIM)),
        Span::styled(
            format!("Tip: {tip}"),
            Style::default().fg(DIM),
        ),
    ]);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(area);

    f.render_widget(
        Paragraph::new(working_line).style(Style::default().bg(STATUS_BG)),
        layout[0],
    );
    f.render_widget(
        Paragraph::new(tip_line).style(Style::default().bg(STATUS_BG)),
        layout[1],
    );
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
                Style::default().fg(DIM),
            ),
        ])
    } else {
        let dir = std::path::Path::new(&state.cwd)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&state.cwd);
        let tok = if state.tokens_in > 0 {
            format!(
                "  ·  {}↑ {}↓",
                fmt_tokens(state.tokens_in),
                fmt_tokens(state.tokens_out)
            )
        } else {
            String::new()
        };
        let perm = if state.permission_mode.is_empty() {
            String::new()
        } else {
            format!("  ·  {}", state.permission_mode)
        };
        let text = format!(
            " {}  ·  {}{}{}",
            state.model, dir, perm, tok,
        );
        Line::from(Span::styled(text, Style::default().fg(GREY)))
    };
    f.render_widget(
        Paragraph::new(line).style(Style::default().bg(STATUS_BG)),
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
    pub fn new(model: String, cwd: String, permission_mode: String) -> (Self, std::sync::mpsc::Receiver<String>) {
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        let (submit_tx, submit_rx) = std::sync::mpsc::channel();
        let app = Self {
            state: Arc::new(Mutex::new(TuiState::new(model, cwd, permission_mode))),
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

                                // Enter with popup:
                                //   /permissions X  → auto-submit (inline handler applies immediately)
                                //   /model X        → auto-submit (inline handler applies immediately)
                                //   anything else   → complete into input box
                                (KeyCode::Enter, KeyModifiers::NONE) if has_popup => {
                                    let sel = state.popup_selected.min(items.len().saturating_sub(1));
                                    let complete = items[sel].complete.clone();
                                    let is_auto = complete.starts_with("/permissions ")
                                        || complete.starts_with("/model ");
                                    if is_auto {
                                        state.input = complete;
                                        state.cursor = state.input.chars().count();
                                        state.popup_selected = 0;
                                        let text = state.input_take();
                                        submit_text = Some(text);
                                    } else {
                                        state.input = complete;
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
