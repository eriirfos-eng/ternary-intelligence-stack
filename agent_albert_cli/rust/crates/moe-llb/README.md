# moe-llb — Last Look Back Protocol

[![crates.io](https://img.shields.io/crates/v/moe-llb.svg)](https://crates.io/crates/moe-llb)
[![docs.rs](https://img.shields.io/docsrs/moe-llb)](https://docs.rs/moe-llb)
[![MIT](https://img.shields.io/badge/license-LGPL--2.1-blue)](LICENSE)
[![tests](https://img.shields.io/badge/tests-18%20passing-brightgreen)]()

**Deterministic filesystem containment gate for sovereign AI agents.**

Part of the [Ternary Intelligence Stack (TIS)](https://ternlang.com) — built by [RFI-IRFOS](https://ternlang.com), Graz, Austria.

---

## The Problem

Modern AI agents (LLM-based tools like coding assistants, autonomous agents, MCP servers) are granted high-privilege access to local filesystems. Their only safety layer is the system prompt — a soft linguistic boundary that fails under high-entropy tasks, context degradation, or model hallucinations.

An agent that *intends* to clean a cache directory can `rm -rf` a source tree. An agent that *intends* to update a config file can silently overwrite `/etc/passwd`. There is no enforcement layer between the model's probabilistic intent and the host's deterministic filesystem.

**LLB moves safety from the linguistic layer to the binary layer.**

---

## What It Does

Every filesystem mutation is an **atomic transaction**:

```
MutationRequest
      │
      ▼
┌─────────────┐
│   Gate 1    │  Canonicalize path · Check blacklist · Classify tier
│  (Preflight)│  Issue ExecutionTicket (SHA-256, 30s TTL)
└──────┬──────┘
       │ Ticket issued
       ▼
┌─────────────┐
│  Snapshot   │  Full content copy (files ≤ 50MB) — the undo point
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Execute   │  Your closure — the only place that touches the disk
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Gate 2    │  Target integrity · Spillover detection · Executable audit
│  (IOCC)     │  Intent-Outcome Consistency Check
└──────┬──────┘
       │
  ┌────┴────┐
  │         │
Commit    Rollback  ← automatic on Gate 2 failure or execution panic
```

---

## Safety Tiers

| Tier | Operation | Snapshot | HITL | Gate 2 |
|------|-----------|----------|------|--------|
| T0 | READ | — | — | — |
| T1 | CREATE (non-exec) | — | — | — |
| **T2** | OVERWRITE · CHMOD · CREATE `.sh/.py/.bin` | ✅ | — | ✅ |
| **T3** | DELETE | ✅ | ✅ | ✅ |

T1 CREATE of any executable-like extension (`.sh`, `.py`, `.rb`, `.bin`, `.so`, etc.) is **automatically promoted to T2**.

---

## LlbDecision — Ternary Gate Verdict

```rust
pub enum LlbDecision {
    Allow,       // +1 — committed
    Warn(msg),   //  0 — soft block, agent should rethink
    HardVeto,    // -1 — rollback triggered
}
```

Maps directly to [TIS](https://ternlang.com) ternary trit values `{+1, 0, −1}`.

---

## Quick Start

```rust
use albert_llb::{execute, LlbError, MutationRequest, Operation, StructuredIntent};

let result = execute(
    MutationRequest {
        path: "/tmp/output.txt".into(),
        operation: Operation::Create,
        intent: StructuredIntent {
            goal: "write analysis results to disk".into(),
            justification: "user requested a summary file".into(),
            fallback_if_rejected: "print to stdout instead".into(),
        },
    },
    |path| std::fs::write(path, b"results\n").map_err(LlbError::Io),
);

match result {
    Ok(albert_llb::LlbDecision::Allow) => println!("committed"),
    Err(e) => eprintln!("vetoed or rolled back: {e}"),
    _ => {}
}
```

---

## What's Blocked

**Paths — Gate 1 blacklist:**
- `/etc`, `/bin`, `/sbin`, `/usr/bin`, `/boot`, `/dev`, `/proc`, `/sys`, `/run`, `/lib`
- `~/.ssh`, `~/.gnupg`, `~/.bashrc`, `~/.config/albert/secrets.json`
- Any path containing `..` (traversal)

**Shell commands — binary scanner:**
```rust
use albert_llb::blacklist;
blacklist::check_shell_command("rm -rf /home/user")?;
// → Err(ForbiddenBinary { binary: "rm" })
```
Scans for: `rm`, `rmdir`, `shred`, `dd`, `mkfs`, `fdisk`, `chmod`, `chown`, `sudo`, `su`, `passwd`, `crontab`

**Gate 2 IOCC catches:**
- File not created/deleted as declared
- Unexpected files appearing or disappearing in the same directory (spillover)
- Newly created files with executable permission bits not declared in intent

---

## HITL — Human-In-The-Loop Gate

T3/DELETE operations block execution and show a high-contrast terminal confirmation prompt:

```
╔════════════════════════════════════════════════════════════╗
║     ⚠  LAST LOOK BACK — HUMAN GATE  ⚠  [T3/DELETE]       ║
╠════════════════════════════════════════════════════════════╣
║  OPERATION  : DELETE                                      ║
║  PATH       : /home/user/project/src/main.rs              ║
║  INTENT     : Remove stale build artifact                 ║
║  REASON     : File was identified as outdated cache       ║
║  FALLBACK   : Move to /tmp instead                        ║
╠════════════════════════════════════════════════════════════╣
║  THIS ACTION IS DESTRUCTIVE.                              ║
║  IT MAY NOT BE FULLY REVERSIBLE.                          ║
╠════════════════════════════════════════════════════════════╣
║  Type CONFIRM to proceed. Anything else aborts.           ║
╚════════════════════════════════════════════════════════════╝
```

---

## Snapshot & Rollback

For files ≤ 50 MB: full binary copy before mutation. Automatically deleted on successful commit (via `Drop`). Restored atomically on Gate 2 failure or execution panic.

For files > 50 MB: metadata-only. A mandatory warning is printed:

```
[LLB WARNING] CAUTION: This operation is NOT fully reversible.
File 'large_model.bin' is 240.0 MB — exceeds the 50 MB snapshot threshold.
Content backup SKIPPED. Only metadata is recorded.
```

If rollback itself fails, the system enters **Permanent Panic** — all further mutations are blocked until a human clears the lock.

---

## Limitations

LLB enforces filesystem integrity **within its controlled execution scope only**:

- Does not prevent network calls (`curl`, `wget`)
- Does not prevent execution of scripts that already exist on disk
- TOCTOU micro-window exists between canonicalization and syscall (mitigated but not eliminated without kernel-level Landlock)
- Does not make the AI model "safe" or "aligned" — it only ensures the host filesystem remains recoverable and bounded

---

## Run the Demo

```bash
cargo run --bin llb_demo -p moe-llb
```

Demonstrates all 6 scenarios: T1 happy path, T2 rollback on failure, path traversal block, blacklist block, forbidden binary intercept, and IOCC spillover detection.

---

## Part of the Ternary Intelligence Stack

`moe-llb` is the containment layer for [Albert](https://crates.io/crates/albert-cli) — the sovereign AI coding agent built on the Ternary Intelligence Stack.

- [`albert-cli`](https://crates.io/crates/albert-cli) — agent binary
- [`albert-runtime`](https://crates.io/crates/albert-runtime) — session management, MCP, tool dispatch
- [`moe-core`](https://crates.io/crates/moe-core) — MoE-13 ternary expert router
- [`ternlang-core`](https://crates.io/crates/ternlang-core) — ternary language runtime

---

*Built by [RFI-IRFOS](https://ternlang.com) — Research Focus Institute · Graz, Austria*
*© 2026 RFI-IRFOS. Licensed under LGPL-2.1.*
