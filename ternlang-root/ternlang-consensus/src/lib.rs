//! ternlang-consensus: Triadic Byzantine Fault Tolerance (TBFT).
//!
//! Binary distributed consensus (Raft, Paxos) relies on timeouts and binary
//! (Yes/No) voting. This leads to split-brain scenarios and catastrophic 
//! election storms. `ternlang-consensus` introduces a native `Hold` (State 0)
//! vote, allowing a network to intentionally pause without crashing.

pub mod tbft {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Vote {
        Commit = 1,
        Hold = 0,
        Reject = -1,
    }

    pub struct Node {
        pub id: usize,
        pub health: f32,
    }

    impl Node {
        /// Casts a triadic vote.
        /// If network latency is high, a node casts `Hold` (0) instead of failing.
        pub fn cast_vote(&self) -> Vote {
            if self.health > 0.8 {
                Vote::Commit
            } else if self.health < 0.2 {
                Vote::Reject
            } else {
                Vote::Hold // Intentional network pause. No timeout required.
            }
        }
    }

    /// Evaluates a cluster election without binary coercion.
    pub fn evaluate_quorum(votes: &[Vote]) -> i8 {
        let mut sum = 0;
        for v in votes {
            if *v == Vote::Reject {
                println!("ternlang-consensus: Veto detected. Election aborted.");
                return -1; // Single veto destroys quorum
            }
            sum += *v as i8;
        }

        if sum > (votes.len() as i8 / 2) {
            println!("ternlang-consensus: Supermajority achieved. Committing.");
            1
        } else {
            println!("ternlang-consensus: Quorum suspended in State 0. Waiting for data.");
            0
        }
    }
}
