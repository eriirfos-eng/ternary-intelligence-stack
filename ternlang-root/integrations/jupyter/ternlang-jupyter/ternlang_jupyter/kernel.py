"""
TernlangKernel — Jupyter kernel for the Ternlang balanced ternary language.

Wraps the ternlang-cli binary. Each cell is a complete .tern program.
Print output streams to the notebook; the final trit result is displayed
as a styled rich output block.

© RFI-IRFOS · LGPL-3.0-or-later · Patent A50296/2026
"""

import os
import re
import subprocess
import tempfile
from ipykernel.kernelbase import Kernel

__version__ = "1.0.0"

# ── Rich HTML output templates ────────────────────────────────────────────────

_STYLE = """
<style>
  .tern-result {
    display: inline-block;
    padding: 6px 18px;
    border-radius: 6px;
    font-family: monospace;
    font-size: 1.1em;
    font-weight: bold;
    letter-spacing: 0.04em;
  }
  .tern-affirm  { background:#0d2b1a; color:#00ff88; border:1px solid #00ff88; }
  .tern-hold    { background:#1a1a0d; color:#ffcc00; border:1px solid #ffcc00; }
  .tern-reject  { background:#2b0d0d; color:#ff3366; border:1px solid #ff3366; }
  .tern-error   { background:#1a0d0d; color:#ff6688; border:1px solid #ff3366;
                  white-space:pre-wrap; font-size:0.9em; padding:8px 14px; }
  .tern-meta    { color:#888; font-family:monospace; font-size:0.8em;
                  margin-top:4px; }
</style>
"""

_TRIT_HTML = {
    1:  '<div class="tern-result tern-affirm">AFFIRM  +1</div>',
    0:  '<div class="tern-result tern-hold">HOLD  0</div>',
    -1: '<div class="tern-result tern-reject">REJECT  −1</div>',
}


def _result_html(trit: int, cycles: int, label: str) -> str:
    block = _TRIT_HTML.get(trit, _TRIT_HTML[0])
    meta = f'<div class="tern-meta">BET VM · {cycles} bytes · ternlang v1.0.0</div>'
    return _STYLE + block + meta


def _error_html(msg: str) -> str:
    safe = msg.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
    return _STYLE + f'<div class="tern-result tern-error">{safe}</div>'


# ── Kernel ─────────────────────────────────────────────────────────────────────

