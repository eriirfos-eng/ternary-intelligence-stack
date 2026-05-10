"""
transfer_weights_v3.py
Transfers vocabulary-agnostic weights from Albert v2.0.0 → v3.0 init checkpoint.

The 12L transformer blocks are vocabulary-agnostic (they process hidden states,
not token IDs). Only embed.weight and lm_head.weight depend on vocab_size.

What transfers:   blocks.* (684 tensors), ln_f.* (2), pos_embed.weight (1)
What's rebuilt:   embed.weight and lm_head.weight at new vocab_size (random init)

Output:
  models/albert_v3.0_init.safetensors    — full init checkpoint for v3.0 training
  models/albert_v3.0.config.json         — updated config with new vocab_size

Usage:
    python3 scripts/transfer_weights_v3.py
    python3 scripts/transfer_weights_v3.py --source models/bible_ternary_v2.0.0_final.safetensors
    python3 scripts/transfer_weights_v3.py --vocab-size 32000
"""

import argparse
import json
import os
import sys

import numpy as np

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT  = os.path.dirname(SCRIPT_DIR)

DEFAULT_SOURCE   = "models/bible_ternary_v2.0.0.safetensors"
DEFAULT_CONFIG   = "models/bible_ternary_v2.0.0.config.json"
DEFAULT_TOKENIZER = "tokenizer_v3/tokenizer_v3.json"
OUTPUT_WEIGHTS   = "models/albert_v3.0_init.safetensors"
OUTPUT_CONFIG    = "models/albert_v3.0.config.json"

# Tensors that are tied to vocab_size — will be rebuilt at new size
VOCAB_TENSORS = {"embed.weight", "lm_head.weight"}

# All other tensors transfer directly
TRANSFER_COMMENT = "blocks.* (12L transformer) + ln_f.* (final norm) + pos_embed.weight"


def get_new_vocab_size(tokenizer_path: str) -> int:
    """Read vocab size from the trained v3 tokenizer."""
    if not os.path.exists(tokenizer_path):
        return None
    with open(tokenizer_path) as f:
        data = json.load(f)
    # HuggingFace tokenizer JSON: model.vocab is a dict of token→id
    model = data.get("model", {})
    vocab = model.get("vocab", {})
    if vocab:
        return len(vocab)
    # Fallback: added_tokens + merge count
    return None


