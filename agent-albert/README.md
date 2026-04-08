# Albert-Code. 🌱🤖

<p align="center">
  <strong>The fast, terminal-first agentic CLI for Albert.</strong>
</p>

<p align="center">
  <img src="assets/clawd-hero.jpeg" alt="Albert" width="300" />
</p>

<p align="center">
  <strong>Harnessing the power of local inference with a full-blown tool harness.</strong>
</p>

---

## What is Albert-Code?

**Albert-Code** is the terminal-based evolution of **Albert**—a strictly local, offline-first AI gateway that writes its own skills and controls local integrations without sending your data to the cloud. 

By utilizing [Ollama](https://ollama.com/) for local inference, Albert-Code achieves lightning-fast responses. Unlike cloud-tethered assistants that monetize your telemetry, Albert relies on **SQLite Sovereign Memory** and **0-Trit Privacy Holds**. Your data never leaves your machine unless you explicitly authorize a team-scaling API call to the enterprise MoE-13 tier.

## Key Features

- **🚀 Terminal Speed & Autonomy**: Direct interaction with the model via Ollama. Albert runs 100% offline.
- **🧠 SQLite Sovereign Memory**: Your context, your data. Albert's memories are strictly local and un-scrapeable.
- **🛡 0-Trit Privacy Holds**: Hardware-level triadic logic guarantees that ambiguous, potentially sensitive commands trigger a `0` (HOLD) state, preventing accidental cloud telemetry or destructive local actions.
- **🔧 Functional Tool Harness**:
  - `execute_bash`: Full shell access on your local machine.
  - `create_file` / `read_file`: Seamless local file management.
  - `web_search`: Search the web via DuckDuckGo.
  - `retrieve_memory` / `log_memory`: Persistent, sovereign data vault.
- **🌐 Enterprise Escalation**: Once your local workflows depend on `.tern` scripts, seamlessly unlock complex team orchestration via the commercial TIS MoE-13 API tier.

---

## Inference & Ternary Logic

**Albert** uses a hybrid intelligence model:
- **LLM Inference (Local)**: General reasoning, text generation, and tool orchestration are handled locally via **Ollama** (`albert:latest`). This ensures privacy, speed, and offline capability for core agent functions.
- **Ternary Decision Gating (Remote API)**: Critical triadic decisions, expert MoE deliberation, and complex consensus logic are routed to the **Ternlang API** (`https://ternlang.com`).
  - `trit_decide`: High-fidelity triadic signal processing.
  - `moe_orchestrate`: Deliberation across the 13-expert Mixture-of-Experts (MoE) stack.
  
*Note: The Ternlang API is a specialized logic engine, not a general-purpose LLM endpoint.*

---

## Requirements & Installation

### 1. Prerequisites
- **Python 3.10+**
- **Ollama**: [Download and install Ollama](https://ollama.com/). You must pull the base model and create the Albert manifest:
  ```bash
  ollama pull qwen2.5:3b
  ollama create albert -f Modelfile
  ```
- **Ternlang API Key**: Set your `TERNLANG_API_KEY` environment variable to access remote expert deliberation.

### 2. Install Dependencies
Clone the repository and install the required Python packages:
```bash
cd agent-albert
pip install -r requirements.txt
```

---

## Quickstart

### 1. Ensure Ollama is running with the Albert model
```bash
ollama run albert
```

### 2. Launch Albert

**Option A: Terminal Chat (Albert-Code)**
```bash
python3 -m src.main chat
```

**Option B: Web UI (Albert-Agent)**
```bash
streamlit run albert.py
```

### 3. Ask Albert to act
```text
> create a file named albert_test.txt with content "Albert is active in the terminal."
> what is the current CPU usage?
> search the web for "latest open source LLM news"
```

---

## Repository Layout

```text
.
├── src/                                # Albert-Code Source
│   ├── main.py                         # CLI Entrypoint (use 'chat' command)
│   ├── query_engine.py                 # Ollama / Albert Brain & Loop
│   ├── tools.py                        # Functional Tool Implementations
│   ├── session_store.py                # Session Persistence
│   ├── models.py                       # Data Structures
│   └── ...
├── tests/                              # Verification Suite
├── assets/                             # Brand assets
└── README.md                           # This file
```

---
