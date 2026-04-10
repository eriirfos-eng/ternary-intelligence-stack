# Ternlang Extension Changelog

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
