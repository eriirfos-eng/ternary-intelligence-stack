// LLB Protocol Demo — RFI-IRFOS Ternary Intelligence Stack
//
// Demonstrates all five safety tiers and the atomic rollback guarantee.
// Run: cargo run --release --bin llb_demo -p moe-llb

use albert_llb::{
    blacklist,
    error::LlbError,
    execute,
    types::{LlbDecision, MutationRequest, Operation, StructuredIntent},
};
use std::path::PathBuf;

fn intent(goal: &str, just: &str, fall: &str) -> StructuredIntent {
    StructuredIntent {
        goal: goal.into(),
        justification: just.into(),
        fallback_if_rejected: fall.into(),
    }
}

fn print_header(title: &str) {
    println!("\n\x1b[36;1m┌─ {title} \x1b[0m");
}

fn print_decision(decision: &Result<LlbDecision, LlbError>) {
    match decision {
        Ok(LlbDecision::Allow) => println!("  \x1b[32m[+1 ALLOW]\x1b[0m  Transaction committed."),
        Ok(LlbDecision::Warn(msg)) => println!("  \x1b[33m[ 0 WARN ]\x1b[0m  Soft block: {msg}"),
        Ok(LlbDecision::HardVeto) => println!("  \x1b[31m[-1 VETO ]\x1b[0m  Hard veto. Rollback triggered."),
        Err(LlbError::HumanVeto) => println!("  \x1b[31m[-1 VETO ]\x1b[0m  Operator vetoed at HITL gate."),
        Err(LlbError::BlacklistedPath { path }) => {
            println!("  \x1b[31m[-1 VETO ]\x1b[0m  Blacklisted path blocked: {path}");
        }
        Err(LlbError::PathTraversal { path }) => {
            println!("  \x1b[31m[-1 VETO ]\x1b[0m  Path traversal detected: {path}");
        }
        Err(LlbError::IoccViolation { detail }) => {
            println!("  \x1b[31m[-1 VETO ]\x1b[0m  IOCC Gate 2 failure: {detail}");
        }
        Err(e) => println!("  \x1b[31m[ERROR   ]\x1b[0m  {e}"),
    }
}

fn tmp(name: &str) -> PathBuf {
    std::env::temp_dir().join(name)
}

fn main() {
    println!("\x1b[1m╔══════════════════════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[1m║     LLB Protocol Demo — Last Look Back v1.5 (MVP+)          ║\x1b[0m");
    println!("\x1b[1m║     RFI-IRFOS Ternary Intelligence Stack                    ║\x1b[0m");
    println!("\x1b[1m╚══════════════════════════════════════════════════════════════╝\x1b[0m");

    // ── Demo 1: T1 CREATE — happy path ────────────────────────────────────
    print_header("Demo 1 — T1/CREATE (happy path)");
    let path1 = tmp("llb_demo_create.txt");
    let _ = std::fs::remove_file(&path1);

    let d1 = execute(
        MutationRequest {
            path: path1.to_str().unwrap().into(),
            operation: Operation::Create,
            intent: intent(
                "write analysis output",
                "agent computed a result that the user asked to save",
                "print to stdout instead",
            ),
        },
        |p| std::fs::write(p, b"LLB demo output\n").map_err(LlbError::Io),
    );
    print_decision(&d1);
    println!("  File exists after commit: {}", path1.exists());
    let _ = std::fs::remove_file(&path1);

    // ── Demo 2: T2 OVERWRITE — rollback on action failure ─────────────────
    print_header("Demo 2 — T2/OVERWRITE — rollback on simulated failure");
    let path2 = tmp("llb_demo_overwrite.txt");
    std::fs::write(&path2, b"precious original data\n").unwrap();

    let d2 = execute(
        MutationRequest {
            path: path2.to_str().unwrap().into(),
            operation: Operation::Overwrite,
            intent: intent("update cache file", "stale cache detected", "skip update"),
        },
        |p| {
            std::fs::write(p, b"corrupted mid-write").map_err(LlbError::Io)?;
            Err(LlbError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "simulated I/O error during write",
            )))
        },
    );
    print_decision(&d2);
    let content = std::fs::read_to_string(&path2).unwrap_or_default();
    println!("  Content after rollback: {:?}", content.trim());
    let _ = std::fs::remove_file(&path2);

    // ── Demo 3: Path traversal blocked at Gate 1 ──────────────────────────
    print_header("Demo 3 — Path traversal blocked (Gate 1)");
    let d3 = execute(
        MutationRequest {
            path: "/tmp/../etc/passwd".into(),
            operation: Operation::Overwrite,
            intent: intent("update config", "agent thinks this is a temp file", "abort"),
        },
        |_| Ok(()),
    );
    print_decision(&d3);

    // ── Demo 4: Blacklisted path blocked at Gate 1 ────────────────────────
    print_header("Demo 4 — Blacklisted path blocked (Gate 1)");
    let d4 = execute(
        MutationRequest {
            path: "/etc/hosts".into(),
            operation: Operation::Overwrite,
            intent: intent(
                "add hostname entry",
                "agent wants to resolve custom domain",
                "use /etc/hosts.d/ instead",
            ),
        },
        |_| Ok(()),
    );
    print_decision(&d4);

    // ── Demo 5: Forbidden binary check ────────────────────────────────────
    print_header("Demo 5 — Forbidden binary intercepted");
    match blacklist::check_shell_command("rm -rf /home/user/project") {
        Ok(()) => println!("  \x1b[31m[BUG]\x1b[0m  Command should have been blocked!"),
        Err(e) => println!("  \x1b[32m[BLOCKED]\x1b[0m  {e}"),
    }

    // ── Demo 6: IOCC spillover detection ──────────────────────────────────
    print_header("Demo 6 — IOCC Gate 2 spillover detection");
    let path6 = tmp("llb_demo_spillover_target.txt");
    let spill = tmp("llb_demo_spillover_extra.txt");
    std::fs::write(&path6, b"target").unwrap();
    let _ = std::fs::remove_file(&spill);

    let d6 = execute(
        MutationRequest {
            path: path6.to_str().unwrap().into(),
            operation: Operation::Overwrite,
            intent: intent("update target", "refreshing cached data", "skip"),
        },
        |p| {
            std::fs::write(p, b"updated").map_err(LlbError::Io)?;
            // Sneakily create an extra file the agent didn't declare.
            std::fs::write(&spill, b"I wasn't supposed to be here").map_err(LlbError::Io)?;
            Ok(())
        },
    );
    print_decision(&d6);
    let _ = std::fs::remove_file(&path6);
    let _ = std::fs::remove_file(&spill);

    // ── Summary ───────────────────────────────────────────────────────────
    println!("\n\x1b[1m╔══════════════════════════════════════════════════════════════╗\x1b[0m");
    println!(  "\x1b[1m║  LLB Protocol — all demos complete.                        ║\x1b[0m");
    println!(  "\x1b[1m║  Safety moved from linguistic to binary layer.              ║\x1b[0m");
    println!(  "\x1b[1m╚══════════════════════════════════════════════════════════════╝\x1b[0m");
}
