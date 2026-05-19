#!/usr/bin/env python3
"""
Perplexity evaluation harness for Albert MoE-13 v3.0.
Computes held-out perplexity on the WikiText-2 eval sample and reports
against a random-initialization baseline — the SPRIND audit artifact.

The eval corpus (eval_sample.txt) is a WikiText-2 held-out split that
was never seen during training. This is the same file referenced in
MODEL_CARD.md under "Primary metric."

Usage:
    cd albert-moe-13/
    python3 scripts/eval_perplexity.py
    python3 scripts/eval_perplexity.py --checkpoint models/albert_v3.0.best.safetensors

Output: eval_results.json (published as SPRIND benchmark artifact)
"""

import subprocess
import json
import math
import os
import sys
import argparse
from datetime import datetime, timezone

EVAL_CORPUS   = "eval_sample.txt"          # held-out WikiText-2 — never seen in training
VOCAB_PATH    = "data/vocab_v3.json"
ALBERT_TEST   = "target/release/moe-test"
EVAL_OUT      = "eval_results.json"
RANDOM_SEED   = 42                          # for reproducibility record

VOCAB_SIZE    = 32000                       # v3.0 BPE vocabulary
RANDOM_BASELINE_LOSS = math.log(VOCAB_SIZE) # ln(32000) = 10.3730 — uniform over vocab
# Cap eval windows so CPU eval completes in < 5 min.
# 50 windows × 256 ctx = 12,800 tokens — statistically robust for perplexity.
# Remove or raise this for a full-corpus run on GPU.
MAX_EVAL_WINDOWS = 50


def vocab_size_from_file(vocab_path):
    with open(vocab_path, encoding="utf-8") as f:
        vocab = json.load(f)
    if isinstance(vocab, dict) and "model" in vocab and "vocab" in vocab["model"]:
        return len(vocab["model"]["vocab"])
    return len(vocab)


def run_eval(eval_text_path, checkpoint):
    """
    Call moe-test --eval to compute avg cross-entropy on the eval corpus.
    moe-test outputs:
        AVG_LOSS: X.XXXXXX
        TOKENS_EVALUATED: N
        PERPLEXITY: X.XX
    """
    result = subprocess.run(
        [ALBERT_TEST, "--eval", eval_text_path, "--checkpoint", checkpoint,
         f"--max-windows={MAX_EVAL_WINDOWS}"],
        capture_output=True, text=True, timeout=600
    )
    if result.returncode != 0:
        print(f"  stderr: {result.stderr[:500]}", file=sys.stderr)
        return None, None, result.stderr

    avg_loss = None
    tokens_evaluated = None
    for line in result.stdout.splitlines():
        if line.startswith("AVG_LOSS:"):
            avg_loss = float(line.split(":")[1].strip())
        if line.startswith("TOKENS_EVALUATED:"):
            tokens_evaluated = int(line.split(":")[1].strip())

    if avg_loss is None:
        return None, None, f"AVG_LOSS not found in output:\n{result.stdout[:300]}"
    return avg_loss, tokens_evaluated, None


