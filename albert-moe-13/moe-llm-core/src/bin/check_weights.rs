use candle_core::{Device, DType};
use candle_nn::VarMap;

fn main() -> anyhow::Result<()> {
    let mut varmap = VarMap::new();
    varmap.load("models/bible_ternary_v1.3.7.safetensors")?;
    for (name, var) in varmap.all_vars().iter() {
        println!("Key: {}, Shape: {:?}", name, var.shape());
    }
    Ok(())
}
