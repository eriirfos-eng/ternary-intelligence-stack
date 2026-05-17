#!/usr/bin/env python3
"""
bench_translate.py — auto-translates albert. bench output to English.

Usage:
  python3 bench_translate.py                # translate latest bench file
  python3 bench_translate.py <file.txt>     # translate specific file

Reads benchmarks/bench_v3.0_*.txt, writes benchmarks/bench_v3.0_*_en.txt
with [A] original and [B] English translation side by side.
"""

import os, sys, re, glob
from deep_translator import GoogleTranslator

HERE       = os.path.dirname(os.path.abspath(__file__))
BENCH_DIR  = os.path.join(HERE, "benchmarks")
SEPARATOR  = "─" * 50

def latest_bench():
    files = sorted(glob.glob(os.path.join(BENCH_DIR, "bench_v3.0_*.txt")))
    files = [f for f in files if not f.endswith("_en.txt")]
    if not files:
        print("[bench_translate] no bench files found in", BENCH_DIR)
        sys.exit(1)
    return files[-1]

def translate(text):
    if not text.strip():
        return text
    try:
        return GoogleTranslator(source="auto", target="en").translate(text)
    except Exception as e:
        return f"[translation error: {e}]"

def parse_blocks(content):
    """
    Returns list of dicts: {header, prompt, raw_output, speed_line}
    header = lines before first [P1]
    """
    header_lines = []
    blocks = []
    current = None

    for line in content.splitlines():
        if re.match(r"^\[P\d+\]", line):
            if current:
                blocks.append(current)
            prompt = re.sub(r"^\[P\d+\]\s*", "", line)
            current = {"prompt": prompt, "pn": re.match(r"^\[(P\d+)\]", line).group(1),
                       "output_lines": [], "speed_line": ""}
        elif current is None:
            header_lines.append(line)
        elif line.strip().startswith("→"):
            current["output_lines"].append(line.strip()[1:].strip())
        elif re.match(r"^\s+\d+\.\d+ tok/s", line):
            current["speed_line"] = line.strip()
        elif current["output_lines"] and line.strip():
            current["output_lines"].append(line.strip())

    if current:
        blocks.append(current)

    return "\n".join(header_lines), blocks

def format_output(header, blocks, src_path):
    src_name = os.path.basename(src_path)
    lines = []
    lines.append(f"=== albert. benchmark eval — ENGLISH TRANSLATION ===")
    lines.append(f"Source: {src_name}")
    lines.append(SEPARATOR)
    lines.append("")

    translator = GoogleTranslator(source="auto", target="en")

    for b in blocks:
        raw = " ".join(b["output_lines"]).strip()
        print(f"  [{b['pn']}] translating ...", end=" ", flush=True)
        try:
            en = translator.translate(raw) if raw else ""
        except Exception as e:
            en = f"[translation error: {e}]"
        print("done")

        lines.append(f"[{b['pn']}] {b['prompt']}")
        lines.append(f"  [A]  {raw}")
        lines.append(f"  [B]  {en}")
        if b["speed_line"]:
            lines.append(f"       {b['speed_line']}")
        lines.append("")

    return "\n".join(lines)

def main():
    src = sys.argv[1] if len(sys.argv) > 1 else latest_bench()
    src = os.path.abspath(src)

    if not os.path.exists(src):
        print(f"[bench_translate] file not found: {src}")
        sys.exit(1)

    out = src.replace(".txt", "_en.txt")
    if out == src:
        out = src + "_en.txt"

    print(f"[bench_translate] translating: {os.path.basename(src)}")

    with open(src) as f:
        content = f.read()

    header, blocks = parse_blocks(content)
    result = format_output(header, blocks, src)

    with open(out, "w") as f:
        f.write(result)

    print(f"[bench_translate] written: {os.path.basename(out)}")

if __name__ == "__main__":
    main()
