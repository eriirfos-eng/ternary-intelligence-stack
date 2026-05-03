from flask import Flask, render_template_string
from flask_socketio import SocketIO, emit
import subprocess
import threading
import time

app = Flask(__name__)
socketio = SocketIO(app, cors_allowed_origins="*")

@app.route('/')
def index():
    return open('index.html').read()

@socketio.on('start_training')
def handle_training():
    def run_training():
        cmd = ["cargo", "run", "--bin", "production_train", "-p", "moe-core"]
        cwd = "../albert-moe-13/crates/moe-core"
        
        # Stream logs in real-time
        process = subprocess.Popen(cmd, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        for line in iter(process.stdout.readline, ''):
            socketio.emit('log', {'data': line.strip()})
        process.stdout.close()
        socketio.emit('status', {'data': 'Training Finished'})

    threading.Thread(target=run_training).start()
    emit('status', {'data': 'Training Started'})

if __name__ == '__main__':
    socketio.run(app, port=5000)
