//! # albert-llb — Last Look Back Protocol
//!
//! Deterministic filesystem containment gate for sovereign AI agents.
//!
//! Moves safety from the linguistic layer (system prompts, alignment) to
//! the binary layer (Rust syscall boundary). Every mutation is an atomic
//! transaction: Gate 1 → [Snapshot] → Execute → Gate 2 → Commit/Rollback.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use albert_llb::{execute, types::{MutationRequest, Operation, StructuredIntent}};
//!
//! let result = execute(
//!     MutationRequest {
//!         path: "/tmp/output.txt".into(),
//!         operation: Operation::Create,
//!         intent: StructuredIntent {
//!             goal: "write analysis results".into(),
//!             justification: "user requested summary output".into(),
//!             fallback_if_rejected: "print to stdout instead".into(),
//!         },
//!     },
//!     |path| std::fs::write(path, b"results").map_err(albert_llb::LlbError::Io),
//! );
//! ```

pub mod blacklist;
pub mod error;
pub mod gate1;
pub mod gate2;
pub mod hitl;
pub mod runtime;
pub mod snapshot;
pub mod tier;
pub mod types;

pub use error::LlbError;
pub use runtime::execute;
pub use types::{LlbDecision, MutationRequest, Operation, StructuredIntent};
