import numpy as np
import gguf
import json
import os

# Path to the Llama 3.2 1B blob
MODEL_PATH = "/usr/share/ollama/.ollama/models/blobs/sha256-74701a8c35f6c8d9a4b91f3f3497643001d63e0c7a84e085bed452548fa88d45"
OUTPUT_PATH = "llama32-1b.tern.json"

def bitnet_threshold(weights):
    return 0.5 * np.mean(np.abs(weights))

def quantize_to_trits(weights, threshold):
    trits = np.zeros_like(weights, dtype=np.int8)
    trits[weights > threshold] = 1
    trits[weights < -threshold] = -1
    return trits

def pack_trits_2bit(trits_flat):
    """
    Pack trits into 2-bit bytes: -1=0b01, +1=0b10, 0=0b11.
    Matches the Rust `pack_trits_2bit` logic in `vm/bet.rs`.
    """
    n_bytes = (len(trits_flat) + 3) // 4
    packed = np.zeros(n_bytes, dtype=np.uint8)
    
    for i, t in enumerate(trits_flat):
        bits = 0
        if t == 1: 
            bits = 0b10 # Affirm
        elif t == -1: 
            bits = 0b01 # Reject
        else: 
            bits = 0b11 # Tend/Hold
        
        byte_idx = i // 4
        shift = (i % 4) * 2
        packed[byte_idx] |= (bits << shift)
        
    return packed.tolist()

def main():
    print("--- RFI-IRFOS TIS: Full Model Transmutation [Phase 12] ---")
    reader = gguf.GGUFReader(MODEL_PATH)
    
    model_data = {
        "source_model": "meta-llama/Llama-3.2-1B",
        "format_version": 1,
        "architecture": "LlamaForCausalLM",
        "vocab_size": 128256, # Standard for Llama 3
        "hidden_size": 2048,
        "num_layers": 16,
        "layers": []
    }
    
    total_sparsity = 0
    count = 0
    
    for tensor in reader.tensors:
        # Skip biases or non-weight tensors if necessary, 
        # but for now we transmute all weights.
        if "weight" not in tensor.name:
            continue
            
        print(f"Processing {tensor.name} ({tensor.data.shape})...")
        
        weights = tensor.data
        tau = bitnet_threshold(weights)
        trits = quantize_to_trits(weights, tau)
        
        sparsity = np.sum(trits == 0) / trits.size
        total_sparsity += sparsity
        count += 1
        
        rows = weights.shape[0]
        cols = weights.shape[1] if len(weights.shape) > 1 else 1
        
        packed = pack_trits_2bit(trits.flatten())
        
        layer = {
            "name": tensor.name,
            "scale": float(np.mean(np.abs(weights))),
            "sparsity": float(sparsity),
            "original_dtype": str(weights.dtype),
            "storage": {
                "Dense": {
                    "rows": rows,
                    "cols": cols,
                    "packed": packed
                }
            }
        }
        model_data["layers"].append(layer)

    print(f"\nFinalizing {OUTPUT_PATH}...")
    with open(OUTPUT_PATH, "w") as f:
        json.dump(model_data, f)
        
    print(f"--- Transmutation Complete ---")
    print(f"Total layers: {count}")
    print(f"Mean Sparsity: {(total_sparsity/count)*100:.2f}%")
    print(f"Output saved to: {os.path.abspath(OUTPUT_PATH)}")

if __name__ == "__main__":
    main()
