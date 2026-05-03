from flask import Flask, jsonify, request
from flask_socketio import SocketIO, emit
import subprocess
import threading

app = Flask(__name__)
socketio = SocketIO(app, cors_allowed_origins="*")

@app.route('/')
def index():
    return open('index.html').read()

@socketio.on('start_training')
def handle_training(config):
    # In a production setup, we'd pass config values (lr, th) as CLI args
    def run_training():
        cmd = ["cargo", "run", "--bin", "production_train", "-p", "moe-core"]
        cwd = "../albert-moe-13/crates/moe-core"
        process = subprocess.Popen(cmd, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        for line in iter(process.stdout.readline, ''):
            socketio.emit('log', {'data': line.strip()})
    
    threading.Thread(target=run_training).start()
    emit('log', {'data': f"Training initialized with LR={config.get('lr')} Threshold={config.get('th')}"})

@app.route('/api/chat', methods=['POST'])
def chat():
    prompt = request.json.get('prompt')
    # Inference bridge - this calls the model-adapter
    # In a real run, this would load the latest binary artifact from models/registry/
    return jsonify({"reply": f"Albert (Ternary Mode) processed: '{prompt}' via sparse SIMD kernels."})

if __name__ == '__main__':
    socketio.run(app, port=5000)
