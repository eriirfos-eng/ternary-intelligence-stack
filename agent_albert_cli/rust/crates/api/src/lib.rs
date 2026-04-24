mod client;
mod error;
mod sse;
mod types;

pub use client::{TernlangClient, LlmProvider, read_base_url, resolve_startup_auth_source, resolve_auth_for_provider, detect_provider_and_model_from_env, OAuthConfig};
pub use error::ApiError;
pub use sse::{parse_frame, SseParser};
pub use types::*;
