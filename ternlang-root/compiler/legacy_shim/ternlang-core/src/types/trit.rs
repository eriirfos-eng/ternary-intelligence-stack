/* --- RFI-IRFOS RUST NATIVE TRIT ---
 * Module: ternlang-core/src/types/trit.rs
 * Purpose: Rust implementation of the triadic primitive with 5-trit block packing.
 * License: LGPL-3.0 (Open Core)
 * Reference: Patent Pending A50296/2026
 */

use std::fmt;
use std::ops::{Add, Mul, Neg};
use serde::{Serialize, Deserialize};

use std::sync::LazyLock;

static ADD_TABLE: LazyLock<[[u8; 243]; 243]> = LazyLock::new(|| {
    let mut table = [[0u8; 243]; 243];
    for i in 0..243 {
        for j in 0..243 {
            let t1 = unpack_5_trits_local(i as u8);
            let t2 = unpack_5_trits_local(j as u8);
            let mut res = [Trit::Tend; 5];
            for k in 0..5 {
                let (sum, _) = t1[k] + t2[k];
                res[k] = sum;
            }
            table[i][j] = pack_5_trits_local(res);
        }
    }
    table
});

static CONSENSUS_TABLE: LazyLock<[[u8; 243]; 243]> = LazyLock::new(|| {
    let mut table = [[0u8; 243]; 243];
    for i in 0..243 {
        for j in 0..243 {
            let t1 = unpack_5_trits_local(i as u8);
            let t2 = unpack_5_trits_local(j as u8);
            let mut res = [Trit::Tend; 5];
            for k in 0..5 {
                res[k] = match (t1[k], t2[k]) {
                    (Trit::Affirm, Trit::Affirm) => Trit::Affirm,
                    (Trit::Reject, Trit::Reject) => Trit::Reject,
                    (Trit::Tend, x) => x,
                    (x, Trit::Tend) => x,
                    _ => Trit::Tend,
                };
            }
            table[i][j] = pack_5_trits_local(res);
        }
    }
    table
});

static NEG_TABLE: LazyLock<[u8; 243]> = LazyLock::new(|| {
    let mut table = [0u8; 243];
    for i in 0..243 {
        let t = unpack_5_trits_local(i as u8);
        let mut res = [Trit::Tend; 5];
        for k in 0..5 {
            res[k] = -t[k];
        }
        table[i] = pack_5_trits_local(res);
    }
    table
});

pub fn packed_neg(a: u8) -> u8 {
    NEG_TABLE[a as usize]
}

pub fn packed_add(a: u8, b: u8) -> u8 {
    ADD_TABLE[a as usize][b as usize]
}

pub fn packed_consensus(a: u8, b: u8) -> u8 {
    CONSENSUS_TABLE[a as usize][b as usize]
}

/// Unpack 5 trits from a u8 — internal helper (same as unpack_5_trits).
fn unpack_5_trits_local(val: u8) -> [Trit; 5] {
    unpack_5_trits(val)
}

/// Pack 5 trits into a u8 — internal helper (same as pack_5_trits).
fn pack_5_trits_local(trits: [Trit; 5]) -> u8 {
    pack_5_trits(trits)
}

/// The balanced ternary primitive — three states: Reject (-1), Tend (0), Affirm (+1).
///
/// Implements Add (returns Sum + Carry), Mul, Neg, and Display so that
/// downstream VM code can use `av + bv`, `av * bv`, `-av`, and `format!("{}", t)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i8)]
pub enum Trit {
    /// Logical -1 — conflict, negation
    Reject = -1,
    /// Logical  0 — hold, uncertainty
    Tend = 0,
    /// Logical +1 — truth, confirmation
    Affirm = 1,
}

impl Trit {
    /// Convert an i8 to a Trit, returning None for values outside {-1, 0, +1}.
    pub fn from_i8(val: i8) -> Option<Self> {
        match val {
            -1 => Some(Trit::Reject),
            0 => Some(Trit::Tend),
            1 => Some(Trit::Affirm),
            _ => None,
        }
    }

    /// Convert any i8 to a trit via saturation: positive → Affirm,
    /// negative → Reject, zero → Tend. No panic on overflow.
    pub fn from_i8_saturating(val: i8) -> Self {
        if val > 0 {
            Trit::Affirm
        } else if val < 0 {
            Trit::Reject
        } else {
            Trit::Tend
        }
    }
}

impl From<i8> for Trit {
    fn from(val: i8) -> Self {
        // VM-PANIC-001: saturate to the nearest trit instead of panicking.
        // Values outside {-1, 0, +1} (e.g. from arithmetic overflow in balanced-ternary
        // tensor ops) previously hard-crashed the VM. Saturate: positive -> Affirm,
        // negative -> Reject, zero -> Tend.
        Trit::from_i8_saturating(val)
    }
}

impl fmt::Display for Trit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trit::Reject => write!(f, "reject"),
            Trit::Tend => write!(f, "tend"),
            Trit::Affirm => write!(f, "affirm"),
        }
    }
}

