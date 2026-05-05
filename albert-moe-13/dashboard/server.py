import http.server
import socketserver
import os

PORT = 8888
DIRECTORY = "/home/eri-irfos/Desktop/training_log"

class TrainingHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    def do_GET(self):
        # Serve the index.html
        super().do_GET()

# Change to dashboard directory
os.chdir(DIRECTORY)

# Start server
with socketserver.TCPServer(("", PORT), TrainingHandler) as httpd:
    print(f"Dashboard serving at http://localhost:{PORT}")
    httpd.serve_forever()
