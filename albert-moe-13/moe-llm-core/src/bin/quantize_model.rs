use candle_core::{Device, Result, Tensor};
use moe_llm_core::model::packing::pack_tensor;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    let device = Device::Cpu;
    let weight_path = "albert-moe-13/models/bible_ternary_v1.3.6.safetensors";
    let output_path = "albert-moe-13/models/bible_ternary_v1.3.6.trit";

    println!("Loading weights from {}...", weight_path);
    let weights = candle_core::safetensors::load(weight_path, &device)?;
    
    let mut output_file = File::create(output_path).map_err(candle_core::Error::wrap)?;
    
    // Simple header: number of tensors
    output_file.write_all(&(weights.len() as u32).to_le_bytes()).map_err(candle_core::Error::wrap)?;

    for (name, tensor) in weights.iter() {
        println!("Quantizing tensor: {} ({:?})", name, tensor.dims());
        
        // Write name
        let name_bytes = name.as_bytes();
        output_file.write_all(&(name_bytes.len() as u32).to_le_bytes()).map_err(candle_core::Error::wrap)?;
        output_file.write_all(name_bytes).map_err(candle_core::Error::wrap)?;
        
        // Write shape
        let dims = tensor.dims();
        output_file.write_all(&(dims.len() as u32).to_le_bytes()).map_err(candle_core::Error::wrap)?;
        for dim in dims {
            output_file.write_all(&(*dim as u32).to_le_bytes()).map_err(candle_core::Error::wrap)?;
        }

        // Determine if we pack or write raw
        // In this prototype, we pack everything EXCEPT embedding, pos_embedding, and layernorm
        if name.contains("embed") || name.contains("pos_embed") || name.contains("ln") {
            println!("  Writing raw f32 (embedding/pos/ln)");
            output_file.write_all(&[0u8]).map_err(candle_core::Error::wrap)?; // Type: Raw
            let data = tensor.flatten_all()?.to_vec1::<f32>()?;
            for f in data {
                output_file.write_all(&f.to_le_bytes()).map_err(candle_core::Error::wrap)?;
            }
        } else {
            println!("  Writing packed ternary");
            output_file.write_all(&[1u8]).map_err(candle_core::Error::wrap)?; // Type: Packed
            let packed = pack_tensor(tensor, 0.05)?;
            output_file.write_all(&(packed.len() as u32).to_le_bytes()).map_err(candle_core::Error::wrap)?;
            output_file.write_all(&packed).map_err(candle_core::Error::wrap)?;
        }
    }

    println!("Success! Quantized model saved to {}", output_path);
    Ok(())
}
