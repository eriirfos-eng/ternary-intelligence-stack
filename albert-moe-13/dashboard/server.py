import http.server
import io
import json
import socketserver
import os
import re
import subprocess
import sys
import threading
import time
import urllib.parse
from collections import defaultdict

PORT = 8888
DIRECTORY = os.path.dirname(os.path.abspath(__file__))
PROJECT   = os.path.dirname(DIRECTORY)
MERGE_PY  = os.path.join(PROJECT, "scripts", "merge_batch_history.py")
MERGE_INTERVAL = 15 * 60  # seconds

_BEST_MODEL = os.path.join(PROJECT, "models", "albert_v3.0.best.safetensors")
_VOCAB_FILE = os.path.join(PROJECT, "tokenizer_v3", "tokenizer_v3.json")


class MyceliumEngine:
    """Lazy-loading embedding space explorer with background checkpoint watching.

    Loads embed.weight from the best checkpoint on first query, computes a PCA
    projection over all 32k tokens (cached), then serves cosine-similarity
    neighbourhood queries in O(V) time.  A background watcher thread polls the
    checkpoint mtime every 60 s and reloads transparently when it changes (e.g.
    after albert-train pull or a net2net surgery).  Old data is served
    uninterrupted during reload.
    """

    def __init__(self, model_path: str, vocab_path: str):
        self._model_path      = model_path
        self._vocab_path      = vocab_path
        self._lock            = threading.Lock()
        self._loaded          = False
        self._loading         = False
        self._watcher_started = False
        self._embeddings      = None   # (V, D) float32, L2-normalised
        self._vocab           = None   # {token_str: int}
        self._id_to_tok       = None   # {int: token_str}
        self._coords          = None   # (V, 2) float32 PCA
        self._mtime           = None   # mtime of loaded checkpoint
        self._load_time       = None   # unix timestamp of last successful load
        self._error           = None   # last reload error message

    def _load(self):
        import numpy as np
        from safetensors import safe_open

        model_path    = self._model_path
        current_mtime = os.path.getmtime(model_path)

        print("[mycelium] loading embeddings ...", flush=True)
        with safe_open(model_path, framework='numpy') as f:
            emb = f.get_tensor('embed.weight').astype(np.float32)  # (V, D)

        # L2-normalise for cosine similarity
        norms = np.linalg.norm(emb, axis=1, keepdims=True)
        emb_normed = emb / np.maximum(norms, 1e-8)

        # PCA via covariance eigenvectors — fast on (D, D) cov matrix
        print("[mycelium] computing PCA ...", flush=True)
        centered = emb - emb.mean(axis=0)
        cov = (centered.T @ centered) / len(centered)   # (D, D)
        _, eigvecs = np.linalg.eigh(cov)                # ascending eigenvalues
        pcs = eigvecs[:, ::-1][:, :2].T                 # (2, D) top-2 PCs
        coords = (centered @ pcs.T).astype(np.float32)  # (V, 2)

        for i in range(2):
            mx = np.abs(coords[:, i]).max()
            if mx > 0:
                coords[:, i] /= mx

        with open(self._vocab_path, encoding='utf-8') as fv:
            t = json.load(fv)
        vocab     = t['model']['vocab']
        id_to_tok = {v: k for k, v in vocab.items()}

        # Atomic swap — _embeddings is written last so query() sees consistent state.
        # CPython GIL guarantees each attribute assignment is atomic.
        self._vocab      = vocab
        self._id_to_tok  = id_to_tok
        self._coords     = coords
        self._mtime      = current_mtime
        self._load_time  = time.time()
        self._error      = None
        self._embeddings = emb_normed   # <- write last; query() snapshots this first
        self._loaded     = True
        print(f"[mycelium] ready — {len(vocab)} tokens, dim={emb.shape[1]}", flush=True)

    def ensure_loaded(self):
        if self._loaded:
            return
        with self._lock:
            if not self._loaded:
                self._loading = True
                try:
                    self._load()
                finally:
                    self._loading = False
                if not self._watcher_started:
                    self._watcher_started = True
                    self._start_watcher()

    # ── background checkpoint watcher ─────────────────────────────────────

    def _check_for_update(self):
        if self._loading:
            return
        try:
            current_mtime = os.path.getmtime(self._model_path)
        except OSError:
            return
        if current_mtime <= (self._mtime or 0):
            return
        self._loading = True
        threading.Thread(target=self._reload_bg, daemon=True).start()

    def _reload_bg(self):
        try:
            self._load()
        except Exception as exc:
            self._error   = str(exc)
            self._loading = False
            print(f"[mycelium] reload error: {exc}", flush=True)
        else:
            self._loading = False

    def _start_watcher(self):
        def _watch():
            while True:
                time.sleep(60)
                self._check_for_update()
        threading.Thread(target=_watch, daemon=True).start()

    # ── status ─────────────────────────────────────────────────────────────

    def status(self) -> dict:
        return {
            'loaded':     self._loaded,
            'loading':    self._loading,
            'mtime':      self._mtime,
            'load_time':  self._load_time,
            'vocab_size': len(self._vocab) if self._vocab else 0,
            'dims':       int(self._embeddings.shape[1]) if self._embeddings is not None else 0,
            'checkpoint': os.path.basename(self._model_path),
            'error':      self._error,
        }

    # ── query ──────────────────────────────────────────────────────────────

    def query(self, word: str, k: int = 40) -> dict:
        import numpy as np
        self.ensure_loaded()

        # Snapshot all array refs for thread safety during concurrent reload.
        embeddings = self._embeddings
        vocab      = self._vocab
        id_to_tok  = self._id_to_tok
        coords     = self._coords

        # Resolve token: prefer space-prefixed (standalone word in running text)
        SPACE = 'Ġ'
        token_id = vocab.get(SPACE + word)
        resolved = SPACE + word
        if token_id is None:
            token_id = vocab.get(word)
            resolved = word
        if token_id is None:
            # Fuzzy: substring match, prefer space-prefixed and longer tokens
            matches = [(tok, tid) for tok, tid in vocab.items()
                       if word.lower() in tok.lower()]
            if not matches:
                return {'error': f'token not found: {word!r}', 'word': word}
            matches.sort(key=lambda x: (not x[0].startswith(SPACE), -len(x[0])))
            resolved, token_id = matches[0]

        # Cosine similarity (dot product on normalised embeddings)
        sims = embeddings @ embeddings[token_id]   # (V,)

        # top-(k+1), drop self
        top_ids = int(k) + 1
        part = np.argpartition(sims, -top_ids)[-top_ids:]
        part = part[np.argsort(sims[part])[::-1]]
        part = [int(i) for i in part if int(i) != token_id][:k]

        neighbors = []
        for nid in part:
            tok = id_to_tok.get(nid, f'[{nid}]')
            display = tok.replace(SPACE, ' ').strip() or tok
            neighbors.append({
                'id':    nid,
                'token': tok,
                'word':  display,
                'x':     float(coords[nid, 0]),
                'y':     float(coords[nid, 1]),
                'sim':   round(float(sims[nid]), 4),
            })

        center_display = resolved.replace(SPACE, ' ').strip() or resolved
        return {
            'word':       center_display,
            'token':      resolved,
            'id':         int(token_id),
            'x':          float(coords[token_id, 0]),
            'y':          float(coords[token_id, 1]),
            'neighbors':  neighbors,
            'vocab_size': len(vocab),
            'dims':       int(embeddings.shape[1]),
        }


