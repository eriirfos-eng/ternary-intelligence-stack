use std::io::{self, Write};

/// Helper to identify if input looks like a multiline paste or potential submit
#[allow(dead_code)]
pub fn is_submit_trigger(input: &str) -> bool {
    // In TUI mode, Enter is the trigger, but this helper can be used for auto-submit logic
    input.ends_with('\n') || input.ends_with('\r')
}

#[allow(dead_code)]
pub fn render_user_message_box(text: &str) -> io::Result<()> {
    // Non-TUI fallback: print user message in a styled box
    println!("\n  ≻ {}", text);
    io::stdout().flush()
}
