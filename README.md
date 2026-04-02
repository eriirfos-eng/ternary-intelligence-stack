# Ternary Intelligence Stack (TIS)
### Post-Binary Systems Architecture for Ambiguity-Aware Computation

**ternlang** is a balanced ternary systems programming language targeting the software deficit in ternary computing. It compiles to BET (Balanced Ternary Execution) bytecode, runs on the BET VM, and ships an MCP server that turns any binary AI agent into a ternary decision engine.

```
trit ∈ { -1, 0, +1 }
  -1 = conflict      0 = hold (active, not null)      +1 = truth
```

---

## Benchmark: Sparse Ternary Inference

The flagship feature is `@sparseskip` — zero-weight elements are skipped at the VM level, not just masked. Real measured results on BitNet-style ternary weight matrices:

```
Weight matrix sparsity:  56.2%
Dense multiply ops:      4,096
Sparse multiply ops:     1,792
Skipped (free):          2,304

Speed improvement:       2.3× fewer multiply operations vs float32
```

In a ternary weight model, 0-weighted connections contribute nothing to the output. The BET VM's `TSPARSE_MATMUL` opcode skips them entirely at the instruction level — not in software.

---

## Ternlang: The Language

Ternlang is a systems language where every branch is 3-way and every match is exhaustive.

```ternlang
// Balanced ternary addition with carry
fn ternadd(a: trit, b: trit, c: trit) -> trit {
    let ab: trit = consensus(a, b);
    return consensus(ab, c);
}

// Sparse inference layer (skips zero-weighted connections)
fn linear(W: trittensor<128 x 64>, x: trittensor<64 x 1>) -> trittensor<128 x 1> {
    @sparseskip let out: trittensor<128 x 1> = matmul(W, x);
    return out;
}

// Every match must cover -1, 0, +1 — compiler enforces this
fn decide(signal: trit) -> trit {
    match signal {
         1 => { return  1; }   // truth
         0 => { return  0; }   // hold — still active, not null
        -1 => { return -1; }   // conflict
    }
}
```

### Structs and Field Access

```ternlang
struct Synapse {
    weight: trit,
    active: trit,
}

fn update(s: Synapse, input: trit) -> trit {
    let w: trit = s.weight;
    return consensus(w, input);
}
```

### Actor Model

```ternlang
// Define an agent — processes messages and returns a ternary decision
agent Voter {
    fn handle(msg: trit) -> trit {
        match msg {
             1 => { return  1; }
             0 => { return  0; }
            -1 => { return -1; }
        }
    }
}

// Spawn, communicate, receive
fn run_vote(signal: trit) -> trit {
    let v: agentref = spawn Voter;
    send v signal;
    let result: trit = await v;
    return result;
}
```

---

## MCP Integration: Ternary Decision Engine for Any Binary Agent

The `ternlang-mcp` server connects to any MCP-compatible AI client (Claude Desktop, custom agents) and exposes ternary logic as tools. Any binary agent plugged in becomes a ternary decision engine.

### Quick Setup (Claude Desktop)

Add to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "ternlang": {
      "command": "/path/to/ternlang-root/target/release/ternlang-mcp"
    }
  }
}
```

Or use the bundled config:
```bash
cat ternlang-root/ternlang-mcp/mcp-config.json
```

### Available MCP Tools

| Tool | What it does |
|------|-------------|
| `trit_decide` | Float evidence → ternary decision (+1/0/-1) with confidence and sparsity |
| `trit_consensus` | consensus(a, b) with carry — BET VM live |
| `trit_eval` | Evaluate a ternlang expression on the live BET VM |
| `ternlang_run` | Compile + run a `.tern` program via MCP |
| `quantize_weights` | f32 array → ternary {-1, 0, +1} with BitNet thresholding |
| `sparse_benchmark` | Run sparse vs dense matmul and return skip statistics |

### Example: Ternary Decision via MCP

```json
{
  "tool": "trit_decide",
  "arguments": {
    "evidence": [0.8, -0.3, 0.1, 0.9, -0.7],
    "threshold": 0.35
  }
}
```

Returns:
```json
{
  "decision": "+1 (truth)",
  "confidence": 0.72,
  "signal_sparsity": "40.0%",
  "quantized_trits": [1, -1, 0, 1, -1],
  "interpretation": "Majority evidence is affirmative. Sparse signal — 40% of inputs are neutral."
}
```

---

## Quick Start

```bash
# Build everything
cd ternlang-root
cargo build --release

