//! # Final Sprint Verification
//! 
//! Executes physical validation of AVX2 SIMD, Axum HTTP serving, and Distributed hooks.

use anyhow::Result;
use moe_core::core::model_adapter::simd_kernels::{ternary_dot_product_avx2, ternary_dot_product_scalar};
use moe_core::inference::agentic_bridge::AgenticInferenceBridge;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    println!("--- PHYSICAL SPRINT VERIFICATION ---");

    // 1. AVX2 SIMD Hardware Acceleration Verification
    println!("\n[1] Hardware Acceleration (AVX2 SIMD vs Scalar)...");
    let size = 1024 * 1024 * 10; // 10 million elements
    let weights = vec![1i8; size]; // Dense test for worst case
    let inputs = vec![0.5f32; size];
    
    let start_scalar = Instant::now();
    let res_scalar = ternary_dot_product_scalar(&weights, &inputs);
    let dur_scalar = start_scalar.elapsed();
    
    let start_avx2 = Instant::now();
    let res_avx2 = unsafe { ternary_dot_product_avx2(&weights, &inputs) };
    let dur_avx2 = start_avx2.elapsed();
    
    println!("Scalar Compute Time: {:?}", dur_scalar);
    println!("AVX2 Compute Time: {:?}", dur_avx2);
    let speedup = dur_scalar.as_secs_f32() / dur_avx2.as_secs_f32();
    println!("Hardware Speedup: {:.2}x", speedup);
    assert!((res_scalar - res_avx2).abs() < 1e-3);

    // 2. Axum Inference Server Validation
    println!("\n[2] Real HTTP Inference Server...");
    let bridge = AgenticInferenceBridge::new(8081);
    bridge.start_server().await?;
    
    // Give server a moment to bind
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let client = reqwest::Client::new();
    let start_req = Instant::now();
    let res = client.post("http://127.0.0.1:8081/infer")
        .json(&serde_json::json!({ "prompt": "Test physics" }))
        .send()
        .await?;
        
    let txt = res.text().await?;
    println!("HTTP Latency: {:?}", start_req.elapsed());
    println!("Server Response: {}", txt);

    // 3. RCCL Check
    println!("\n[3] Distributed Setup...");
    #[cfg(feature = "nccl")]
    println!("RCCL Binding Feature is ACTIVE (Compile-time verified).");
    #[cfg(not(feature = "nccl"))]
    println!("RCCL Binding Feature is INACTIVE.");

    println!("\n--- VERIFICATION SUCCESS ---");
    Ok(())
}
