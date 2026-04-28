# GEMINI.md - Albert CLI Development Log

## Stable Release: 2026-04-27 (TUI & Autonomous Core)

### 🌐 Ternlang Studio
- **Fetch Encoding Fix**: Resolved `TypeError` in `Window.fetch` caused by non-ASCII characters (specifically em-dashes `—`) leaking into the `X-Ternlang-Key` header.
    - Implemented `sanitizeHeader` utility to strip all non-ASCII characters and whitespace from headers.
    - Applied sanitization to all API key entry points and `fetch` calls (`loadPremiumTree`, `syncFleetRegistry`, `deleteAgent`, `fetchUsage`, `buildFileTree`, `openFile`, `deployAgent`, `tryApiUsage`, `tryApiRun`).
    - Fixed `TranslatorView` auth bridge to also use sanitized keys.

### 🎨 TUI & UX Refinements
- **Slash Command Pause**: Fixed a bug where terminal-based slash commands (like `/status`, `/cost`, `/clear`, `/version`) would print to stdout and immediately be overwritten by the TUI redraw, appearing as if "nothing happened". The TUI now pauses and waits for `Enter` so you can read the output before returning to the chat window.
- **Dingir Sigil (𒀭)**: Restored as the primary brand sigil and pulsing "Energy Core" status indicator.
- **Subtle HUD**: Refined the absolute bottom footer (Model, CWD, Permissions) into a dimmed "sidenote" to prioritize the Tips row.
- **Compact Paste**: Implemented a "Pasted Text" badge for inputs >= 3 lines or > 200 characters, preventing buffer bloat while preserving the payload.
- **Typewriter Effect**: Optimized streaming render interval to 33ms (~30fps) for smooth, token-by-token visual progression.
- **Interaction Logic**:
    - Reserved `Up/Down` arrow keys strictly for history navigation.
    - Dedicated Mouse Wheel and `Shift+Up/Down` to chat scrolling.
    - Set user prefix to `≻` for a cleaner chat aesthetic.
    - Added "Press any key to exit" gate to the Session Report Card.

### 🤖 Autonomous Commands (Harness Upgrade)
Converted "UI-Only" placeholders into active `agent_override` workflows:
- `/tdd`: Test-Driven Development autonomous loop.
- `/verify`: Full workspace verification (build, lint, test, type-check).
- `/buildfix`: Autonomous compilation error resolution.
- `/codereview`: Deep security and quality assessment.
- `/aside`: Tangential inquiry mode (isolated context).
- `/refactor` & `/learn`: Structural optimization and knowledge absorption.

### 🛡️ Security & Runtime
- **Startup Security Guide**: Integrated a "Trust Workspace" gate with options to:
    1. Trust folder (sets session trust bit, removes HUD warnings).
    2. Change directory (interative `cd` before startup).
    3. Exit for safety.
- **Dynamic Control**: Added `@cd <path>` and `@trust` as immediate runtime commands.
- **HUD Warning**: Added `⚠ Untrusted Folder` indicator for non-trusted environments.

### 📊 Session Report Card
- Expanded the Power-down summary to include:
    - **Resources**: Total Tokens In/Out (Cumulative).
    - **Performance**: Wall Time vs. Active Agent Time (including API vs. Tool breakdown).
    - **Success Rate**: Precise tool call success/failure metrics.

---
*Locked & Verified by Gemini CLI.*
