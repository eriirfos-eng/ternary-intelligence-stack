# ternlang-engram

**Ternary episodic memory for AI agents.** Part of the RFI-IRFOS Ternary
Intelligence Stack.

A plain vector store answers *"what is similar?"*. An **episodic** store answers
*"what happened, when, and how much did it matter?"* — the autobiographical memory
an agent needs to carry experience across sessions.

## Why it's different

Built on the Ternary Intelligence Stack (`ternlang-ml` TritFloat + `ternlang-ruvector`
+ `@sparseskip`), `ternlang-engram` is not another RAG wrapper:

- **Native, propagated confidence.** Embeddings are stored as
  [`TritFloat`](https://crates.io/crates/ternlang-ml) (T-FLOAT32). Each component
  carries the episode's salience in its built-in confidence field, so every recall
  arrives with an *in-band, propagated* epistemic certainty — no separate
  uncertainty model. A low-salience memory yields a low-confidence recall.
- **Composite recall** modeled on human episodic retrieval:
  `relevance × recency × salience × frequency`. Recall **reinforces** the episodes
  it returns (access raises stability and slows future decay).
- **Time is first-class.** Every memory is stamped; `timeline(from, to)` slices
  history — the query a vector store cannot answer.
- **Forgetting is a feature.** `consolidate()` applies an Ebbinghaus-style
  forgetting curve where vivid, frequently-recalled memories decay far slower than
  trivial ones; `forget()` is a GDPR right-to-erasure primitive.
- **@sparseskip recall.** Similarity runs over ternary vectors with zero-phase
  skipping; cost scales with non-zero density, not dimension. ~16× smaller than an
  f32 store.
- **Append-only & auditable.** State is a replayable JSONL journal.

## Quick start

```rust
use ternlang_engram::{EngramStore, HashEmbedder};

// 256-dim store with the built-in offline embedder.
let mut mem = EngramStore::new(256)
    .with_embedder(Box::new(HashEmbedder::new(256)));

let t0 = 1_750_000_000_000_i64; // caller supplies wall-clock (the core is deterministic)
mem.remember("met Ana in Mendoza about wellbeing research", 0.9,
             vec!["people".into()], "albert", t0)?;
mem.remember("fixed the LayerNorm gradient wall in albert", 0.8,
             vec!["albert".into()], "albert", t0 + 3_600_000)?;

for hit in mem.recall("who works on wellbeing?", 3, t0 + 7_200_000)? {
    println!("[{:.2}] conf {:.2} | {}", hit.score, hit.confidence, hit.content);
}
# Ok::<(), anyhow::Error>(())
```

Bring your own embeddings with `remember_vec` / `recall_vec`. Persist across
sessions with `EngramStore::open(path, dim)`. Export the whole corpus to a
`RuVectorDB` for batch RAG with `to_ruvector()`.

## Embeddings

The store is embedding-agnostic at its API boundary. The bundled
[`HashEmbedder`] is deterministic, offline, and dependency-free (feature hashing),
capturing lexical-distributional similarity. For deep semantic recall, implement the
[`Embedder`] trait over a transformer / NVIDIA / albert embedder and drop it in —
the store does not change.

## MCP

Exposed to any agent through the `ternlang-mcp` server as six tools:
`engram_remember`, `engram_recall`, `engram_timeline`, `engram_consolidate`,
`engram_forget`, `engram_stats`. The server holds one durable, file-backed store
(`$TERNLANG_ENGRAM_PATH`, default `~/.ternlang/engram.jsonl`), giving an agent a
persistent episodic memory across sessions. This complements — not duplicates — the
stateless `trit_mem_*` working-memory tools.

## License

Commercial tier — `LicenseRef-Ternlang-Commercial`. See the repository root.
Patent reference: A50296/2026 (TIS platform; `@sparseskip` = Claim 3).
