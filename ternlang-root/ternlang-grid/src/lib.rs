//! ternlang-grid: Triadic Energy Distribution Standard (T-GRID).
//!
//! Eliminates cascading grid failures by introducing the Phase-Hold (0) state,
//! preventing binary relays from blindly tripping under unexpected load.

pub mod distribution {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum RelayState {
        Closed = 1,  // Power Flowing
        Tripped = -1, // Critical Disconnect
        PhaseHold = 0, // Throttled equilibrium (State 0)
    }

    /// Evaluates distribution based on local load and grid consensus.
    /// Forces a mandatory Hold (0) before a catastrophic trip (-1) can occur.
    pub fn evaluate_relay(load_percent: f32, grid_consensus: i8) -> RelayState {
        if load_percent < 90.0 {
            RelayState::Closed
        } else if load_percent >= 90.0 && grid_consensus == 1 {
            // High load, but grid is stable enough to absorb
            RelayState::PhaseHold 
        } else {
            // Absolute failure threshold
            RelayState::Tripped
        }
    }
}
