import subprocess
import json
import os
from flask import Flask, render_template, request, jsonify

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/api/translate', methods=['POST'])
def translate():
    data = request.json
    code = data.get('code')
    
    # Save code to a temp file
    temp_input = 'temp_input.py'
    with open(temp_input, 'w') as f:
        f.write(code)
    
    # Run the translator binary
    # We assume 'cargo run -p ternlang-translator' is the command
    result = subprocess.run(
        ['cargo', 'run', '-p', 'ternlang-translator', '--', '--input', temp_input, '--output', 'temp_output.tern'],
        capture_output=True, text=True
    )
    
    # Read output
    with open('temp_output.tern', 'r') as f:
        translated_code = f.read()
    
    # Read manifest if it exists
    manifest = {}
    if os.path.exists('TernarySafetyManifest.json'):
        with open('TernarySafetyManifest.json', 'r') as f:
            manifest = json.load(f)
            
    return jsonify({
        'code': translated_code,
        'manifest': manifest
    })

if __name__ == '__main__':
    print("🚀 Ternlang Translator Web UI starting at http://127.0.0.1:5000")
    app.run(debug=True, port=5000)
