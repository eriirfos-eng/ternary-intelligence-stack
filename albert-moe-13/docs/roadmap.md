# Albert-MoE-13 Model Roadmap

---

## Completed — v2.0.0 (Current Run)

*   [x] Straight-Through Estimator (STE) ternary training from random init — weights constrained to `{-γ, 0, +γ}` throughout
*   [x] MoE architecture — 12 experts, Top-3 routing, @sparseskip (patent pending A50296/2026)
*   [x] Ternary Traffic Light Routing (TTL) — EMA-based trit states (G/O/R) per expert per layer, anti-stagnation burst mechanism
*   [x] EvolutionManager — autonomous Net2Net surgery, 4L→12L (max_layers=12 cap enforced on both plateau and COLLAPSE→SURGERY paths)
*   [x] Mycelium — 20-epoch rolling gradient pressure monitor; dead=0 throughout entire 12L run
*   [x] Stage-aware corpus curriculum — stage_3 through stage_7+ active at 12L (Bible, Alice, Gutenberg, Wikipedia EN)
*   [x] 83 tok/s sustained CPU inference on HP ZBook i7-4800MQ (measured at 4L config, 2026-05-07)
*   [x] Layer crystallization — L0–L3 grad norms ~0.00002–0.001 (nearly frozen), L11 hot at 0.013–0.022; internal differentiation without architecture growth
*   [x] Sparsity gradient — L0:10.6% → L11:26.5% across the 12-layer stack (TELE confirmed)
*   [x] Cycling reds self-organization — TTL R3 states migrate through cold layers (L2→L3→L5), self-resolving, dead=0
*   [x] Global Epoch 454+ · best loss 6.8821 · 12L checkpoint (316 MB)

---

## Phase Next — Complete Current Run

*   [ ] Run to Global Epoch 500 (cosine LR floor — 23 epochs remaining as of 2026-05-10)
*   [ ] Perplexity evaluation at 12L final checkpoint: `./target/release/moe-test --eval data/corpus/stage_7/simple_wikipedia.txt`
*   [ ] Full benchmark suite at 12L: `./target/release/moe-test --bench --csv albert_12L_final.csv`
*   [ ] Archive final checkpoint as `bible_ternary_v2.0.0_final.safetensors`
*   [ ] Extract 12L transformer block weights (`blocks.0`–`blocks.11`) for v3.0 initialization

---

## Phase v3.0 — Multilingual European Albert

**Decision (2026-05-10):** Albert must speak all major European languages. The current 8k English-only BPE vocab cannot support this — German compound nouns, French morphology, and other European languages tokenize catastrophically inefficiently against an English-trained vocabulary. v3.0 rebuilds the tokenizer foundation while preserving all learned architecture via weight transfer.

**Weight transfer:** The 12L transformer blocks (attention weights, expert MLPs, MoE routing, TTL state) are vocabulary-agnostic — they process hidden states, not token IDs, and transfer directly. Only the embedding matrix and `lm_head` are discarded and rebuilt at the new vocabulary size. This preserves 500 epochs of learned sequence modeling, routing specialization, and expert differentiation.

### Tokenizer
*   [ ] Collect balanced European corpus for tokenizer training (min: EN, DE, FR, ES, PT, IT, NL, PL)
*   [ ] Train multilingual BPE tokenizer — target 32k–64k tokens
*   [ ] Validate tokenization fertility per language (tokens per word; target: ≤2.5 for all target languages)

### European Corpus

*   [ ] **Brockhaus** — German encyclopedia, 14th edition (1894–1896), public domain, Internet Archive. Dense factual German, wide vocabulary. (Lisa Scharler)
*   [ ] **Europarl** — EU Parliament proceedings, 21 EU languages, parallel corpus. On-brand for European sovereign AI positioning.
*   [ ] **Wikipedia dumps** — DE, FR, ES, PT, IT Wikipedia. Multilingual factual grounding.
*   [ ] **Gutenberg multilingual** — French, German, Spanish, Portuguese literature (public domain)
*   [ ] **EU legal corpus** — EU AI Act and supporting documents in all official EU languages
*   [ ] Design multilingual stage-aware curriculum (stage_M1 → stage_MN, unlocking language breadth with depth)

### Architecture & Training
*   [ ] Transfer `blocks.0`–`blocks.11` weights from v2.0.0 final checkpoint into v3.0 init
*   [ ] Rebuild embedding matrix at new vocab size (overlapping tokens copied, new tokens random-init)
*   [ ] Rebuild `lm_head` to match new vocab size
*   [ ] Validate forward pass stability post-transfer before training begins
*   [ ] Train on multilingual corpus — monitor TTL routing heatmap for language-correlated expert specialization

### Milestones
*   [ ] Albert generates coherent text in ≥5 European languages
*   [ ] TTL routing shows measurable language-correlated expert specialization (routing heatmap evidence)
*   [ ] Perplexity competitive with v2.0.0 English baseline within 50 epochs of transfer training

---

## Phase GPU — Ternary CUDA Backend

*   [ ] Implement `ternary_gemv_dp4a` CUDA kernel — INT2-packed weights (`+1→01`, `0→00`, `-1→11`), 4 trits per byte, dp4a instruction
*   [ ] Validate correctness vs CPU reference (max divergence < 1e-4)
*   [ ] Benchmark on A100: confirm @sparseskip memory-bandwidth advantage (projected 8–15× over CPU)
*   [ ] Wire into candle `CustomOp1` interface
*   [ ] Enable v3.0 multilingual training on GPU (10–50× faster than current CPU run)
*   [ ] Element-level @sparseskip within active expert MLPs (next sparsity tier after expert-level skip)

---

## Phase Hardware — European Silicon

*   [ ] Lower @sparseskip and ternary matmul primitives to FPGA/HDL targets
*   [ ] Native support for European AI silicon as it matures
*   [ ] Multi-node distributed inference for v3.0+ models
*   [ ] On-device inference target: ternary weights at 32k vocab fit in ~20MB — viable for edge deployment
