//! Triadic Bioinformatics and Genomics
//!
//! Resolves binary alignment errors in DNA/RNA sequencing. Standard binary 
//! systems force an ambiguous nucleotide read into a false positive or negative. 
//! `ternlang-bio` utilizes `Trit::Tend` (0) to preserve ambiguity, ensuring 
//! deterministic precision in clinical trials and genomic research.

use ternlang_core::Trit;

pub struct NucleotideRead {
    pub confidence: f64,
    pub is_match: bool,
}

impl NucleotideRead {
    /// Evaluates a genomic sequence alignment.
    pub fn evaluate_alignment(&self) -> Trit {
        if self.confidence < 0.85 {
            // Ambiguous read preserved as State 0
            Trit::Tend 
        } else if self.is_match {
            Trit::Affirm
        } else {
            Trit::Reject
        }
    }
}
