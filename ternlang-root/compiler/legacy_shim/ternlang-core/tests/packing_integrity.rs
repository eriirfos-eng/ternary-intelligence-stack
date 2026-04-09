/* --- RFI-IRFOS RUST PACKING INTEGRITY AUDIT ---
 * Module: ternlang-core/tests/packing_integrity.rs
 * Purpose: Verify 100% round-trip integrity for 5-trit block packing.
 * License: LGPL-3.0
 * Reference: Patent Pending A50296/2026
 */

use ternlang_core::types::trit::{Trit, TritBlock5};
use std::time::Instant;
use rand::Rng;

#[test]
fn test_one_million_block_conversions() {
    let mut rng = rand::thread_rng();
    let total_blocks = 1_000_000;
    let mut successful_conversions = 0;
    
    let start_time = Instant::now();

    for _ in 0..total_blocks {
        let original_trits: [Trit; 5] = [
            Trit::from_i8(rng.gen_range(-1..=1)).unwrap(),
            Trit::from_i8(rng.gen_range(-1..=1)).unwrap(),
            Trit::from_i8(rng.gen_range(-1..=1)).unwrap(),
            Trit::from_i8(rng.gen_range(-1..=1)).unwrap(),
            Trit::from_i8(rng.gen_range(-1..=1)).unwrap(),
        ];
        
        // Logical verification: 5 trits into 1 byte.
        let packed = TritBlock5::pack(original_trits);
        let unpacked = packed.unpack();
        
        if original_trits == unpacked {
            successful_conversions += 1;
        }
    }
    
    let duration = start_time.elapsed();
    let nanoseconds_per_block = duration.as_nanos() as f64 / total_blocks as f64;

    // Logging nanosecond latency for audit trail (Reuters-clean output).
    println!("AUDIT_LOG: Total Blocks: {}, Nanoseconds/Block: {:.2}ns", total_blocks, nanoseconds_per_block);
    
    // Result: 100% round-trip integrity required.
    assert_eq!(successful_conversions, total_blocks, "Packing integrity failure: Not all blocks matched original trit values.");
}