impl Neg for Trit {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Trit::Reject => Trit::Affirm,
            Trit::Tend => Trit::Tend,
            Trit::Affirm => Trit::Reject,
        }
    }
}

/// Trit addition returns (sum, carry) in balanced ternary.
/// This implements the full ternary addition table so VM arithmetic
/// works with native trit-to-trit operations.
impl Add for Trit {
    type Output = (Self, Self); // (Sum, Carry)

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Trit::Reject, Trit::Reject) => (Trit::Affirm, Trit::Reject),
            (Trit::Reject, Trit::Tend) => (Trit::Reject, Trit::Tend),
            (Trit::Reject, Trit::Affirm) => (Trit::Tend, Trit::Tend),
            (Trit::Tend, Trit::Reject) => (Trit::Reject, Trit::Tend),
            (Trit::Tend, Trit::Tend) => (Trit::Tend, Trit::Tend),
            (Trit::Tend, Trit::Affirm) => (Trit::Affirm, Trit::Tend),
            (Trit::Affirm, Trit::Reject) => (Trit::Tend, Trit::Tend),
            (Trit::Affirm, Trit::Tend) => (Trit::Affirm, Trit::Tend),
            (Trit::Affirm, Trit::Affirm) => (Trit::Reject, Trit::Affirm),
        }
    }
}

impl Mul for Trit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Trit::Tend, _) | (_, Trit::Tend) => Trit::Tend,
            (Trit::Affirm, Trit::Affirm) | (Trit::Reject, Trit::Reject) => Trit::Affirm,
            (Trit::Affirm, Trit::Reject) | (Trit::Reject, Trit::Affirm) => Trit::Reject,
        }
    }
}

/// Packs 5 trits into a single u8 using a base-3 scheme.
/// 3^5 = 243, which fits in 0-255.
pub fn pack_5_trits(trits: [Trit; 5]) -> u8 {
    let mut val: u8 = 0;
    let mut multiplier: u8 = 1;
    for &t in &trits {
        // Offset trit values to 0, 1, 2 for unsigned packing.
        let trit_val = ((t as i8) + 1) as u8;
        val += trit_val * multiplier;
        multiplier *= 3;
    }
    val
}

/// Unpacks a u8 into 5 trits.
pub fn unpack_5_trits(mut val: u8) -> [Trit; 5] {
    let mut trits = [Trit::Tend; 5];
    for i in 0..5 {
        let trit_offset = (val % 3) as i8;
        trits[i] = Trit::from_i8(trit_offset - 1).unwrap();
        val /= 3;
    }
    trits
}

/// A packed block of 5 trits stored in exactly 1 byte.
/// 3^5 = 243, which maps into the 256-state capacity of a u8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TritBlock5(u8);

impl TritBlock5 {
    pub fn pack(trits: [Trit; 5]) -> Self {
        TritBlock5(pack_5_trits(trits))
    }

    pub fn unpack(&self) -> [Trit; 5] {
        unpack_5_trits(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_trit_block_packing_efficiency() {
        assert_eq!(mem::size_of::<TritBlock5>(), 1);

        let original = [Trit::Affirm, Trit::Reject, Trit::Tend, Trit::Affirm, Trit::Reject];
        let packed = TritBlock5::pack(original);
        let unpacked = packed.unpack();
        assert_eq!(original, unpacked);
    }

    #[test]
    fn test_negation() {
        assert_eq!(-Trit::Reject, Trit::Affirm);
        assert_eq!(-Trit::Tend, Trit::Tend);
        assert_eq!(-Trit::Affirm, Trit::Reject);
    }

    #[test]
    fn test_addition() {
        assert_eq!(Trit::Reject + Trit::Reject, (Trit::Affirm, Trit::Reject));
        assert_eq!(Trit::Reject + Trit::Tend, (Trit::Reject, Trit::Tend));
        assert_eq!(Trit::Reject + Trit::Affirm, (Trit::Tend, Trit::Tend));
        assert_eq!(Trit::Tend + Trit::Reject, (Trit::Reject, Trit::Tend));
        assert_eq!(Trit::Tend + Trit::Tend, (Trit::Tend, Trit::Tend));
        assert_eq!(Trit::Tend + Trit::Affirm, (Trit::Affirm, Trit::Tend));
        assert_eq!(Trit::Affirm + Trit::Reject, (Trit::Tend, Trit::Tend));
        assert_eq!(Trit::Affirm + Trit::Tend, (Trit::Affirm, Trit::Tend));
        assert_eq!(Trit::Affirm + Trit::Affirm, (Trit::Reject, Trit::Affirm));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(Trit::Reject * Trit::Reject, Trit::Affirm);
        assert_eq!(Trit::Reject * Trit::Tend, Trit::Tend);
        assert_eq!(Trit::Reject * Trit::Affirm, Trit::Reject);
        assert_eq!(Trit::Tend * Trit::Tend, Trit::Tend);
        assert_eq!(Trit::Affirm * Trit::Affirm, Trit::Affirm);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Trit::Affirm), "affirm");
        assert_eq!(format!("{}", Trit::Tend), "tend");
        assert_eq!(format!("{}", Trit::Reject), "reject");
    }
}
