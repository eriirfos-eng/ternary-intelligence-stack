// SPDX-License-Identifier: LGPL-3.0-or-later
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS
// Open-core compiler. See LICENSE-LGPL in the repository root.

//! # ternlang-cli
//!
//! Command-line interface for ternlang — run, build, sim, fmt, repl, and compat commands
//! for the Balanced Ternary Execution VM.
//!
//! ## Quick start
//!
//! ```no_run
//! // This crate is a binary, use `ternlang run program.tern` from the shell.
//! ```

use clap::{Parser as ClapParser, Subcommand};

/// Ternlang — Balanced Ternary Language CLI
#[derive(ClapParser, Debug)]
#[command(name = "ternlang", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a .tern program in the BET VM
    Run {
        /// Path to the .tern source file
        path: String,
        /// Maximum instruction steps (default: 10M)
        #[arg(short, long, default_value_t = 10_000_000)]
        max_steps: u64,
    },

    /// Compile .tern to BET bytecode
    Build {
        /// Input .tern file
        input: String,
        /// Output .ternbc file (default: <input>.ternbc)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Simulate a binary .ternbc file step-by-step
    Sim {
        /// Path to the .ternbc file
        path: String,
    },

    /// Format .tern source files
    Fmt {
        /// Files to format (default: stdin)
        files: Vec<String>,
    },

    /// Start the interactive REPL
    Repl,

    /// Convert legacy binary format to ternlang
    Compat {
        /// Input file path
        input: String,
        /// Output file path
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { path, max_steps } => {
            println!("Running {} (max_steps={})", path, max_steps);
            // TODO: integrate with ternlang-core VM
        }
        Commands::Build { input, output } => {
            let out = output.unwrap_or_else(|| format!("{}.ternbc", input));
            println!("Building {} -> {}", input, out);
        }
        Commands::Sim { path } => {
            println!("Simulating {}", path);
        }
        Commands::Fmt { files } => {
            if files.is_empty() {
                println!("Formatting stdin...");
            } else {
                println!("Formatting {} file(s)", files.len());
            }
        }
        Commands::Repl => {
            println!("ternlang REPL — type :quit to exit");
        }
        Commands::Compat { input, output } => {
            println!("Converting {} -> {}", input, output);
        }
    }
}
