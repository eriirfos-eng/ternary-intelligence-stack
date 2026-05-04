//! # Benchmark Generation Orchestrator
//!
//! Compiles and executes all TIS benchmark bins to generate repeatable 
//! mathematical outputs for the technical reports.

use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("--- TIS Benchmark Suite Initiation ---");

    let targets = [
        ("bench_moe", "moe-core"),
        ("fair_benchmark", "moe-core"),
        ("zero_skip_bench", "moe-core"),
        ("sparsity_curve_bench", "moe-core"),
        ("real_task_bench", "moe-core"),
    ];

    let mut report = String::from("# Automated Benchmark Report\n\n");

    for (bin, package) in targets.iter() {
        println!("Running: {} from package {}...", bin, package);
        let output = Command::new("cargo")
            .args(&["run", "--release", "--bin", bin, "-p", package])
            .output()
            .expect("Failed to execute benchmark");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            report.push_str(&format!("## {}\n\n```\n{}\n```\n\n", bin, stdout));
        } else {
            eprintln!("Error running benchmark {}", bin);
        }
    }

    let mut file = File::create("projects/ternary-intelligence-stack/docs/AUTOMATED_BENCHMARKS.md").unwrap();
    file.write_all(report.as_bytes()).unwrap();
    println!("Benchmark report generated in docs/AUTOMATED_BENCHMARKS.md");
}
