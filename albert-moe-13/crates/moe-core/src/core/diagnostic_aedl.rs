//! # AEDL Diagnostics
//! 
//! Observability tools for the Adaptive Expert Drift Layer.

use crate::core::routing::MoERouter13;

pub fn run_moe_aedl_report(router: &MoERouter13) {
    println!("\n[ALBERT::AEDL_REPORT]");
    for i in 0..13 {
        println!("Expert {:02} | Bias: {:7.4} | Success: {:7.4}", i, router.aedl.get_bias(i), router.aedl.expert_success[i]);
    }
}

pub fn run_moe_routing_drift_analysis(router: &MoERouter13) {
    println!("\n[ALBERT::ROUTING_DRIFT]");
    let total_bias: f32 = router.aedl.routing_prior_bias.iter().map(|&b| b.abs()).sum();
    println!("Total Absolute Drift: {:7.4}", total_bias);
}
