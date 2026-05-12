pub mod config;
pub mod embedding;
pub mod attention;
pub mod mlp;
pub mod ste;
pub mod ternary_linear;
pub mod transformer;
pub mod moe;
pub mod traffic_light;
pub mod packing;
pub mod loader;
pub mod cuda_matmul; // GPU backend sketch (TRL 3 → roadmap) — see module docs
pub use transformer::Transformer;
pub use config::TransformerConfig;
pub use moe::{
    clear_routing_capture, take_routing_capture,
    clear_entropy_capture, take_entropy_capture,
    clear_lb_capture, take_lb_capture,
    clear_tlight_capture, take_tlight_capture,
    clear_div_capture, take_div_capture, take_div_log_capture, take_div_f32_log_capture,
    enter_eval_mode,
    set_div_enabled,
    set_gate_diversity_scale,
};
