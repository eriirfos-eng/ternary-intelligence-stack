use candle_core::safetensors::load;
use candle_core::Device;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let tensors = load(path, &Device::Cpu)?;
    for (name, tensor) in tensors.iter() {
        println!("Key: {}, Shape: {:?}", name, tensor.shape());
    }
    Ok(())
}
