# Ternlang Studio — The Visual SDK for the Ternary Intelligence Stack

Ternlang Studio is the flagship industrial IDE and orchestration platform for the **Ternary Intelligence Stack (TIS)**. It provides a high-density, visual environment for designing, simulating, and deploying agents based on a non-binary, state-based execution paradigm.

Unlike classical binary systems, Ternlang operates on **Balanced Ternary Logic**:
- **+1 (Affirm / Truth)**: Positive signal or consensus.
- **0 (Tend / Hold)**: Uncertainty, pending state, or neutral signal.
- **-1 (Reject / Conflict)**: Veto, error, or hard rejection.

## Core Architecture

Ternlang Studio is engineered for high-performance orchestration of massive agent swarms.

### JIT (Just-In-Time) DOM Hydration
To support industrial-scale projects, the Studio's Explorer utilizes a **Lazy Hydration** engine. It can manage directories containing up to 30,000 files while maintaining a footprint of fewer than 1000 active DOM nodes. 
- **On Collapse**: Child nodes are strictly pruned via `innerHTML = ""` to reclaim memory and prevent DOM thrashing.
- **On Expand**: Nodes are dynamically injected only for the visible branch, ensuring a sub-10ms UI response time even in deep hierarchies.

### Secure Fly.io API Proxy
Production deployments and premium asset delivery are handled via a hardened **Rust-based API (Axum)** hosted on Fly.io. This backend acts as a secure proxy for GitHub PAT-restricted assets and provides the registry gateway for Fleet synchronization, ensuring that sensitive credentials never reach the client-side execution context.

## The Local Secrets Vault

The Studio implements a bidirectional, persistent **Local Secrets Vault** managed via `localStorage` (key: `ternflow_secrets`). 

- **Provider-Agnostic**: Supports centralized management for OpenAI, Anthropic, Google, Grok, and Custom Webhooks.
- **Hot-Syncing**: Entering an API key in the Config UI immediately updates the global vault. Conversely, adding an LLM Bridge node automatically pulls the relevant key from the vault based on the selected protocol.
- **Zero-Leak Policy**: Keys are injected only at the moment of the proxy call and are never stored within the `.flow` or `.tern` source files.

## XML-Shielded LLM Bridge

The **LLM Bridge** acts as a probabilistic routing engine between deterministic ternary logic and stochastic large language models.

### Contextual Shielding
To prevent prompt poisoning and ensure structural integrity, injected downstream payloads from `runtime_buffer` are wrapped in strict XML tags:
```xml
<context>
  <data_payload>
    {{INJECTED_DATA}}
  </data_payload>
</context>
```

### Token Safety Circuit Breakers
Before execution, a heuristic token counter estimates the payload size. If the estimate exceeds **80% of the target model's context window** (e.g., 128k for GPT-4, 1M for Gemini 1.5), the Studio triggers a **Safety Halt**, logs an error to the Inspector, and emits a `-1` (Reject) signal to prevent execution failure or massive token wastage.

## The Ternary Multiverse Scrubber

The simulation engine features a high-performance **Time Travel** debugger powered by a hardware-accelerated HTML5 Canvas overlay (`#scrub-layer`).

- **Delta-Logging Ring Buffer**: Instead of expensive full-state snapshots, the engine records state deltas in a **2000-tick ring buffer**.
- **Hardware Acceleration**: The canvas overlay renders interpolated signal particles and "multiverse ghosts" (terminal outcomes) at 60FPS using `requestAnimationFrame`, bypassing the performance bottlenecks of DOM-based rendering.
- **Transient Scrubbing**: Developers can visually scrub through micro-second transit pulses, observing how signals diverge and converge across the graph in real-time.

## Build & Deploy Pipeline

Ternlang Studio provides a seamless transition from visual design to production runtime.

1. **Visual Canvas**: Design the agent swarm using the industrial drag-and-drop interface.
2. **Graph Validation**: Automated checks for circular dependencies (Kahn's algorithm), isolated nodes, and schema mismatches.
3. **.tern Code Compilation**: The visual graph is compiled into a unified `.tern` manifest, preserving the topological integrity of the swarm.
4. **Local Export**: Download the compiled `.tern` source for local execution via `ternlang-cli`.
5. **Direct API Deployment**: Publish directly to the Fly.io production registry. This registers the agent slug (e.g., `/api/agent/my-agent`), syncs it with your Fleet Dashboard, and activates the live endpoint for authenticated API consumers.

---
*© 2026 RFI-IRFOS. All rights reserved. Managed by the Ternary Intelligence Stack.*
