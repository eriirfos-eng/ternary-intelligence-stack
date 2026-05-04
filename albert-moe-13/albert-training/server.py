from flask import Flask, jsonify, request, send_file
from flask_socketio import SocketIO, emit
import subprocess
import threading
import json
import os

app = Flask(__name__, static_folder='/home/eri-irfos/Desktop/training_log', static_url_path='/static')
socketio = SocketIO(app, cors_allowed_origins="*")

@app.route('/')
def index():
    return send_file('/home/eri-irfos/Desktop/training_log/index.html')

@app.route('/training.log')
def get_training_log_file():
    return send_file('/home/eri-irfos/Desktop/training_log/training.log')

# ... keep existing routes ...


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
    socketio.run(app, port=8888, debug=False, host='0.0.0.0')
