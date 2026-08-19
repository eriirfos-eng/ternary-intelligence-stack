#!/usr/bin/env python3
"""Generate README.md for all crates missing one."""
from pathlib import Path
import re

WORKSPACE = Path.home() / "projects" / "ternary-intelligence-stack"
CALLLAURA = Path.home() / "projects" / "call-laura"
BIFP = Path.home() / "projects" / "bifp" / "bifp-core"

EXTENDED_DESCRIPTIONS = {
    "moe-core": "Internal core engine for MoE-13 ternary inference and routing. Part of the Ternary Intelligence Stack.",
    "moe-data": "Data loading and preprocessing utilities for the MoE-13 ternary machine learning pipeline.",
    "moe-ddel": "DDEL (Dynamic Data Exchange Layer) for ternary runtime communication.",
    "moe-llm-core": "LLM integration core for the MoE-13 ternary inference engine.",
    "moe-platform": "Platform abstraction layer for MoE-13 deployments across hardware targets.",
    "moe-plugin-sdk": "Plugin SDK for extending MoE-13 with custom ternary modules and hooks.",
    "moe-runtime": "Runtime execution environment for MoE-13 ternary inference pipelines.",
    "moe-sdk": "High-level SDK for building ternary AI applications with MoE-13.",
    "moe-test": "Test harness and assertion utilities for MoE-13 components.",
    "moe-uril": "URIL (Unified Resource Interface Layer) for ternary resource management.",
    "moe-validation-suite": "Validation and benchmarking suite for MoE-13 model outputs.",
    "moe-compute": "Compute kernel abstractions for ternary tensor operations on CPU/GPU.",
    "token_train": "Token-level training utilities for the Ternary Intelligence Stack.",
    "ternlang-api": "Ternlang Institutional API Gateway — Core REST interface for the Ternary Intelligence Stack. Powers /api/trit_decide, /api/trit_vector, and the TaaS mesh.",
    "ternlang-astro": "Orbital and Satellite Routing Control. MoE-13 orchestrator and T-POSIX framework for non-binary, noisy space communication (GEO-LEO).",
    "ternlang-auth": "Triadic Decentralized Identity (T-DID). Eliminates binary All-or-Nothing authorization with ternary trust states.",
    "ternlang-bci": "Biologically Native Programming Middleware. Maps EEG/EMG neural signals directly to the TIS Trit data type for Brain-Computer Interfaces.",
    "ternlang-bio": "Triadic Bioinformatics and Genomics. Eliminates binary alignment errors in DNA sequence processing by utilizing the Tend (0) state for ambiguity.",
    "ternlang-bridge": "Trojan Horse Binary-to-Ternary Transpiler. Bridges legacy binary code into the BET VM.",
    "ternlang-cad": "Triadic Topology Optimization (T-CAD). Replaces binary Solid/Void voxel geometry with ternary spatial states.",
    "ternlang-cli": "Command-line interface for ternlang — run, build, sim, fmt, repl, and compat commands for the Balanced Ternary Execution VM.",
    "ternlang-codegen": "C transpiler backend for the Ternlang compiler — emits C source from the Ternlang AST for native cross-compilation targets.",
    "ternlang-compat": "Compatibility bridges for the ternary ecosystem — .tasm 9-trit assembler → BET bytecode, Owlet S-expression front-end.",
    "ternlang-compress": "LLM-to-ternary compression pipeline — quantize float models to {-1,0,+1}, build sparse zero-index, export .tern files for ternlang-ml inference.",
    "ternlang-consensus": "Triadic Consensus Mechanisms for the T-Fi Credit Ledger. Replaces Proof-of-Work/Proof-of-Stake with Proof-of-Ambiguity-Resolution.",
    "ternlang-contract": "Sovereign Smart Contracts for the T-Fi Economy. Self-pausing triadic execution layer bypassing the EVM.",
    "ternlang-core": "Compiler and VM for Ternlang — balanced ternary language with affirm/tend/reject trit semantics, @sparseskip codegen, and BET bytecode execution.",
    "ternlang-crypto": "Post-Quantum Ternary-Hardened Cryptography. Implements modulo-3 hashing and balanced-ternary permutation as a defense against Shor's Algorithm.",
    "ternlang-driver": "Universal Hardware Abstraction Layer (HAL) for the BET VM. Provides standardized device interfaces for ternary hardware.",
    "ternlang-edu": "The RFI-IRFOS Educational Cartel. Standardized curriculum tools for ternary literacy.",
    "ternlang-engram": "Ternary episodic memory — time-stamped, TritFloat-quantized autobiographical recall scored by relevance × recency × salience × frequency.",
    "ternlang-fs": "Triadic File System Abstraction. Treats file state as Read/Write/Tend instead of binary read/write.",
    "ternlang-gate": "Security gate and policy enforcement for the Ternary Intelligence Stack.",
    "ternlang-gfx": "Triadic Graphics Pipeline (T-GPU). Standardizes Depth-as-a-Trit for hardware-accelerated ternary rendering.",
    "ternlang-grid": "Triadic Energy Distribution Standard (T-GRID). Eliminates cascading grid failures with ternary redundancy.",
    "ternlang-harmony": "Triadic Harmony OS NDK Bindings. Native BET VM runtime integration for mobile and desktop environments.",
    "ternlang-hdl": "Verilog-2001 codegen for balanced ternary — BET processor primitives, sparse matmul array, FPGA simulation wrapper.",
    "ternlang-hft": "HFT Latency Arbitrage FPGA Integrator. Maps trading logic to physical triadic gates for zero-instruction-fetch execution.",
    "ternlang-lsp": "LSP 3.17 language server for ternlang — hover docs, code completion, and live diagnostics for .tern files.",
    "ternlang-mcp": "MCP server for ternlang — connects any AI agent to balanced ternary decision logic via trit_decide and friends.",
    "ternlang-mkl": "Ternary Math Kernels (cuTern). Sparse matrix primitives optimized for {-1, 0, +1} tensor operations.",
    "ternlang-ml": "Ternary ML inference kernels — TritFloat (confidence-native ternary float), TritFloatTensor, sparse_matmul (@sparseskip), TritMatrix, deliberation engine, action gate, and MLP.",
    "ternlang-moe": "Ternary Mixture-of-Experts orchestrator (MoE-13). Dual-key routing, triad synthesis, safety hard gate, three-tier memory, 13-agent deliberation.",
    "ternlang-net": "Triadic Networking Stack for the BET VM. HTTP/TCP reimagined with ternary packet semantics and Tend-state routing.",
    "ternlang-posix": "T-POSIX integration layer. Injects the triadic state directly into OS kernels for edge, drone, and RTOS environments.",
    "ternlang-qutrit": "Native 3-state Quantum (Qutrit) Compiler. Maps triadic {-1, 0, +1} logic states directly to qutrit spin levels (0, 1, 2) for quantum supremacy.",
    "ternlang-ros2": "Ternary-optimized DDS Middleware bridging ROS 2 and PX4. Enforces deterministic collision avoidance for autonomous drones.",
    "ternlang-runtime": "Distributed actor runtime for ternlang — TCP-based TernNode with remote spawn/send/await over newline-JSON protocol.",
    "ternlang-ruvector": "Ternary-optimized Sparse GEMV acceleration for Vector Databases (RuVector Bridge).",
    "ternlang-sec": "Ternary Security Enforcement. Hardened hardware-level safety gates and MoE-13 Veto logic for mission-critical authorization.",
    "ternlang-sql": "Native Ternary Graph Database. Demonstrates 50% performance yield via triadic relationship modeling.",
    "ternlang-swarm": "Triadic Kinematics for Autonomous Robotics. Binary hazard avoidance replaced with ternary state machines.",
    "ternlang-test": "Test harness and assertion utilities for ternlang programs — BET-VM test runner, trit assertions, and golden-file diffing.",
    "ternlang-time": "Triadic Network Time Protocol (T-NTP). Replaces legacy binary NTP jitter errors with ternary consensus timing.",
    "ternlang-translator": "Natural language to ternlang source translator. Converts human-readable specifications to .tern files.",
    "ternlang-tson": "TSON (Ternary JSON) parser. Native triadic state encoding with 30% higher density and proprietary MoE-13 compression.",
    "ternlang-ttp": "Triadic Transfer Protocol (TTP). HTTP is binary: 200 OK or 400/500 Error. TTP introduces the Tend state.",
    "ternlang-ui": "Triadic State Management for User Interfaces. UI state as Affirm/Tend/Reject instead of boolean flags.",
    "ternlang-wasm": "WebAssembly bindings for the Ternlang BET VM — runs real .tern programs in the browser.",
    "ternpkg": "Package manager for the ternlang ecosystem — ternlang.toml manifest, GitHub-backed registry, install/list/info commands.",
    "ternpkg-registry": "Centralized package registry backend. The npm/crates.io equivalent for the Ternary Intelligence Stack.",
    "reproducibility_verifier": "Albert-MoE-13 Checkpoint Verifier. Loads a saved .safetensors checkpoint and verifies integrity for reproducible ML experiments.",
    "lauras-core": "Laura Serna Gaviria's Human-AI Co-Evolution framework core library. Review, clarity, and triadic reasoning primitives.",
    "lauras-mcp": "MCP server implementation for the Laura review framework.",
    "lauras-api": "REST API wrapper for the Laura review framework.",
    "lauras-team": "Team coordination and orchestration for the Laura framework.",
    "bifp-core": "Bidirectional Instant Feedback Protocol (BIFP) — native ternary flag/consensus/teach-then-handoff primitives, with real (non-tag-encoded) trit persistence.",
}

def generate_readme(name, desc):
    return f"""# {name}

{desc}

## Installation

```toml
[dependencies]
{name} = "2.0.0"
```

## Usage

Add usage examples here.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.
"""

added = 0
for root in [WORKSPACE, CALLLAURA, BIFP]:
    if not root.exists():
        continue
    for toml in root.rglob("Cargo.toml"):
        try:
            txt = toml.read_text(errors="ignore").splitlines()
        except Exception:
            continue
        name = ""
        in_pkg = False
        for line in txt:
            s = line.strip()
            if s.startswith("["):
                in_pkg = s == "[package]"
                continue
            if not in_pkg:
                continue
            if m := re.match(r'name\s*=\s*"([^"]+)"', s):
                name = m.group(1)
                break
        if not name or name == "ternary-intelligence-stack":
            continue
        
        readme = toml.parent / "README.md"
        if not readme.exists():
            desc = EXTENDED_DESCRIPTIONS.get(name, f"{name} — part of the Ternary Intelligence Stack.")
            readme.write_text(generate_readme(name, desc))
            added += 1
            print(f"✓ Created README: {name}")

print(f"\nTotal READMEs created: {added}")
