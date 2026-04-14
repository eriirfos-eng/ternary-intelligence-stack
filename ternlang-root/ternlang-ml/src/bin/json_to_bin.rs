use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use ternlang_ml::coherence::{ModelData};

fn main() -> anyhow::Result<()> {
    let json_path = Path::new("../../llama32-1b.tern.json");
    let bin_path  = Path::new("../../llama32-1b.tern.bin");

    if !json_path.exists() {
        println!("Error: llama32-1b.tern.json not found in the root.");
        return Ok(());
    }

    println!("--- TIS Model Transmuter (JSON -> Binary) ---");
    println!("Loading JSON (this may take a minute for 1.2GB)...");
    let start = Instant::now();
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    let model: ModelData = serde_json::from_reader(reader)?;
    println!("JSON loaded in {:.2}s.", start.elapsed().as_secs_f32());

    println!("Saving Binary...");
    let start = Instant::now();
    model.save_bin(bin_path)?;
    println!("Binary saved to {:?} in {:.2}s.", bin_path, start.elapsed().as_secs_f32());

    Ok(())
}
