use std::fmt;
use std::ops::{Add, Mul, Neg};
use serde::{Serialize, Deserialize};

use std::sync::LazyLock;

static ADD_TABLE: LazyLock<[[u8; 243]; 243]> = LazyLock::new(|| {
    let mut table = [[0u8; 243]; 243];
    for i in 0..243 {
        for j in 0..243 {
            let t1 = unpack_5_trits(i as u8);
            let t2 = unpack_5_trits(j as u8);
            let mut res = [Trit::Tend; 5];
            for k in 0..5 {
                let (sum, _) = t1[k] + t2[k];
                res[k] = sum;
            }
            table[i][j] = pack_5_trits(res);
        }
    }
    table
});

static CONSENSUS_TABLE: LazyLock<[[u8; 243]; 243]> = LazyLock::new(|| {
    let mut table = [[0u8; 243]; 243];
    for i in 0..243 {
        for j in 0..243 {
            let t1 = unpack_5_trits(i as u8);
            let t2 = unpack_5_trits(j as u8);
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
            table[i][j] = pack_5_trits(res);
        }
    }
    table
});

static NEG_TABLE: LazyLock<[u8; 243]> = LazyLock::new(|| {
    let mut table = [0u8; 243];
    for i in 0..243 {
        let t = unpack_5_trits(i as u8);
        let mut res = [Trit::Tend; 5];
        for k in 0..5 {
            res[k] = -t[k];
        }
        table[i] = pack_5_trits(res);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trit {
    Reject = -1,   // logical -1 — conflict, negation
    Tend   =  0,   // logical  0 — hold, uncertainty
    Affirm =  1,   // logical +1 — truth, confirmation
}

impl From<i8> for Trit {
    fn from(val: i8) -> Self {
        // VM-PANIC-001: saturate to the nearest trit instead of panicking.
        // Values outside {-1, 0, +1} (e.g. from arithmetic overflow in balanced-ternary
        // tensor ops) previously hard-crashed the VM. Saturate: positive → Affirm,
        // negative → Reject, zero → Tend.
        if val > 0 { Trit::Affirm }
        else if val < 0 { Trit::Reject }
        else { Trit::Tend }
    }
}

impl fmt::Display for Trit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trit::Reject => write!(f, "reject"),
            Trit::Tend   => write!(f, "tend"),
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
        let t_val = match t {
            Trit::Reject => 0,
            Trit::Tend   => 1,
            Trit::Affirm => 2,
        };
        val += t_val * multiplier;
        multiplier *= 3;
    }
    val
}

/// Unpacks a u8 into 5 trits.
pub fn unpack_5_trits(mut val: u8) -> [Trit; 5] {
    let mut trits = [Trit::Tend; 5];
    for i in 0..5 {
        let t_val = val % 3;
        trits[i] = match t_val {
            0 => Trit::Reject,
            1 => Trit::Tend,
            2 => Trit::Affirm,
            _ => unreachable!(),
        };
        val /= 3;
    }
    trits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packing() {
        let test_cases = [
            [Trit::Reject; 5],
            [Trit::Tend; 5],
            [Trit::Affirm; 5],
            [Trit::Reject, Trit::Tend, Trit::Affirm, Trit::Reject, Trit::Tend],
            [Trit::Affirm, Trit::Reject, Trit::Tend, Trit::Affirm, Trit::Reject],
        ];

        for &original in &test_cases {
            let packed = pack_5_trits(original);
            let unpacked = unpack_5_trits(packed);
            assert_eq!(original, unpacked);
        }
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
        assert_eq!(Trit::Tend * Trit::Reject, Trit::Tend);
        assert_eq!(Trit::Tend * Trit::Tend, Trit::Tend);
        assert_eq!(Trit::Tend * Trit::Affirm, Trit::Tend);
        assert_eq!(Trit::Affirm * Trit::Reject, Trit::Reject);
        assert_eq!(Trit::Affirm * Trit::Tend, Trit::Tend);
        assert_eq!(Trit::Affirm * Trit::Affirm, Trit::Affirm);
    }
}
