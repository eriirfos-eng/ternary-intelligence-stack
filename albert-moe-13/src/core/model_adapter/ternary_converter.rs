//! # Ternary Converter
//! 
//! Floats to {-1, 0, +1} conversion pipeline for ingested weights.

pub struct TernaryShard {
    pub weights: Vec<i8>,
}

pub fn convert_to_ternary(weights: &[f32], threshold: f32) -> TernaryShard {
    let ternary_weights = weights.iter().map(|&w| {
        if w > threshold { 1 }
        else if w < -threshold { -1 }
        else { 0 }
    }).collect();
    
    TernaryShard { weights: ternary_weights }
}
