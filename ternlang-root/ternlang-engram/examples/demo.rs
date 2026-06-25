// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
//! Episodic memory walkthrough: remember a day, recall it, then forget.
//! Run with: cargo run -p ternlang-engram --example demo

use ternlang_engram::{EngramStore, HashEmbedder};

const HOUR: i64 = 3_600_000;

fn main() -> anyhow::Result<()> {
    let mut mem = EngramStore::new(384).with_embedder(Box::new(HashEmbedder::new(384)));

    // A morning of an agent's life. (caller supplies the clock — deterministic core)
    let t0 = 1_750_000_000_000;
    let day = [
        ("met Ana Diez about wellbeing research in Mendoza", 0.9, "people"),
        ("fixed the LayerNorm gradient wall in albert training", 0.85, "albert"),
        ("had coffee, the vienna weather was sunny", 0.3, "misc"),
        ("shipped the TernaryDense PR to keras upstream", 0.8, "ship"),
        ("audited Coin Master: pre-consent tracking confirmed", 0.95, "audit"),
    ];
    for (i, (text, salience, tag)) in day.iter().enumerate() {
        let t = t0 + i as i64 * HOUR;
        let id = mem.remember(*text, *salience, vec![tag.to_string()], "albert", t)?;
        // The id IS the timestamp — see ternlang_engram::unix_ms_to_stamp.
        println!("  [{}] id={id} | {text}", ternlang_engram::unix_ms_to_stamp(t));
    }

    println!("\nRecall — \"what did we find in the app audit?\" (6h later)");
    for r in mem.recall("what did we find in the app audit?", 3, t0 + 6 * HOUR)? {
        println!(
            "  [{:.3}] conf {:.2} | {:<11} | {}",
            r.score, r.confidence, r.age, r.content
        );
    }

    println!("\nTimeline — first two hours:");
    for e in mem.timeline(t0, t0 + HOUR) {
        println!("  t+{}h | {}", (e.t_ms - t0) / HOUR, e.content);
    }

    let s = mem.stats();
    println!(
        "\nStats — {} episodes, dim {}, avg salience {:.2}, avg sparsity {:.1}%, {} total recalls",
        s.episodes, s.dim, s.avg_importance, s.avg_sparsity * 100.0, s.total_access
    );

    println!("\nConsolidate one year later (decay + evict under 0.1):");
    let report = mem.consolidate(t0 + 365 * 24 * HOUR, 0.1, HOUR)?;
    println!("  decayed {}, evicted {}", report.decayed, report.evicted);
    println!("  surviving memories: {}", mem.len());

    Ok(())
}