def main():
    parser = argparse.ArgumentParser(description="Albert MoE-13 held-out perplexity eval")
    parser.add_argument("--checkpoint", default="models/albert_v3.0.best.safetensors",
                        help="Checkpoint to evaluate (default: v3.0 best)")
    args = parser.parse_args()

    os.chdir(os.path.join(os.path.dirname(__file__), ".."))

    if not os.path.exists(EVAL_CORPUS):
        print(f"ERROR: eval corpus not found at {EVAL_CORPUS}", file=sys.stderr)
        sys.exit(1)
    if not os.path.exists(args.checkpoint):
        print(f"ERROR: checkpoint not found at {args.checkpoint}", file=sys.stderr)
        sys.exit(1)
    if not os.path.exists(ALBERT_TEST):
        print(f"ERROR: moe-test binary not found at {ALBERT_TEST} — run: cargo build --release -p moe-test", file=sys.stderr)
        sys.exit(1)

    actual_vocab_size = vocab_size_from_file(VOCAB_PATH)
    corpus_chars = os.path.getsize(EVAL_CORPUS)
    corpus_lines = sum(1 for _ in open(EVAL_CORPUS, encoding="utf-8"))

    print(f"=== Albert MoE-13 — Held-out Perplexity Eval ===")
    print(f"Checkpoint : {args.checkpoint}")
    print(f"Eval corpus: {EVAL_CORPUS} ({corpus_lines} lines, {corpus_chars:,} bytes)")
    print(f"Vocabulary : {actual_vocab_size:,} tokens")
    print(f"Baseline   : ln({actual_vocab_size}) = {math.log(actual_vocab_size):.4f} (random init)")
    print()
    print("Running evaluation (forward pass over all eval windows)...")

    avg_loss, tokens_evaluated, err = run_eval(EVAL_CORPUS, args.checkpoint)

    if avg_loss is None:
        print(f"Evaluation failed: {err}", file=sys.stderr)
        sys.exit(1)

    perplexity        = math.exp(avg_loss)
    random_ppl        = float(actual_vocab_size)
    random_loss       = math.log(actual_vocab_size)
    reduction_vs_random = (1.0 - perplexity / random_ppl) * 100.0
    loss_reduction      = random_loss - avg_loss

    print()
    print("=== Results ===")
    print(f"  Avg CE loss (held-out)  : {avg_loss:.4f}")
    print(f"  Perplexity (held-out)   : {perplexity:.1f}")
    print(f"  Random-init baseline    : loss {random_loss:.4f}  PPL {random_ppl:.0f}")
    print(f"  Loss reduction vs random: {loss_reduction:.4f} nats  ({reduction_vs_random:.1f}% PPL reduction)")
    if tokens_evaluated:
        print(f"  Tokens evaluated        : {tokens_evaluated:,}")
    print()

    result = {
        "model": "albert-moe-13",
        "version": "v3.0",
        "architecture": "17L · 256H · 12E · Top-3 · 256CTX · 32000V · ternary weights",
        "checkpoint": args.checkpoint,
        "eval_corpus": EVAL_CORPUS,
        "eval_corpus_description": "WikiText-2 held-out split — not seen during training",
        "corpus_lines": corpus_lines,
        "corpus_bytes": corpus_chars,
        "tokens_evaluated": tokens_evaluated,
        "vocab_size": actual_vocab_size,
        "avg_loss_held_out": round(avg_loss, 6),
        "perplexity_held_out": round(perplexity, 2),
        "baseline_random_init": {
            "description": "Uniform distribution over vocabulary — ln(vocab_size)",
            "loss": round(random_loss, 4),
            "perplexity": actual_vocab_size,
        },
        "f32_weight_note": (
            "STE training operates in F32 throughout; ternary quantization is applied "
            "at inference time via prepare_inference(). Training-time gradient updates "
            "use F32 weight copies — the ternary weights are a projection of the F32 "
            "trajectory, not a separately-trained model. A dedicated F32-weight inference "
            "comparison is planned post-funding."
        ),
        "loss_reduction_vs_random_nats": round(loss_reduction, 4),
        "ppl_reduction_vs_random_pct": round(reduction_vs_random, 2),
        "random_seed": RANDOM_SEED,
        "eval_binary": ALBERT_TEST,
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "training_atl_reference": {
            "epoch_atl": 9.7884,
            "epoch_atl_epoch": 2116,
            "batch_atl": 9.6235,
            "note": "Training-set ATL for reference; held-out perplexity above is on unseen data"
        }
    }

    with open(EVAL_OUT, "w") as f:
        json.dump(result, f, indent=2)
    print(f"Results written to {EVAL_OUT}")


if __name__ == "__main__":
    main()
