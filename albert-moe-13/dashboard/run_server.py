import http.server
import json
import socketserver
import os
import sys
import re
import threading
import time
import urllib.parse

# CONFIGURATION
PORT = 8888
DIRECTORY   = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))  # albert-moe-13/
PROJECT     = DIRECTORY
_BEST_MODEL = os.path.join(PROJECT, "models", "albert_v3.0.best.safetensors")
_VOCAB_FILE = os.path.join(PROJECT, "tokenizer_v3", "tokenizer_v3.json")

# Training log lives outside the repo so git ops (filter-repo, reset, clean)
# can never delete it. The dashboard URL /dashboard/training.log is remapped
# to this stable path by the handler below.
LOG_PATH = os.path.expanduser("~/.albert/training.log")
os.makedirs(os.path.expanduser("~/.albert"), exist_ok=True)


class MyceliumEngine:
    """Lazy-loading embedding space explorer with background checkpoint watching."""

    def __init__(self, model_path: str, vocab_path: str):
        self._model_path      = model_path
        self._vocab_path      = vocab_path
        self._lock            = threading.Lock()
        self._loaded          = False
        self._loading         = False
        self._watcher_started = False
        self._embeddings      = None
        self._vocab           = None
        self._id_to_tok       = None
        self._coords          = None
        self._mtime           = None
        self._load_time       = None
        self._error           = None

    def _load(self):
        import numpy as np
        from safetensors import safe_open
        model_path    = self._model_path
        current_mtime = os.path.getmtime(model_path)
        with safe_open(model_path, framework='numpy') as f:
            emb = f.get_tensor('embed.weight').astype(np.float32)
        norms = np.linalg.norm(emb, axis=1, keepdims=True)
        emb_normed = emb / np.maximum(norms, 1e-8)
        centered = emb - emb.mean(axis=0)
        cov = (centered.T @ centered) / len(centered)
        _, eigvecs = np.linalg.eigh(cov)
        pcs = eigvecs[:, ::-1][:, :2].T
        coords = (centered @ pcs.T).astype(np.float32)
        for i in range(2):
            mx = np.abs(coords[:, i]).max()
            if mx > 0:
                coords[:, i] /= mx
        with open(self._vocab_path, encoding='utf-8') as fv:
            t = json.load(fv)
        vocab     = t['model']['vocab']
        id_to_tok = {v: k for k, v in vocab.items()}
        self._vocab      = vocab
        self._id_to_tok  = id_to_tok
        self._coords     = coords
        self._mtime      = current_mtime
        self._load_time  = time.time()
        self._error      = None
        self._embeddings = emb_normed
        self._loaded     = True

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
                    threading.Thread(target=self._watch, daemon=True).start()

    def _watch(self):
        while True:
            time.sleep(60)
            if self._loading:
                continue
            try:
                current_mtime = os.path.getmtime(self._model_path)
            except OSError:
                continue
            if current_mtime <= (self._mtime or 0):
                continue
            self._loading = True
            try:
                self._load()
            except Exception as exc:
                self._error   = str(exc)
            finally:
                self._loading = False

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

    def query(self, word: str, k: int = 40) -> dict:
        import numpy as np
        self.ensure_loaded()
        embeddings = self._embeddings
        vocab      = self._vocab
        id_to_tok  = self._id_to_tok
        coords     = self._coords
        SPACE = 'Ġ'
        token_id = vocab.get(SPACE + word)
        resolved = SPACE + word
        if token_id is None:
            token_id = vocab.get(word)
            resolved = word
        if token_id is None:
            matches = [(tok, tid) for tok, tid in vocab.items()
                       if word.lower() in tok.lower()]
            if not matches:
                return {'error': f'token not found: {word!r}', 'word': word}
            matches.sort(key=lambda x: (not x[0].startswith(SPACE), -len(x[0])))
            resolved, token_id = matches[0]
        q   = embeddings[token_id]
        sim = (embeddings @ q).tolist()
        top = sorted(range(len(sim)), key=lambda i: -sim[i])[:k + 1]
        neighbors = [
            {'word': id_to_tok.get(i, f'<{i}>'), 'sim': round(sim[i], 4),
             'x': float(coords[i, 0]), 'y': float(coords[i, 1])}
            for i in top if i != token_id
        ][:k]
        return {
            'word':       resolved,
            'center_x':  float(coords[token_id, 0]),
            'center_y':  float(coords[token_id, 1]),
            'neighbors':  neighbors,
            'vocab_size': len(vocab),
            'dims':       int(embeddings.shape[1]),
        }


_mycelium = MyceliumEngine(_BEST_MODEL, _VOCAB_FILE)

# --cpu: redirect bare /dashboard/ to CPU-safe thresholds (5-min stale, 30-min panel).
# Passed by albert-train on contributor machines so any manual navigation still
# gets the right params even if the auto-opened browser tab is closed.
CPU_MODE = '--cpu' in sys.argv
CPU_PARAMS = 'poll_ms=2000&stale_s=300&panel_stale_s=1800'

