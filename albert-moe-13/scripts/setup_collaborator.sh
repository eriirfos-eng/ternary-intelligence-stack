#!/usr/bin/env bash
# albert. collaborator setup — one-line drop-in for Linux
#
# Usage (run this on the collaborator's machine):
#   curl -sSL https://raw.githubusercontent.com/eriirfos-eng/ternary-intelligence-stack/main/albert-moe-13/scripts/setup_collaborator.sh | bash
#
# What this does:
#   0. Installs GitHub CLI (gh) + authenticates
#   1. Installs Rust (if missing)
#   2. Installs Python deps (safetensors, numpy; modal optional)
#   3. Clones the training repo via gh
#   4. Builds albert-train / albert-test binaries
#   5. Installs albert-train (CPU), albert-test, albert-spore into ~/bin
#   6. Clones the private albert-spores repo via gh
#   7. Prints next steps

set -euo pipefail

BOLD="\033[1m"; GREEN="\033[1;92m"; CYAN="\033[96m"; YELLOW="\033[93m"; RED="\033[91m"; R="\033[0m"
info()  { echo -e "${CYAN}[albert-setup]${R} $*"; }
ok()    { echo -e "${GREEN}[albert-setup]${R} $*"; }
warn()  { echo -e "${YELLOW}[albert-setup]${R} $*"; }
die()   { echo -e "${RED}[albert-setup] ERROR:${R} $*"; exit 1; }

PROJECT_DIR="$HOME/projects/ternary-intelligence-stack"
ALBERT_DIR="$PROJECT_DIR/albert-moe-13"
SPORES_DIR="$HOME/projects/albert-spores"
BIN_DIR="$HOME/bin"

echo ""
echo -e "${BOLD}══════════════════════════════════════════════════${R}"
echo -e "${BOLD}  albert. collaborator setup                      ${R}"
echo -e "${BOLD}══════════════════════════════════════════════════${R}"
echo ""

# ── 0. GitHub CLI ────────────────────────────────────────────────────────────
if ! command -v gh &>/dev/null; then
    info "Installing GitHub CLI (gh)..."
    if command -v apt-get &>/dev/null; then
        curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
            | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg 2>/dev/null
        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" \
            | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
        sudo apt-get update -qq && sudo apt-get install -y gh
    elif command -v dnf &>/dev/null; then
        sudo dnf install -y gh
    elif command -v pacman &>/dev/null; then
        sudo pacman -S --noconfirm github-cli
    else
        die "Cannot auto-install gh — install manually: https://cli.github.com and re-run"
    fi
    ok "GitHub CLI installed: $(gh --version | head -1)"
else
    ok "GitHub CLI already installed: $(gh --version | head -1)"
fi

# Authenticate with GitHub if not already
if ! gh auth status &>/dev/null; then
    echo ""
    echo -e "${BOLD}  GitHub login required — a browser window will open.${R}"
    echo -e "  Follow the prompts, then return here."
    echo ""
    gh auth login --web --git-protocol https
    ok "GitHub authenticated as $(gh api user --jq .login)"
else
    ok "GitHub already authenticated as $(gh api user --jq .login)"
fi

# ── 1. Rust ──────────────────────────────────────────────────────────────────
if ! command -v cargo &>/dev/null; then
    info "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    source "$HOME/.cargo/env"
    ok "Rust installed: $(rustc --version)"
else
    ok "Rust already installed: $(rustc --version)"
    source "$HOME/.cargo/env" 2>/dev/null || true
fi

# ── 2. Python deps ───────────────────────────────────────────────────────────
info "Installing Python dependencies..."
pip3 install --quiet --upgrade safetensors numpy 2>/dev/null || \
    pip  install --quiet --upgrade safetensors numpy 2>/dev/null || \
    warn "pip install had warnings — safetensors/numpy may need manual install"
ok "Python deps ready"

# ── 3. Clone main repo ───────────────────────────────────────────────────────
mkdir -p "$HOME/projects"
if [ -d "$PROJECT_DIR/.git" ]; then
    info "Repo already cloned — pulling latest..."
    git -C "$PROJECT_DIR" pull --ff-only
else
    info "Cloning training repo via gh..."
    gh repo clone eriirfos-eng/ternary-intelligence-stack "$PROJECT_DIR"
fi
ok "Repo ready at $PROJECT_DIR"

# ── 4. Build binaries ────────────────────────────────────────────────────────
info "Building moe-llm-core binaries (first build ~3-5 min)..."
cd "$ALBERT_DIR"
cargo build --release --bin train_bible --bin moe-test 2>&1 \
    | grep -E "^(   Compiling|   Finished|error\[)" | tail -10 || true