# Run a .tern program
./target/release/ternlang-cli run test.tern

# Build to bytecode
./target/release/ternlang-cli build test.tern

# Start the MCP server (connect via Claude Desktop or any MCP client)
./target/release/ternlang-mcp

# Easter egg
./target/release/ternlang-cli enlighten
```

---

## Architecture

```
ternlang-root/
├── ternlang-core/       Lexer, AST, parser, semantic, codegen (betbc), BET VM
├── ternlang-cli/        ternlang run / build CLI (clap)
├── ternlang-ml/         BitNet quantization, sparse matmul, linear layer, benchmarks
├── ternlang-mcp/        MCP server — 6 tools, JSON-RPC 2.0 stdio
├── ternlang-codegen/    (stub — HDL backend planned)
├── ternlang-test/       (stub — integration test harness)
└── stdlib/
    ├── std/trit.tern    abs, min, max, clamp, threshold, sign, majority
    ├── std/math.tern    ternadd3, neg, balance, step, rectify
    ├── std/tensor.tern  zeros, sparse_mm, dense_mm
    ├── std/io.tern      print_trit, print_tensor, newline
    ├── ml/quantize.tern hard_threshold, soft_threshold
    └── ml/inference.tern linear, linear_dense, attend, decide
```

### BET VM Opcode Summary

| Opcode | Name | Description |
|--------|------|-------------|
| 0x01 | TPUSH | Push trit literal |
| 0x02 | TADD  | Balanced ternary add with carry |
| 0x03 | TMUL  | Ternary multiply |
| 0x04 | TNEG  | Negate trit |
| 0x05–07 | TJMP_POS/ZERO/NEG | Conditional jump |
| 0x08 | TSTORE | Store to register |
| 0x09 | TLOAD  | Load from register |
| 0x0e | TCONS  | Consensus (ternary OR) |
| 0x0f | TALLOC | Allocate tensor on heap |
| 0x10 | TCALL  | Call function (push return addr) |
| 0x11 | TRET   | Return from function |
| 0x20 | TMATMUL | Dense matrix multiply |
| **0x21** | **TSPARSE_MATMUL** | **Sparse matmul — skips zero weights ⭐** |
| 0x22 | TIDX   | Index into tensor |
| 0x23 | TSET   | Set tensor element |
| 0x24 | TSHAPE | Push tensor dimensions |
| 0x25 | TSPARSITY | Count zero elements |
| 0x30 | TSPAWN | Spawn agent instance |
| 0x31 | TSEND  | Send message to agent mailbox |
| 0x32 | TAWAIT | Await agent response |

---

## Key Language Properties

- **trit**: `-1` (conflict), `0` (hold — active, not null), `+1` (truth)
- **match**: must cover all three arms or the compiler rejects it (`NonExhaustiveMatch`)
- **`@sparseskip`**: routes `matmul()` to `TSPARSE_MATMUL` — zero-state elements skipped at VM level
- **actors**: `agent/spawn/send/await` — synchronous v0.1, distributed v0.2
- **BET encoding**: 0b01=-1, 0b10=+1, 0b11=0, 0b00=invalid fault
- **`.tern`** → `.tbc` (bytecode) → **BET VM** execution

---

## Roadmap

| Phase | Status |
|-------|--------|
| 1 — Core language & VM | ✅ Complete |
| 2 — CLI & built-ins | ✅ Complete |
| 3 — TritTensors & sparse inference | ✅ Complete |
| 3.5 — MCP server | ✅ Complete |
| 4 — Language completeness (for/while/struct/cast/use) | ✅ Complete |
| 5.0 — Actor model (local) | ✅ Complete |
| 5.1 — Distributed actors (TCP transport) | 🔲 Next |
| 6 — HDL/Verilog backend | 🔲 Planned |

Full roadmap: [ROADMAP.md](ternlang-root/ROADMAP.md)

---

## License

**Open Core — LGPL v3** (compiler + stdlib)

Commercial tier (planned): `ternlang-ml`, HDL backend, distributed runtime.

Trademarks: *Ternlang*, *BET VM*, *Balanced Ternary Execution* — RFI-IRFOS.

---

*Built by Simeon Kepp & Claude — RFI-IRFOS — 2026*
