use candle_core::{Device, Result, Shape, Tensor};
/// Quantizes an f32 tensor to 5-trit packed blocks (-1, 0, 1).
/// Uses the same logic as ternlang-core/src/types/trit.rs
pub fn pack_tensor(tensor: &Tensor, threshold: f32) -> Result<Vec<u8>> {
    let flat = tensor.flatten_all()?.to_vec1::<f32>()?;
    let mut packed = Vec::new();
    
    let mut i = 0;
    while i < flat.len() {
        let mut val: u16 = 0;
        let mut multiplier: u16 = 1;
        
        for j in 0..5 {
            let f = if i + j < flat.len() { flat[i + j] } else { 0.0 };
            let trit_val = if f > threshold { 2 } else if f < -threshold { 0 } else { 1 };
            val += trit_val * multiplier;
            multiplier *= 3;
        }
        
        packed.push(val as u8);
        i += 5;
    }
    
    Ok(packed)
}

/// Decompresses 5-trit packed blocks into an f32 tensor.
pub fn unpack_tensor(data: &[u8], shape: &Shape, device: &Device) -> Result<Tensor> {
    let num_elements = shape.elem_count();
    let mut flat = Vec::with_capacity(num_elements);
    
    for byte in data {
        let mut val = *byte as u16;
        for _ in 0..5 {
            if flat.len() < num_elements {
                let trit_offset = (val % 3) as i8;
                let f = (trit_offset - 1) as f32;
                flat.push(f);
                val /= 3;
            }
        }
    }
    
    Tensor::from_vec(flat, shape, device)
}
