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

**Albert-Code** is the terminal-based evolution of **Albert** (the ecocentric, ternary logic model from RFI-IRFOS). While the original `albert.py` provides a rich web interface via Streamlit, **Albert-Code** is designed for maximum speed and efficiency in the terminal.

By utilizing [Ollama](https://ollama.com/) for local inference of the `albert:latest` model (based on `qwen2.5:3b`), Albert-Code achieves lightning-fast responses while maintaining access to a powerful set of sovereign tools.

## Key Features

- **🚀 Terminal Speed**: Direct interaction with the model via Ollama, bypassing web UI overhead.
- **🔧 Functional Tool Harness**:
  - `execute_bash`: Full shell access on your local machine.
  - `create_file` / `read_file`: Seamless local file management.
  - `web_search`: Search the web via DuckDuckGo.
  - `retrieve_memory` / `log_memory`: Persistent SQLite-based memory vault shared with `albert.py`.
  - `get_system_health`: Real-time system telemetry.
- **🛡 Sovereignty Checks**: Built-in mechanisms to ensure Albert acts rather than narrates.
- **🧠 Neurosymbolic Fallback**: Intercepts plain-text tool requests and routes them to the correct actuator.

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
cd albert-agent
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
