<p align="center">
  <img src="https://img.shields.io/badge/Engine-Rust-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/Architecture-Ternary%20Intelligence-black?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Security-Deny--First%20AST-red?style=for-the-badge" />
  <img src="https://img.shields.io/badge/LLM-Agnostic-blue?style=for-the-badge" />
  <img src="https://img.shields.io/badge/License-BSL--1.1-lightgrey?style=for-the-badge" />
</p>

<p align="center">
  <b>Albert CLI — Ternlang Engine</b><br/>
</p>

---

# 🧠 Albert CLI (Ternlang Engine)

> **Sovereign. Hardened. Self-reasoning. Token-efficient.**

The **Albert CLI** is the terminal-native evolution of the Albert agentic framework — now powered by the high-performance Rust-based **Ternlang Engine**.

---

# ⚡ Core Principles

- **Local-First Sovereignty** — No forced cloud dependency  
- **Deny-by-Default Security** — Nothing executes unless proven safe  
- **Ternary Reasoning** — No hallucinated execution paths  
- **Self-Extending Agents** — Capabilities evolve at runtime  
- **Token Minimalism** — Efficiency is engineered, not optimized later  

---

# 🛡️ Sovereign Security Architecture

## Deny-First AST Interception
All shell commands are parsed through a native Rust interception pipeline *before execution*.

**Blocked by design:**
- Command substitution (`$()`, backticks)
- Dangerous pipes (`curl | bash`)
- Unauthorized redirects
- Shell injection patterns

➡️ If it smells unsafe, it never reaches the shell.

---

## Policy-Enforced Execution

- Security flags are **no longer LLM-controlled**
- Critical toggles (e.g. sandbox bypass) are **hard-restricted**
- Prompt injection attacks targeting execution are **neutralized structurally**

---

## Ternary Execution Validation

Every action resolves into a deterministic state:

| State | Meaning | Behavior |
|------|--------|---------|
| `+1` | Safe / Success | Proceed |
| `0`  | Ambiguous / Missing Context | Halt + Ask |
| `-1` | Failure | Reflect + Retry |

➡️ No blind execution. No fake confidence.

---

# 🌐 LLM-Agnostic Core

## BYOK (Bring Your Own Key)

Connect to any provider via:

```bash
/auth
````

**Supported ecosystem:**

* OpenAI
* Anthropic
* Google (Gemini)
* Hugging Face
* AWS
* Azure
* Ternlang (native)

---

## Dynamic Provider Resolution

* Model names are auto-mapped to providers
* Configuration is resolved at runtime
* Seamless switching between models

➡️ One interface. Entire LLM ecosystem.

---

# 🧠 Reasoning & Memory Arsenal

## SequentialThinking

Structured multi-step reasoning engine:

* Tracks evolving thought chains
* Allows revision and correction mid-process
* Enables deeper planning loops

---

## Memory (Knowledge Graph)

Persistent, local-first intelligence layer:

* Stores entities, relationships, observations
* Builds long-term project awareness
* Survives across sessions

---

## RepoMap

Automated codebase cognition:

* Generates structured maps of repositories
* Accelerates navigation and comprehension
* Reduces token load dramatically

---

## MCP Client (Model Context Protocol)

* Native MCP integration
* Connect to external tool servers
* Extend capabilities without modifying core

---

# 🏛️ System Architecture

```
                ┌──────────────────────────────┐
                │       Ternlang Engine        │
                │   (Rust Execution Core)      │
                └────────────┬─────────────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
 ┌──────────────┐   ┌────────────────┐   ┌────────────────┐
 │  API Client  │   │   Runtime Core │   │     Tools       │
 │ (LLM Layer)  │   │ (Loop + Guard) │   │ (Arsenal + Core)│
 └──────────────┘   └────────────────┘   └────────────────┘
                             │
                     ┌──────────────┐
                     │  Ternary Loop │
                     │ (+1 / 0 / -1) │
                     └──────────────┘
```

---

# 🔧 Key Components

## Engine Block (`rust/`)

* Hardened autonomous loop
* Ternary validation system
* Security enforcement layer

---

## RTK — Rust Token Killer

* CLI-level token optimization proxy
* 60–90% reduction in token usage
* Context compression via sliding windows

---

## Multi-Provider API Client

* Fully agnostic communication layer
* Handles provider-specific quirks internally
* Clean abstraction for agent logic

---

## Standardized Toolset

### Core Tools

* Bash
* File Operations
* Glob
* Grep

### Arsenal Tools

* Memory
* SequentialThinking
* RepoMap

---

# 🚀 Quickstart

## 1. Build & Install

```bash
cd rust
cargo install --path crates/rusty-ternlang-cli --force
```

---

## 2. Authenticate

```bash
ternlang-cli /auth
```

---

## 3. Initialize Repository

```bash
ternlang-cli /init
```

---

## 4. Launch REPL

```bash
ternlang-cli
```

---

# 📂 Repository Layout

```text
.
├── rust/           # High-performance Ternlang Engine
│   ├── crates/
│   │   ├── api/        # LLM-agnostic client layer
│   │   ├── runtime/    # Core loop, security, memory
│   │   ├── tools/      # Arsenal + core tools
│   │   └── ...
│
├── src/            # Legacy Python agent (reference only)
│
└── README.md       # You are here
```

---

# 🔁 Execution Loop (Conceptual)

```
[Research] → [Strategy] → [Execution]
      ↓            ↓            ↓
     (+1)        (0 Halt)      (-1 Retry)
```

* Research builds context
* Strategy plans execution
* Execution is validated through ternary state

---

# 🛡️ Threat Model

Albert CLI is explicitly designed against:

* Prompt injection attacks
* Shell command smuggling
* Unsafe piping & redirection
* Context poisoning
* Blind autonomous loops

➡️ Security is not a feature — it is the baseline.

---

# 📜 License

**Business Source License 1.1 (BSL-1.1)**
Converts to **Apache 2.0** on **2030-04-03**

---

# 🏗️ Built By

**RFI-IRFOS**
*Research Focus Institute – Interdisciplinary Research Facility for Open Sciences*
Graz, Styria 🇦🇹

---

# 💡 Final Note

> Most AI systems scale by consuming more tokens.
> This one scales by **needing less intelligence to waste**.

Albert CLI is not trying to be bigger.

It is trying to be **correct, efficient, and sovereign**.

```
```