class TernlangKernel(Kernel):
    implementation         = "ternlang"
    implementation_version = __version__
    language               = "ternlang"
    language_version       = "1.0.0"
    language_info = {
        "name":            "ternlang",
        "version":         "1.0.0",
        "mimetype":        "text/x-ternlang",
        "file_extension":  ".tern",
        "codemirror_mode": "text/plain",
        "pygments_lexer":  "text",
    }
    banner = (
        "Ternlang BET VM kernel v1.0.0\n"
        "Balanced ternary · affirm(+1) / hold(0) / reject(−1)\n"
        "RFI-IRFOS · https://ternlang.com"
    )

    # ── public ────────────────────────────────────────────────────────────────

    def do_execute(self, code, silent, store_history=True,
                   user_expressions=None, allow_stdin=False):
        code = code.strip()
        if not code:
            return self._ok()

        # Magic commands
        if code.startswith("%"):
            return self._handle_magic(code, silent)

        cli = self._find_cli()
        if cli is None:
            self._send_error(
                "ternlang-cli not found.\n"
                "Install with:  cargo install ternlang-cli\n"
                "Then ensure 'ternlang' is on your PATH.",
                silent,
            )
            return self._err()

        with tempfile.NamedTemporaryFile(suffix=".tern", mode="w",
                                         delete=False) as f:
            f.write(code)
            tmp = f.name

        try:
            proc = subprocess.run(
                [cli, "run", tmp],
                capture_output=True, text=True, timeout=30,
            )
        except subprocess.TimeoutExpired:
            self._send_error("Execution timed out (30 s).", silent)
            return self._err()
        finally:
            os.unlink(tmp)

        # Parse trit result first — ternlang exits 0 for affirm/hold, 1 for
        # reject (valid result).  Real errors produce stderr with no result line.
        trit, label = self._parse_trit(proc.stdout, proc.stderr)
        has_result = re.search(r"result\s+[+-]?\d", proc.stdout + proc.stderr,
                               re.IGNORECASE) is not None

        # Stream print() output lines (everything before the separator block)
        if not silent:
            for line in proc.stdout.splitlines():
                # Skip the CLI banner/separator lines — only stream program output
                if not re.match(r"^\s*[─┄━─]{4}", line):
                    stripped = line.strip()
                    if stripped and not re.match(
                        r"(result|compiled|ternlang|tip:|→|✓|program exited)", stripped
                    ):
                        self._stream(stripped)

        if proc.returncode != 0 and not has_result:
            stderr = proc.stderr.strip() or proc.stdout.strip() or "non-zero exit"
            if not silent:
                self._display_html(_error_html(stderr))
            return self._err()

        if not silent:
            cycles = self._parse_cycles(proc.stdout)
            self._display_html(_result_html(trit, cycles, label))

        return self._ok()

    def do_complete(self, code, cursor_pos):
        keywords = [
            "fn", "let", "match", "return", "if", "else", "while", "for", "in",
            "affirm", "hold", "reject", "tend",
            "trit", "int", "float", "string", "bool",
            "struct", "agent", "spawn", "send", "await",
            "true", "false", "nodeid",
            "print", "len", "abs", "min", "max", "pow", "invert",
        ]
        token = re.split(r"[\s\(\)\{\}\[\],;]", code[:cursor_pos])[-1]
        matches = [k for k in keywords if k.startswith(token)]
        return {
            "matches": matches,
            "cursor_start": cursor_pos - len(token),
            "cursor_end": cursor_pos,
            "metadata": {},
            "status": "ok",
        }

    def do_inspect(self, code, cursor_pos, detail_level=0):
        docs = {
            "affirm": "Trit literal +1. Positive signal — proceed.",
            "hold":   "Trit literal 0. Insufficient evidence — wait.",
            "reject": "Trit literal -1. Negative signal — block.",
            "tend":   "Alias for hold (trit 0).",
            "match":  "Ternary pattern match. Arms: -1 => {}, 0 => {}, 1 => {} or _ => {}",
            "fn":     "Function definition. Example: fn name(x: trit) -> trit { ... }",
            "trit":   "Ternary scalar type. Values: -1 (reject), 0 (hold/tend), +1 (affirm).",
            "print":  "Built-in: print(value) — outputs to notebook stream.",
            "len":    "Built-in: len(x) — length of string or tensor.",
        }
        token = re.split(r"[\s\(\)\{\}\[\],;]", code[:cursor_pos])[-1]
        text = docs.get(token, "")
        return {"status": "ok", "found": bool(text),
                "data": {"text/plain": text}, "metadata": {}}

    # ── internals ─────────────────────────────────────────────────────────────

    def _find_cli(self):
        import shutil
        # Respect TERNLANG_CLI env var, then PATH
        env_path = os.environ.get("TERNLANG_CLI")
        if env_path and os.path.isfile(env_path):
            return env_path
        return shutil.which("ternlang")

    def _parse_trit(self, stdout: str, stderr: str) -> tuple[int, str]:
        """Extract the final trit from CLI output.

        ternlang-cli v1.0.0 prints to stdout:
            result     +1  affirm
            result      0  hold
            result     -1  reject
        """
        combined = (stdout + "\n" + stderr).lower()
        for line in reversed(combined.splitlines()):
            # Primary: "result  +1  affirm" format
            if re.search(r"result\s+\+1", line) or re.search(r"result\s+1\b", line):
                return 1, "affirm"
            if re.search(r"result\s+-1", line):
                return -1, "reject"
            if re.search(r"result\s+0\b", line):
                return 0, "hold"
            # Fallback keyword scan
            if "affirm" in line and "result" in line:
                return 1, "affirm"
            if ("reject" in line or "conflict" in line) and "result" in line:
                return -1, "reject"
            if ("hold" in line or "tend" in line) and "result" in line:
                return 0, "hold"
        return 0, "hold"

    def _parse_cycles(self, output: str) -> int:
        """Extract compiled byte count from CLI stdout (e.g. 'compiled   14 bytes')."""
        m = re.search(r"compiled\s+(\d+)\s+byte", output)
        return int(m.group(1)) if m else 0

    def _stream(self, text: str):
        self.send_response(self.iopub_socket, "stream",
                           {"name": "stdout", "text": text + "\n"})

    def _display_html(self, html: str):
        self.send_response(self.iopub_socket, "display_data", {
            "data": {"text/html": html, "text/plain": ""},
            "metadata": {},
        })

    def _send_error(self, msg: str, silent: bool):
        if not silent:
            self._display_html(_error_html(msg))

    def _handle_magic(self, code: str, silent: bool) -> dict:
        cmd = code.lstrip("%").strip()
        if cmd in ("version", "ver"):
            self._stream(f"ternlang-jupyter v{__version__}  |  BET VM v1.0.0")
        elif cmd == "help":
            self._stream(
                "Ternlang Jupyter kernel — magic commands:\n"
                "  %version  — show kernel version\n"
                "  %help     — this message\n\n"
                "Each cell is a complete .tern program (needs fn main() -> trit).\n"
                "Set TERNLANG_CLI env var to override the CLI path."
            )
        else:
            self._stream(f"Unknown magic: {cmd}. Try %help")
        return self._ok()

    @staticmethod
    def _ok():
        return {"status": "ok", "execution_count": None,
                "payload": [], "user_expressions": {}}

    @staticmethod
    def _err():
        return {"status": "error", "execution_count": None,
                "ename": "TernlangError", "evalue": "", "traceback": []}
