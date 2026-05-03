from flask import Flask, jsonify
import subprocess
import os

app = Flask(__name__)

@app.route('/')
def index():
    return open('index.html').read()

@app.route('/api/train', methods=['POST'])
def train():
    # Trigger the Rust training harness
    cmd = ["cargo", "run", "--bin", "production_train", "-p", "moe-core"]
    cwd = "../albert-moe-13/crates/moe-core"
    result = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    return jsonify({"message": result.stdout + result.stderr})

if __name__ == '__main__':
    app.run(port=5000)
