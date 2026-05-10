"""
train_tokenizer_v3.py
Trains the Albert v3.0 multilingual BPE tokenizer.

Input:  data/corpus/multilingual/{lang}.txt  (built by build_multilingual_corpus.py)
Output: tokenizer_v3/tokenizer_v3.json       (HuggingFace tokenizer format)
        tokenizer_v3/vocab_v3.json           (vocab only, for quick inspection)

The output is a drop-in replacement for data/vocab.json — same format,
loaded by the existing BpeTokenizer Rust wrapper via Tokenizer::from_file().

Usage:
    python3 scripts/train_tokenizer_v3.py
    python3 scripts/train_tokenizer_v3.py --vocab-size 32000
"""

import argparse
import json
import os
import sys

SCRIPT_DIR   = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT    = os.path.dirname(SCRIPT_DIR)
CORPUS_DIRS  = [
    os.path.join(REPO_ROOT, "data", "corpus", "multilingual"),  # Wikipedia base
    os.path.join(REPO_ROOT, "data", "corpus", "academic"),       # Abstracts layer
    os.path.join(REPO_ROOT, "data", "corpus", "fulltext"),       # Full papers layer
    os.path.join(REPO_ROOT, "data", "corpus", "chaos"),          # Chaos protocol (~10%)
]
OUT_DIR      = os.path.join(REPO_ROOT, "tokenizer_v3")

DEFAULT_VOCAB_SIZE = 32000