_mycelium = MyceliumEngine(_BEST_MODEL, _VOCAB_FILE)


def _auto_merge_loop():
    """Background thread: merge Downloads CSVs into batch_history every 15 minutes."""
    while True:
        time.sleep(MERGE_INTERVAL)
        try:
            result = subprocess.run(
                [sys.executable, MERGE_PY],
                cwd=PROJECT, capture_output=True, text=True, timeout=120,
            )
            new_pts = "0"
            for line in result.stdout.splitlines():
                if "Total unique points" in line:
                    m = re.search(r'\+([0-9,]+)\s*\)', line)
                    if m:
                        new_pts = m.group(1).replace(",", "")
            if int(new_pts) == 0:
                continue
            subprocess.run(["git", "add", "dashboard/batch_history.csv"], cwd=PROJECT)
            msg = f"data: auto-merge batch_history +{new_pts} points from Downloads"
            r = subprocess.run(["git", "commit", "-m", msg], cwd=PROJECT,
                               capture_output=True, text=True)
            if r.returncode == 0:
                subprocess.run(["git", "push"], cwd=PROJECT,
                               capture_output=True, text=True, timeout=30)
                print(f"[auto-merge] batch_history updated +{new_pts} pts — pushed")
        except Exception as e:
            print(f"[auto-merge] error: {e}")


