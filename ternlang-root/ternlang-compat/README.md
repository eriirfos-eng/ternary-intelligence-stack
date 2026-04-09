# ternlang-compat

Compatibility bridges for the [Ternlang](https://ternlang.com) ecosystem. Lets existing ternary computing projects target the BET VM as a common runtime.

## Bridges

### `.tasm` — 9-trit RISC assembly

Translates the Brandon Smith / ternary-computing.com `.tasm` assembly dialect (9-trit word, RISC-like mnemonics) into BET bytecode.

```rust
use ternlang_compat::tasm::assemble;

let bytecode = assemble("LOAD R0, 1\nADD R0, R1\nHALT")?;
```

### Owlet — S-expression front-end

Parses Owlet-style S-expressions into ternlang AST nodes for evaluation on the BET VM.

```rust
use ternlang_compat::owlet::eval;

let result = eval("(consensus (affirm) (tend))")?;
```

## License

LGPL-3.0-or-later. See [LICENSE](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/LICENSE).
