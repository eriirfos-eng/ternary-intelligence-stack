//! Quantum Leap (Qutrit Supremacy)
//!
//! While legacy quantum systems use qubits (2-state), advanced physics is 
//! moving toward qutrits (3-state). `ternlang-qutrit` provides the 
//! native compiler for 3-state quantum circuits, mapping triadic 
//! {-1, 0, +1} directly to qutrit spin levels (0, 1, 2).

use ternlang_core::Trit;

/// Represents a 3-state Quantum Trit (Qutrit)
pub enum QutritState {
    Zero,      // Level 0 (Rest)
    One,       // Level 1 (Excitatory)
    Two,       // Level 2 (Super-Excitatory)
}

/// Compiles a triadic logic state into a physical qutrit energy level.
pub fn compile_to_quantum_level(t: Trit) -> QutritState {
    match t {
        Trit::Tend   => QutritState::Zero, // 0  -> Level 0
        Trit::Affirm => QutritState::One,  // +1 -> Level 1
        Trit::Reject => QutritState::Two,  // -1 -> Level 2
    }
}