if [ ! -f "$ALBERT_DIR/target/release/moe-test" ]; then
    die "Build failed — check Cargo errors above"
fi
if [ ! -f "$ALBERT_DIR/target/release/train_bible" ]; then
    die "train_bible build failed — check Cargo errors above"
fi
ok "Binaries built"

# ── 5. Install albert-* commands ─────────────────────────────────────────────
mkdir -p "$BIN_DIR"
info "Installing albert-train (CPU), albert-test, albert-spore into $BIN_DIR..."

# albert-train — runs LOCAL CPU training by default; pass --modal for GPU
cat > "$BIN_DIR/albert-train" << 'TRAIN_EOF'
#!/usr/bin/env python3
"""
albert-train — run albert. training

  albert-train             # local CPU training (default for collaborators)
  albert-train --modal     # GPU training on Modal (requires modal token new)
  albert-train pull        # pull checkpoint from Modal volume
  albert-train --no-update # skip the auto git-pull at startup
"""
import os, sys, subprocess, threading, signal, time, webbrowser

PROJECT  = os.path.expanduser("~/projects/ternary-intelligence-stack/albert-moe-13")
REPO     = os.path.expanduser("~/projects/ternary-intelligence-stack")
BINARY   = os.path.join(PROJECT, "target", "release", "train_bible")
MODAL_PY = os.path.join(PROJECT, "train_modal.py")
LOG      = os.path.expanduser("~/.albert/training.log")   # matches dashboard server path
DASH_SRV = os.path.join(PROJECT, "dashboard", "run_server.py")

os.makedirs(os.path.expanduser("~/.albert"), exist_ok=True)

R="\033[0m"; CYAN="\033[96m"; GREEN="\033[1;92m"; BOLD="\033[1;94m"; YELLOW="\033[93m"; DIM="\033[2m"

# pull subcommand — syncs from Modal volume
if len(sys.argv) > 1 and sys.argv[1] == "pull":
    os.chdir(PROJECT)
    sys.exit(subprocess.run([sys.executable, MODAL_PY, "pull"]).returncode)

use_modal  = "--modal"      in sys.argv
no_browser = "--no-browser" in sys.argv
no_update  = "--no-update"  in sys.argv

# Auto-pull latest code + dashboard (keeps index.html and training binary in sync)
if not no_update:
    try:
        r = subprocess.run(
            ["git", "-C", REPO, "pull", "--ff-only", "--quiet"],
            capture_output=True, text=True, timeout=15
        )
        if r.returncode == 0 and r.stdout.strip():
            print(f"{GREEN}[albert-train] repo updated — rebuilding binary...{R}")
            subprocess.run(["cargo","build","--release","--bin","train_bible"], cwd=PROJECT, check=False)
        elif r.returncode != 0:
            print(f"{DIM}[albert-train] git pull skipped: {r.stderr.strip()}{R}")
    except Exception:
        pass  # network unavailable — proceed with local version

# Rebuild if binary missing
if not use_modal and not os.path.exists(BINARY):
    print(f"{YELLOW}[albert-train] binary not found — rebuilding...{R}")
    r = subprocess.run(["cargo","build","--release","--bin","train_bible"], cwd=PROJECT)
    if r.returncode != 0:
        sys.exit(r.returncode)

# Start dashboard server with --cpu flag for collaborator-appropriate polling rates
server_proc = subprocess.Popen(
    [sys.executable, DASH_SRV, "--cpu"],
    cwd=PROJECT,
    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
)
time.sleep(0.5)
if not no_browser:
    try: webbrowser.open("http://localhost:8888/dashboard/")
    except: pass

if use_modal:
    print(f"{BOLD}--- albert. Training (Modal GPU) ---{R}")
    detach = "--detach" in sys.argv
    cmd    = ["modal","run"] + (["--detach"] if detach else []) + [MODAL_PY]
else:
    print(f"{BOLD}--- albert. Training (CPU — spore mode) ---{R}")
    print(f"{CYAN}Dashboard: http://localhost:8888/dashboard/{R}")
    print(f"{CYAN}Stop with Ctrl-C — then run  albert-spore  to push a spore{R}\n")
    cmd = [
        BINARY,
        f"--root={PROJECT}",
        "--gate-diversity=0.01",
        "--lb-weight=0.01",
        "--div-weight=0.001",
    ]

open(LOG, "w").close()
log_f = open(LOG, "a")

train_proc = subprocess.Popen(
    cmd, cwd=PROJECT,
    stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
    text=True, bufsize=1,
)

def stream():
    global log_f
    for line in train_proc.stdout:
        if line.startswith("RUN_START "):
            log_f.close(); open(LOG,"w").close(); log_f = open(LOG,"a")
        sys.stdout.write(line); sys.stdout.flush()
        log_f.write(line); log_f.flush()

