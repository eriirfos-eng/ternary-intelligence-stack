# Albert MoE-13 — Stage-Aware Corpus Curriculum

**RFI-IRFOS | 2026**

---

## Design Principle

Albert's training corpus is not static. It expands automatically as the model grows deeper — each time the EvolutionManager adds a transformer layer (Net2Net surgery), the corpus loader unlocks the next data stage. Simpler text at shallow depth, richer and more diverse text as capacity increases.

This mirrors a real pedagogical progression: vocabulary and grammar before complex narrative, narrative before factual diversity, facts before instruction-following.

The mechanism is in `train_bible.rs:load_corpus()`: stages are loaded for all `stage_N` directories where `N ≤ num_layers`. Adding a layer automatically includes the next corpus stage on the following training restart.

---

## Stage Map

| Stage | Unlocked at | Files | Size | Rationale |
|---|---|---|---|---|
| `stage_3` | 3+ layers | Bible (KJV), Alice in Wonderland | ~4.6 MB | Dense, consistent prose. High token reuse. Grammatical regularity trains syntax and vocabulary from the first epoch. |
| `stage_6` | 6+ layers | 12 Gutenberg novels | ~12.0 MB | Complex narrative, wider vocabulary, varied author voice. Crime & Punishment, Moby Dick, War & Peace, Ulysses, Pride & Prejudice, Frankenstein, Sherlock Holmes, Huck Finn, Great Expectations, Tale of Two Cities, The Picture of Dorian Gray, The Prince. |
| `stage_7` | 7+ layers | Simple Wikipedia | ~9.9 MB | Factual, topic-diverse, encyclopedic structure. Contrasts with narrative — teaches the model that different registers exist. |
| `stage_9` | 9+ layers | QA instruction pairs | ~0.5 MB | User:/Albert: instruction format. Teaches the model to respond, not just continue. |
| `stage_10` | 16+ layers | dev_blogs, github_bugs, hn_discussions, gourmet_recipes, repair_guides, trails_travel | ~varied | Real-world diverse internet text; unlocked at ep701 (16L→17L surgery). |
| `stage_11` | 11+ layers | Linux documentation, EU AI Act | ~0.4 MB | Technical, structured, domain-specific. Extends beyond natural language into command-and-effect reasoning. |

**v3.0 (all stages active at 17L):** 32k ByteLevel BPE vocab (EN/DE/FR/ES/PT/IT/NL/PL), 256-token context windows. Total corpus ~635 MB including multilingual, academic, fulltext, and chaos layers.

---

## Cumulative Token Count by Stage

| Depth reached | Active stages | Approx. tokens (8k vocab) |
|---|---|---|
| 3–5 layers | stage_3 | ~900k |
| 6 layers | stage_3 + stage_6 | ~3.3M |
| 7–8 layers | stage_3 + stage_6 + stage_7 | ~5.3M |
| 9–15 layers | + stage_9 | ~5.4M |
| 16+ layers | + stage_10 | ~5.4M+ |
| all stages | + stage_11 | ~5.5M+ |

**v3.0 (17L, 32k vocab):** The above stage corpus is supplemented by the v3.0 corpus layers (multilingual ~446 MB, academic ~46 MB, fulltext ~68 MB, chaos ~43 MB = ~635 MB total). Context window: 256 tokens.

---

## Why This Ordering

**Bible + Alice at stage_3:** Both are long, clean, extensively proofread texts with high internal consistency. The KJV Bible in particular has a very constrained vocabulary (~12k unique words) used with extreme regularity — ideal for a model learning BPE tokenization and syntactic structure for the first time. Alice provides narrative variety at lower reading level. Combined they give the model ~900k tokens of dense, reliable signal before it sees anything noisier.

**Gutenberg novels at stage_6:** By 6 layers the model has enough capacity to handle vocabulary variation and longer-range dependencies. 12 novels from different centuries and authors introduce the concept that the same underlying grammar can produce radically different surface text — Dostoevsky, Melville, Joyce, and Dickens in the same training batch.

**Simple Wikipedia at stage_7:** The largest single corpus addition (~10MB). Topic diversity is the main contribution — the model now encounters thousands of distinct subjects in a consistent, factual register. This stage is where routing diversity in the MoE experts begins to matter most: different topics should activate different experts.

**QA instruction at stage_9:** The instruction pairs establish the User:/Albert: format that makes the model useful for dialogue. Introduced late so the model already has broad language competence and isn't learning grammar from instruction pairs.

**Linux docs at stage_11:** Specialist technical language, command syntax, man-page structure. Reserved for deep configurations where the model has sufficient capacity to represent technical registers without forgetting natural language.

---

## Relationship to EvolutionManager

The `EvolutionManager` monitors epoch-average loss and triggers Net2Net layer surgery when improvement plateaus. Each surgery:

1. Increments `num_layers` in `config.json`
2. Copies the last layer's weights to the new layer (with Gaussian noise σ=0.01 for symmetry breaking)
3. On the next training restart, `load_corpus()` reads the updated `num_layers` and automatically includes the next stage

The corpus therefore expands as a direct consequence of the model earning more capacity — not on a fixed schedule.

---

## Reproducibility

The train/test split for evaluation is deterministic: seed 42, 5% holdout by line shuffling (see `scripts/eval_perplexity.py`). The same seed produces the same test split every run, making perplexity evaluations comparable across checkpoints.

**Run the perplexity evaluation:**
```
cd albert-moe-13
cargo run --release -p moe-llm-core --bin eval_perplexity
# or against a specific file:
cargo run --release -p moe-llm-core --bin eval_perplexity data/corpus/stage_3/alice.txt
```
