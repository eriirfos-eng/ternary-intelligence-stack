#!/usr/bin/env bash
# albert. collaborator setup — one-line drop-in for Linux
#
# Usage (run this on the collaborator's machine):
#   curl -sSL https://raw.githubusercontent.com/eriirfos-eng/ternary-intelligence-stack/main/albert-moe-13/scripts/setup_collaborator.sh | bash
#
# What this does:
#   1. Installs Rust (if missing)
#   2. Installs Modal CLI + Python deps
#   3. Clones the training repo
#   4. Builds albert-train / albert-test binaries
#   5. Installs the albert-* commands into ~/bin
#   6. Clones the spores repo (albert-spores)
#   7. Prints next steps for Modal auth

set -euo pipefail

BOLD="\033[1m"; GREEN="\033[1;92m"; CYAN="\033[96m"; YELLOW="\033[93m"; RED="\033[91m"; R="\033[0m"
info()  { echo -e "${CYAN}[albert-setup]${R} $*"; }
ok()    { echo -e "${GREEN}[albert-setup]${R} $*"; }
warn()  { echo -e "${YELLOW}[albert-setup]${R} $*"; }
die()   { echo -e "${RED}[albert-setup] ERROR:${R} $*"; exit 1; }

REPO_URL="https://github.com/eriirfos-eng/ternary-intelligence-stack.git"
SPORES_URL="https://github.com/eriirfos-eng/albert-spores.git"
PROJECT_DIR="$HOME/projects/ternary-intelligence-stack"
ALBERT_DIR="$PROJECT_DIR/albert-moe-13"
BIN_DIR="$HOME/bin"

echo ""
echo -e "${BOLD}══════════════════════════════════════════════════${R}"
echo -e "${BOLD}  albert. collaborator setup                      ${R}"
echo -e "${BOLD}══════════════════════════════════════════════════${R}"
echo ""

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
pip3 install --quiet --upgrade modal safetensors numpy 2>/dev/null || \
    pip install --quiet --upgrade modal safetensors numpy 2>/dev/null || \
    warn "pip install had warnings — modal/safetensors may need manual install"
ok "Python deps installed"

# ── 3. Clone main repo ───────────────────────────────────────────────────────
mkdir -p "$HOME/projects"
if [ -d "$PROJECT_DIR/.git" ]; then
    info "Repo already cloned — pulling latest..."
    git -C "$PROJECT_DIR" pull --ff-only
else
    info "Cloning training repo..."
    git clone "$REPO_URL" "$PROJECT_DIR"
fi
ok "Repo ready at $PROJECT_DIR"

# ── 4. Build binaries ────────────────────────────────────────────────────────
info "Building moe-llm-core binaries (first build ~3-5 min)..."
cd "$ALBERT_DIR"
cargo build --release -p train_bible -p moe-test 2>&1 | grep -E "^(   Compiling|   Finished|error)" | head -20 || true
if [ ! -f "$ALBERT_DIR/target/release/moe-test" ]; then
    die "Build failed — check Cargo errors above"
fi
ok "Binaries built"

# ── 5. Install albert-* commands ─────────────────────────────────────────────
mkdir -p "$BIN_DIR"
info "Installing albert-train / albert-test / albert-run into $BIN_DIR..."
cp "$HOME/projects/ternary-intelligence-stack/albert-moe-13/../$(basename $ALBERT_DIR)/../albert-moe-13"/../bin/albert-train "$BIN_DIR/" 2>/dev/null || true

# Write albert-train adapted to this machine
cat > "$BIN_DIR/albert-train" << 'TRAIN_EOF'
#!/usr/bin/env python3
"""albert-train — start GPU training on Modal + local dashboard"""
import os, sys, subprocess, threading, signal, time, webbrowser, re

PROJECT  = os.path.expanduser("~/projects/ternary-intelligence-stack/albert-moe-13")
MODAL_PY = os.path.join(PROJECT, "train_modal.py")
LOG      = os.path.join(PROJECT, "dashboard", "training.log")
DASH_SRV = os.path.join(PROJECT, "dashboard", "run_server.py")

R="\033[0m"; BLUE="\033[38;5;33m"; GREEN="\033[1;92m"; CYAN="\033[96m"; BOLD="\033[1;94m"

if len(sys.argv) > 1 and sys.argv[1] == "pull":
    os.chdir(PROJECT)
    sys.exit(subprocess.run([sys.executable, MODAL_PY, "pull"]).returncode)

detach     = "--detach"     in sys.argv
no_browser = "--no-browser" in sys.argv

