<p align="center">
  <img src="https://img.shields.io/badge/Engine-Rust-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/Architecture-Ternary%20Intelligence-black?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Security-Deny--First%20AST-red?style=for-the-badge" />
  <img src="https://img.shields.io/badge/LLM-Agnostic-blue?style=for-the-badge" />
  <img src="https://img.shields.io/badge/License-BSL--1.1-lightgrey?style=for-the-badge" />
</p>

<p align="center">
  <b>Albert CLI — Ternlang Engine</b><br/>
  <i>Operation Strip-Mine Edition</i>
</p>

---

# 🧠 Albert CLI (Ternlang Engine)

> **Sovereign. Hardened. Self-reasoning. Token-efficient.**

The **Albert CLI** is the terminal-native evolution of the Albert agentic framework — now powered by the high-performance Rust-based **Ternlang Engine**.

Following **Operation Strip-Mine**, the system has been aggressively hardened, expanded, and fully decoupled from proprietary dependencies.

This is not a wrapper.  
This is an **execution-grade AI runtime**.

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
