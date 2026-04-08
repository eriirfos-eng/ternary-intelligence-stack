//! Biological Computing & BCI (The Wetware Hijack)
//!
//! The human nervous system computes in triadic states: Excitatory (+1), 
//! Inhibitory (-1), and Resting Potential (0). 
//! `ternlang-bci` maps EEG/EMG neural signals directly to the `Trit` data type, 
//! creating the first "Biologically Native" programming language.

use ternlang_core::Trit;

/// A raw neural signal reading from a BCI sensor (e.g., microvolt potential).
pub struct NeuralSignal {
    pub microvolts: f64,
}

impl NeuralSignal {
    /// Translates an analog neural potential into a deterministic triadic state.
    /// Bypasses the massive latency of binary floating-point conversion.
    pub fn to_trit(&self) -> Trit {
        if self.microvolts > 15.0 {
            Trit::Affirm // Excitatory
        } else if self.microvolts < -15.0 {
            Trit::Reject // Inhibitory
        } else {
            Trit::Tend   // Resting Potential (HOLD)
        }
    }
}
