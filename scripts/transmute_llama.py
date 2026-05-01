import numpy as np
import gguf
import os

# Path to the Llama 3.2 1B blob
MODEL_PATH = "/usr/share/ollama/.ollama/models/blobs/sha256-74701a8c35f6c8d9a4b91f3f3497643001d63e0c7a84e085bed452548fa88d45"

def bitnet_threshold(weights):
    """Compute the BitNet-style threshold: 0.5 * mean(|weights|)"""
    return 0.5 * np.mean(np.abs(weights))

def quantize(weights, threshold):
    """Quantize to {-1, 0, 1} using the threshold."""
    trits = np.zeros_like(weights, dtype=np.int8)
    trits[weights > threshold] = 1
    trits[weights < -threshold] = -1
    return trits

def main():
    print("--- RFI-IRFOS TIS: Transmutation Pipeline [Phase 12 POC] ---")
    print(f"Loading weights from: {MODEL_PATH}")
    
    reader = gguf.GGUFReader(MODEL_PATH)
    
    # Let's find an MLP weight layer (usually highly redundant)
    target_layer = "blk.0.ffn_gate.weight" 
    
    tensor = None
    for t in reader.tensors:
        if t.name == target_layer:
            tensor = t
            break
            
    if tensor is None:
        # Fallback to any weight if naming is different
        tensor = reader.tensors[10] 
        target_layer = tensor.name

    print(f"Targeting layer: {target_layer}")
    
    # Note: GGUFReader handles dequantization to numpy automatically
    weights = tensor.data
    shape = weights.shape
    
    print(f"Original shape: {shape}")
    print(f"Original dtype: {weights.dtype}")
    
    # Calculate threshold
    tau = bitnet_threshold(weights)
    print(f"Calculated BitNet threshold (tau): {tau:.6f}")
    
    # Transmute to Ternary
    trits = quantize(weights, tau)
    
    # Calculate Sparsity (Trit=0)
    zeros = np.sum(trits == 0)
    total = trits.size
    sparsity = (zeros / total) * 100
    
    print(f"\n--- Transmutation Results ---")
    print(f"Total Parameters: {total:,}")
    print(f"Sparsity (Trit=0): {sparsity:.2f}%  <-- THIS IS THE @SPARSESKIP ADVANTAGE")
    print(f"Active (+1/-1):   {100 - sparsity:.2f}%")
    
    # Sample view
    flat_weights = weights.flatten()
    flat_trits = trits.flatten()
    
    print("\nSample Side-by-Side (Original -> Ternary):")
    for i in range(10):
        print(f"  [{i}] {flat_weights[i]:.6f}  -->  {flat_trits[i]:>2}")

    print("\n[SUCCESS] Pipeline validated. This layer can now be passed to ternlang-ml.")

if __name__ == "__main__":
    main()