class RangeRequestHandler(http.server.SimpleHTTPRequestHandler):
    """
    A SimpleHTTPRequestHandler that supports HTTP Range requests.
    This allows the dashboard to only download the 'tail' of the training log.
    """
    def _serve_log(self):
        """Serve ~/.albert/training.log with full Range support and proper 416 headers."""
        if not os.path.isfile(LOG_PATH):
            self.send_error(404, "Training log not found — is albert-train running?")
            return
        file_size = os.path.getsize(LOG_PATH)
        range_header = self.headers.get('Range', '')
        if not range_header:
            # Full file
            self.send_response(200)
            self.send_header('Content-Type', 'text/plain; charset=utf-8')
            self.send_header('Content-Length', str(file_size))
            self.send_header('Cache-Control', 'no-store')
            self.end_headers()
            with open(LOG_PATH, 'rb') as f:
                self.wfile.write(f.read())
            return
        m = re.match(r'bytes=(\d*)-(\d*)', range_header)
        if not m:
            self.send_error(400)
            return
        s, e = m.groups()
        if s == '':
            start = max(0, file_size - int(e))
            end   = file_size - 1
        else:
            start = int(s)
            end   = int(e) if e else file_size - 1
        if start >= file_size:
            self.send_response(416)
            self.send_header('Content-Range', f'bytes */{file_size}')
            self.end_headers()
            return
        length = end - start + 1
        self.send_response(206)
        self.send_header('Content-Type', 'text/plain; charset=utf-8')
        self.send_header('Content-Range', f'bytes {start}-{end}/{file_size}')
        self.send_header('Content-Length', str(length))
        self.send_header('Cache-Control', 'no-store')
        self.end_headers()
        with open(LOG_PATH, 'rb') as f:
            f.seek(start)
            self.wfile.write(f.read(length))

    def _handle_server_config(self):
        body = json.dumps({'cpu_mode': CPU_MODE}).encode()
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
                body = json.dumps(_mycelium.status()).encode()
                code = 200
            except Exception as exc:
                body = json.dumps({'error': str(exc)}).encode()
                code = 500
        else:
            params = urllib.parse.parse_qs(parsed.query)
            word   = params.get('word', [''])[0].strip()
            try:
                k = min(max(1, int(params.get('k', ['40'])[0])), 200)
            except ValueError:
                k = 40
            if not word:
                body = json.dumps({'error': 'word parameter required'}).encode()
                code = 400
            else:
                try:
                    body = json.dumps(_mycelium.query(word, k)).encode()
                    code = 200
                except Exception as exc:
                    body = json.dumps({'error': str(exc)}).encode()
                    code = 500
        self.send_response(code)
        self.send_header('Content-Type', 'application/json; charset=utf-8')
        self.send_header('Content-Length', str(len(body)))
        self.send_header('Cache-Control', 'no-store')
        self.end_headers()
        self.wfile.write(body)

    def do_GET(self):
        # Server config API — lets dashboard self-configure without URL params
        if self.path.startswith('/api/server-config'):
            self._handle_server_config()
            return

        # Mycelium API
        if self.path.startswith('/api/mycelium'):
            self._handle_mycelium()
            return

        # CPU mode: redirect bare dashboard requests to CPU-safe URL params.
        if CPU_MODE and 'stale_s' not in self.path:
            clean = self.path.split('?')[0].rstrip('/')
            if clean in ('/dashboard', ''):
                self.send_response(302)
                self.send_header('Location', f'/dashboard/?{CPU_PARAMS}')
                self.end_headers()
                return

        # Remap training.log URL to the stable out-of-repo path
        clean = self.path.split('?')[0]
        if clean.endswith('training.log'):
            self._serve_log()
            return
        range_header = self.headers.get('Range')
        if not range_header or not os.path.isfile(self.translate_path(self.path)):
            return super().do_GET()

        # Parse range header: e.g. "bytes=-50000" or "bytes=0-100"
        path = self.translate_path(self.path)
        file_size = os.path.getsize(path)
        
        match = re.match(r'bytes=(\d*)-(\d*)', range_header)
        if not match:
            return super().do_GET()

        start, end = match.groups()
        if start == '': # Suffix range: -50000
            start = max(0, file_size - int(end))
            end = file_size - 1
        else:
            start = int(start)
            end = int(end) if end != '' else file_size - 1

        if start >= file_size:
            self.send_response(416)
            self.send_header('Content-Range', f'bytes */{file_size}')
            self.end_headers()
            return

        # Send 206 Partial Content
        self.send_response(206)
        self.send_header('Content-type', self.guess_type(path))
        self.send_header('Accept-Ranges', 'bytes')
        self.send_header('Content-Range', f'bytes {start}-{end}/{file_size}')
        self.send_header('Content-Length', str(end - start + 1))
        self.send_header('Access-Control-Allow-Origin', 'http://localhost:8888')
        self.end_headers()

        with open(path, 'rb') as f:
            f.seek(start)
            self.wfile.write(f.read(end - start + 1))

    def log_message(self, format, *args):
        pass # Silence logs

def main():
    if not os.path.exists(DIRECTORY):
        os.makedirs(DIRECTORY, exist_ok=True)

    os.chdir(DIRECTORY)
    socketserver.TCPServer.allow_reuse_address = True
    
    try:
        with socketserver.TCPServer(("127.0.0.1", PORT), RangeRequestHandler) as httpd:
            print(f"--- MoE-13 v2.2 HIGH-SPEED DASHBOARD SERVER ACTIVE ---")
            print(f"URL: http://localhost:{PORT}")
            print(f"(bound to 127.0.0.1 — use ssh -L {PORT}:localhost:{PORT} for remote access)")
            httpd.serve_forever()
    except Exception as e:
        print(f"Server Fatal Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
