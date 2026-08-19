#!/bin/bash
# Publish remaining ternlang v2.0.0 crates in topo order, rate-limited (4 / 5 min)
set -e
cd /home/eri-irfos/projects/ternary-intelligence-stack

# Remaining crates NOT yet published (deps already on crates.io)
# Topo order: deps before dependents
REMAINING=(
  ternlang-posix ternlang-bci ternlang-cad ternlang-consensus
  ternlang-gate ternlang-qutrit ternlang-auth ternlang-hdl
  ternlang-edu ternlang-net ternlang-mkl ternlang-driver
  ternlang-harmony ternlang-contract ternlang-ttp ternlang-ui
  ternlang-hft ternlang-ros2 ternlang-crypto ternlang-bio
  ternlang-tson ternlang-swarm ternlang-gfx ternlang-sec
)

LOG=/tmp/publish_progress.txt
mkdir -p /tmp/pubstate
touch /tmp/pubstate/index

# Read index
if [ -f /tmp/pubstate/index ]; then
  idx=$(cat /tmp/pubstate/index)
else
  idx=0
fi

# Publish up to 4 crates this run
for i in $(seq 0 3); do
  n=$((idx + i))
  if [ "$n" -ge "${#REMAINING[@]}" ]; then
    break
  fi
  crate="${REMAINING[$n]}"
  echo "=== Publishing $crate ===" | tee -a "$LOG"
  if cargo publish -p "$crate" --allow-dirty 2>&1 | tee -a "$LOG"; then
    echo "✅ $crate published" | tee -a "$LOG"
  else
    echo "❌ $crate FAILED (rate limit?)" | tee -a "$LOG"
    # stop on rate limit
    break
  fi
  sleep 3
done

# Update index
echo "$((idx + 4))" > /tmp/pubstate/index
echo "Progress: $((idx + 4)) / ${#REMAINING[@]} crates attempted" | tee -a "$LOG"
