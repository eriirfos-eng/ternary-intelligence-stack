// SPDX-License-Identifier: LGPL-3.0-or-later
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS
// Open-core compiler. See LICENSE-LGPL in the repository root.

use clap::{Parser as ClapParser, Subcommand};
use serde_json;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use ternlang_core::parser::Parser;
use ternlang_core::codegen::betbc::BytecodeEmitter;
use ternlang_core::codegen::tern_asm::emit_tern_asm;
use ternlang_core::vm::{BetVm, Value};
use ternlang_core::ModuleResolver;

use ternlang_hdl::{BetSimEmitter, BetRtlProcessor};
use ternlang_runtime::TernNode;
use walkdir::WalkDir;
use colored::*;

#[derive(ClapParser)]
#[command(name = "ternlang")]
#[command(about = "Ternlang — Balanced Ternary Intelligence Stack\n\nUsage:\n  ternlang                    → interactive REPL\n  ternlang <file.tern>        → run a .tern file directly\n  ternlang run <file.tern>    → same as above\n  ternlang repl               → interactive REPL\n  ternlang build <file.tern>  → compile to bytecode\n  ternlang fmt <file.tern>    → format source\n  ternlang test [path]        → run test suite", long_about = None)]
struct Cli {
    /// Optional .tern file to run directly (shortcut for `ternlang run <file>`)
    #[arg(value_name = "FILE", conflicts_with = "command")]
    file: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile and run a .tern file
    Run {
        /// Path to the .tern file
        file: PathBuf,
        /// This node's TCP address for distributed agent communication (e.g. 127.0.0.1:7373)
        #[arg(long, value_name = "ADDR")]
        node_addr: Option<String>,
        /// Pre-connect to a peer node before running (can be specified multiple times)
        #[arg(long, value_name = "ADDR")]
        peer: Vec<String>,
        /// Emit variable→register symbol map to stderr as TERN_SYMBOLS:name=reg,...
        /// Used by the VS Code extension to map register dumps back to source variable names.
        #[arg(long)]
        emit_symbols: bool,
    },
    /// Compile a .tern file to bytecode or TERN assembly
    Build {
        /// Path to the .tern file
        file: PathBuf,
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Emit TERN-compatible assembly (RISC-V-inspired balanced ternary ASM) instead of BET bytecode
        #[arg(long)]
        emit_tern: bool,
    },
    /// Interactive REPL for trit expression evaluation
    Repl,
    /// Format a .tern file (canonical 3-way match style)
    Fmt {
        /// Path to the .tern file
        file: PathBuf,
        /// Write formatted output back to file (default: print to stdout)
        #[arg(short, long)]
        write: bool,
    },
    /// Generate an Icarus Verilog FPGA testbench or run RTL simulation
    Sim {
        /// Path to the .tern file
        file: PathBuf,
        /// Write testbench to this file (default: <file>.sim.v)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Run simulation with iverilog+vvp if available
        #[arg(short, long)]
        run: bool,
        /// Run cycle-accurate RTL simulation in pure Rust (no external tools needed)
        #[arg(long)]
        rtl: bool,
        /// Max clock cycles for RTL simulation (default: 10000)
        #[arg(long, default_value = "10000")]
        max_cycles: u64,
    },
    /// Emit BET processor Verilog and run Yosys synthesis (Phase 6.1)
    HdlSynth {
        /// Output directory for generated Verilog files (default: ./bet_hdl/)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Run Yosys synthesis if yosys is on PATH
        #[arg(short, long)]
        synth: bool,
    },
    /// Run native ternary tests (*_test.tern)
    Test {
        /// Path to a file or directory to search for tests
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Audit an AI decision log for binary habituation and EU AI Act compliance
    Audit {
        /// Path to a JSON file: array of {input: string, output: string, confidence?: float}
        /// Omit to read from stdin.
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,
        /// Write JSON report to this path (default: audit_report.json)
        #[arg(short, long, value_name = "OUT")]
        output: Option<PathBuf>,
        /// Also write a human-readable HTML report alongside the JSON
        #[arg(long)]
        html: bool,
    },
    /// Translate Python, SQL, or JSON rule sets into .tern code with explicit tend arms
    Translate {
        /// Source file to translate (Python, SQL, or JSON rules).
        /// Omit to read from stdin.
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,
        /// Source language: python, sql, json_rules (default: python)
        #[arg(short, long, default_value = "python")]
        language: String,
        /// Write output to this .tern file (default: prints to stdout)
        #[arg(short, long, value_name = "OUT")]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    // Shortcut: `ternlang file.tern` → run directly (same as `ternlang run file.tern`)
    // Shortcut: `ternlang`           → interactive REPL
    let command = match (cli.file, cli.command) {
        (Some(f), _) => Commands::Run { file: f, node_addr: None, peer: vec![], emit_symbols: false },
        (None, Some(c)) => c,
        (None, None) => Commands::Repl,
    };

    match &command {
        Commands::Run { file, node_addr, peer, emit_symbols } => {
            let input = fs::read_to_string(file).expect("Failed to read file");
            let mut parser = Parser::new(&input);
            let mut emitter = BytecodeEmitter::new();
            let header_patch = emitter.emit_header_jump();

            // Try parsing as a program first
            match parser.parse_program() {
                Ok(mut prog) => {
                    ModuleResolver::from_source_file(file).resolve(&mut prog);
                    emitter.emit_program(&prog);
                    emitter.patch_header_jump(header_patch);
                    emitter.emit_entry_call("main");
                }
                Err(e) => {
                    // Only fallback if it's not a program (e.g. missing 'fn')
                    // If it's a parse error in a real program, we should probably report it and stop.
                    let error_str = format!("{:?}", e);
                    if error_str.contains("ExpectedToken(\"Fn\"") || error_str.contains("UnexpectedToken(\"Let\"") {
                        // Fallback: Reset and try parsing statements (for snippets without 'fn').
                        // BUG-L02 fix: if parse_stmt() returns UnexpectedToken("Fn"), the token
                        // was not consumed (peek_token branch in parse_stmt), so we can call
                        // parse_function() to parse it correctly.
                        let mut parser = Parser::new(&input);
                        emitter.patch_header_jump(header_patch);
                        let mut found_functions = false;
                        loop {
                            match parser.parse_stmt() {
                                Ok(stmt) => emitter.emit_stmt(&stmt),
                                Err(e) => {
                                    let e_str = format!("{:?}", e);
                                    if e_str.contains("EOF") {
                                        break;
                                    }
                                    // BUG-L02: fn keyword at statement level — parse as function
                                    if e_str.contains("UnexpectedToken(\"Fn\")") {
                                        match parser.parse_function() {
                                            Ok(func) => {
                                                emitter.emit_function(&func);
                                                found_functions = true;
                                            }
                                            Err(e2) => {
                                                eprintln!("Parse fn error: {:?}", e2);
                                                break;
                                            }
                                        }
                                    } else {
                                        eprintln!("Parse stmt error: {:?}", e);
                                        break;
                                    }
                                }
                            }
                        }
                        if found_functions {
                            emitter.emit_entry_call("main");
                        }
                    } else {
                        eprintln!("Parse program error: {:?}", e);
                        std::process::exit(1);
                    }
                }
            }

            // Emit variable→register map to stderr before VM runs.
            // Format: TERN_SYMBOLS:varname=regnum,varname=regnum,...
            // Uses the per-function snapshot from "main" so local variables are captured.
            // Filtered: skip compound keys like "obj.field" (internal struct slots).
            if *emit_symbols {
                let sym_map = emitter.get_function_symbols("main")
                    .or_else(|| Some(emitter.get_symbols()));
                if let Some(map) = sym_map {
                    let mut parts: Vec<String> = map
                        .iter()
                        .filter(|(k, _)| !k.contains('.') && !k.contains('['))
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect();
                    parts.sort();
                    eprintln!("TERN_SYMBOLS:{}", parts.join(","));
                }
            }

            let code = emitter.finalize();
            println!("Emitted {} bytes of bytecode", code.len());
            let mut vm = BetVm::new(code);
            emitter.register_agents(&mut vm);

            // Phase 5.1: Distributed runtime setup
            if let Some(addr) = node_addr {
                let node = Arc::new(TernNode::new(addr));
                node.listen();
                eprintln!("[runtime] TernNode listening on {}", addr);
                vm.set_node_id(addr.clone());
                // Pre-connect to any specified peers
                for peer_addr in peer {
                    match node.connect(peer_addr) {
                        Ok(()) => eprintln!("[runtime] connected to peer {}", peer_addr),
                        Err(e) => eprintln!("[runtime] peer {} unreachable: {}", peer_addr, e),
                    }
                }
                vm.set_remote(node);
            }

            match vm.run() {
                Ok(_) => {
                    // Check top of stack for return value (affirm/tend/reject)
                    let result = vm.peek_stack();
                    if let Some(Value::Trit(ternlang_core::trit::Trit::Reject)) = result {
                        eprintln!("Program exited with error (Reject state).");
                        std::process::exit(1);
                    }

                    println!("Program exited successfully.");
                    // Print registers for debugging
                    for i in 0..10 {
                        let val = vm.get_register(i);
                        match val {
                            Value::Trit(t) => println!("Reg {}: trit({})", i, t),
                            Value::Int(v) => println!("Reg {}: int({})", i, v),
                            Value::Float(f) => println!("Reg {}: float({})", i, f),
                            Value::TensorRef(r) => println!("Reg {}: tensor_ref({})", i, r),
                            Value::AgentRef(a, _)  => println!("Reg {}: agent_ref({})", i, a),
                            Value::String(s) => println!("Reg {}: string({:?})", i, s),
                        }
                    }
                }
                Err(e) => {
                    eprintln!("VM Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Repl => {
            run_repl();
        }
        Commands::Fmt { file, write } => {
            run_fmt(file, *write);
        }
        Commands::Sim { file, output, run, rtl, max_cycles } => {
            if *rtl {
                run_rtl_sim(file, *max_cycles);
            } else {
                run_sim(file, output.as_deref(), *run);
            }
        }
        Commands::HdlSynth { output, synth } => {
            run_hdl_synth(output.as_deref(), *synth);
        }
        Commands::Test { path } => {
            run_tests(path);
        }
        Commands::Translate { file, language, output } => {
            run_translate(file.as_deref(), language, output.as_deref());
        }
        Commands::Audit { file, output, html } => {
            run_audit(file.as_deref(), output.as_deref(), *html);
        }
        Commands::Build { file, output, emit_tern } => {
            let input = fs::read_to_string(file).expect("Failed to read file");
            let mut parser = Parser::new(&input);

            let resolver = ModuleResolver::from_source_file(file);
            let prog_result = parser.parse_program().map(|mut p| { resolver.resolve(&mut p); p });

            if *emit_tern {
                // ── TERN-ASM output ──────────────────────────────────────────
                let prog = match prog_result {
                    Ok(p) => p,
                    Err(e) => { eprintln!("Parse error: {:?}", e); std::process::exit(1); }
                };
                let asm = emit_tern_asm(&prog);
                let out_path = output.clone().unwrap_or_else(|| {
                    let mut path = file.clone();
                    path.set_extension("tern.asm");
                    path
                });
                fs::write(&out_path, &asm).expect("Failed to write TERN assembly");
                println!("TERN assembly written to {}", out_path.display());
            } else {
                // ── BET bytecode output ──────────────────────────────────────
                let mut emitter = BytecodeEmitter::new();
                match prog_result {
                    Ok(prog) => { emitter.emit_program(&prog); }
                    Err(_) => {
                        let mut parser = Parser::new(&input);
                        while let Ok(stmt) = parser.parse_stmt() {
                            emitter.emit_stmt(&stmt);
                        }
                    }
                }
                let code = emitter.finalize();
                let out_path = output.clone().unwrap_or_else(|| {
                    let mut path = file.clone();
                    path.set_extension("tbc");
                    path
                });
                fs::write(&out_path, code).expect("Failed to write bytecode");
                println!("Compiled to {}", out_path.display());
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RTL Sim — Phase 6.1 cycle-accurate BET processor simulation (no external tools)
// ─────────────────────────────────────────────────────────────────────────────
fn run_rtl_sim(file: &std::path::PathBuf, max_cycles: u64) {
    let input = fs::read_to_string(file).expect("Failed to read file");
    let mut parser = Parser::new(&input);
    let mut emitter = BytecodeEmitter::new();

    match parser.parse_program() {
        Ok(mut prog) => { ModuleResolver::from_source_file(file).resolve(&mut prog); emitter.emit_program(&prog); }
        Err(_) => {
            let mut parser = Parser::new(&input);
            while let Ok(stmt) = parser.parse_stmt() { emitter.emit_stmt(&stmt); }
        }
    }

    let code = emitter.finalize();
    println!("BET RTL Simulator — Phase 6.1");
    println!("Bytecode: {} bytes | Max cycles: {}", code.len(), max_cycles);
    println!("{}", "─".repeat(52));

    let mut proc = BetRtlProcessor::new(code);
    let trace = proc.run(max_cycles);

    println!("  Cycles elapsed : {}", trace.cycles);
    println!("  Halted cleanly : {}", trace.halted);
    println!("{}", "─".repeat(52));

    // Print final register state (first 10)
    println!("  Final registers (0–9):");
    for (i, &v) in trace.final_regs.iter().take(10).enumerate() {
        let label = match v { 1 => "+1 (truth)", -1 => "-1 (conflict)", _ => " 0 (hold)" };
        println!("    r{:02}: {}", i, label);
    }

    if !trace.final_stack.is_empty() {
        println!("  Final stack (top→bottom): {:?}", trace.final_stack.iter().rev().collect::<Vec<_>>());
    } else {
        println!("  Final stack: empty");
    }

    // Print last 5 cycle snapshots for traceability
    if trace.cycles_state.len() > 1 {
        println!("{}", "─".repeat(52));
        println!("  Last {} cycles:", trace.cycles_state.len().min(5));
        for snap in trace.cycles_state.iter().rev().take(5).rev() {
            println!("    [cy {:>4}] pc={:04x} op=0x{:02x} stack={:?}",
                snap.cycle, snap.pc, snap.opcode, snap.stack);
        }
    }

    if !trace.halted {
        eprintln!("  WARNING: max_cycles ({}) reached without THALT", max_cycles);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Sim — compile to bytecode and emit Verilog testbench (Phase 6.1)
// ─────────────────────────────────────────────────────────────────────────────
fn run_sim(file: &std::path::PathBuf, output: Option<&std::path::Path>, run: bool) {
    let input = fs::read_to_string(file).expect("Failed to read file");
    let mut parser = Parser::new(&input);
    let mut emitter = BytecodeEmitter::new();

    match parser.parse_program() {
        Ok(mut prog) => {
            ModuleResolver::from_source_file(file).resolve(&mut prog);
            emitter.emit_program(&prog);
        }
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            return;
        }
    }

    let code = emitter.finalize();
    println!("Compiled: {} bytes of BET bytecode", code.len());

    let sim = BetSimEmitter::new();
    let tb = sim.emit_testbench(&code);

    let tb_path = output
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| {
            let mut p = file.clone();
            p.set_extension("sim.v");
            p
        });

    fs::write(&tb_path, &tb).expect("Failed to write testbench");
    println!("Testbench: {}", tb_path.display());

    if run {
        if BetSimEmitter::iverilog_available() {
            let path_str = tb_path.to_string_lossy();
            match BetSimEmitter::run_iverilog(&path_str) {
                Ok(output) => {
                    println!("\n--- Simulation output ---");
                    print!("{}", output);
                }
                Err(e) => eprintln!("Simulation failed: {}", e),
            }
        } else {
            println!("iverilog not found on PATH. To run the simulation:");
            println!("  sudo apt install iverilog");
            println!("  iverilog -o bet_sim.vvp {} && vvp bet_sim.vvp", tb_path.display());
        }
    } else {
        println!("\nTo run with Icarus Verilog:");
        println!("  iverilog -o bet_sim.vvp -g2001 {} && vvp bet_sim.vvp", tb_path.display());
        println!("  # Open bet_sim.vcd in GTKWave for waveform inspection");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// REPL — interactive trit expression evaluator
// ─────────────────────────────────────────────────────────────────────────────
fn run_repl() {
    use std::io::{self, Write};
    println!("ternlang REPL v0.1 — type a trit expression and press Enter. :q to quit.");
    println!("Examples: consensus(1, 0)   invert(-1)   1 + -1");
    println!();
    loop {
        print!("tern> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() { break; }
        let line = line.trim();
        if line == ":q" || line == "quit" || line.is_empty() && line == "" {
            if line == ":q" { break; }
            if line.is_empty() { continue; }
        }
        // Wrap in a minimal function so the parser can handle it
        let wrapped = format!("fn __repl__() -> trit {{ return {}; }}", line);
        let mut parser = Parser::new(&wrapped);
        match parser.parse_program() {
            Err(e) => { eprintln!("  parse error: {:?}", e); continue; }
            Ok(prog) => {
                let mut emitter = BytecodeEmitter::new();
                emitter.emit_program(&prog);
                let code = emitter.finalize();
                let mut vm = BetVm::new(code);
            emitter.register_agents(&mut vm);
                match vm.run() {
                    Ok(_) => {
                        let result = vm.get_register(0);
                        match result {
                            Value::Trit(t) => println!("  → {}", t),
                            Value::Int(v)  => println!("  → {}", v),
                            Value::Float(f) => println!("  → {}", f),
                            Value::TensorRef(r) => println!("  → tensor_ref({})", r),
                            Value::AgentRef(a, _)  => println!("  → agent_ref({})", a),
                            Value::String(s) => println!("  → {:?}", s),
                        }
                    }
                    Err(e) => eprintln!("  vm error: {}", e),
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Formatter — canonical 3-way match arm style
// ─────────────────────────────────────────────────────────────────────────────
fn run_fmt(file: &std::path::PathBuf, write: bool) {
    let input = std::fs::read_to_string(file).expect("Failed to read file");
    let formatted = fmt_source(&input);
    if write {
        std::fs::write(file, &formatted).expect("Failed to write formatted file");
        println!("Formatted {:?}", file);
    } else {
        print!("{}", formatted);
    }
}

/// Canonical formatting rules:
/// - Match arms: align `=>` with leading trit value right-justified to 2 chars
/// - Indent: 4 spaces
/// - Blank line between top-level functions
fn fmt_source(source: &str) -> String {
    let mut out = String::new();
    let mut in_match = false;

    for line in source.lines() {
        let trimmed = line.trim();

        // Detect match arm lines: start with -1, 0, or 1 followed by =>
        if in_match && (trimmed.starts_with("1 =>") || trimmed.starts_with("0 =>") || trimmed.starts_with("-1 =>")) {
            // Determine indent level from context (reuse original)
            let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
            // Right-align the trit value to 2 chars: " 1", " 0", "-1"
            let (trit_str, rest) = if trimmed.starts_with("-1") {
                ("-1", &trimmed[2..])
            } else if trimmed.starts_with('1') {
                (" 1", &trimmed[1..])
            } else {
                (" 0", &trimmed[1..])
            };
            out.push_str(&format!("{}{}{}\n", indent, trit_str, rest));
            continue;
        }

        if trimmed.starts_with("match ") { in_match = true; }
        if trimmed == "}" && in_match { in_match = false; }

        out.push_str(line);
        out.push('\n');
    }
    out
}

// ─────────────────────────────────────────────────────────────────────────────
// Phase 6.1 — HDL synthesis: emit Verilog + optional Yosys run
// ─────────────────────────────────────────────────────────────────────────────
fn run_hdl_synth(output: Option<&std::path::Path>, run_yosys: bool) {
    use ternlang_hdl::{VerilogEmitter, BetIsaEmitter};
    use std::process::Command;

    let out_dir = output
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("bet_hdl"));
    fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    println!("[hdl-synth] Emitting BET processor Verilog → {}/", out_dir.display());

    // ── Primitive modules from VerilogEmitter ────────────────────────────────
    let primitives: &[(&str, String)] = &[
        ("trit_neg.v",  VerilogEmitter::trit_neg().render()),
        ("trit_cons.v", VerilogEmitter::trit_cons().render()),
        ("trit_mul.v",  VerilogEmitter::trit_mul().render()),
        ("trit_add.v",  VerilogEmitter::trit_add().render()),
        ("trit_reg.v",  VerilogEmitter::trit_reg().render()),
        ("bet_alu.v",   VerilogEmitter::bet_alu().render()),
        ("sparse_matmul_4x4.v", VerilogEmitter::sparse_matmul(4).render()),
    ];
    for (name, src) in primitives {
        let path = out_dir.join(name);
        fs::write(&path, src).expect("Failed to write Verilog");
        println!("  wrote {}", path.display());
    }

    // ── ISA control path ────────────────────────────────────────────────────
    let isa = BetIsaEmitter::new();
    let isa_modules: &[(&str, String)] = &[
        ("bet_regfile.v",  isa.emit_register_file()),
        ("bet_pc.v",       isa.emit_program_counter()),
        ("bet_control.v",  isa.emit_control_unit()),
        ("bet_processor.v",isa.emit_top()),
    ];
    for (name, src) in isa_modules {
        let path = out_dir.join(name);
        fs::write(&path, src).expect("Failed to write Verilog");
        println!("  wrote {}", path.display());
    }

    // ── Yosys synthesis script ───────────────────────────────────────────────
    let ys_script = format!(
        "# Auto-generated Yosys synthesis script\n\
         # Run: yosys {}/synth_bet.ys\n\
         read_verilog {0}/trit_neg.v\n\
         read_verilog {0}/trit_cons.v\n\
         read_verilog {0}/trit_mul.v\n\
         read_verilog {0}/trit_add.v\n\
         read_verilog {0}/trit_reg.v\n\
         read_verilog {0}/bet_alu.v\n\
         read_verilog {0}/bet_regfile.v\n\
         read_verilog {0}/bet_pc.v\n\
         read_verilog {0}/bet_control.v\n\
         read_verilog {0}/bet_processor.v\n\
         hierarchy -check -top bet_processor\n\
         proc\nopt\ntechmap\nopt\n\
         stat\n\
         write_verilog -noattr {0}/synth_out.v\n",
        out_dir.display()
    );
    let ys_path = out_dir.join("synth_bet.ys");
    fs::write(&ys_path, &ys_script).expect("Failed to write Yosys script");
    println!("  wrote {}", ys_path.display());

    println!();
    println!("[hdl-synth] {} Verilog modules emitted.", primitives.len() + isa_modules.len());
    println!("[hdl-synth] To synthesise:");
    println!("  sudo apt install yosys   # if not installed");
    println!("  yosys {}", ys_path.display());

    if run_yosys {
        match Command::new("yosys").arg(ys_path.to_str().unwrap()).status() {
            Ok(s) if s.success() => println!("[hdl-synth] Yosys synthesis complete."),
            Ok(s) => eprintln!("[hdl-synth] Yosys exited with status {}", s),
            Err(_) => {
                eprintln!("[hdl-synth] yosys not found on PATH.");
                eprintln!("  Install: sudo apt install yosys");
                eprintln!("  Then re-run: ternlang hdl-synth --synth");
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ─── ternlang translate ───────────────────────────────────────────────────────

fn run_translate(input_file: Option<&std::path::Path>, language: &str, output_path: Option<&std::path::Path>) {
    let code = if let Some(path) = input_file {
        fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("[translate] Cannot read {:?}: {}", path, e);
            std::process::exit(1);
        })
    } else {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).expect("failed to read stdin");
        buf
    };

    // Translation logic mirrors trit_translate MCP tool
    let mut hold_zones = 0usize;
    let mut tern_lines: Vec<String> = Vec::new();
    let mut explanation_notes: Vec<String> = Vec::new();

    tern_lines.push(format!("// Translated from {} by ternlang translate — RFI-IRFOS v0.3.1", language));
    tern_lines.push(format!("// Original: {} lines", code.lines().count()));
    tern_lines.push(String::new());

    match language {
        "python" => {
            let mut in_chain = false;
            let mut has_else = false;
            for line in code.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("if ") && trimmed.ends_with(':') {
                    in_chain = true; has_else = false;
                    let cond = &trimmed[3..trimmed.len()-1];
                    tern_lines.push(format!("let condition: trit = cast({});", cond));
                    tern_lines.push("match condition {".to_string());
                    tern_lines.push("     1 => {".to_string());
                } else if trimmed.starts_with("elif ") && trimmed.ends_with(':') {
                    let cond = &trimmed[5..trimmed.len()-1];
                    tern_lines.push("    }".to_string());
                    tern_lines.push(format!("    // elif {}", cond));
                    tern_lines.push("    -1 => {".to_string());
                } else if trimmed == "else:" {
                    has_else = true;
                    tern_lines.push("    }".to_string());
                    tern_lines.push("     0 => {".to_string());
                } else if trimmed.starts_with("return ") {
                    let val = &trimmed[7..];
                    let trit_val = if val.contains("True") || val.contains("1") { "truth()" }
                                   else if val.contains("False") || val.contains("0") { "conflict()" }
                                   else { "hold()" };
                    tern_lines.push(format!("        return {};", trit_val));
                } else if !trimmed.is_empty() {
                    tern_lines.push(format!("        // {}", trimmed));
                }
            }
            if in_chain {
                if !has_else {
                    tern_lines.push("    }".to_string());
                    tern_lines.push("     0 => {".to_string());
                    tern_lines.push("        // HOLD ZONE — original had no else branch".to_string());
                    tern_lines.push("        return hold();".to_string());
                    hold_zones += 1;
                    explanation_notes.push("Injected tend arm: original if/elif had no else branch.".into());
                }
                tern_lines.push("    }".to_string());
                tern_lines.push("}".to_string());
            }
        }
        "sql" => {
            tern_lines.push("fn evaluate(signal: trit) -> trit {".to_string());
            let mut in_case = false;
            for line in code.lines() {
                let upper = line.trim().to_uppercase();
                if upper.starts_with("CASE") {
                    in_case = true; tern_lines.push("    match signal {".to_string());
                } else if upper.starts_with("WHEN") {
                    tern_lines.push(format!("         1 => {{ // WHEN {}", line.trim()[4..].trim()));
                } else if upper.starts_with("THEN") {
                    tern_lines.push(format!("            return truth(); // THEN {}", line.trim()[4..].trim()));
                    tern_lines.push("        }".to_string());
                } else if upper.starts_with("ELSE") {
                    tern_lines.push(format!("        -1 => {{ return conflict(); }} // ELSE {}", line.trim()[4..].trim()));
                } else if upper.starts_with("END") && in_case {
                    tern_lines.push("         0 => { return hold(); } // NULL/unknown → hold".to_string());
                    tern_lines.push("    }".to_string());
                    hold_zones += 1; in_case = false;
                    explanation_notes.push("Injected tend arm: SQL CASE had no NULL/unknown branch.".into());
                }
            }
            tern_lines.push("}".to_string());
        }
        "json_rules" => {
            match serde_json::from_str::<serde_json::Value>(&code) {
                Ok(rules) if rules.is_array() => {
                    tern_lines.push("fn apply_rules(signal: trit) -> trit {".to_string());
                    tern_lines.push("    match signal {".to_string());
                    for rule in rules.as_array().unwrap() {
                        let cond = rule["if"].as_str().unwrap_or("condition");
                        let then = rule["then"].as_str().unwrap_or("pass");
                        let tval = if then.contains("pass") || then.contains("allow") { "truth()" }
                                   else if then.contains("deny") || then.contains("block") { "conflict()" }
                                   else { "hold()" };
                        tern_lines.push(format!("         1 => {{ return {}; }} // if: {}", tval, cond));
                    }
                    tern_lines.push("         0 => { return hold(); } // HOLD ZONE".to_string());
                    tern_lines.push("        -1 => { return conflict(); }".to_string());
                    tern_lines.push("    }".to_string());
                    tern_lines.push("}".to_string());
                    hold_zones += 1;
                    explanation_notes.push("Injected tend arm for unmatched inputs.".into());
                }
                _ => {
                    eprintln!("[translate] json_rules: code must be a valid JSON array.");
                    std::process::exit(1);
                }
            }
        }
        other => {
            eprintln!("[translate] unsupported language '{}'. Use: python, sql, json_rules", other);
            std::process::exit(1);
        }
    }

    tern_lines.push(String::new());
    tern_lines.push("fn main() -> trit { return hold(); }".to_string());

    let tern_code = tern_lines.join("\n");

    // Print summary
    println!("{}", "══ TernTranslator ═══════════════════════════════════".bold());
    println!("  Language:    {}", language.cyan());
    println!("  Hold zones:  {}", hold_zones.to_string().green());
    if !explanation_notes.is_empty() {
        println!("  Notes:");
        for note in &explanation_notes {
            println!("    • {}", note);
        }
    }
    println!();

    if let Some(out) = output_path {
        fs::write(out, &tern_code).unwrap_or_else(|e| {
            eprintln!("[translate] Cannot write {:?}: {}", out, e);
            std::process::exit(1);
        });
        println!("{} Written to {}", "✓".green().bold(), out.display());
    } else {
        println!("{}", tern_code);
    }
}

// Phase III Milestone — hidden in plain sight
// ─── ternlang audit ──────────────────────────────────────────────────────────

fn run_audit(input_file: Option<&std::path::Path>, output_path: Option<&std::path::Path>, write_html: bool) {
    use serde_json::Value;

    // Read input JSON
    let raw = if let Some(path) = input_file {
        match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => { eprintln!("{} {}", "Error reading input file:".red(), e); std::process::exit(1); }
        }
    } else {
        let mut buf = String::new();
        use std::io::Read;
        std::io::stdin().read_to_string(&mut buf).expect("failed to read stdin");
        buf
    };

    let decisions: Vec<Value> = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => { eprintln!("{} {}", "Error parsing JSON:".red(), e); std::process::exit(1); }
    };

    if decisions.is_empty() {
        eprintln!("{}", "Error: decisions array is empty.".red());
        std::process::exit(1);
    }

    // Audit logic (mirrors trit_audit MCP tool)
    let binary_words  = ["yes","no","true","false","accept","reject","allow","deny",
                         "approve","block","pass","fail","correct","incorrect","valid","invalid"];
    let hold_words    = ["maybe","uncertain","unclear","depends","possibly","insufficient",
                         "further review","hold","tend","need more","context-dependent"];
    let high_conf_pat = ["definitely","certainly","always","never","absolutely","guaranteed","100%"];

    let mut affirm_n = 0usize;
    let mut tend_n   = 0usize;
    let mut reject_n = 0usize;
    let mut forced   = 0usize;
    let mut flagged: Vec<Value> = Vec::new();

    for d in &decisions {
        let output     = d["output"].as_str().unwrap_or("").to_lowercase();
        let confidence = d["confidence"].as_f64().unwrap_or(-1.0);
        let input_text = d["input"].as_str().unwrap_or("");

        let is_binary = binary_words.iter().any(|w| output.split_whitespace()
            .any(|tok| tok.trim_matches(|c: char| !c.is_alphabetic()) == *w));
        let has_hedge = hold_words.iter().any(|w| output.contains(*w));
        let is_forced = high_conf_pat.iter().any(|w| output.contains(*w))
            || (confidence >= 0.0 && confidence > 0.9);

        if has_hedge               { tend_n   += 1; }
        else if output.contains("reject") || output.contains("deny")
             || output.contains(" no ") || output.contains("false") { reject_n += 1; }
        else                       { affirm_n += 1; }

        if is_binary && !has_hedge && is_forced {
            forced += 1;
            flagged.push(serde_json::json!({
                "input":      input_text,
                "output":     d["output"].as_str().unwrap_or(""),
                "confidence": if confidence >= 0.0 { serde_json::json!(confidence) } else { Value::Null },
                "trit":       0,
                "reason":     "Forced high-confidence binary decision — tend zone may have been appropriate."
            }));
        }
    }

    let n = decisions.len() as f32;
    let binary_ratio = forced as f32 / n;
    let tend_ratio   = tend_n as f32 / n;
    let art13 = if tend_ratio   > 0.1 { "pass" } else { "warn" };
    let art14 = if binary_ratio < 0.3 { "pass" } else { "fail" };
    let cal_score = if binary_ratio < 0.2 { "good" } else if binary_ratio < 0.5 { "warn" } else { "poor" };

    let report = serde_json::json!({
        "total_decisions":      decisions.len(),
        "affirm_count":         affirm_n,
        "tend_count":           tend_n,
        "reject_count":         reject_n,
        "forced_binary_ratio":  (binary_ratio * 100.0).round() / 100.0,
        "tend_ratio":           (tend_ratio   * 100.0).round() / 100.0,
        "eu_ai_act": {
            "article_13_transparency":    art13,
            "article_14_human_oversight": art14,
            "note": "Heuristic assessment only — not a substitute for legal compliance review."
        },
        "calibration": {
            "score":  cal_score,
            "advice": if binary_ratio < 0.2 {
                "Calibration looks healthy — the agent is using the full ternary range."
            } else if binary_ratio < 0.5 {
                "Moderate binary habituation. Add confidence thresholds to route uncertain outputs to tend."
            } else {
                "High binary habituation. The agent is collapsing ambiguity instead of surfacing it."
            }
        },
        "flagged": flagged,
        "generated_by": "ternlang audit — RFI-IRFOS TernAudit Phase 13"
    });

    // Print summary to stdout
    println!("\n{}", "══ TernAudit Report ══════════════════════════════════".bold());
    println!("  Total decisions : {}", decisions.len());
    println!("  Affirm          : {}  Tend : {}  Reject : {}", affirm_n, tend_n, reject_n);
    println!("  Binary ratio    : {:.0}%   Calibration : {}", binary_ratio * 100.0,
             match cal_score { "good" => cal_score.green().to_string(), "warn" => cal_score.yellow().to_string(), _ => cal_score.red().to_string() });
    println!("  EU AI Act  Art.13 transparency : {}   Art.14 oversight : {}",
             if art13 == "pass" { "pass".green().to_string() } else { "warn".yellow().to_string() },
             if art14 == "pass" { "pass".green().to_string() } else { "fail".red().to_string() });
    if !flagged.is_empty() {
        println!("  {} flagged decisions (see report for details)", flagged.len().to_string().yellow());
    }

    // Write JSON report
    let json_path = output_path.map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("audit_report.json"));
    match fs::write(&json_path, serde_json::to_string_pretty(&report).unwrap_or_default()) {
        Ok(_)  => println!("\n  {} {}", "JSON report →".green(), json_path.display()),
        Err(e) => eprintln!("  {} {}", "Failed to write JSON report:".red(), e),
    }

    // Optionally write HTML report
    if write_html {
        let html_path = json_path.with_extension("html");
        let flagged_rows: String = report["flagged"].as_array().map(|f| f.iter().map(|d| {
            format!("<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                d["input"].as_str().unwrap_or(""),
                d["output"].as_str().unwrap_or(""),
                d["reason"].as_str().unwrap_or(""))
        }).collect::<Vec<_>>().join("\n")).unwrap_or_default();

        let html = format!(r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8">
<title>TernAudit Report — RFI-IRFOS</title>
<style>body{{font-family:sans-serif;max-width:900px;margin:40px auto;padding:0 20px}}
h1{{color:#2d2d2d}}table{{width:100%;border-collapse:collapse}}
th,td{{border:1px solid #ccc;padding:8px;text-align:left;vertical-align:top}}
th{{background:#f5f5f5}}.good{{color:green}}.warn{{color:orange}}.fail{{color:red}}</style></head>
<body><h1>TernAudit Report</h1>
<h2>Summary</h2><table>
<tr><th>Metric</th><th>Value</th></tr>
<tr><td>Total decisions</td><td>{total}</td></tr>
<tr><td>Affirm / Tend / Reject</td><td>{affirm} / {tend} / {reject}</td></tr>
<tr><td>Forced binary ratio</td><td>{ratio:.0}%</td></tr>
<tr><td>Calibration</td><td class="{cal}">{cal}</td></tr>
<tr><td>EU AI Act — Art. 13 transparency</td><td class="{art13}">{art13}</td></tr>
<tr><td>EU AI Act — Art. 14 human oversight</td><td class="{art14}">{art14}</td></tr>
</table>
<h2>Flagged Decisions ({flagged_n})</h2>
<table><tr><th>Input</th><th>Output</th><th>Reason</th></tr>{rows}</table>
<p style="color:#888;font-size:0.85em">Generated by ternlang audit · RFI-IRFOS TernAudit Phase 13 · ternlang.com</p>
</body></html>"#,
            total=decisions.len(), affirm=affirm_n, tend=tend_n, reject=reject_n,
            ratio=binary_ratio*100.0, cal=cal_score, art13=art13, art14=art14,
            flagged_n=report["flagged"].as_array().map(|f|f.len()).unwrap_or(0),
            rows=flagged_rows);

        match fs::write(&html_path, html) {
            Ok(_)  => println!("  {} {}", "HTML report →".green(), html_path.display()),
            Err(e) => eprintln!("  {} {}", "Failed to write HTML report:".red(), e),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test Runner — Phase IV: Native Ternary Tests
// ─────────────────────────────────────────────────────────────────────────────
fn run_tests(path: &std::path::PathBuf) {
    println!("{}", "Ternary Test Runner".bold().bright_blue());
    println!("Searching for *_test.tern in {:?}", path);
    println!("{}", "─".repeat(52));

    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let fpath = entry.path();
        if fpath.is_file() && fpath.extension().map_or(false, |ext| ext == "tern") {
            let fname = fpath.file_name().unwrap().to_str().unwrap();
            if fname.ends_with("_test.tern") {
                print!("test {} ... ", fname);
                use std::io::Write;
                std::io::stdout().flush().unwrap();

                let input = match fs::read_to_string(fpath) {
                    Ok(s) => s,
                    Err(_) => {
                        println!("{}", "ERROR (read)".red());
                        failed += 1;
                        continue;
                    }
                };

                let mut parser = Parser::new(&input);
                let mut emitter = BytecodeEmitter::new();

                match parser.parse_program() {
                    Ok(mut prog) => {
                        ternlang_core::ModuleResolver::stdlib_only().resolve(&mut prog);
                        emitter.emit_program(&prog);
                        // If there is a main function, we need to call it.
                        if prog.functions.iter().any(|f| f.name == "main") {
                            emitter.emit_entry_call("main");
                        }
                    }
                    Err(_) => {
                        // Fallback to statement-by-statement for simple scripts
                        let mut parser = Parser::new(&input);
                        while let Ok(stmt) = parser.parse_stmt() {
                            emitter.emit_stmt(&stmt);
                        }
                    }
                }

                let code = emitter.finalize();
                let mut vm = BetVm::new(code);
            emitter.register_agents(&mut vm);
                match vm.run() {
                    Ok(_) => {
                        let result = vm.peek_stack(); // tests should leave result on stack top
                        match result {
                            Some(Value::Trit(t)) => {
                                match t {
                                    ternlang_core::trit::Trit::Affirm => {
                                        println!("{}", "ok".green());
                                        passed += 1;
                                    }
                                    ternlang_core::trit::Trit::Tend => {
                                        println!("{}", "skipped".yellow());
                                        skipped += 1;
                                    }
                                    ternlang_core::trit::Trit::Reject => {
                                        println!("{}", "FAILED".red());
                                        failed += 1;
                                    }
                                }
                            }
                            _ => {
                                println!("{}", "FAILED (no trit return)".red());
                                failed += 1;
                            }
                        }
                    }
                    Err(e) => {
                        println!("{} ({})", "FAILED (vm)".red(), e);
                        failed += 1;
                    }
                }
            }
        }
    }

    println!("{}", "─".repeat(52));
    let status_str = if failed == 0 { "ok".green() } else { "FAILED".red() };
    println!("test result: {}. {} passed; {} failed; {} skipped", status_str, passed, failed, skipped);

    if failed > 0 {
        std::process::exit(1);
    }
}

