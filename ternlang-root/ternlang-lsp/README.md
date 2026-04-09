# ternlang-lsp

LSP 3.17 language server for [Ternlang](https://ternlang.com). Provides hover docs, code completion, and live parse diagnostics for `.tern` files in any LSP-capable editor.

## Transport

JSON-RPC 2.0 over stdio — the standard MCP/LSP pattern.

## Features

- **Hover** — trit type info and keyword documentation on cursor
- **Diagnostics** — parse errors highlighted in real time as you type
- **Completion** — ternlang keywords, trit literals, and stdlib identifiers
- **Go-to-definition** — jump to function and variable declarations

## Editor setup

**VS Code** — install the Ternlang extension (see `ternlang-root/integrations/vscode/`), which auto-configures the server.

**Neovim / other** — point your LSP client at `ternlang-lsp`:

```lua
require('lspconfig').ternlang_lsp.setup {
  cmd = { 'ternlang-lsp' },
  filetypes = { 'tern' },
}
```

## Install

```sh
cargo install ternlang-lsp
```

## License

LGPL-3.0-or-later. See [LICENSE](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/LICENSE).
