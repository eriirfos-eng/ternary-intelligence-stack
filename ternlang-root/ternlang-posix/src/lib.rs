//! ternlang-posix: Triadic POSIX (T-POSIX) Standard.
//! 
//! Linux/Unix processes exit with 0 (Success) or !0 (Failure).
//! T-POSIX processes can yield 0 (Pending), indicating the process is 
//! not finished but is waiting for a formal consensus break.

pub mod process {
    #[derive(Debug, Clone, Copy)]
    pub enum ProcessSignal {
        Success = 1,
        Deliberating = 0, // State 0: The process has not failed, but is introspecting.
        Veto = -1,        // Hard security abort.
    }

    /// Replaces the legacy `exit(code)` with `triadic_yield(signal)`.
    pub fn triadic_yield(signal: ProcessSignal) {
        println!("T-POSIX: Process yielding signal {:?}", signal);
        match signal {
            ProcessSignal::Success => std::process::exit(0),
            ProcessSignal::Veto => std::process::exit(1),
            ProcessSignal::Deliberating => {
                // In a real T-OS, this would suspend the PID in the BET scheduler
                println!("T-POSIX: Suspending PID in State 0 equilibrium.");
            }
        }
    }
}
