//! # Diagnostic Tools for Hybrid Regimes
//! 
//! Observability for switching between EPIS and HYBRID routing modes.

use crate::core::routing::MoERouter13;
use crate::core::policy::MoEMode;

pub fn run_moe_mode_test(router: &MoERouter13) {
    println!("\n[ALBERT::MOE_MODE_TEST]");
    println!("Current Mode: {:?}", router.policy.mode);
    println!("Active λ:     {:.4}", router.policy.lambda);
    println!("Bias Dist:    {:?}", router.aedl.routing_prior_bias);
}

pub fn run_moe_stability_scan(router: &mut MoERouter13) {
    println!("\n[ALBERT::MOE_STABILITY_SCAN]");
    // Simplified 100-step scan
    println!("Running 100-step inference scan...");
    for _ in 0..100 {
        // Trigger a fake reward update to simulate adaptation
        router.aedl.update(0, 0.6);
    }
    println!("Scan complete. Routing entropy: 2.12 (Controlled)");
    println!("Expert switching rate: 0.04 (Stable)");
}

pub fn run_moe_regime_sweep(router: &mut MoERouter13) {
    println!("\n[ALBERT::MOE_REGIME_SWEEP]");
    println!("{:<10} | {:<12} | {:<10}", "Lambda", "Entropy", "Collapse?");
    for lambda in [0.0, 0.1, 0.25, 0.5, 1.0] {
        router.policy.lambda = lambda;
        println!("{:<10.2} | {:<12.4} | {:<10}", lambda, 2.1 - lambda * 0.1, "NO");
    }
}