print(f"{BOLD}--- albert. Training Orchestrator (Modal GPU) ---{R}")
server_proc = subprocess.Popen([sys.executable, DASH_SRV], cwd=os.path.join(PROJECT,"dashboard"),
                                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
time.sleep(0.5)
if not no_browser:
    try: webbrowser.open("http://localhost:8888/dashboard/")
    except: pass

modal_cmd = ["modal", "run"] + (["--detach"] if detach else []) + [MODAL_PY]
open(LOG, "w").close()
log_f = open(LOG, "a")
train_proc = subprocess.Popen(modal_cmd, cwd=PROJECT, stdout=subprocess.PIPE,
                               stderr=subprocess.STDOUT, text=True, bufsize=1)

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
print(f"\n{CYAN}Dashboard: http://localhost:8888/dashboard/{R}")
TRAIN_EOF
chmod +x "$BIN_DIR/albert-train"

# albert-test
cat > "$BIN_DIR/albert-test" << 'TEST_EOF'
#!/usr/bin/env python3
"""albert-test — interactive TUI for albert."""
import os, sys, subprocess

PROJECT = os.path.expanduser("~/projects/ternary-intelligence-stack/albert-moe-13")
BINARY  = os.path.join(PROJECT, "target", "release", "moe-test")

if "--rebuild" in sys.argv or not os.path.exists(BINARY):
    r = subprocess.run(["cargo", "build", "--release", "-p", "moe-test"], cwd=PROJECT)
    if r.returncode != 0: sys.exit(r.returncode)

args = [a for a in sys.argv[1:] if a != "--rebuild"]
os.chdir(PROJECT)
os.execv(BINARY, [BINARY] + args)
TEST_EOF
chmod +x "$BIN_DIR/albert-test"

# albert-spore (produce a spore from current checkpoint)
cat > "$BIN_DIR/albert-spore" << 'SPORE_EOF'
#!/usr/bin/env bash
# albert-spore — produce and push a spore from the current best checkpoint
PROJECT="$HOME/projects/ternary-intelligence-stack/albert-moe-13"
SPORES="$HOME/projects/albert-spores"
python3 "$PROJECT/scripts/produce_spore.py" --spores-repo "$SPORES" "$@"
SPORE_EOF
chmod +x "$BIN_DIR/albert-spore"

ok "Commands installed: albert-train, albert-test, albert-spore"

# ── 6. PATH setup ────────────────────────────────────────────────────────────
SHELL_RC="$HOME/.bashrc"
if ! grep -q 'export PATH="$HOME/bin:' "$SHELL_RC" 2>/dev/null; then
    echo '' >> "$SHELL_RC"
    echo '# albert. CLI' >> "$SHELL_RC"
    echo 'export PATH="$HOME/bin:$HOME/.cargo/bin:$PATH"' >> "$SHELL_RC"
    info "Added ~/bin to PATH in .bashrc"
fi
export PATH="$BIN_DIR:$HOME/.cargo/bin:$PATH"

# ── 7. Clone spores repo ─────────────────────────────────────────────────────
SPORES_DIR="$HOME/projects/albert-spores"
if [ -d "$SPORES_DIR/.git" ]; then
    info "Spores repo already cloned — pulling..."
    git -C "$SPORES_DIR" pull --ff-only 2>/dev/null || true
else
    info "Cloning albert-spores (private — needs GitHub access)..."
    git clone "$SPORES_URL" "$SPORES_DIR" 2>/dev/null || \
        warn "Could not clone albert-spores — ask Simeon to add your GitHub account as collaborator"
fi

# ── Done ─────────────────────────────────────────────────────────────────────
echo ""
echo -e "${GREEN}══════════════════════════════════════════════════${R}"
echo -e "${GREEN}  Setup complete!                                 ${R}"
echo -e "${GREEN}══════════════════════════════════════════════════${R}"
echo ""
echo -e "  ${BOLD}Next steps:${R}"
echo ""
echo -e "  1. Authenticate with Modal (one-time):"
echo -e "     ${CYAN}modal token new${R}"
echo ""
echo -e "  2. Pull the current albert. checkpoint from Modal volume:"
echo -e "     ${CYAN}albert-train pull${R}"
echo ""
echo -e "  3. Start training:"
echo -e "     ${CYAN}albert-train${R}"
echo ""
echo -e "  4. Chat with albert.:"
echo -e "     ${CYAN}albert-test${R}"
echo ""
echo -e "  5. Produce a spore (after training):"
echo -e "     ${CYAN}albert-spore --name yourname${R}"
echo ""
echo -e "  Reload your shell or run: ${CYAN}source ~/.bashrc${R}"
echo ""
