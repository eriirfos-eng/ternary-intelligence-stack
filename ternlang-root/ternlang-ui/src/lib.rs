//! ternlang-ui: Triadic State Management for User Interfaces.
//!
//! Modern web/app development requires manually managing three states with 
//! two booleans (e.g., `isLoading = true`, `hasError = false`). 
//! `ternlang-ui` provides a unified DOM renderer where a single trit 
//! natively represents UI state: -1 (Error), 0 (Loading/Pending), 1 (Render).

pub mod dom {
    #[derive(Debug, Clone, Copy)]
    #[repr(i8)]
    pub enum UIState {
        Render = 1,
        Pending = 0,
        Error = -1,
    }

    pub struct TriadicComponent {
        pub name: String,
        pub state: UIState,
    }

    impl TriadicComponent {
        pub fn new(name: &str) -> Self {
            TriadicComponent {
                name: name.to_string(),
                state: UIState::Pending, // Components initialize in State 0 naturally
            }
        }

        /// Renders the component by directly mapping the hardware trit to the DOM.
        pub fn render(&self) -> String {
            match self.state {
                UIState::Render => format!("<div class='data-ready'>{} content</div>", self.name),
                UIState::Pending => format!("<div class='spinner'>Loading {}...</div>", self.name),
                UIState::Error => format!("<div class='error-boundary'>Failed to load {}.</div>", self.name),
            }
        }

        /// In a React/Vue binary system, updating state involves two boolean mutations.
        /// In `ternlang-ui`, it is a single atomic hardware instruction.
        pub fn update_state(&mut self, signal: i8) {
            self.state = match signal {
                1 => UIState::Render,
                -1 => UIState::Error,
                _ => UIState::Pending,
            };
        }
    }
}
