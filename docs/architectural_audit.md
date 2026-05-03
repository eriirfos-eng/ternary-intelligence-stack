# Architectural and Capability Audit: Albert-MoE-13 / TIS

## 1. System Decomposition
*   **A. Real Computation Layer**: 
    *   `reproducibility_verifier`: Validates JSON log state and computes checksums (deterministic math).
    *   `albert-test` REPL: CLI interaction loop with string parsing.
    *   Inference Kernel: Basic dot-product math over `i8` vectors (not neural, but real linear algebra computation).
*   **B. Symbolic / Rule-Based Layer**: 
    *   `MoEInferenceKernel::forward`: Uses input-hash modulo arithmetic to map text inputs to ternary activations.
    *   Routing: Hardcoded logical flow based on thresholds.
    *   Response Synthesis: String templates mapping activation thresholds to static messages.
*   **C. Missing LLM Components**: 
    *   Embeddings: No learned semantic vector space exists.
    *   Attention Mechanism: No query/key/value projection or softmax scaling.
    *   Autoregressive Decoding: No token probability distributions or beam search.
    *   Tokenization: Basic byte-sum hash.

## 2. MoE Validity Check
*   **Experts**: FAKE/SYMBOLIC. They do not exist as independent function approximators (e.g., MLPs); logic is a single path mapped through index-based math.
*   **Routing**: DETERMINISTIC/RULE-BASED. No learned gating function (Softmax/Top-K).
*   **Learned Structure**: NONE. The system is currently an algorithmic rule-set operating over scalar inputs.

## 3. Checkpoint Analysis (`copernicus-v1`)
*   **Classification**: NON-NEURAL STATE.
*   **Evidence**: The `snapshot.bin` is read as raw bytes and mapped to `{-1, 0, 1}` trits. There is no structural evidence of layer-weights, bias vectors, or normalization parameters. It is an encoded set of constants rather than a learned weight matrix.

## 4. Conversational Capability Gap
*   **The Problem**: The system maps text -> scalar -> static template. It lacks semantic understanding, sequence processing, and probability-based token prediction. 
*   **Missing Link**: The current architecture is a mapping function, not a generative model. It cannot perform autoregressive completion (predicting the next likely token).

## 5. Minimal Path to Real LLM Behavior
1.  **Tokenizer**: Integrate a standard Byte-Pair Encoding (BPE) tokenizer to replace the byte-hash.
2.  **Architecture**: Replace the hash-based kernel with a genuine Transformer decoder-only stack (embedding table + attention blocks).
3.  **Inference**: Integrate a C-based or Rust-native inference engine (e.g., `candle` or `burn`) to perform tensor-based matrix multiplication over learned weights.
4.  **Bridge**: Use the current `albert-test` shell as the interface, routing inputs to the new tensor engine instead of the hash-calculator.

## 6. Honest System Classification
*   **Classification**: Deterministic symbolic inference engine.

Audit Note: The system currently serves as a structured, deterministic demonstration of ternary logic and reproducibility pipelines, but it does not contain neural network inference functionality.
