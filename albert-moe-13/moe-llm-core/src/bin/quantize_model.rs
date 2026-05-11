use candle_core::{Device, Result};
use moe_llm_core::model::packing::pack_tensor;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let weight_path = args.get(1).map(String::as_str)
        .unwrap_or("albert-moe-13/models/albert_v3.0.safetensors");
    let output_path = args.get(2).map(String::as_str)
        .unwrap_or("albert-moe-13/models/albert_v3.0.trit");

    let device = Device::Cpu;
    println!("Loading weights from {}...", weight_path);
    let weights = candle_core::safetensors::load(weight_path, &device)?;

    let mut output_file = File::create(output_path).map_err(candle_core::Error::wrap)?;

    // Header: number of tensors
    output_file.write_all(&(weights.len() as u32).to_le_bytes()).map_err(candle_core::Error::wrap)?;

    for (name, tensor) in weights.iter() {
        println!("Quantizing tensor: {} ({:?})", name, tensor.dims());

        let name_bytes = name.as_bytes();
        output_file.write_all(&(name_bytes.len() as u32).to_le_bytes()).map_err(candle_core::Error::wrap)?;
        output_file.write_all(name_bytes).map_err(candle_core::Error::wrap)?;

        let dims = tensor.dims();
        output_file.write_all(&(dims.len() as u32).to_le_bytes()).map_err(candle_core::Error::wrap)?;
        for dim in dims {
            output_file.write_all(&(*dim as u32).to_le_bytes()).map_err(candle_core::Error::wrap)?;
        }

        // Embeddings, pos embeddings, and layer norms stay in raw f32
        if name.contains("embed") || name.contains("pos_embed") || name.contains("ln") {
            println!("  Writing raw f32 (embedding/pos/ln)");
            output_file.write_all(&[0u8]).map_err(candle_core::Error::wrap)?;
            let data = tensor.flatten_all()?.to_vec1::<f32>()?;
            for f in data {
                output_file.write_all(&f.to_le_bytes()).map_err(candle_core::Error::wrap)?;
            }
        } else {
            println!("  Writing packed ternary");
            output_file.write_all(&[1u8]).map_err(candle_core::Error::wrap)?;
            let packed = pack_tensor(tensor, 0.05)?;
            output_file.write_all(&(packed.len() as u32).to_le_bytes()).map_err(candle_core::Error::wrap)?;
            output_file.write_all(&packed).map_err(candle_core::Error::wrap)?;
        }
    }

    println!("Success! Quantized model saved to {}", output_path);
    Ok(())
}
