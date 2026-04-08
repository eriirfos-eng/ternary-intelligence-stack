/* --- RFI-IRFOS RUST NATIVE TRIT ---
 * Module: ternlang-core/src/types/trit.rs
 * Purpose: Rust implementation of the triadic primitive with 5-trit block packing.
 * License: LGPL-3.0 (Open Core)
 * Reference: Patent Pending A50296/2026
 */

use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum Trit {
    Reject = -1,
    Tend = 0,
    Affirm = 1,
}

impl Trit {
    pub fn from_i8(val: i8) -> Option<Self> {
        match val {
            -1 => Some(Trit::Reject),
            0  => Some(Trit::Tend),
            1  => Some(Trit::Affirm),
            _  => None,
        }
    }
}

/// A packed block of 5 trits stored in exactly 1 byte.
/// 3^5 = 243, which maps into the 256-state capacity of a u8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TritBlock5(u8);

impl TritBlock5 {
    pub fn pack(trits: [Trit; 5]) -> Self {
        let mut val: u16 = 0;
        let mut multiplier: u16 = 1;
        
        for t in trits.iter() {
            // Offset trit values to 0, 1, 2 for unsigned packing.
            let trit_val = ((*t as i8) + 1) as u16;
            val += trit_val * multiplier;
            multiplier *= 3;
        }
        
        TritBlock5(val as u8)
    }

    pub fn unpack(&self) -> [Trit; 5] {
        let mut trits = [Trit::Tend; 5];
        let mut val = self.0 as u16;
        
        for i in 0..5 {
            let trit_offset = (val % 3) as i8;
            trits[i] = Trit::from_i8(trit_offset - 1).unwrap();
            val /= 3;
        }
        
        trits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trit_block_packing_efficiency() {
        // Physical memory verification:
        // mem::size_of::<TritBlock5>() should be 1 byte.
        assert_eq!(mem::size_of::<TritBlock5>(), 1);
        
        // Logical verification: 5 trits into 1 byte.
        let original = [Trit::Affirm, Trit::Reject, Trit::Tend, Trit::Affirm, Trit::Reject];
        let packed = TritBlock5::pack(original);
        let unpacked = packed.unpack();
        
        assert_eq!(original, unpacked);
    }
}
