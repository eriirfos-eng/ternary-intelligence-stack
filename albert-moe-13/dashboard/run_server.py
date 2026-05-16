import http.server
import socketserver
import os
import sys
import re

# CONFIGURATION
PORT = 8888
DIRECTORY = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))  # albert-moe-13/

# Training log lives outside the repo so git ops (filter-repo, reset, clean)
# can never delete it. The dashboard URL /dashboard/training.log is remapped
# to this stable path by the handler below.
LOG_PATH = os.path.expanduser("~/.albert/training.log")
os.makedirs(os.path.expanduser("~/.albert"), exist_ok=True)

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

    def do_GET(self):
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
