import http.server
import socketserver
import os
import sys
import re

# CONFIGURATION
PORT = 8888
DIRECTORY = os.path.dirname(os.path.abspath(__file__))

class RangeRequestHandler(http.server.SimpleHTTPRequestHandler):
    """
    A SimpleHTTPRequestHandler that supports HTTP Range requests.
    This allows the dashboard to only download the 'tail' of the training log.
    """
    def do_GET(self):
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
            self.send_error(416, "Requested Range Not Satisfiable")
            return

        # Send 206 Partial Content
        self.send_response(206)
        self.send_header('Content-type', self.guess_type(path))
        self.send_header('Accept-Ranges', 'bytes')
        self.send_header('Content-Range', f'bytes {start}-{end}/{file_size}')
        self.send_header('Content-Length', str(end - start + 1))
        self.send_header('Access-Control-Allow-Origin', '*') # Safety for dashboard
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
        with socketserver.TCPServer(("", PORT), RangeRequestHandler) as httpd:
            print(f"--- MoE-13 v2.2 HIGH-SPEED DASHBOARD SERVER ACTIVE ---")
            print(f"URL: http://localhost:{PORT}")
            httpd.serve_forever()
    except Exception as e:
        print(f"Server Fatal Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
