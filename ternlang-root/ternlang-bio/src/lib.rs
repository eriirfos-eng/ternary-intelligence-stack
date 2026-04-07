//! ternlang-bio: Triadic Genomic Sequencing Standard (T-BIO).
//!
//! Binary genomics represents DNA as ATCG in 2 bits.
//! T-BIO natively represents Epigenetic Methylation as State 0 (TEND).

pub mod genome {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum EpigeneticState {
        Expressed = 1,   // Active Transcription
        Methylated = -1, // Active Repression
        Poised = 0,      // Equilibrium (State 0)
    }

    /// Evaluates whether a gene is ready for transcription.
    /// Does not proceed on binary "True", requires an explicit Affirm (+1) 
    /// from the environmental consensus engine.
    pub fn evaluate_expression(state: EpigeneticState, environmental_consensus: i8) -> i8 {
        match state {
            EpigeneticState::Expressed => 1,
            EpigeneticState::Methylated => -1,
            EpigeneticState::Poised => environmental_consensus, // Yields authority to MoE
        }
    }
}
