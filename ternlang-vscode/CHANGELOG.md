# Ternlang Extension Changelog

## 0.4.0 — 2026-04-10

### Tier 1 — LSP auto-download (cross-platform)
- Extension detects platform+arch at activation and resolves the correct `ternlang-lsp` binary
- Resolution order: bundled binary → cached in globalStorageUri → download from GitHub Releases
- Progress notification while downloading; single dismissible warning on failure
- `ternlang.suppressLspWarning` still available
- GitHub Actions workflow `.github/workflows/lsp-release.yml`: build matrix for
  `linux-x64`, `linux-arm64`, `darwin-x64`, `win32-x64` — triggered by `vscode-v*` tags

### Tier 2 — Inline trit ghost decorations (live after run)
- Press `Ctrl+Shift+R` — after the run completes, every `let` binding in the source gets a ghost annotation showing its resolved trit state:
  - `→ Affirm` in teal
  - `→ Tend` in amber
  - `→ Reject` in red
- Requires `ternlang.apiKey` starting with `tern_t2_`
- Implemented via `--emit-symbols` flag added to `ternlang-cli run` (compiler change)
- `TERN_SYMBOLS:varname=reg,...` emitted to stderr; `Reg N: trit(...)` from stdout — correlated to produce variable→trit map
- Decorations cleared automatically when switching files

### Compiler: `--emit-symbols` flag
- `ternlang-cli run --emit-symbols <file>` emits `TERN_SYMBOLS:...` to stderr
- `BytecodeEmitter::get_function_symbols("main")` — new public API; snapshots local symbol table per function before scope restore
- `BytecodeEmitter::get_function_symbols()` — added `function_symbols` field to emitter struct

## 0.3.0 — 2026-04-10

### Tier architecture introduced
- **4-tier feature model**: Free / Pro / Industrial / Enterprise — gated by `ternlang.apiKey`
  - `tern_t2_…` → Tier 2 Pro · `tern_t3_…` → Tier 3 Industrial · `tern_t4_…` → Tier 4 Enterprise
- **Status bar**: shows tier label; click for status + settings shortcut

### Tier 1 (Free) additions
- **19 TextMate snippets** — `fn`, `fnmain`, `let`, `letmut`, `match` (3-arm), `if`, `for`, `while`, `loop`, `struct`, `agent`, `spawn`, `sendawait`, `sparseskip`, `tensor`, `cast`, `consensus`, `invert`, `use`
- **Run .tern File** command — `Ctrl+Shift+R` / `⌘⇧R` — runs via `ternlang-cli`, output in dedicated panel
  - `ternlang.cliPath` setting for custom binary path
  - Play button in editor title bar
- **Graceful LSP fallback** — no crash if `bin/ternlang-lsp` is missing; one-time dismissible warning
  - `ternlang.suppressLspWarning` to silence permanently

### Tier 2–4 commands (stubs — show upgrade prompt or "coming soon")
- Pro: `ternlang.inlineTritHints`, `ternlang.cloudDiagnostics`
- Industrial: `ternlang.debugBetVm`, `ternlang.viewTensor`, `ternlang.sparseskipCoverage`
- Enterprise: `ternlang.clusterPanel`, `ternlang.agentMonitor`

## 0.2.0 — 2026-04-10

- Grammar: `affirm`, `tend`, `reject` highlighted as `constant.language.trit.ternlang`
- Grammar: `<=`, `>=` added to operators pattern
- Published to Open VSX registry as `rfi-irfos/ternlang`

## 0.1.0 — 2026-04-03

Initial release.

- Syntax highlighting for `.tern` files (keywords, trit literals, types, directives, operators)
- 19 code snippets with tab stops (fn, let, match, agent, struct, @sparseskip, builtins)
- Hover documentation for all ternlang keywords via `ternlang-lsp`
- Live diagnostics — syntax errors underlined as you type
- Auto-close pairs, bracket matching, comment toggling
- Language configuration: word pattern, indent rules
