# ternlang-cli

Command-line interface for the [Ternlang](https://ternlang.com) Balanced Ternary Execution VM.

## Install

```sh
cargo install ternlang-cli
```

## Commands

| Command | Description |
|---------|-------------|
| `ternlang run <file.tern>` | Compile and execute a `.tern` program on the BET VM |
| `ternlang build <file.tern>` | Compile to `.bet` bytecode |
| `ternlang sim <file.tern>` | Hardware simulation (RTL/Verilog via ternlang-hdl) |
| `ternlang fmt <file.tern>` | Format source file |
| `ternlang repl` | Interactive ternlang REPL |
| `ternlang compat <file>` | Run compatibility bridge (`.tasm` or Owlet) |
| `ternlang benchmark` | Run @sparseskip vs dense matmul benchmark |

## Quick start

```sh
# Run a ternlang program
ternlang run hello.tern

# Compile to bytecode
ternlang build logic.tern -o logic.bet

# Interactive REPL
ternlang repl
>>> let x: trit = affirm;
>>> return consensus(x, tend);
```

## License

LGPL-3.0-or-later. See [LICENSE](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/LICENSE).
