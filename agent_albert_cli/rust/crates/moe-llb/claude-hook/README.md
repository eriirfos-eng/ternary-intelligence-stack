# LLB — Claude Code Hook Integration

Wires the Last Look Back Protocol into Claude Code as a `PreToolUse` hook.
Every file mutation and git commit/push is gated by LLB before it executes.

## Install

```bash
cargo install moe-llb        # installs moe-llb-mcp to ~/.cargo/bin/
cp llb-hook.sh ~/.claude/    # or any path you prefer
chmod +x ~/.claude/llb-hook.sh
```

Merge the `hooks` block from `settings.json` into your `~/.claude/settings.json`,
updating the `command` path to match where you placed the script.

## What it gates

| Tool | Check |
|------|-------|
| `Edit` | `llb_validate` — Gate 1 path + blacklist check before overwrite |
| `Write` | `llb_validate` — Gate 1 path + blacklist check (CREATE or OVERWRITE) |
| `Bash` (git commit/push) | Sensitive filename scan + `llb_check` on all staged paths |

## Decisions

- `+1 ALLOW` — tool call proceeds normally
- ` 0 WARN`  — tool call proceeds (soft signal, logged)
- `-1 VETO`  — tool call is **blocked**, reason printed to stderr

## Blocked by default

Paths: `/etc`, `/bin`, `/sys`, `/proc`, `~/.ssh`, `~/.gnupg`, and other protected directories.

Shell commands containing: `rm`, `sudo`, `chmod`, `dd`, `shred`, `fdisk`, `mkfs`, `crontab`.

Staged filenames matching: `.env`, `secret`, `credential`, `token`, `password`, `id_rsa`, `id_ed25519`, `.pem`, `.p12`, `.pfx`, `api_key`.
