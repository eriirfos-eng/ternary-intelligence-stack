# ternlang-core

Compiler and VM for Ternlang — balanced ternary language with affirm/tend/reject trit semantics, `@sparseskip` codegen, and BET bytecode execution.

## Installation

```sh
cargo add ternlang-core
```

## Usage

```rust
use ternlang_core::{Trit, pack_trits, unpack_trits, BetVm};

// Pack trits into BET bytecode
let trits = vec![Trit::Affirm, Trit::Tend, Trit::Reject];
let packed = pack_trits(&trits);
let round_tripped = unpack_trits(&packed, trits.len()).unwrap();

// Run a VM
let mut vm = BetVm::new(vec![]);
vm.run().unwrap();
```

## Features

- `default` — standard library support
- `simd` — SIMD-accelerated trit packing
- `wasm` — WebAssembly-compatible VM build

## License

LGPL-3.0-or-later
