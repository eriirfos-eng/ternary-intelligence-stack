# Ternlang Studio
## The Visual IDE & SDK for the Ternary Intelligence Stack

Welcome to **Ternlang Studio**, the flagship orchestration platform for the Ternary Intelligence Stack (TIS). This isn't just a code editor; it is a high-density, visual environment designed for architecting, simulating, and deploying neurosymbolic AI swarms.

By moving away from binary constraints (True/False) and embracing **Balanced Ternary Logic** (+1 Affirm, 0 Tend, -1 Reject), Ternlang Studio allows operators to model uncertainty, consensus, and conflict natively within their agent workflows.

---

## 🖥️ The IDE Experience

Ternlang Studio seamlessly bridges visual node-based programming with raw, code-first development. The workspace is divided into specialized modules to handle every phase of the engineering lifecycle:

*   📊 **Dash**: Your high-level command center. View active fleet metrics, recent compilations, and global workspace health.
*   **</> Editor**: A fully-featured code environment for writing raw `.tern` scripts. Includes syntax highlighting, linting for ternary exhaustiveness, and instant compilation to the BET VM.
*   🧬 **Lab (Canvas)**: The visual orchestration engine. Drag and drop Data Sources, LLM Bridges, and Logic Gates to draw your flow. Hit "Simulate" to watch data navigate the network.
*   🛰️ **Fleet**: Manage your local and remote `albert-agents`. Monitor telemetry, sync offline nodes, and assign compiled workflows to specific agents in the field.
*   📖 **Docs**: Integrated, offline-first documentation. Instantly search the Standard Library (STDLIB) or your authenticated Premium Modules without leaving the IDE.
*   🐞 **Tracer**: The visual debugger. A hardware-accelerated time-travel scrubber that lets you rewind simulations and inspect exact signal states (-1, 0, +1) at any millisecond.
*   📦 **Registry**: The GUI for `ternpkg`. Search, install, and manage modular dependencies and community crates directly from the TIS ecosystem.
*   ⚙️ **Config**: Your persistent, Tier-3 workspace settings. Manage your Fly.io API connections and your local Secrets Vault.

---

## 🧠 Core SDK Capabilities

Ternlang Studio is more than an IDE; it is a complete pipeline for turning visual logic into deployable software.

### 1. Build & Deploy Pipeline
Transition from visual design to production effortlessly. The Studio validates your drawn graph using automated checks for circular dependencies (**Kahn's algorithm**), isolated nodes, and schema mismatches. It then compiles the logic into a strict `.tern` manifest, allowing you to export code locally or deploy directly to the live **Fly.io proxy registry** (powered by a hardened Rust/Axum backend).

### 2. XML-Shielded LLM Bridge
Connect deterministic logic to probabilistic LLMs safely. When injecting payloads, the Bridge automatically wraps data in strict `<context>` XML tags to prevent prompt poisoning. Built-in **Token Safety circuit breakers** prevent massive credit wastage by aborting executions that exceed 80% of a model's context window.

### 3. The Ternary Multiverse Scrubber
Debug like never before. The Studio uses a GPU-accelerated HTML5 canvas overlay and a **2000-tick delta-logging ring buffer** to let you scrub backward and forward through a simulation. Watch signals pulse along the wires, branch into parallel multiverses, and observe transient states at 60FPS without lagging the browser.

### 4. High-Performance DOM Hydration
Working with the massive Premium Library? The Explorer uses **Just-In-Time (JIT) Lazy Hydration**, allowing you to seamlessly navigate directories with up to 30,000 files while keeping the browser's active DOM footprint well under 1,000 nodes by strictly pruning collapsed branches.

### 5. Local Secrets Vault
Never hardcode an API key again. The Config tab features a bidirectional, encrypted `localStorage` vault (`ternflow_secrets`). Save your OpenAI, Anthropic, Grok, or Custom Webhook keys once, and all your LLM Bridges will dynamically inherit them securely at runtime without leaking them into source files.

---

## 🚀 Getting Started

1.  **Configure**: Set your Endpoint URL and API Key in the **Config** tab.
2.  **Persist**: Ensure "Save Key Locally" is checked so your session persists across reloads.
3.  **Design**: Open the **Lab** to start dragging nodes, or open the **Editor** to write raw `.tern` code.
4.  **Deploy**: Click **Compile** to generate your payload, and **Deploy** to push it to the swarm.

---

*© 2026 RFI-IRFOS. All rights reserved. Technology must serve people, not their surveillance.*
