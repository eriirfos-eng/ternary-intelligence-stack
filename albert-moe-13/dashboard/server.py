import http.server
import socketserver
import os
import re

PORT = 8888
DIRECTORY = os.path.dirname(os.path.abspath(__file__))

class RangeAwareHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    def send_head(self):
        # Only intercept range requests for training.log — everything else served normally.
        clean_path = self.path.split('?')[0]
        if not clean_path.endswith('training.log'):
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

with socketserver.TCPServer(("", PORT), RangeAwareHandler) as httpd:
    httpd.allow_reuse_address = True
    print(f"Dashboard serving at http://localhost:{PORT}")
    httpd.serve_forever()
