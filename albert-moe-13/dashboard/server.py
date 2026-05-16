import http.server
import socketserver
import os
import re
import subprocess
import sys
import threading
import time
from collections import defaultdict

PORT = 8888
DIRECTORY = os.path.dirname(os.path.abspath(__file__))
PROJECT   = os.path.dirname(DIRECTORY)
MERGE_PY  = os.path.join(PROJECT, "scripts", "merge_batch_history.py")
MERGE_INTERVAL = 15 * 60  # seconds


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

    def log_message(self, format, *args):
        # Suppress per-request noise for the hot poll endpoint
        if 'training.log' not in (args[0] if args else ''):
            super().log_message(format, *args)

with socketserver.TCPServer(("127.0.0.1", PORT), RangeAwareHandler) as httpd:
    httpd.allow_reuse_address = True
    print(f"Dashboard serving at http://localhost:{PORT}")
    print(f"(bound to 127.0.0.1 — use ssh -L {PORT}:localhost:{PORT} for remote access)")
    httpd.serve_forever()
