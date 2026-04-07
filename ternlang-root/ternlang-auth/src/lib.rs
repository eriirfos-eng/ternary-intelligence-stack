//! ternlang-auth: Triadic Decentralized Identity (T-DID).
//!
//! Eliminates binary All-or-Nothing (Authorized/Unauthorized) tokens.
//! Introduces "Provisional Identity" for secure, audited agent actions.

pub mod did {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum AuthState {
        Authorized = 1,
        Provisional = 0, // State 0: Partial access while MoE-13 audit is pending.
        Unauthorized = -1,
    }

    pub struct IdentityToken {
        pub user_id: String,
        pub state: AuthState,
    }

    impl IdentityToken {
        pub fn new(id: &str) -> Self {
            IdentityToken { user_id: id.to_string(), state: AuthState::Provisional }
        }

        /// Evaluates if an action is permitted.
        /// State 0 permits read-only or low-mass operations while blocking 
        /// destructive hardware instructions.
        pub fn can_execute_opcode(&self, opcode_mass: u8) -> bool {
            match self.state {
                AuthState::Authorized => true,
                AuthState::Unauthorized => false,
                AuthState::Provisional => {
                    // State 0 logic: Only allow light-mass operations
                    opcode_mass < 50 
                }
            }
        }
    }
}
