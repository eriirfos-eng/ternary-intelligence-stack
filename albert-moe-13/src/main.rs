use albert_moe::cli::commands::CliDriver;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let driver = CliDriver::new();
    driver.execute(&args);
}
