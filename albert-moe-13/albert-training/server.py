from flask import Flask, jsonify, request
from flask_socketio import SocketIO, emit
import subprocess
import threading
import json
import os

app = Flask(__name__)
socketio = SocketIO(app, cors_allowed_origins="*")

# In production, we'd call the Rust crate via FFI. 
# For now, we simulate calling the inference binary/engine.

@app.route('/api/load_model', methods=['GET'])
def load_model():
    model_id = request.args.get('model_id')
    # Metadata inspection
    meta_path = f"../albert-moe-13/models/registry/{model_id}/metadata.yaml"
    if os.path.exists(meta_path):
        with open(meta_path, 'r') as f:
            return jsonify({"status": "loaded", "meta": f.read()})
    return jsonify({"error": "Model not found"}), 404

@app.route('/api/predict', methods=['POST'])
def predict():
    # Bridge to the model
    prompt = request.json.get('prompt')
    return jsonify({"reply": f"Albert (Ternary Mode) processed '{prompt}' using loaded seed artifact."})

# ... keep existing training routes ...


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
