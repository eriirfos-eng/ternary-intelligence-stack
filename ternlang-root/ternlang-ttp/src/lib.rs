//! ternlang-ttp: Triadic Transfer Protocol (TTP).
//! 
//! HTTP is binary: 200 OK or 400/500 Error.
//! TTP introduces Status 000 (Deliberating), allowing a server to 
//! maintain a stateful connection without "spinning" or "timing out".

pub mod ttp {
    pub enum TriadicStatus {
        Affirm200 = 1,  // Resource delivered
        Deliberate000 = 0, // Request accepted, consensus pending (State 0)
        Reject400 = -1, // Request vetoed
    }

    pub struct TtpResponse {
        pub status: TriadicStatus,
        pub body: String,
    }

    impl TtpResponse {
        /// Natively maps the status to the BET VM networking register.
        pub fn emit_signal(&self) -> i8 {
            match self.status {
                TriadicStatus::Affirm200 => 1,
                TriadicStatus::Deliberate000 => 0,
                TriadicStatus::Reject400 => -1,
            }
        }
    }
}
