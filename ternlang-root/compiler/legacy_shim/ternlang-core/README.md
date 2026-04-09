# ternlang-core

The compiler and virtual machine for **Ternlang** — a balanced ternary language for AI and ML systems.

Every value is a `trit`: `reject` (−1), `tend` (0), or `affirm` (+1). The `tend` state is a first-class computational directive — not null, not false, but *"hold until evidence crosses threshold."*

## What's in this crate

| Component | Description |
|-----------|-------------|
| Lexer + Parser | Tokenises and parses `.tern` source into an AST |
| Semantic analyser | Type-checks, validates exhaustive 3-arm `match` |
| Bytecode emitter | Compiles AST → BET (Balanced Ternary Execution) bytecode |
| BET VM | Stack-based VM — 51 opcodes, 27 registers, tensor heap, agent runtime |
| `@sparseskip` | Codegen directive: emits `TSPARSE_MATMUL` to skip zero-state weights |
| StdlibLoader | Resolves `use std::trit` etc. at compile time with zero runtime I/O |

## Quick start

```bash
cargo install ternlang-cli
ternlang run my_program.tern
ternlang repl
```

Or use the API directly:

```rust
use ternlang_core::{Parser, codegen::betbc::BytecodeEmitter, vm::BetVm};

let mut parser  = Parser::new("let x: trit = affirm;");
let mut emitter = BytecodeEmitter::new();
while let Ok(stmt) = parser.parse_stmt() { emitter.emit_stmt(&stmt); }
let mut vm = BetVm::new(emitter.finalize());
vm.run().unwrap();
```

## Language snapshot

```ternlang
fn triage(conscious: trit, vitals: trit) -> trit {
    match conscious {
        reject => { return reject; }   // hard gate — skip all further checks
        tend   => { return tend;   }   // wait for more data
        affirm => { return consensus(conscious, vitals); }
    }
}
```

- [Language reference](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-/blob/main/ternlang-root/LANGUAGE.md)
- [BET-ISA spec](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-/blob/main/ternlang-root/BET-ISA-SPEC.md)
- [Whitepaper (DOI: 10.17605/OSF.IO/TZ7DC)](https://doi.org/10.17605/OSF.IO/TZ7DC)

## License

LGPL-3.0 — open core. See [LICENSE-LGPL](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-/blob/main/ternlang-root/LICENSE-LGPL).
