# Ternlang Studio
## The Visual IDE & SDK for the Ternary Intelligence Stack

Welcome to **Ternlang Studio**, the flagship orchestration platform for the Ternary Intelligence Stack (TIS). This is a high-density, visual environment designed for architecting, simulating, and deploying neurosymbolic AI swarms using **Balanced Ternary Logic** (+1 Affirm, 0 Tend, -1 Reject).

By moving away from binary constraints (True/False), Ternlang Studio allows operators to model uncertainty, consensus, and conflict natively within their agent workflows.

---

## ⚡ Recent Technical Milestones: "Liquid Time"

We have recently evolved the Studio from a discrete step-based simulator into a **Continuous Temporal Flow** engine.

*   **Global Clock Integration**: The entire simulation is now anchored to a millisecond-accurate master playhead. Moving the playhead 1 unit forward moves every signal dot across the entire swarm by exactly 1 unit.
*   **Physical Path Interpolation**: Utilizing `SVGPath.getPointAtLength`, signal particles now glide smoothly along wire paths at a consistent 60FPS, eliminating the "jitter" of traditional logic simulators.
*   **DAW-Style Multiverse Timeline**: A professional-grade scrubber that allows for frame-perfect "scratching" through simulation history. Park a signal at any exact millimeter of a wire to inspect state transitions.
*   **Industrial UI Refactor**: A complete structural overhaul featuring resizable sidebars, telescoping bridge geometry, and a centered Industrial Dashboard (`#0f131a`) for a focused, high-precision operator experience.

---

## 🖥️ The IDE Experience

Ternlang Studio bridges visual node-based programming with raw, code-first development across specialized modules:

*   📊 **Dash**: High-level command center for fleet metrics, recent compilations, and global workspace health.
*   **</> Editor**: Fully-featured Monaco-based environment for writing `.tern` scripts with syntax highlighting and ternary exhaustiveness linting.
*   🧬 **Lab (Canvas)**: The visual orchestration engine. Drag-and-drop Data Sources, LLM Bridges, and Logic Gates. Hit **Simulate** to watch the "Liquid Time" engine in action.
*   🛰️ **Fleet**: Monitor telemetry from local and remote `albert-agents`, sync offline nodes, and assign compiled workflows.
*   📖 **Docs (TernWiki)**: Integrated, offline-first technical reference for the Standard Library (STDLIB) and Premium Modules.
*   🐞 **Tracer (Inspector)**: The visual debugger. A hardware-accelerated time-travel scrubber for inspecting exact signal states (-1, 0, +1) at any millisecond.
*   📦 **Registry**: GUI for `ternpkg`. Search, install, and manage modular dependencies and community crates.
*   ⚙️ **Config**: Centered Industrial Dashboard for managing Fly.io API connections and the **Local Secrets Vault**.
*   🔄 **TernTranslator**: Migrate logic **cross-domain** between binary architectures and balanced ternary.

---

## 🧠 Core SDK Capabilities

### 1. The Ternary Multiverse Scrubber
Debug like never before. The Studio uses a GPU-accelerated HTML5 canvas overlay and a **2000-tick delta-logging ring buffer** to let you scrub through a simulation. Watch signals pulse, branch into parallel multiverses, and observe transient states with zero lag.

### 2. Build & Deploy Pipeline
The Studio validates your graph using **Kahn's algorithm** for circular dependency checks. It compiles logic into a strict `.tern` manifest, allowing for local export or direct deployment to the **Fly.io proxy registry** via a hardened Rust/Axum backend.

### 3. XML-Shielded LLM Bridge
Connect deterministic logic to probabilistic LLMs safely. Payloads are wrapped in strict `<context>` XML tags to prevent prompt poisoning, with built-in **Token Safety circuit breakers** to prevent exceeding model context windows.

