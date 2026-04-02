use std::fmt;
use std::ops::{Add, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trit {
    NegOne = -1,   // logical -1
    Zero = 0,      // logical  0
    PosOne = 1,    // logical +1
}

impl From<i8> for Trit {
    fn from(val: i8) -> Self {
        match val {
            -1 => Trit::NegOne,
            0 => Trit::Zero,
            1 => Trit::PosOne,
            _ => panic!("Invalid trit value: {}", val),
        }
    }
}

impl fmt::Display for Trit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Trit::NegOne => write!(f, "-1"),
            Trit::Zero => write!(f, "0"),
            Trit::PosOne => write!(f, "+1"),
        }
    }
}

impl Neg for Trit {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Trit::NegOne => Trit::PosOne,
            Trit::Zero => Trit::Zero,
            Trit::PosOne => Trit::NegOne,
        }
    }
}

impl Add for Trit {
    type Output = (Self, Self); // (Sum, Carry)

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Trit::NegOne, Trit::NegOne) => (Trit::PosOne, Trit::NegOne),
            (Trit::NegOne, Trit::Zero) => (Trit::NegOne, Trit::Zero),
            (Trit::NegOne, Trit::PosOne) => (Trit::Zero, Trit::Zero),
            (Trit::Zero, Trit::NegOne) => (Trit::NegOne, Trit::Zero),
            (Trit::Zero, Trit::Zero) => (Trit::Zero, Trit::Zero),
            (Trit::Zero, Trit::PosOne) => (Trit::PosOne, Trit::Zero),
            (Trit::PosOne, Trit::NegOne) => (Trit::Zero, Trit::Zero),
            (Trit::PosOne, Trit::Zero) => (Trit::PosOne, Trit::Zero),
            (Trit::PosOne, Trit::PosOne) => (Trit::NegOne, Trit::PosOne),
        }
    }
}

impl Mul for Trit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Trit::Zero, _) | (_, Trit::Zero) => Trit::Zero,
            (Trit::PosOne, Trit::PosOne) | (Trit::NegOne, Trit::NegOne) => Trit::PosOne,
            (Trit::PosOne, Trit::NegOne) | (Trit::NegOne, Trit::PosOne) => Trit::NegOne,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negation() {
        assert_eq!(-Trit::NegOne, Trit::PosOne);
        assert_eq!(-Trit::Zero, Trit::Zero);
        assert_eq!(-Trit::PosOne, Trit::NegOne);
    }

    #[test]
    fn test_addition() {
        assert_eq!(Trit::NegOne + Trit::NegOne, (Trit::PosOne, Trit::NegOne));
        assert_eq!(Trit::NegOne + Trit::Zero, (Trit::NegOne, Trit::Zero));
        assert_eq!(Trit::NegOne + Trit::PosOne, (Trit::Zero, Trit::Zero));
        assert_eq!(Trit::Zero + Trit::NegOne, (Trit::NegOne, Trit::Zero));
        assert_eq!(Trit::Zero + Trit::Zero, (Trit::Zero, Trit::Zero));
        assert_eq!(Trit::Zero + Trit::PosOne, (Trit::PosOne, Trit::Zero));
        assert_eq!(Trit::PosOne + Trit::NegOne, (Trit::Zero, Trit::Zero));
        assert_eq!(Trit::PosOne + Trit::Zero, (Trit::PosOne, Trit::Zero));
        assert_eq!(Trit::PosOne + Trit::PosOne, (Trit::NegOne, Trit::PosOne));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(Trit::NegOne * Trit::NegOne, Trit::PosOne);
        assert_eq!(Trit::NegOne * Trit::Zero, Trit::Zero);
        assert_eq!(Trit::NegOne * Trit::PosOne, Trit::NegOne);
        assert_eq!(Trit::Zero * Trit::NegOne, Trit::Zero);
        assert_eq!(Trit::Zero * Trit::Zero, Trit::Zero);
        assert_eq!(Trit::Zero * Trit::PosOne, Trit::Zero);
        assert_eq!(Trit::PosOne * Trit::NegOne, Trit::NegOne);
        assert_eq!(Trit::PosOne * Trit::Zero, Trit::Zero);
        assert_eq!(Trit::PosOne * Trit::PosOne, Trit::PosOne);
    }
}
