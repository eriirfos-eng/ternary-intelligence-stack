/* --- RFI-IRFOS INTEGRATION TEST ---
 * Module: ternlang-core/tests/integration_trit_lifecycle.rs
 * Purpose: Verify the full trit lifecycle — creation, packing, consensus,
 *          and round-trip through the BET VM value stack.
 * License: LGPL-3.0
 */

use ternlang_core::{pack_5_trits, pack_trits, unpack_5_trits, unpack_trits, Trit, TritBlock5};

#[test]
fn test_trit_block5_pack_unpack_roundtrip() {
    let test_cases = [
        [Trit::Reject, Trit::Reject, Trit::Reject, Trit::Reject, Trit::Reject],
        [Trit::Affirm, Trit::Affirm, Trit::Affirm, Trit::Affirm, Trit::Affirm],
        [Trit::Tend, Trit::Tend, Trit::Tend, Trit::Tend, Trit::Tend],
        [Trit::Affirm, Trit::Reject, Trit::Tend, Trit::Affirm, Trit::Reject],
        [Trit::Reject, Trit::Affirm, Trit::Tend, Trit::Reject, Trit::Affirm],
    ];

    for original in &test_cases {
        let packed = TritBlock5::pack(*original);
        let unpacked = packed.unpack();
        assert_eq!(*original, unpacked, "Round-trip failed for {:?}", original);
    }
}

#[test]
fn test_trit_block5_size_is_one_byte() {
    use std::mem;
    assert_eq!(
        mem::size_of::<TritBlock5>(),
        1,
        "TritBlock5 must be exactly 1 byte"
    );
}

#[test]
fn test_pack_5_trits_consistency() {
    let trits = [Trit::Affirm, Trit::Tend, Trit::Reject, Trit::Affirm, Trit::Tend];
    let packed = pack_5_trits(trits);
    let unpacked = unpack_5_trits(packed);
    assert_eq!(trits, unpacked);
}

#[test]
fn test_unified_pack_unpack_roundtrip() {
    let trits = vec![Trit::Affirm, Trit::Tend, Trit::Reject, Trit::Affirm, Trit::Reject];
    let packed = pack_trits(&trits);
    let round_tripped = unpack_trits(&packed, trits.len()).unwrap();
    assert_eq!(trits, round_tripped);
}

#[test]
fn test_large_trit_vector_roundtrip() {
    let trits: Vec<Trit> = (0..250)
        .map(|i| match i % 3 {
            0 => Trit::Affirm,
            1 => Trit::Tend,
            _ => Trit::Reject,
        })
        .collect();
    let packed = pack_trits(&trits);
    let round_tripped = unpack_trits(&packed, trits.len()).unwrap();
    assert_eq!(trits, round_tripped);
}