### 4. High-Performance DOM Hydration
The Premium Library Explorer uses **Just-In-Time (JIT) Lazy Hydration**, maintaining a minimal DOM footprint even when navigating directories containing over 30,000 files.

### 5. Local Secrets Vault
A bidirectional, encrypted `localStorage` vault (`ternflow_secrets`) manages API credentials for OpenAI, Anthropic, Google, and more. Bridges dynamically inherit these keys at runtime, ensuring security without leaking secrets into source files.

---

## 🚀 Getting Started

1.  **Configure**: Set your Endpoint URL and API Key in the **Config** tab.
2.  **Persist**: Enable "Save Key Locally" to persist your session across reloads.
3.  **Design**: Use the **Lab** to drag nodes and draw connections.
4.  **Simulate**: Use the **Timeline Scrubber** to visualize the signal flow.
5.  **Deploy**: Click **Compile** to generate your payload and push it to the swarm.

---
# TernFlow – Playground Capability Audit (v2 Upgrade)
Date: Sunday, April 19, 2026
Status: Production-Grade Engine Active

## 1. System Overview

*   **Architecture**: Event-driven asynchronous execution engine (`TernaryEngine v2`) using a `SignalQueue`.
*   **Execution Trigger**: `runSimulation()` now initializes an event loop that supports feedback loops and complex causal chains.
*   **Causal Model**: Move from simple topological sorting to a reactive, algebraic signal propagation model.

---

## 2. Node System

### 2.1 Supported Node Types

| Name | Purpose | Functional | Implementation Logic |
| :--- | :--- | :--- | :--- |
| **Agent** | Deterministic logic node | **YES** | Executes `.tern` code via browser WASM BET VM. |
| **LLM Bridge** | Probabilistic adapter | **YES** | Real API integration via `/api/run` proxy. Maps text to signal. |
| **Trit Gate** | Signal routing/consensus | **YES** | Uses `TernaryAlgebra.merge()` for multi-input resolution. |

### 2.2 Node Configuration (v2 Updates)

| Field | Status | Notes |
| :--- | :--- | :--- |
| **LLM Provider** | WORKING | Support for OpenAI, Anthropic, and Local endpoints. |
| **Prompt Template** | WORKING | Supports `{{input}}` injection of incoming ternary state. |
| **Algebraic Merge** | WORKING | Multi-input nodes automatically resolve conflicts using weighted mean logic. |

---

## 3. Edge System (Wiring)

### 3.1 Edge Logic (Algebraic)

| Feature | Status | Implementation |
| :--- | :--- | :--- |
| **Condition** | **WORKING** | Verified during event dispatch. |
| **Transform** | **WORKING** | Flip, Block, and Hold now affect both value AND confidence. |
| **Confidence Decay** | **WORKING** | Signals lose confidence based on wire priority and path length. |

---

## 4. Simulation Engine (v2)

### 4.1 Execution Model
*   **Event-Driven**: Nodes fire when they receive a signal, not just in fixed order.
*   **Cycle Support**: Feedback loops are fully supported with a `MAX_ENGINE_TICKS` safety limit.
*   **Signal Propagation**: Real-time signal objects: `{ val, conf, origin }`.

### 4.2 Visual Simulation
*   **Confidence Visualization**: Wire opacity and thickness scale dynamically with signal confidence.
*   **Glowing Particles**: 10px glowing dots track signal movement with value-matched colors.

---

## 5. Summary Table

| System Component | Status | Notes |
| :--- | :--- | :--- |
| **Node Execution** | WORKING | v2 Engine with event-loop support. |
| **Edge Logic** | WORKING | Algebraic transforms and confidence decay. |
| **Simulation** | WORKING | Real-time, async, supports feedback loops. |
| **LLM Bridge** | WORKING | Real API connectivity with ternary mapping. |
| **Archetypes** | WORKING | Fully valid, pre-wired causal templates. |

---
*© 2026 RFI-IRFOS. All rights reserved. Technology must serve people, not their surveillance.*
