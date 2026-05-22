# Response to Peer Review — 2026-05-22
**From:** Simeon Kepp, RFI-IRFOS  
**Re:** Phase 1–4 Audit Report, received 2026-05-22  
**Request:** Factual corrections + second pass under revised context

---

Thank you for a technically thorough audit. The code verification work in Phase 1 and the ternary authenticity section are genuinely useful — you read the right files and reached the right conclusions on STE, the forward pass, and `.trit` export. We'd like to address a set of factual errors and a fundamental context mismatch before requesting a second pass.

---

## Factual Corrections

**1. Architecture — 17L is stale. Current: 18L.**  
Surgery 6 (17L → 18L) completed at ep2487. The codebase you reviewed may have contained an older README. All current docs, the config file (`albert-moe-13/models/albert_v3.0.config.json`), and the training log reflect 18 layers.

**2. Parameter count — significant error.**  
You wrote: *"256M parameters ≈ 1GB float."* The `256` in the architecture spec is the **hidden size**, not the parameter count. Actual parameter count, computed directly from the safetensors header:

| | Count |
|---|---|
| Total stored (FP32 checkpoint) | **134,754,816 (~134M)** |
| Active per token (Top-3/12 routing) | **33,172,992 (~33M)** |

The ~134M figure is now reflected in all documentation (ROADMAP, whitepaper, HuggingFace model card), updated as of today.

**3. `train_bible.rs` is a binary filename, not the training dataset.**  
The report repeatedly references *"bible training"* and *"limited data (bible training) for proof-of-concept."* `train_bible.rs` is the name of the training binary — it was named during early development and refers to iterating over a corpus in "chapters." The actual training corpus is:

- 177,654,147 tokens
- 8 languages (EN, DE, FR, HU, ZH, AR, KO, SV)
- Sources: Wikipedia (15 languages), Project Gutenberg, OpenWebText, GitHub, structured data
- Enforced 10% chaos layer invariant
- See `DATA_PROVENANCE.md` for full governance documentation

**4. Training depth — the convergence log you cited is months behind.**  
You cited loss reaching ~9.79 at epoch ~1553 from an old `convergence_log.md`. Current state as of 2026-05-22:

- Global epoch: **2946**
- Epoch-average ATL: **9.4873** (ep2938, 2026-05-22)
- Chip ATL (single-batch best): **9.1254**
- Descent: active, ep2946 closed at 9.4885 (0.0012 from ATL, four consecutive epochs of descent)
- Three full cliff dives through structural resistance floors documented
- WALD self-calibration event documented: dead_low threshold 9.25 → 9.00

**5. `@sparseskip` — not incremental MoE sparsity.**  
The report classifies @sparseskip as *"likely incremental relative to MoE."* @sparseskip is a **patent-pending** sparse matmul primitive (Austrian Patent Office, application A50296/2026) that operates at the hardware instruction level — zero-weight positions in ternary tensors are never loaded into the execution pipeline. It is not the same as top-k routing gates. The 83 tok/s CPU inference figure is benchmarked against this primitive, not against standard MoE routing. This is the core inference efficiency claim and the subject of active IP protection.

---

## Context Correction — SPRIND "Next Frontier AI" Mandate

The Phase 3 verdict *"Ready for SPRIND? No."* applies a production-readiness standard that is explicitly **not** what SPRIND's Next Frontier AI programme evaluates.

SPRIND Next Frontier AI is:
- **Ten teams selected for 24 months of frontier research**
- **€125M non-dilutive** across the cohort
- **Three teams from those ten** go on toward €1B backing each
- The stated mandate: *"Europe's most ambitious initiative to build the next generation of AI"*

SPRIND is not evaluating whether this is a deployable enterprise product today. They are evaluating whether this team can **build a frontier AI lab** over the next 24 months. TRL 4–5 for a research programme at this stage is appropriate — that is exactly what SPRIND funds. A TRL 8 submission would suggest the work is done and funding is unnecessary.

The correct evaluation questions are:
1. Is the technical direction genuinely frontier?
2. Is the team credible to execute at lab scale?
3. Is there a European sovereignty angle?
4. Is there working proof-of-concept that the approach is viable?
5. Is there a credible 24-month roadmap to something transformative?

The answer to all five is yes, and the code audit you completed strongly supports points 2 and 4.

---

## What Was Not Assessed

The following material was not covered in the audit and is directly relevant to SPRIND criteria:

- **193× Chinchilla efficiency argument**: albert. has processed 521B token-steps for $61.87 total compute. Chinchilla-optimal for 134M params is ~2.68B unique tokens. The ternary STE anti-saturation mechanism (discrete {-1,0,+1} cannot memorise surface patterns → continued gradient signal after thousands of passes) is documented in the whitepaper (`\subsubsection{Multi-epoch corpus efficiency and ternary STE dynamics}`, added today).
- **Read B milestone**: The four core semantic roles (PLN, CMP, INT, ABS) are stable across all 5 post-architecture surgeries, confirmed via token probe cosine similarity to 4 decimal places. This is the primary "architecture precedes learning" evidence — a key SPRIND narrative anchor.
- **Cross-lingual semantic geometry**: Verified clustering of conceptually related tokens across 8 languages without multilingual training signal. `death → amen`, `freedom → contrat` clusters survive all six surgeries unchanged.
- **EU AI Act compliance mapping**: Present in MODEL_CARD.md and HuggingFace model card — all Article 53 obligations mapped. Not mentioned in the audit.
- **SPORE federated training**: Live contributor protocol for decentralised weight sharing across CPU contributors (`SporeManager`, `albert-spores` private repo). Direct relevance to European sovereignty claims.

---

## Request for Second Pass

We would welcome a revised assessment covering:

1. **Updated architecture and training state** (18L, 134M params, ep2946, ATL 9.4873)
2. **Re-evaluation under SPRIND's actual mandate** (frontier research team, not production product)
3. **@sparseskip as primary inference innovation** (patent pending, benchmark evidence)
4. **Multi-epoch ternary efficiency section** (193× Chinchilla, $62 total compute, anti-saturation mechanism)
5. **Read B milestone and semantic geometry** (architecture-precedes-learning evidence)
6. **The two legitimate gaps we agree on**: inference demo binary and float baseline benchmark (we are addressing both before May 30)

We are also commissioning two additional independent peer reviews before the pitch date.

We appreciate the rigour of the code verification work — it is genuinely useful to have a third party confirm the ternary authenticity claims. The corrections above are factual, not defensive.

---

**Contact:** s.kepp@ternlang.com  
**Pitch date:** 2026-05-30  
**Repository:** https://github.com/eriirfos-eng/ternary-intelligence-stack
