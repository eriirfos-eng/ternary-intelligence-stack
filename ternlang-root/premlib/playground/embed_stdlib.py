#!/usr/bin/env python3
"""
embed_stdlib.py — Regenerates playground/index.html with the latest stdlib files baked in.

Usage:
    cd ternlang-root
    python3 playground/embed_stdlib.py

Run this after adding new .tern files to stdlib/errors/ or stdlib/core/.
The playground/index.html is self-contained after this — no server, no dialog.
"""

import os, json, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
STDLIB_BASE = os.path.join(ROOT, "stdlib")
PLAYGROUND = os.path.join(ROOT, "playground", "index.html")

EMBED_DIRS = ["errors", "core"]

MARKER_START = "const STDLIB_EMBED = {"
MARKER_END   = "};"

def build_embed():
    files = {}
    for d in EMBED_DIRS:
        dirpath = os.path.join(STDLIB_BASE, d)
        if not os.path.isdir(dirpath):
            print(f"  warn: {dirpath} not found, skipping", file=sys.stderr)
            continue
        for fname in sorted(os.listdir(dirpath)):
            if fname.endswith(".tern"):
                with open(os.path.join(dirpath, fname)) as f:
                    content = f.read()
                files[f"{d}/{fname}"] = content

    lines = [
        "// ═══════════════════════════════════════════════════════════════════════════",
        f"// STDLIB EMBED — {len(files)} files from: {', '.join(EMBED_DIRS)}",
        "// Regenerate: python3 playground/embed_stdlib.py",
        "// ═══════════════════════════════════════════════════════════════════════════",
        "const STDLIB_EMBED = {",
    ]
    for key, content in files.items():
        escaped = content.replace('\\', '\\\\').replace('`', '\\`').replace('${', '\\${')
        lines.append(f"  {json.dumps(key)}: `{escaped}`,")
    lines.append("};")
    return "\n".join(lines), len(files)

def patch_html(embed_js):
    with open(PLAYGROUND) as f:
        html = f.read()

    # Find and replace the STDLIB_EMBED block
    start = html.find(MARKER_START)
    if start == -1:
        print("ERROR: Could not find 'const STDLIB_EMBED = {' in playground/index.html")
        sys.exit(1)
    # Walk back to find the comment block before it
    block_start = html.rfind("// ═══", 0, start)
    if block_start == -1:
        block_start = start
    # Find the closing }; after MARKER_START
    end = html.find(MARKER_END, start) + len(MARKER_END)

    new_html = html[:block_start] + embed_js + html[end:]
    with open(PLAYGROUND, "w") as f:
        f.write(new_html)
    return len(new_html)

if __name__ == "__main__":
    print("Building stdlib embed...")
    embed_js, count = build_embed()
    print(f"  {count} files embedded from: {EMBED_DIRS}")
    size = patch_html(embed_js)
    print(f"  playground/index.html updated ({size:,} bytes)")
    print("Done.")
