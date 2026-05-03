from flask import Flask, jsonify, request
from flask_socketio import SocketIO, emit
import subprocess
import threading
import random

app = Flask(__name__)
# Enable CORS for all
socketio = SocketIO(app, cors_allowed_origins="*")

@app.route('/')
def index():
    with open('index.html', 'r') as f:
        return f.read()

@app.route('/api/telemetry', methods=['GET'])
def get_telemetry():
    # Production: Hook this to actual Rust shared state via FFI or Shared Memory
    return jsonify({
        "experts": [random.randint(0, 100) for _ in range(13)],
        "grad_norm": random.uniform(0.1, 1.0),
        "io": random.randint(500, 2000)
    })

@app.route('/api/chat', methods=['POST'])
def chat():
    data = request.json
    prompt = data.get('prompt', '')
    return jsonify({"reply": f"Albert (Ternary Mode) processed: '{prompt}' via sparse SIMD kernels."})

@socketio.on('start_training')
def handle_training(config):
    def run_training():
        cmd = ["cargo", "run", "--bin", "production_train", "-p", "moe-core"]
        cwd = "../albert-moe-13/crates/moe-core"
        
        try:
            process = subprocess.Popen(cmd, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
            for line in iter(process.stdout.readline, ''):
                socketio.emit('log', {'data': line.strip()})
            process.stdout.close()
            socketio.emit('status', {'data': 'Training Finished'})
        except Exception as e:
            socketio.emit('log', {'data': f"Error: {str(e)}"})

    threading.Thread(target=run_training).start()
    emit('log', {'data': f"--- INITIALIZING SWEEP: LR={config.get('lr')} TH={config.get('th')} ---"})

if __name__ == '__main__':
    # Important: use socketio.run(app) to handle the routing
    socketio.run(app, port=5000, debug=True)
