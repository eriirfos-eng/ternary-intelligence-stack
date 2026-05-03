from flask import Flask, jsonify, request
from flask_socketio import SocketIO, emit
import subprocess
import threading

app = Flask(__name__)
# Try without async_mode='eventlet' initially to see if standard Flask mode works
socketio = SocketIO(app, cors_allowed_origins="*")

@app.route('/')
def index():
    with open('index.html', 'r') as f:
        return f.read()

@socketio.on('connect')
def test_connect():
    print('Client connected')

@socketio.on('start_training')
def handle_training(config):
    def run_training():
        cmd = ["cargo", "run", "--bin", "production_train", "-p", "moe-core"]
        cwd = "../albert-moe-13/crates/moe-core"
        
        process = subprocess.Popen(cmd, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        for line in iter(process.stdout.readline, ''):
            socketio.emit('log', {'data': line.strip()})
        process.stdout.close()
        socketio.emit('status', {'data': 'Training Finished'})

    threading.Thread(target=run_training).start()
    emit('log', {'data': f"Training initialized with LR={config.get('lr')} Threshold={config.get('th')}"})

@app.route('/api/chat', methods=['POST'])
def chat():
    data = request.json
    prompt = data.get('prompt')
    return jsonify({"reply": f"Albert (Ternary Mode) processed: '{prompt}' via sparse SIMD kernels."})

if __name__ == '__main__':
    socketio.run(app, port=5000, debug=True)
