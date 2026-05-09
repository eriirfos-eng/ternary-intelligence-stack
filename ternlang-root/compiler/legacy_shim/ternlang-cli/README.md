# ternlang-cli

Command-line interface for the [Ternlang](https://ternlang.com) Balanced Ternary Execution VM.

## Install

```sh
# One line — installs Rust (if needed) + ternlang-cli, ready immediately
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source "$HOME/.cargo/env" && cargo install ternlang-cli
```
> **Note:** Do not use `sudo apt install cargo` — Ubuntu's packaged version is too old (1.75). The line above installs the current toolchain via rustup.

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
