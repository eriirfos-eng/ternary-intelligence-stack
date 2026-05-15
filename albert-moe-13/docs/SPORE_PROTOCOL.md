# Spore Protocol — albert. Federated Weight Sharing

## What a spore is

A spore is a safetensors checkpoint produced by a collaborator's local CPU training loop, committed to the private `albert-spores` repo, and asynchronously ingested into the main GPU training run via weight blending.

The metaphor is exact: biochemical spores carry genetic material across environments that the parent organism cannot reach.  Zabih's laptop, Lisa's workstation, any contributor's machine — none of them have the Modal T4 budget or continuous uptime.  But each machine produces real gradient signal.  A spore packages that signal and lets it drift back into the colony.

## Collaborator setup

One-liner (Linux):

```bash
curl -sSL https://raw.githubusercontent.com/eriirfos-eng/ternary-intelligence-stack/main/albert-moe-13/scripts/setup_collaborator.sh | bash
```

This installs `gh` CLI, authenticates GitHub, builds the training binaries, and installs three commands:

| Command | Description |
|---|---|
| `albert-train` | Local CPU training loop (default). `--modal` for GPU. |
| `albert-test` | Interactive TUI — chat with albert. |
| `albert-spore --name you` | Export checkpoint → push spore to colony |

## Workflow

```
albert-train              # runs train_bible binary locally, CPU, spore mode
                          # trains until you stop with Ctrl-C

albert-spore --name zabih # exports current checkpoint as a spore
                          # pushes to albert-spores/spores/zabih/YYYY-MM-DD/
```

The spore script (`scripts/produce_spore.py`):
1. Finds the best available safetensors in `models/`
2. Reads epoch and loss from `dashboard/epoch_history.log`
3. Copies checkpoint to `albert-spores/spores/{name}/{date}/spore_ep{N}_{loss}.safetensors`
4. Writes companion JSON metadata
5. `git add / commit / push` to the private repo

## Ingestion mechanics

The main GPU training loop (train_bible) runs `SporeManager::scan_pending()` at the end of every epoch.  When a new spore passes the fitness gate, `SporeManager::ingest()` blends it into the live VarMap and immediately re-saves the checkpoint.

### Fitness gate

```
spore.loss_at_production  <  current_best_epoch_loss + 1.0 nat
```

The 1.0 nat margin is generous.  CPU spores train at ~0.5 tok/s vs ~18 tok/s on T4, so they are always behind on absolute loss.  What matters is that they are not garbage — they must be within 1.0 nat of the colony's current best.

Architecture must match on `num_layers` and `hidden_size`.  Expert count and vocab size checks are advisory (warn only).

### Blend formula

```
α = 0.08  (8% spore, 92% main)

w_merged = (1-α) · w_main + α · w_spore

For ternary weight tensors (expert fc1/fc2, attention Q/K/V/O):
  re-ternarize after blend:
    v >  0.04  →  1
    v < -0.04  → -1
    else       →  0
```

With α=0.08:
- A zero in the main model can be flipped by a clear ±1 in the spore (blend=0.08 > 0.04).
- The main model wins all sign-flip contests (blend magnitude 0.84 for main=1, spore=-1).
- Gate logits, biases, layer-norm scales, and embedding rows blend as raw F32.

### State

Ingested spore filenames are persisted to `models/albert_v3.0.spore_state`, one per line.  This file is auto-created on first ingestion.  Backward-compatible: if absent, all spores in the repo are treated as new.

## Dashboard

Ingested spores appear as **green dashed vertical lines** on the loss chart, labelled `S`.  An event entry appears in the event log with contributor name, epoch, and loss.  A browser push notification fires immediately on ingestion.

## Training flags

```
--spores-dir=PATH   Override default spores directory (default: ~/projects/albert-spores/spores)
--spores-dir=none   Disable spore scanning entirely
```

## Adding a collaborator

1. Accept GitHub invitation to `eriirfos-eng/albert-spores` (private repo).
2. Run the one-liner setup script above.
3. `albert-train` to start local training.
4. `albert-spore --name yourname` to push spores.

The main training loop picks them up automatically on the next epoch boundary.

## Multi-spore batches

If multiple spores arrive between epochs (e.g., after a multi-day gap), they are ingested sequentially.  Each blend sees the already-merged VarMap from the previous spore.  The effective weighting for two simultaneous spores A and B:

```
w_final = (1-α)² · w_main + (1-α)·α · w_A + α · w_B
```

This gives exponentially decaying weight to older spores.  Fine in practice — the main model dominates.

## Design rationale

**Why safetensors and not gradient deltas?**  Gradient deltas require synchronization of optimizer state across machines with different batch histories.  Safetensors checkpoints are self-contained and asynchronous — the spore carries everything needed.

**Why blend rather than average?**  Averaging would require a matched base checkpoint.  Blending with small α is a conservative update that preserves the main model's learned routing while allowing collaborator signal to shift zeros and calibrate gate biases.

**Why chaos corpus ingestion is not implemented yet (but planned):**  Weight statistics (mean, std, sparsity per layer) could be serialized as structured text and appended to the chaos corpus, making albert. literally train on descriptions of its own weight evolution.  This is the "mycelial network reading its own spore trail" idea — planned as a Stage 2 feature once spore ingestion is stable.