threading.Thread(target=stream, daemon=True).start()
signal.signal(signal.SIGINT, lambda s,f: train_proc.send_signal(signal.SIGINT))
train_proc.wait()
log_f.close()

print(f"\n{BOLD}--- Training stopped ---{R}")
print(f"{CYAN}Push your spore:  albert-spore{R}")
print(f"{CYAN}Dashboard:        http://localhost:8888/dashboard/{R}")
TRAIN_EOF
chmod +x "$BIN_DIR/albert-train"

# albert-test — interactive TUI
cat > "$BIN_DIR/albert-test" << 'TEST_EOF'
#!/usr/bin/env python3
"""albert-test — interactive TUI for albert. (chat, /bench, /export)"""
import os, sys, subprocess

PROJECT = os.path.expanduser("~/projects/ternary-intelligence-stack/albert-moe-13")
BINARY  = os.path.join(PROJECT, "target", "release", "moe-test")

if "--rebuild" in sys.argv or not os.path.exists(BINARY):
    r = subprocess.run(["cargo","build","--release","--bin","moe-test"], cwd=PROJECT)
    if r.returncode != 0: sys.exit(r.returncode)

args = [a for a in sys.argv[1:] if a != "--rebuild"]
os.chdir(PROJECT)
os.execv(BINARY, [BINARY] + args)
TEST_EOF
chmod +x "$BIN_DIR/albert-test"

# albert-spore — export checkpoint and push to albert-spores
cat > "$BIN_DIR/albert-spore" << 'SPORE_EOF'
#!/usr/bin/env bash
# albert-spore — produce a spore from current checkpoint and push to albert-spores
PROJECT="$HOME/projects/ternary-intelligence-stack/albert-moe-13"
SPORES="$HOME/projects/albert-spores"
if [ ! -d "$SPORES/.git" ]; then
    echo "[albert-spore] albert-spores repo not found at $SPORES"
    echo "               Run setup again or: gh repo clone eriirfos-eng/albert-spores $SPORES"
    exit 1
fi
python3 "$PROJECT/scripts/produce_spore.py" --spores-repo "$SPORES" "$@"
SPORE_EOF
chmod +x "$BIN_DIR/albert-spore"

ok "Commands installed: albert-train (CPU), albert-test, albert-spore"

# ── 6. PATH setup ────────────────────────────────────────────────────────────
SHELL_RC="$HOME/.bashrc"
if ! grep -q 'PATH="$HOME/bin' "$SHELL_RC" 2>/dev/null; then
    { echo ''; echo '# albert. CLI'; echo 'export PATH="$HOME/bin:$HOME/.cargo/bin:$PATH"'; } >> "$SHELL_RC"
    info "Added ~/bin to PATH in .bashrc"
fi
export PATH="$BIN_DIR:$HOME/.cargo/bin:$PATH"

# ── 7. Clone spores repo ─────────────────────────────────────────────────────
if [ -d "$SPORES_DIR/.git" ]; then
    info "albert-spores already cloned — pulling..."
    git -C "$SPORES_DIR" pull --ff-only 2>/dev/null || true
else
    info "Cloning albert-spores (private)..."
    gh repo clone eriirfos-eng/albert-spores "$SPORES_DIR" || \
        warn "Could not clone albert-spores — accept the GitHub invitation first, then run: gh repo clone eriirfos-eng/albert-spores ~/projects/albert-spores"
fi

# ── Done ─────────────────────────────────────────────────────────────────────
echo ""
echo -e "${GREEN}══════════════════════════════════════════════════${R}"
echo -e "${GREEN}  Setup complete!                                 ${R}"
echo -e "${GREEN}══════════════════════════════════════════════════${R}"
echo ""
echo -e "  ${BOLD}Your commands:${R}"
echo ""
echo -e "  ${CYAN}albert-train${R}               start CPU training + dashboard"
echo -e "  ${CYAN}albert-train --modal${R}       GPU training on Modal (optional)"
echo -e "  ${CYAN}albert-test${R}                chat with albert."
echo -e "  ${CYAN}albert-spore${R}               push spore to the colony"
echo ""
echo -e "  ${BOLD}First-time steps:${R}"
echo ""
echo -e "  Pull the current checkpoint from Modal:"
echo -e "    ${CYAN}albert-train pull${R}"
echo ""
echo -e "  Then start training:"
echo -e "    ${CYAN}albert-train${R}"
echo ""
echo -e "  When ready to contribute:"
echo -e "    ${CYAN}albert-spore${R}              (uses your GitHub login automatically)"
echo ""
echo -e "  Reload shell: ${CYAN}source ~/.bashrc${R}"
echo ""
