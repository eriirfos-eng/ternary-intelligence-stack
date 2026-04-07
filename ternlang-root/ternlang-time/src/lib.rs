//! ternlang-time: Triadic Network Time Protocol (T-NTP).
//!
//! Replaces legacy binary NTP jitter errors with deterministic Temporal Hold.
//! Eliminates clock-skew artifacts in distributed database logs.

pub mod ntp {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum TimeSyncState {
        Synchronized = 1,
        TemporalHold = 0, // State 0: Clock is drifting, entering triadic equilibrium.
        Desynchronized = -1,
    }

    pub struct TriadicClock {
        pub last_sync_unix: u64,
        pub drift_ms: i32,
        pub state: TimeSyncState,
    }

    impl TriadicClock {
        pub fn new(unix: u64) -> Self {
            TriadicClock { last_sync_unix: unix, drift_ms: 0, state: TimeSyncState::Synchronized }
        }

        /// Evaluates sync status. 
        /// Unlike binary NTP, if drift is detected, we don't just "jump" the time.
        /// We enter State 0, alerting the BET VM to pause timestamp-dependent 
        /// IO until consensus is reached.
        pub fn sync_with_mesh(&mut self, average_mesh_ms: u64) -> TimeSyncState {
            let diff = (self.last_sync_unix as i64 - average_mesh_ms as i64).abs();
            
            if diff < 10 {
                self.state = TimeSyncState::Synchronized;
            } else if diff < 500 {
                println!("T-NTP: Drift detected ({}ms). Entering Temporal Hold (State 0).", diff);
                self.state = TimeSyncState::TemporalHold;
            } else {
                self.state = TimeSyncState::Desynchronized;
            }
            self.state
        }
    }
}
