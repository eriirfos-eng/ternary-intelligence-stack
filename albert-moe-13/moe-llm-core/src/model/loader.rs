use candle_core::{Device, Result, Shape, Tensor};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use super::packing::unpack_tensor;

pub struct TritLoader {
    tensors: HashMap<String, Tensor>,
}

impl TritLoader {
    pub fn load(path: &str, device: &Device) -> Result<Self> {
        let mut file = File::open(path).map_err(candle_core::Error::wrap)?;
        let mut tensors = HashMap::new();

        let mut u32_buf = [0u8; 4];
        file.read_exact(&mut u32_buf).map_err(candle_core::Error::wrap)?;
        let num_tensors = u32::from_le_bytes(u32_buf);

        for _ in 0..num_tensors {
            // Read name
            file.read_exact(&mut u32_buf).map_err(candle_core::Error::wrap)?;
            let name_len = u32::from_le_bytes(u32_buf) as usize;
            let mut name_buf = vec![0u8; name_len];
            file.read_exact(&mut name_buf).map_err(candle_core::Error::wrap)?;
            let name = String::from_utf8(name_buf).map_err(candle_core::Error::wrap)?;

            // Read shape
            file.read_exact(&mut u32_buf).map_err(candle_core::Error::wrap)?;
            let dims_len = u32::from_le_bytes(u32_buf) as usize;
            let mut dims = Vec::new();
            for _ in 0..dims_len {
                file.read_exact(&mut u32_buf).map_err(candle_core::Error::wrap)?;
                dims.push(u32::from_le_bytes(u32_buf) as usize);
            }
            let shape = Shape::from(dims);

            // Read type
            let mut type_buf = [0u8; 1];
            file.read_exact(&mut type_buf).map_err(candle_core::Error::wrap)?;
            let tensor_type = type_buf[0];

            let tensor = if tensor_type == 0 {
                // Raw f32
                let num_elements = shape.elem_count();
                let mut f32_data = vec![0.0f32; num_elements];
                let mut f32_buf = [0u8; 4];
                for i in 0..num_elements {
                    file.read_exact(&mut f32_buf).map_err(candle_core::Error::wrap)?;
                    f32_data[i] = f32::from_le_bytes(f32_buf);
                }
                Tensor::from_vec(f32_data, shape, device)?
            } else {
                // Packed ternary
                file.read_exact(&mut u32_buf).map_err(candle_core::Error::wrap)?;
                let packed_len = u32::from_le_bytes(u32_buf) as usize;
                let mut packed_data = vec![0u8; packed_len];
                file.read_exact(&mut packed_data).map_err(candle_core::Error::wrap)?;
                unpack_tensor(&packed_data, &shape, device)?
            };

            tensors.insert(name, tensor);
        }

        Ok(Self { tensors })
    }

    pub fn get(&self, name: &str) -> Option<&Tensor> {
        self.tensors.get(name)
    }

    pub fn into_tensors(self) -> HashMap<String, Tensor> {
        self.tensors
    }
}
