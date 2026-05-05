import http.server
import socketserver
import os
import sys

# CONFIGURATION
PORT = 8888
DIRECTORY = os.path.dirname(os.path.abspath(__file__))

class RobustHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    def log_message(self, format, *args):
        # Silence logs for cleaner terminal
        pass

def main():
    if not os.path.exists(DIRECTORY):
        print(f"Error: Directory {DIRECTORY} not found.", file=sys.stderr)
        sys.exit(1)

    os.chdir(DIRECTORY)
    
    # Allow port reuse to prevent "Address already in use" errors on restart
    socketserver.TCPServer.allow_reuse_address = True
    
    try:
        with socketserver.TCPServer(("", PORT), RobustHandler) as httpd:
            print(f"--- Dashboard Production Server Active ---")
            print(f"URL: http://localhost:{PORT}")
            print(f"Serving Logs from: {DIRECTORY}/training.log")
            httpd.serve_forever()
    except Exception as e:
        print(f"Server Fatal Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