_merge_thread = threading.Thread(target=_auto_merge_loop, daemon=True)
_merge_thread.start()

# ── CSV validation: ensure batch_history.csv is not being written ────────────
_csv_cache = None
_csv_cache_mtime = None
_csv_cache_lock = threading.Lock()

def _is_csv_being_written(csv_path: str) -> bool:
    """Check if CSV file is currently being written (size changing)."""
    try:
        size1 = os.path.getsize(csv_path)
        time.sleep(0.01)  # 10ms check interval
        size2 = os.path.getsize(csv_path)
        return size1 != size2  # Size changed = being written
    except OSError:
        return False

def _get_batch_history_csv() -> str:
    """Get batch_history.csv with caching, skipping if file is being written."""
    global _csv_cache, _csv_cache_mtime
    csv_path = os.path.join(DIRECTORY, "batch_history.csv")
    try:
        mtime = os.path.getmtime(csv_path)
        with _csv_cache_lock:
            # Return cached version if file hasn't changed
            if _csv_cache is not None and _csv_cache_mtime == mtime:
                return _csv_cache
            # Skip read if file is currently being written (merge in progress)
            if _is_csv_being_written(csv_path):
                return _csv_cache or ""  # Return cached or empty if none cached yet
            # Read and cache the file
            with open(csv_path, 'r', encoding='utf-8', errors='ignore') as f:
                _csv_cache = f.read()
                _csv_cache_mtime = mtime
                return _csv_cache
    except OSError:
        return _csv_cache or ""

# Simple token-bucket rate limiter: max 10 requests/second per IP for training.log
_rate_buckets: dict[str, tuple[float, float]] = defaultdict(lambda: (10.0, time.monotonic()))
_RATE_LIMIT   = 10.0   # requests per second
_BURST        = 20.0   # burst capacity

def _check_rate(ip: str) -> bool:
    tokens, last = _rate_buckets[ip]
    now = time.monotonic()
    tokens = min(_BURST, tokens + (now - last) * _RATE_LIMIT)
    if tokens < 1.0:
        _rate_buckets[ip] = (tokens, now)
        return False
    _rate_buckets[ip] = (tokens - 1.0, now)
    return True


class RangeAwareHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    def send_head(self):
        # Special handling for batch_history.csv — avoid serving partial writes during merge
        clean_path = self.path.split('?')[0]
        if clean_path.endswith('batch_history.csv'):
            csv_content = _get_batch_history_csv()
            try:
                csv_bytes = csv_content.encode('utf-8')
                self.send_response(200)
                self.send_header('Content-Type', 'text/csv; charset=utf-8')
                self.send_header('Content-Length', str(len(csv_bytes)))
                self.send_header('Cache-Control', 'no-cache, must-revalidate')
                self.end_headers()
                return io.BytesIO(csv_bytes) if csv_bytes else io.BytesIO(b'')
            except Exception:
                self.send_error(500)
                return None
        # Only intercept range requests for training.log — everything else served normally.
        if clean_path.endswith('training.log'):
            ip = self.client_address[0]
            if not _check_rate(ip):
                self.send_response(429)
                self.send_header('Retry-After', '1')
                self.end_headers()
                return None
        if not clean_path.endswith('training.log'):
            # Force revalidation for index.html — prevents browser from serving stale builds.
            if clean_path in ('/', '/index.html', ''):
                full_path = os.path.join(DIRECTORY, 'index.html')
                try:
                    f = open(full_path, 'rb')
                    size = os.path.getsize(full_path)
                    self.send_response(200)
                    self.send_header('Content-Type', 'text/html; charset=utf-8')
                    self.send_header('Content-Length', str(size))
                    self.send_header('Cache-Control', 'no-cache, must-revalidate')
                    self.end_headers()
                    return f
                except OSError:
                    pass  # fall through to default handler
            return super().send_head()

        range_header = self.headers.get('Range', '')
        if not range_header:
            return super().send_head()

        m = re.match(r'bytes=(\d+)-', range_header)
        if not m:
            return super().send_head()

        start = int(m.group(1))
        full_path = os.path.join(DIRECTORY, 'training.log')

        try:
            size = os.path.getsize(full_path)
        except OSError:
            self.send_error(404)
            return None

        if start >= size:
            self.send_response(416)
            self.send_header('Content-Range', f'bytes */{size}')
            self.end_headers()
            return None

        end = size - 1
        length = end - start + 1

        f = open(full_path, 'rb')
        f.seek(start)

        self.send_response(206)
        self.send_header('Content-Type', 'text/plain; charset=utf-8')
        self.send_header('Content-Range', f'bytes {start}-{end}/{size}')
        self.send_header('Content-Length', str(length))
        self.send_header('Cache-Control', 'no-store')
        self.end_headers()
        return f

    def do_GET(self):
        if self.path.startswith('/api/mycelium'):
            self._handle_mycelium()
            return
        if self.path.startswith('/api/csv_mtime'):
            self._handle_csv_mtime()
            return
        super().do_GET()

    def _handle_csv_mtime(self):
        csv_path = os.path.join(DIRECTORY, "batch_history.csv")
        try:
            mtime = os.path.getmtime(csv_path)
            size  = os.path.getsize(csv_path)
        except OSError:
            mtime, size = 0, 0
        body = json.dumps({"mtime": mtime, "size": size}).encode()
        self.send_response(200)
        self.send_header('Content-Type', 'application/json; charset=utf-8')
        self.send_header('Content-Length', str(len(body)))
        self.send_header('Cache-Control', 'no-store')
        self.end_headers()
        self.wfile.write(body)

    def _handle_mycelium(self):
        parsed = urllib.parse.urlparse(self.path)

        if parsed.path == '/api/mycelium/status':
            try:
                result = _mycelium.status()
                body   = json.dumps(result).encode()
                code   = 200
            except Exception as exc:
                body = json.dumps({'error': str(exc)}).encode()
                code = 500
        else:
            params = urllib.parse.parse_qs(parsed.query)
            word = params.get('word', [''])[0].strip()
            try:
                k = min(max(1, int(params.get('k', ['40'])[0])), 200)
            except ValueError:
                k = 40

            if not word:
                body = json.dumps({'error': 'word parameter required'}).encode()
                code = 400
            else:
                try:
                    result = _mycelium.query(word, k)
                    body   = json.dumps(result).encode()
                    code   = 200
                except Exception as exc:
                    body = json.dumps({'error': str(exc)}).encode()
                    code = 500

        self.send_response(code)
        self.send_header('Content-Type', 'application/json; charset=utf-8')
        self.send_header('Content-Length', str(len(body)))
        self.send_header('Cache-Control', 'no-store')
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, format, *args):
        # Suppress per-request noise for the hot poll and mycelium endpoints.
        # Cast to str: in Python 3.13 send_error passes HTTPStatus enum objects as args.
        first = str(args[0]) if args else ''
        if 'training.log' in first or '/api/mycelium' in first or '/api/csv_mtime' in first:
            return
        super().log_message(format, *args)

# ── Startup merge — run once synchronously before serving any requests ────────
# Ensures batch_history.csv includes all Downloads CSVs before the browser
# fetches it for the preseed, so the historical chart is never stale at boot.
print("[startup] merging Downloads CSVs into batch_history.csv ...")
try:
    _r = subprocess.run(
        [sys.executable, MERGE_PY],
        cwd=PROJECT, capture_output=True, text=True, timeout=300,
    )
    if _r.returncode == 0:
        for _l in _r.stdout.splitlines():
            if "Total unique points" in _l or "Wrote" in _l or "error" in _l.lower():
                print(f"[startup]  {_l}")
    else:
        print(f"[startup] merge failed (rc={_r.returncode}): {_r.stderr[:300]}")
except Exception as _e:
    print(f"[startup] merge error: {_e}")

with socketserver.TCPServer(("127.0.0.1", PORT), RangeAwareHandler) as httpd:
    httpd.allow_reuse_address = True
    print(f"Dashboard serving at http://localhost:{PORT}")
    print(f"(bound to 127.0.0.1 — use ssh -L {PORT}:localhost:{PORT} for remote access)")
    httpd.serve_forever()
