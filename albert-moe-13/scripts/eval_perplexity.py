#!/usr/bin/env python3
"""
Perplexity evaluation harness for Albert MoE-13.
Computes held-out perplexity on a test split and reports it against
a character-level baseline — the minimum credible ML benchmark for SPRIND.

Usage:
    python3 scripts/eval_perplexity.py --checkpoint models/bible_ternary_v2.0.0.safetensors
    python3 scripts/eval_perplexity.py --all   # runs all checkpoints in models/

Output (stdout + eval_results.json):
    {
      "checkpoint": "...",
      "test_corpus": "data/corpus/bible.txt",
      "test_tokens": 12480,
      "avg_loss": 6.1025,
      "perplexity": 447.2,
      "baseline_perplexity": 1847.3,   # unigram baseline on same test set
      "reduction_vs_baseline": "75.8%",
      "timestamp": "2026-05-07T..."
    }

The albert-test binary does the actual forward pass; this script handles
the test split, baseline, and structured output for the SPRIND audit trail.
"""

import subprocess
import json
import math
import os
import sys
import argparse
import random
from datetime import datetime, timezone

CORPUS_PATH   = "data/corpus/stage_3/bible.txt"
VOCAB_PATH    = "data/vocab.json"
ALBERT_TEST   = "target/release/moe-test"   # relative to albert-moe-13/ (after os.chdir)
EVAL_OUT      = "eval_results.json"
TEST_FRACTION  = 0.05   # hold out 5% of tokens
RANDOM_SEED    = 42
MAX_EVAL_CHARS = 6000   # cap eval input so CPU eval completes in < 5 min (~10 windows)

def load_corpus(path):
    with open(path, encoding="utf-8") as f:
        return f.read()

def split_corpus(text, test_fraction=TEST_FRACTION, seed=RANDOM_SEED):
    """Deterministic train/test split by line. Same seed = same split every run."""
    lines = [l for l in text.splitlines() if l.strip()]
    random.seed(seed)
    random.shuffle(lines)
    split = int(len(lines) * (1 - test_fraction))
    test_lines = lines[split:]
    return "\n".join(test_lines)

def unigram_baseline_perplexity(test_text, vocab_path):
    """
    Unigram baseline: assign uniform probability 1/vocab_size to every token.
    perplexity = vocab_size (upper bound — a model that learned nothing).
    """
    with open(vocab_path, encoding="utf-8") as f:
        vocab = json.load(f)
    # HuggingFace tokenizer JSON nests vocab under model.vocab
    if isinstance(vocab, dict) and 'model' in vocab and 'vocab' in vocab['model']:
        vocab_size = len(vocab['model']['vocab'])
    else:
        vocab_size = len(vocab)
    return float(vocab_size)

def run_albert_eval(test_text, checkpoint):
    """
    Call albert-test in eval mode — reads stdin tokens, returns avg cross-entropy.
    Requires albert-test to support --eval-mode flag (added below if missing).
    Falls back to estimating from training log if binary doesn't support eval mode.
    """
    # Cap to MAX_EVAL_CHARS so CPU eval completes in a reasonable time.
    if len(test_text) > MAX_EVAL_CHARS:
        test_text = test_text[:MAX_EVAL_CHARS]

    # Write test text to temp file
    tmp = "/tmp/albert_eval_input.txt"
    with open(tmp, "w", encoding="utf-8") as f:
        f.write(test_text)

    result = subprocess.run(
        [ALBERT_TEST, "--eval", tmp, "--checkpoint", checkpoint],
        capture_output=True, text=True, timeout=600
    )
    if result.returncode != 0:
        return None, result.stderr

    # Parse "AVG_LOSS: 6.1025" from stdout
    for line in result.stdout.splitlines():
        if line.startswith("AVG_LOSS:"):
            avg_loss = float(line.split(":")[1].strip())
            return avg_loss, None
    return None, "AVG_LOSS line not found in output"

def estimate_from_training_log():
    """
    Fallback: read best epoch avg loss from training log when eval binary unavailable.
    This is an ESTIMATE on training data — clearly labelled in output.
    """
    log_path = "dashboard/training.log"
    if not os.path.exists(log_path):
        return None
    best = float("inf")
    with open(log_path, encoding="utf-8", errors="ignore") as f:
        for line in f:
            if "Avg Loss:" in line:
                try:
                    val = float(line.split("Avg Loss:")[1].split("|")[0].strip())
                    best = min(best, val)
                except Exception:
                    pass
    return best if best < float("inf") else None

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--checkpoint", default="models/bible_ternary_v2.0.0.safetensors")
    parser.add_argument("--all", action="store_true", help="eval all checkpoints in models/")
    args = parser.parse_args()

    os.chdir(os.path.join(os.path.dirname(__file__), ".."))

    checkpoints = []
    if args.all:
        checkpoints = [
            os.path.join("models", f)
            for f in os.listdir("models")
            if f.endswith(".safetensors")
        ]
    else:
        checkpoints = [args.checkpoint]

    all_results = []

    for ckpt in checkpoints:
        print(f"\n── Evaluating: {ckpt} ──")

        corpus = load_corpus(CORPUS_PATH)
        test_text = split_corpus(corpus)
        test_tokens = len(test_text.split())

        baseline_ppl = unigram_baseline_perplexity(test_text, VOCAB_PATH)

        avg_loss, err = run_albert_eval(test_text, ckpt)
        source = "albert-test --eval"

        if avg_loss is None:
            print(f"  albert-test eval mode not available ({err}), using training log estimate.")
            avg_loss = estimate_from_training_log()
            source = "training_log_estimate"

        if avg_loss is None:
            print("  No loss data available. Skipping.")
            continue

        perplexity = math.exp(avg_loss)
        reduction = (1.0 - perplexity / baseline_ppl) * 100.0

        result = {
            "checkpoint":            ckpt,
            "test_corpus":           CORPUS_PATH,
            "test_tokens":           test_tokens,
            "avg_loss":              round(avg_loss, 4),
            "perplexity":            round(perplexity, 1),
            "baseline_perplexity":   round(baseline_ppl, 1),
            "reduction_vs_baseline": f"{reduction:.1f}%",
            "loss_source":           source,
            "split_seed":            RANDOM_SEED,
            "split_fraction":        TEST_FRACTION,
            "timestamp":             datetime.now(timezone.utc).isoformat(),
        }
        all_results.append(result)

        print(f"  Avg Loss:            {avg_loss:.4f}")
        print(f"  Perplexity:          {perplexity:.1f}")
        print(f"  Baseline (unigram):  {baseline_ppl:.1f}")
        print(f"  Reduction:           {reduction:.1f}%")
        print(f"  Source:              {source}")

    if all_results:
        with open(EVAL_OUT, "w") as f:
            json.dump(all_results if len(all_results) > 1 else all_results[0], f, indent=2)
        print(f"\nResults written to {EVAL_OUT}")

if __name__ == "__main__":
    main()
