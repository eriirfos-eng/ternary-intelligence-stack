use clap::{Parser as ClapParser, Subcommand};
use std::fs;
use std::path::PathBuf;
use ternlang_core::parser::Parser;
use ternlang_core::codegen::betbc::BytecodeEmitter;
use ternlang_core::vm::{BetVm, Value};

#[derive(ClapParser)]
#[command(name = "ternlang")]
#[command(about = "Ternlang CLI - Balanced Ternary Systems Language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile and run a .tern file
    Run {
        /// Path to the .tern file
        file: PathBuf,
    },
    /// Compile a .tern file to bytecode
    Build {
        /// Path to the .tern file
        file: PathBuf,
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { file } => {
            let input = fs::read_to_string(file).expect("Failed to read file");
            let mut parser = Parser::new(&input);
            let mut emitter = BytecodeEmitter::new();

            // Try parsing as a program first
            match parser.parse_program() {
                Ok(prog) => {
                    emitter.emit_program(&prog);
                }
                Err(e) => {
                    eprintln!("Parse program error: {:?}", e);
                    // Fallback: Reset and try parsing statements (for snippets without 'fn')
                    let mut parser = Parser::new(&input);
                    loop {
                        match parser.parse_stmt() {
                            Ok(stmt) => emitter.emit_stmt(&stmt),
                            Err(e) => {
                                if format!("{:?}", e).contains("EOF") {
                                    break;
                                }
                                eprintln!("Parse stmt error: {:?}", e);
                                break;
                            }
                        }
                    }
                }
            }

            let code = emitter.finalize();
            println!("Emitted {} bytes of bytecode", code.len());
            let mut vm = BetVm::new(code);
            
            match vm.run() {
                Ok(_) => {
                    println!("Program exited successfully.");
                    // Print registers for debugging
                    for i in 0..10 {
                        let val = vm.get_register(i);
                        match val {
                            Value::Trit(t) => println!("Reg {}: trit({})", i, t),
                            Value::Int(v) => println!("Reg {}: int({})", i, v),
                            Value::TensorRef(r) => println!("Reg {}: tensor_ref({})", i, r),
                        }
                    }
                }
                Err(e) => eprintln!("VM Error: {}", e),
            }
        }
        Commands::Build { file, output } => {
            let input = fs::read_to_string(file).expect("Failed to read file");
            let mut parser = Parser::new(&input);
            let mut emitter = BytecodeEmitter::new();

            match parser.parse_program() {
                Ok(prog) => emitter.emit_program(&prog),
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

            fs::write(out_path, code).expect("Failed to write bytecode");
            println!("Compiled to {:?}", file);
        }
    }
}
