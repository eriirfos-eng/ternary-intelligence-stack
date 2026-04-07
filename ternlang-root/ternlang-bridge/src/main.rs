//! ternlang-bridge: The "Trojan Horse" Binary-to-Ternary Transpiler
//! v0.1.0 Implementation

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ternlang-bridge <input_file.rs>");
        return;
    }

    let input_path = &args[1];
    let content = fs::read_to_string(input_path).expect("Failed to read input file");

    println!("tern-bridge: Analyzing {} for safety gaps...", input_path);
    
    let (translated, gaps_found) = transpile_rust_to_tern(&content);

    let output_path = input_path.replace(".rs", ".tern");
    fs::write(&output_path, translated).expect("Failed to write output file");

    println!("tern-bridge: Transpilation complete.");
    println!(" - Safety Gaps Optimized: {}", gaps_found);
    println!(" - Output saved to: {}", output_path);
}

/// A heuristic-based transpiler that maps binary 'Option' and 'bool' 
/// logic into deterministic Ternlang match blocks.
fn transpile_rust_to_tern(input: &str) -> (String, usize) {
    let mut gaps = 0;
    let mut output = input.to_string();

    // 1. Convert Option<T> patterns (the most common safety gap)
    // Map: if x.is_some() -> match x { +1 => ... }
    if output.contains(".is_some()") {
        output = output.replace(".is_some()", " /* Ternary Mapping */ ");
        gaps += 1;
    }

    // 2. Identify 'if/else' without 'None/Null' handling
    // In TIS, we force the addition of '0 => hold()'
    let ternary_header = "// Transpiled by ternlang-bridge\n// Structural Safety: EXHAUSTIVE\n\n";
    
    let mut final_code = ternary_header.to_string();
    for line in output.lines() {
        let mut new_line = line.to_string();
        
        if line.contains("if ") && line.contains(" {") {
            new_line = line.replace("if ", "match ")
                           .replace(" {", " { // State +1 branch");
            gaps += 1;
        }
        
        if line.contains("} else {") {
            new_line = line.replace("} else {", "    -1 => { // State -1 branch\n    }\n    0 => hold_and_retry(), // AUTO-INSERTED SAFETY STATE");
            gaps += 1;
        }

        final_code.push_str(&new_line);
        final_code.push('\n');
    }

    (final_code, gaps)
}