SPECIAL_TOKENS = ["[UNK]", "[PAD]", "[BOS]", "[EOS]", "[MASK]"]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--vocab-size", type=int, default=DEFAULT_VOCAB_SIZE)
    parser.add_argument("--min-frequency", type=int, default=2)
    args = parser.parse_args()

    # Import here so we get a clean error if tokenizers isn't installed
    try:
        from tokenizers import Tokenizer
        from tokenizers.models import BPE
        from tokenizers.trainers import BpeTrainer
        from tokenizers.pre_tokenizers import ByteLevel
        from tokenizers.decoders import ByteLevel as ByteLevelDecoder
        from tokenizers.normalizers import NFC
    except ImportError:
        print("ERROR: 'tokenizers' library not found. Install with: pip install tokenizers")
        sys.exit(1)

    # Collect corpus files from all staged directories
    available = [d for d in CORPUS_DIRS if os.path.isdir(d)]
    if not available:
        print(f"ERROR: No corpus directories found. Expected one of:")
        for d in CORPUS_DIRS:
            print(f"  {d}")
        print("Run python3 scripts/build_multilingual_corpus.py first.")
        sys.exit(1)

    corpus_files = []
    for d in available:
        for f in sorted(os.listdir(d)):
            if f.endswith(".txt"):
                corpus_files.append(os.path.join(d, f))

    if not corpus_files:
        print(f"ERROR: No .txt files found in corpus directories")
        sys.exit(1)

    print(f"Corpus files ({len(corpus_files)}):")
    total_bytes = 0
    # Group by directory for clarity
    for d in available:
        label = os.path.basename(d)
        dir_files = [p for p in corpus_files if os.path.dirname(p) == d]
        if dir_files:
            dir_bytes = sum(os.path.getsize(p) for p in dir_files)
            total_bytes += dir_bytes
            print(f"  [{label}] {dir_bytes/1024/1024:.1f}MB across {len(dir_files)} files:")
            for p in dir_files:
                sz = os.path.getsize(p)
                print(f"    {os.path.basename(p)}: {sz/1024/1024:.1f}MB")
    print(f"  TOTAL: {total_bytes/1024/1024:.1f}MB\n")

    # ── 90/10 Invariant check ─────────────────────────────────────────────────
    # The chaos layer must always be ~10% of total corpus. This is a standing
    # team rule: as the corpus grows, chaos grows proportionally. The uncertainty
    # tithe is non-negotiable. See docs/architecture.md §7 for rationale.
    chaos_dir   = os.path.join(REPO_ROOT, "data", "corpus", "chaos")
    chaos_bytes = sum(
        os.path.getsize(os.path.join(chaos_dir, f))
        for f in os.listdir(chaos_dir)
        if f.endswith(".txt")
    ) if os.path.isdir(chaos_dir) else 0
    chaos_pct = chaos_bytes / total_bytes * 100 if total_bytes else 0
    print(f"90/10 Invariant: chaos layer = {chaos_bytes/1024/1024:.1f}MB "
          f"({chaos_pct:.1f}% of total corpus)")
    if chaos_pct < 8.0:
        print(f"  WARNING: chaos layer is {chaos_pct:.1f}% — below the 8% floor.")
        print(f"  Run: python3 scripts/build_chaos_corpus.py --target-mb "
              f"{int(total_bytes * 0.10 / 1024 / 1024)}")
        print(f"  The uncertainty tithe must be maintained. See docs/architecture.md §7.")
    elif chaos_pct > 14.0:
        print(f"  WARNING: chaos layer is {chaos_pct:.1f}% — above the 14% ceiling.")
        print(f"  Consider expanding the clean corpus layers to restore balance.")
    else:
        print(f"  OK — within the 8–14% accepted band.")
    print()

    # Build byte-level BPE tokenizer
    # ByteLevel pre-tokenizer maps any Unicode character to a visible byte
    # representation, making the tokenizer fully multilingual with no UNK tokens
    # for any script — same approach as GPT-2, RoBERTa.
    tokenizer = Tokenizer(BPE(unk_token="[UNK]"))
    tokenizer.normalizer = NFC()
    tokenizer.pre_tokenizer = ByteLevel(add_prefix_space=False)
    tokenizer.decoder = ByteLevelDecoder()

    trainer = BpeTrainer(
        vocab_size=args.vocab_size,
        min_frequency=args.min_frequency,
        special_tokens=SPECIAL_TOKENS,
        show_progress=True,
    )

    print(f"Training BPE tokenizer — vocab_size={args.vocab_size} min_freq={args.min_frequency}")
    print("This may take several minutes on large corpora...\n")
    tokenizer.train(corpus_files, trainer)

    actual_vocab_size = tokenizer.get_vocab_size()
    print(f"\nTraining complete. Actual vocab size: {actual_vocab_size}")

    # Verify special tokens are present
    for tok in SPECIAL_TOKENS:
        tid = tokenizer.token_to_id(tok)
        if tid is None:
            print(f"WARNING: special token {tok} not in vocab")
        else:
            print(f"  {tok} → id {tid}")

    # Smoke test across all target languages
    test_sentences = {
        "en": "The quick brown fox jumps over the lazy dog.",
        "de": "Der schnelle braune Fuchs springt über den faulen Hund.",
        "fr": "Le renard brun rapide saute par-dessus le chien paresseux.",
        "es": "El rápido zorro marrón salta sobre el perro perezoso.",
        "pt": "A raposa marrom rápida pula sobre o cachorro preguiçoso.",
        "it": "La volpe marrone veloce salta sopra il cane pigro.",
    }
    print("\nSmoke test — tokenization fertility (tokens per word):")
    for lang, sentence in test_sentences.items():
        enc = tokenizer.encode(sentence)
        words = sentence.split()
        fertility = len(enc.ids) / len(words)
        print(f"  [{lang}] {len(enc.ids):3d} tokens / {len(words)} words = {fertility:.2f} fertility")

    # Save
    os.makedirs(OUT_DIR, exist_ok=True)
    out_path = os.path.join(OUT_DIR, "tokenizer_v3.json")
    tokenizer.save(out_path)
    print(f"\nSaved: {out_path}")

    # Also save vocab list for inspection
    vocab = tokenizer.get_vocab()
    vocab_sorted = {k: v for k, v in sorted(vocab.items(), key=lambda x: x[1])}
    vocab_path = os.path.join(OUT_DIR, "vocab_v3_inspect.json")
    with open(vocab_path, "w", encoding="utf-8") as f:
        json.dump(vocab_sorted, f, ensure_ascii=False, indent=2)
    print(f"Saved vocab inspect: {vocab_path}")

    # Write summary
    summary = {
        "vocab_size": actual_vocab_size,
        "target_vocab_size": args.vocab_size,
        "min_frequency": args.min_frequency,
        "tokenizer_type": "ByteLevel BPE",
        "special_tokens": SPECIAL_TOKENS,
        "corpus_dirs": [os.path.basename(d) for d in available],
        "corpus_files": [os.path.relpath(p, REPO_ROOT) for p in corpus_files],
        "total_corpus_mb": round(total_bytes / 1024 / 1024, 1),
        "tokenizer_path": out_path,
    }
    summary_path = os.path.join(OUT_DIR, "training_summary.json")
    with open(summary_path, "w") as f:
        json.dump(summary, f, indent=2)
    print(f"Saved summary: {summary_path}")

    print(f"\nNext step: python3 scripts/transfer_weights_v3.py")
    print(f"  (when v2.0.0 training completes at epoch 500)")


if __name__ == "__main__":
    main()
