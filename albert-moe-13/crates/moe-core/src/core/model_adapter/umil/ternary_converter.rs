use crate::core::model_adapter::umil::cmir::UnifiedWeightTensor;

pub struct TernaryConverterV2;

impl TernaryConverterV2 {
    pub fn convert(uwt: UnifiedWeightTensor, threshold: f32) -> Vec<i8> {
        uwt.data
            .iter()
            .map(|&val| {
                if val > threshold {
                    1
                } else if val < -threshold {
                    -1
                } else {
                    0
                }
            })
            .collect()
    }
}
