pub mod hf_loader;
pub mod weight_stream;
pub mod moe_parser;
pub mod expert_mapper;
pub mod ternary_converter;
pub mod shard_registry;

pub struct ModelTopology {
    pub is_moe: bool,
    pub expert_count: Option<usize>,
    pub layer_map: Vec<LayerDescriptor>,
}

pub struct LayerDescriptor {
    pub id: String,
    pub weights_shape: Vec<usize>,
}
