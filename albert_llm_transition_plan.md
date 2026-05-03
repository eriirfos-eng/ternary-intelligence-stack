# Albert-MoE-13: LLM Transition Plan (Ternary Transformer)

## 1. Architecture Diagram
```text
               ┌──────────────┐
               │ albert-test  │  (Orchestrator / Shell / Audit)
               └──────┬───────┘
                      │
        ┌─────────────▼─────────────┐
        │ albert-llm-core           │ (In-House Ternary Transformer)
        │ - BPE Tokenizer           │
        │ - Ternary Embedding Layer │
        │ - Multi-Head Attention    │
        │ - Autoregressive Decoder  │
        └─────────────┬─────────────┘
                      │
        ┌─────────────▼─────────────┐
        │ training_lab              │ (Backprop / Loss / Optimizer)
        └───────────────────────────┘
```

## 2. Module Breakdown
*   `albert-llm-core/`: Contains the tensor-based inference runtime.
*   `albert-llm-core/tokenizer/`: BPE-based tokenization (e.g., using `tokenizers` crate).
*   `albert-llm-core/model/`: Ternary Transformer architecture (Ternary-native weights $W \in \{-1, 0, 1\}$).
*   `training_lab/`: Refactored to support Cross-Entropy Loss, AdamW optimizer, and BPTT (Backpropagation Through Time).

## 3. Minimal Working Prototype Plan
1.  **Tokenizer**: Integrate standard BPE tokenizer.
2.  **Model**: Implement a "Tiny Transformer" (e.g., 2 layers, 128 hidden dim) in Rust using `burn` (configured for non-binary backends initially, then custom ternary ops).
3.  **Inference**: Replace `MoEInferenceKernel` with the new transformer forward-pass.
4.  **Baseline**: Train on a small Bible corpus subset (100k tokens) until loss convergence.

## 4. Recommended Path
*   **Path B (In-House Model)**: We proceed with the **Ternary Transformer**. This preserves our core mission (ternary-native AI) while providing genuine language capability. It avoids relying on external inference APIs (Ollama), keeping the entire system verifiable and audit-ready in our TIS stack.

## 5. Dependency List
*   `burn` (or `candle`): High-performance tensor backends.
*   `tokenizers`: Industrial-grade subword tokenization.
*   `serde`: Artifact serialization.

## 6. First Runnable Milestone: "Hello World LLM"
*   **Objective**: Get the CLI (`albert-test`) to generate one coherent sentence based on the Bible training data.
*   **Task**: 
    1. Define `Transformer` config.
    2. Load weights.
    3. Tokenize "The" -> Predict "LORD".
    4. Print to `albert-test` shell.

## 7. Training Reality Check
*   **Backprop**: Requires implementing gradient calculation for the ternary projection (e.g., using Straight-Through Estimator).
*   **Training Loop**: Unlike logs, we need a formal optimizer (AdamW) and a Loss Function (Cross-Entropy).
*   **Data**: Logs are for auditing; data must be tokenized into indices for the embedding layer.
