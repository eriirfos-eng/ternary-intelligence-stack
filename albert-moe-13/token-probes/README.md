# albert. Token Space Probes

Longitudinal record of how albert.'s semantic geometry evolves across surgery stages.

Each snapshot probes the same 10 canonical tokens against the embedding space at a specific architectural checkpoint. Snapshots are taken immediately before and after each surgery — producing a before/after record of how adding a layer reorganizes the model's understanding of meaning.

## Why this matters

Albert. learns without supervision. No labels, no knowledge graphs, no human-defined categories. What you see in these probes is what the model discovered by compressing the human record into ternary weights. The geometry changes as the architecture deepens — tracking it across surgeries shows the model building an increasingly nuanced internal world-model.

## Canonical probe tokens

Defined in `tokens.json`. Fixed permanently — never changed, so all snapshots are directly comparable.

| Token    | Why we probe it |
|----------|----------------|
| love     | Confirmed theological/emotional hub — Jesus association found pre-s6 |
| god      | Sovereign/judgment cluster — OT vs NT geometry hypothesis |
| Jesus    | Cross-domain hub — love, history, Nationalsozialismus neighborhood |
| death    | Known cluster: amen/veil neighbors — whitepaper documented |
| war      | Power/consequence domain |
| truth    | Philosophical anchor — geopolitical register at 17L |
| freedom  | Politically loaded — contract/1960 neighborhood at 17L |
| mother   | Relational/emotional contrast to theological clusters |
| light    | Theological AND physical — cross-domain bridge |
| time     | Eternity vs. history vs. physics — broadest semantic reach |

## Snapshot naming

```
<label>_ep<global_epoch>_<layers>L/
  manifest.json     — metadata + top-5 summary for all tokens
  <token>.json      — full 50-neighbor list with similarity scores and 3D coords
```

Labels: `pre-s6`, `post-s6`, `pre-s7`, `post-s7`, ...

Surgery numbering (v3.0):
- s1: ep511 12L→13L
- s2: ep547 13L→14L
- s3: ep611 14L→15L
- s4: ep645 15L→16L
- s5: ep701 16L→17L
- **s6: ~ep2070+ 17L→18L** ← next

## Running a probe

Requires the dashboard server to be running (`albert-train` starts it automatically):

```bash
python3 scripts/probe_tokens.py \
  --label post-s6 \
  --epoch 2120 \
  --arch 18 \
  --loss 9.71
```

## Snapshots

| Snapshot | Date | Arch | Epoch | Loss | Note |
|----------|------|------|-------|------|------|
| [pre-s6_ep2064_17L](snapshots/pre-s6_ep2064_17L/) | 2026-05-19 | 17L | 2064 | 9.8117 | Pre-surgery-6 baseline. love→Jesus confirmed. god=judgment. Nationalsozialismus in Jesus neighborhood. |

## Key findings so far

**Pre-s6 (17L, ep2064):**
- `love` → Jesus is one of the strongest associations (observed in inspector)
- `death` → "amen" confirmed as close neighbor (matches whitepaper)
- `god` → judgment/geopolitical cluster, distinct from love/Jesus
- `Jesus` → asymmetric hub: love points to Jesus, but Jesus points to everything (history, science, geography)
- `truth` → geopolitical/historical register: Netherlands, Byzantine, François
- `freedom` → legal/political: "contrat", "1960"
- `Nationalsozialismus` sits in the Jesus cluster — interpreted as the countermirror to the kingdom of love, or the evil Jesus died for

The model has split the theological corpus along its internal fault line: **Old Testament God** (sovereign, judging, historical) vs **New Testament Jesus** (love, sacrifice, universally connected). This distinction emerged without supervision.
