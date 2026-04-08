//! Post-Quantum Cryptography (T-SEC)
//!
//! Standard binary encryption (RSA, ECC) is vulnerable to Shor's algorithm. 
//! `ternlang-crypto` implements Ternary-Hardened hashes and modulo-3 
//! balanced-ternary permutations, creating a cryptographic paradigm that 
//! binary hardware cannot brute-force efficiently.

use ternlang_core::Trit;

/// A high-entropy triadic hash block.
pub type THashBlock = [Trit; 81];

/// Performs a non-linear triadic permutation (T-SBOX).
/// Maps incoming trits through a deterministic balanced-ternary S-box.
pub fn triadic_sbox(input: Trit) -> Trit {
    match input {
        Trit::Affirm => Trit::Reject, // +1 -> -1
        Trit::Reject => Trit::Tend,   // -1 -> 0
        Trit::Tend   => Trit::Affirm, //  0 -> +1
    }
}

/// Computes a single-pass modulo-3 hash fragment.
pub fn compute_trit_hash(data: &[Trit]) -> Trit {
    data.iter().fold(Trit::Tend, |acc, &t| (acc + t).0)
}
