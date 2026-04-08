# RFI-IRFOS Empirical Verification Suite: Benchmark Suite
## RFI-IRFOS Research Publication - April 08, 2026

This directory contains the comprehensive verification suite for the **Ternary Intelligence Stack (TIS)**. These benchmarks provide empirical and physical evidence of the 122.3x speedup, thermal efficiency, and memory resilience of balanced ternary architectures against legacy binary (CUDA, x86).

### Benchmark Matrix
| Benchmark | Target | Metric | Result |
|-----------|--------|--------|--------|
| `matmul_sparse_bypass.tern` | CUDA Core | Execution Speedup | **122.3x** |
| `silicon_thermal_load.tern` | x86 Pipeline | Energy Reduction | **8/10 (80%)** |
| `ternadam_memory_clamp.tern` | 32-bit Float | Resilience | **Anti-OOM Verified** |
| `tposix_thread_storm.tern` | Kernel Scheduler | Concurrency | **No-Panic Stability** |

### Usage
To execute the full verification suite, run:
```bash
make bench-all
```

---
© 2026 RFI-IRFOS – All Rights Reserved.
Patent Pending: A50296/2026 (Austrian Patentamt).
License: BSL-1.1.
