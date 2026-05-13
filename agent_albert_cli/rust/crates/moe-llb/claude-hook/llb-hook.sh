#!/usr/bin/env bash
# LLB PreToolUse hook — Gate 1 before file mutations and git commit/push.
# Input: JSON on stdin with tool_name and tool_input fields.
# Exit 0 = allow, exit 2 = block.

set -euo pipefail

LLB=~/.cargo/bin/moe-llb-mcp

llb_call() {
    local method_json="$1"
    printf '%s\n%s\n' \
        '{"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-03-26","clientInfo":{"name":"llb-hook","version":"1.0"},"capabilities":{}}}' \
        "$method_json" \
        | "$LLB" 2>/dev/null | tail -1
}

llb_trit() {
    echo "$1" | python3 -c "
import sys,json
try:
    d=json.load(sys.stdin)
    t=json.loads(d['result']['content'][0]['text'])
    print(t.get('trit',1), t.get('detail', t.get('decision','')))
except:
    print('1 ok')
" 2>/dev/null
}

input=$(cat)
tool=$(echo "$input" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('tool_name',''))" 2>/dev/null)

# ── Edit / Write: validate the target path ───────────────────────────────────
if [[ "$tool" == "Edit" || "$tool" == "Write" ]]; then
    path=$(echo "$input" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('tool_input',{}).get('file_path',''))" 2>/dev/null)
    [[ -z "$path" ]] && exit 0

    if [[ "$tool" == "Write" && ! -f "$path" ]]; then op="CREATE"; else op="OVERWRITE"; fi

    req=$(python3 -c "
import json,sys
print(json.dumps({'jsonrpc':'2.0','id':1,'method':'tools/call','params':{'name':'llb_validate','arguments':{
    'path':sys.argv[1],'operation':sys.argv[2],
    'intent_goal':'Claude Code file mutation',
    'intent_justification':'Authorized edit during active coding session'
}}}))
" "$path" "$op")

    read trit detail <<< "$(llb_trit "$(llb_call "$req")")"
    if [[ "$trit" == "-1" ]]; then
        echo "LLB HARD VETO (write): $detail" >&2
        exit 2
    fi
    exit 0
fi

# ── Bash: scan staged files before git commit / push ────────────────────────
if [[ "$tool" == "Bash" ]]; then
    cmd=$(echo "$input" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('tool_input',{}).get('command',''))" 2>/dev/null)

    # Only intercept commit and push
    if ! echo "$cmd" | grep -qE 'git (commit|push)'; then
        exit 0
    fi

    # Grab staged file paths
    staged=$(git diff --staged --name-only 2>/dev/null || true)
    [[ -z "$staged" ]] && exit 0

    # Sensitive filename patterns — block immediately without LLB round-trip
    sensitive_pat='(\.env|\.env\.|secret|credential|token|password|id_rsa|id_ed25519|\.pem|\.p12|\.pfx|api[_-]?key)'
    bad=$(echo "$staged" | grep -iE "$sensitive_pat" || true)
    if [[ -n "$bad" ]]; then
        echo "LLB HARD VETO (commit): sensitive filename detected in staging area:" >&2
        echo "$bad" >&2
        exit 2
    fi

    # Run llb_check on each staged path
    while IFS= read -r rel; do
        abs="$(git rev-parse --show-toplevel 2>/dev/null)/$rel"
        req=$(python3 -c "
import json,sys
print(json.dumps({'jsonrpc':'2.0','id':1,'method':'tools/call','params':{'name':'llb_check','arguments':{
    'path':sys.argv[1]
}}}))
" "$abs")
        read trit detail <<< "$(llb_trit "$(llb_call "$req")")"
        if [[ "$trit" == "-1" ]]; then
            echo "LLB HARD VETO (commit): blocked path in staging: $rel — $detail" >&2
            exit 2
        fi
    done <<< "$staged"

    exit 0
fi

exit 0
