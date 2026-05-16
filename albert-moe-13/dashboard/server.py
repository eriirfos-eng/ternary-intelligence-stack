import http.server
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
    """Lazy-loading embedding space explorer.

    Loads embed.weight from the best checkpoint on first query, computes a PCA
    projection over all 32k tokens (cached), then serves cosine-similarity
    neighbourhood queries in O(V) time.
    """

    def __init__(self, model_path: str, vocab_path: str):
        self._model_path = model_path
        self._vocab_path = vocab_path
        self._lock       = threading.Lock()
        self._loaded     = False
        self._embeddings = None   # (V, D) float32, L2-normalised
        self._vocab      = None   # {token_str: int}
        self._id_to_tok  = None   # {int: token_str}
        self._coords     = None   # (V, 2) float32 PCA

    def _load(self):
        import numpy as np
        from safetensors import safe_open

        print("[mycelium] loading embeddings ...", flush=True)
        with safe_open(self._model_path, framework='numpy') as f:
            emb = f.get_tensor('embed.weight').astype(np.float32)  # (V, 256)

        # L2-normalise for cosine similarity
        norms = np.linalg.norm(emb, axis=1, keepdims=True)
        emb_normed = emb / np.maximum(norms, 1e-8)

        # PCA via covariance eigenvectors — fast on (256, 256) cov matrix
        print("[mycelium] computing PCA ...", flush=True)
        centered = emb - emb.mean(axis=0)
        cov = (centered.T @ centered) / len(centered)   # (256, 256)
        _, eigvecs = np.linalg.eigh(cov)                # ascending eigenvalues
        pcs = eigvecs[:, ::-1][:, :2].T                 # (2, 256) top-2 PCs
        coords = (centered @ pcs.T).astype(np.float32)  # (V, 2)

        # Normalise to [-1, 1] per axis
        for i in range(2):
            mx = np.abs(coords[:, i]).max()
            if mx > 0:
                coords[:, i] /= mx

        with open(self._vocab_path, encoding='utf-8') as f:
            t = json.load(f)
        vocab = t['model']['vocab']

        self._embeddings = emb_normed
        self._vocab      = vocab
        self._id_to_tok  = {v: k for k, v in vocab.items()}
        self._coords     = coords
        self._loaded     = True
        print(f"[mycelium] ready — {len(vocab)} tokens, dim={emb.shape[1]}", flush=True)

    def ensure_loaded(self):
        if self._loaded:
            return
        with self._lock:
            if not self._loaded:
                self._load()

    def query(self, word: str, k: int = 40) -> dict:
        import numpy as np
        self.ensure_loaded()

        # Resolve token: prefer space-prefixed (standalone word in running text)
        SPACE = 'Ġ'
        token_id = self._vocab.get(SPACE + word)
        resolved = SPACE + word
        if token_id is None:
            token_id = self._vocab.get(word)
            resolved = word
        if token_id is None:
            # Fuzzy: substring match, prefer space-prefixed and longer tokens
            matches = [(tok, tid) for tok, tid in self._vocab.items()
                       if word.lower() in tok.lower()]
            if not matches:
                return {'error': f'token not found: {word!r}', 'word': word}
            matches.sort(key=lambda x: (not x[0].startswith(SPACE), -len(x[0])))
            resolved, token_id = matches[0]

        # Cosine similarity (dot product on normalised embeddings)
        sims = self._embeddings @ self._embeddings[token_id]   # (V,)

        # top-(k+1), drop self
        top_ids = int(k) + 1
        part = np.argpartition(sims, -top_ids)[-top_ids:]
        part = part[np.argsort(sims[part])[::-1]]
        part = [int(i) for i in part if int(i) != token_id][:k]

        neighbors = []
        for nid in part:
            tok = self._id_to_tok.get(nid, f'[{nid}]')
            display = tok.replace(SPACE, ' ').strip() or tok
            neighbors.append({
                'id':    nid,
                'token': tok,
                'word':  display,
                'x':     float(self._coords[nid, 0]),
                'y':     float(self._coords[nid, 1]),
                'sim':   round(float(sims[nid]), 4),
            })

        center_display = resolved.replace(SPACE, ' ').strip() or resolved
        return {
            'word':       center_display,
            'token':      resolved,
            'id':         int(token_id),
            'x':          float(self._coords[token_id, 0]),
            'y':          float(self._coords[token_id, 1]),
            'neighbors':  neighbors,
            'vocab_size': len(self._vocab),
            'dims':       int(self._embeddings.shape[1]),
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
        # Only intercept range requests for training.log — everything else served normally.
        clean_path = self.path.split('?')[0]
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
        super().do_GET()

    def _handle_mycelium(self):
        parsed = urllib.parse.urlparse(self.path)
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
        # Suppress per-request noise for the hot poll and mycelium endpoints
        first = args[0] if args else ''
        if 'training.log' in first or '/api/mycelium' in first:
            return
        super().log_message(format, *args)

with socketserver.TCPServer(("127.0.0.1", PORT), RangeAwareHandler) as httpd:
    httpd.allow_reuse_address = True
    print(f"Dashboard serving at http://localhost:{PORT}")
    print(f"(bound to 127.0.0.1 — use ssh -L {PORT}:localhost:{PORT} for remote access)")
    httpd.serve_forever()