def kaiming_uniform(shape, fan_in=None):
    """Kaiming uniform init — same as PyTorch default for nn.Embedding."""
    if fan_in is None:
        fan_in = shape[-1]
    bound = np.sqrt(1.0 / fan_in)
    return np.random.uniform(-bound, bound, shape).astype(np.float32)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", default=DEFAULT_SOURCE,
                        help="Source checkpoint (v2.0.0 final or current)")
    parser.add_argument("--config", default=DEFAULT_CONFIG)
    parser.add_argument("--tokenizer", default=DEFAULT_TOKENIZER,
                        help="Path to trained v3 tokenizer JSON (to read vocab_size)")
    parser.add_argument("--vocab-size", type=int, default=None,
                        help="Override vocab size (if tokenizer not yet available)")
    args = parser.parse_args()

    try:
        from safetensors import safe_open
        from safetensors.numpy import save_file
    except ImportError:
        print("ERROR: safetensors not found. Install with: pip install safetensors")
        sys.exit(1)

    source_path = os.path.join(REPO_ROOT, args.source)
    config_path = os.path.join(REPO_ROOT, args.config)
    tokenizer_path = os.path.join(REPO_ROOT, args.tokenizer)
    out_weights = os.path.join(REPO_ROOT, OUTPUT_WEIGHTS)
    out_config  = os.path.join(REPO_ROOT, OUTPUT_CONFIG)

    if not os.path.exists(source_path):
        print(f"ERROR: Source checkpoint not found: {source_path}")
        print("Wait for v2.0.0 training to complete, or use --source to specify a path.")
        sys.exit(1)

    # Determine new vocab size
    new_vocab_size = args.vocab_size
    if new_vocab_size is None:
        new_vocab_size = get_new_vocab_size(tokenizer_path)
    if new_vocab_size is None:
        print("ERROR: Cannot determine new vocab size.")
        print("Either train the tokenizer first (scripts/train_tokenizer_v3.py)")
        print("or pass --vocab-size explicitly.")
        sys.exit(1)

    # Load source config
    with open(config_path) as f:
        config = json.load(f)
    old_vocab_size = config.get("vocab_size", 8000)
    hidden_size    = config["hidden_size"]
    max_seq_len    = config["max_seq_len"]

    print(f"Source checkpoint: {source_path}")
    print(f"Old vocab size:    {old_vocab_size}")
    print(f"New vocab size:    {new_vocab_size}")
    print(f"Hidden size:       {hidden_size}")
    print(f"Transfer:          {TRANSFER_COMMENT}")
    print()

    # Load all tensors from source
    print("Loading source checkpoint...")
    tensors_out = {}
    transfer_count = 0
    rebuild_count = 0

    with safe_open(source_path, framework="numpy", device="cpu") as f:
        all_keys = list(f.keys())
        for key in all_keys:
            if key in VOCAB_TENSORS:
                # These will be rebuilt — skip for now
                continue
            t = f.get_tensor(key)
            tensors_out[key] = t
            transfer_count += 1

    print(f"Transferred: {transfer_count} tensors")

    # Rebuild vocab-dependent tensors with new size
    print(f"Rebuilding vocab tensors at vocab_size={new_vocab_size}...")
    np.random.seed(42)  # reproducible init

    embed = kaiming_uniform((new_vocab_size, hidden_size))
    lm_head = kaiming_uniform((new_vocab_size, hidden_size))

    tensors_out["embed.weight"]   = embed
    tensors_out["lm_head.weight"] = lm_head
    rebuild_count = 2

    print(f"Rebuilt: {rebuild_count} tensors")
    print(f"  embed.weight:   {embed.shape}")
    print(f"  lm_head.weight: {lm_head.shape}")

    total_params = sum(t.size for t in tensors_out.values())
    print(f"\nTotal tensors: {len(tensors_out)}")
    print(f"Total params:  {total_params:,} ({total_params*4/1024/1024:.1f}MB fp32)")

    # Save init checkpoint
    print(f"\nSaving init checkpoint: {out_weights}")
    save_file(tensors_out, out_weights, metadata={
        "albert_version": "v3.0-init",
        "source": os.path.basename(source_path),
        "transferred": str(transfer_count),
        "rebuilt": "embed.weight lm_head.weight",
        "new_vocab_size": str(new_vocab_size),
    })
    size_mb = os.path.getsize(out_weights) / 1024 / 1024
    print(f"Saved: {size_mb:.1f}MB")

    # Write v3.0 config
    config_v3 = {
        "hidden_size":  hidden_size,
        "max_seq_len":  max_seq_len,
        "num_experts":  config["num_experts"],
        "num_heads":    config["num_heads"],
        "num_layers":   config["num_layers"],
        "vocab_size":   new_vocab_size,
    }
    with open(out_config, "w") as f:
        json.dump(config_v3, f, indent=2)
    print(f"Saved config: {out_config}")
    print(f"  {json.dumps(config_v3)}")

    print("\n=== Transfer complete ===")
    print(f"Init checkpoint: {out_weights}")
    print(f"Config:          {out_config}")
    print(f"\nNext steps:")
    print(f"  1. Copy tokenizer:  cp {tokenizer_path} data/vocab_v3.json")
    print(f"  2. Update train_bible.rs: MODEL_PATH / VOCAB_PATH / config to v3.0")
    print(f"  3. Fire: albert-train")


if __name__ == "__main__":
    main()
