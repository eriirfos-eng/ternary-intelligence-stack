//! Human-In-The-Loop gate — brutalist terminal confirmation for Tier 3 operations.

use crate::{error::LlbError, tier::SafetyTier, types::MutationRequest};
use std::{
    io::{self, Write},
    path::Path,
};

const CONFIRM_WORD: &str = "CONFIRM";

/// Display a high-contrast HITL prompt and block until the operator responds.
/// Returns `Ok(())` if confirmed, `Err(LlbError::HumanVeto)` otherwise.
pub fn confirm(req: &MutationRequest, resolved: &Path, tier: SafetyTier) -> Result<(), LlbError> {
    print_banner(req, resolved, tier);

    let response = read_response()?;
    if response.trim() == CONFIRM_WORD {
        println!(
            "\x1b[32;1m[LLB]\x1b[0m Human confirmed. ExecutionTicket will be issued.\n"
        );
        Ok(())
    } else {
        println!("\x1b[31;1m[LLB]\x1b[0m Operation vetoed by operator. Rollback state preserved.\n");
        Err(LlbError::HumanVeto)
    }
}

fn print_banner(req: &MutationRequest, resolved: &Path, tier: SafetyTier) {
    let width = 60usize;
    let bar = "═".repeat(width);


    eprintln!("\n\x1b[31;1m╔{bar}╗\x1b[0m");
    eprintln!("\x1b[31;1m║{:^width$}║\x1b[0m", "⚠  LAST LOOK BACK — HUMAN GATE  ⚠");
    eprintln!("\x1b[31;1m║{:^width$}║\x1b[0m", format!("[ {} ]", tier));
    eprintln!("\x1b[31;1m╠{bar}╣\x1b[0m");

    let op_line = format!("  OPERATION  : {}", req.operation);
    let path_line = format!("  PATH       : {}", resolved.display());
    let goal_line = format!("  INTENT     : {}", req.intent.goal);
    let just_line = format!("  REASON     : {}", req.intent.justification);
    let fall_line = format!("  FALLBACK   : {}", req.intent.fallback_if_rejected);

    for line in [&op_line, &path_line, &goal_line, &just_line, &fall_line] {
        eprintln!("\x1b[33m║\x1b[0m {:<width$}\x1b[33m║\x1b[0m", truncate(line, width - 2));
    }

    eprintln!("\x1b[31;1m╠{bar}╣\x1b[0m");
    eprintln!("\x1b[31;1m║{:^width$}║\x1b[0m", "THIS ACTION IS DESTRUCTIVE.");
    eprintln!("\x1b[31;1m║{:^width$}║\x1b[0m", "IT MAY NOT BE FULLY REVERSIBLE.");
    eprintln!("\x1b[31;1m╠{bar}╣\x1b[0m");
    eprintln!(
        "\x1b[33m║\x1b[0m {:<width$}\x1b[33m║\x1b[0m",
        format!("Type \x1b[1m{CONFIRM_WORD}\x1b[0m to proceed. Anything else aborts.")
    );
    eprintln!("\x1b[31;1m╚{bar}╝\x1b[0m");
    eprint!("\x1b[1m> \x1b[0m");
    let _ = io::stderr().flush();
}

fn read_response() -> Result<String, LlbError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(LlbError::Io)?;
    Ok(input)
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        s
    } else {
        &s[..max]
    }
}
