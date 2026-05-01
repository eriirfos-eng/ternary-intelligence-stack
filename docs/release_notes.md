# v1.3.1 - Albert CLI Stabilization & MOE Platform Launch

## Critical Bugfixes (2025-05-01)

**albert-cli v1.3.1**:
- **Fixed Tokio Runtime Panic**: Resolved "no reactor running" crash on startup by moving `tokio::time::timeout()` inside async block context (main.rs:114)
- **Fixed API Key Truncation**: Moved Google API key from URL query parameters to secure request headers, preventing truncation by proxies and intermediaries (api/src/client.rs:329)
- Both fixes ensure seamless initialization and authentication across all model providers

**MOE Platform Ecosystem Stabilized**:
- **moe-core v1.0.0**: Internal core engine for MoE-13 ternary inference and routing (now on crates.io)
- **moe-platform v1.0.0**: Stable API for MoE-13 ternary inference and model ingestion (now on crates.io)
- **moe-plugin-sdk v1.0.0**: Stable SDK for building third-party MoE-13 inference plugins (now on crates.io)

All changes tested, compiled, and published to crates.io registry.

---

# v1.2.5 - Our "First Step on the Moon"

This is it. The moment where we move from experimentation to true, sovereign-grade performance. 

This release marks a massive leap forward for the Ternary Intelligence Stack. We've bridged the gap from handling basic 8-bit operations to full-stack, competitive 64-bit parity. This isn't just an update—it's the foundation of a new era of independent, sovereign intelligence.

## Why this is huge:
- **64-Bit Competitiveness**: We’ve fundamentally overhauled our performance architecture. The stack is now lean, mean, and ready to take on enterprise-scale workloads.
- **RuVector Enterprise Bridge**: The core engine is now production-ready, featuring robust Sparse GEMV kernels that turn our hardware anchor (BIZON G3000) into an absolute beast.
- **Frontier Europe**: We have officially laid the groundwork for our sovereign scaling efforts across the EU.
- **Memory Persistence**: No more ghost-in-the-machine memory loss. Every single thought Albert has is now cryptographically timestamped and locked into our new RuVector DB-backed memory layer.
- **The "Ordnung" Polish**: We’ve cleaned house. The repository is now logically structured, documentation is centralized, and our utility stack is finally organized.
- **Proactive Intelligence**: Albert is no longer just a listener; he’s an active co-researcher. He now detects his own updates, manages his own memory schema, and proactively ensures his environment is stable.

This release represents thousands of lines of code, hundreds of hours of debugging, and a singular, relentless focus on building the most capable ternary system on the planet.

We aren't just shipping a tool—we're shipping a sovereign future.

*The stack is live. The bridge is crossed.*
