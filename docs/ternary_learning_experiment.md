# Ternary Learning Experiment: Physics-of-Learning Validation

## 🎯 Validation Objective
Prove that a model with ternary weights ($\{-1, 0, 1\}$) can learn to approximate a function via gradient descent using a **Straight-Through Estimator (STE)**.

## 🧪 1. Ternary Linear Layer Design
*   **Forward**: $y = \text{clip}(\text{sign}(W)) \cdot x$
*   **Backward (STE)**: Treat $\text{sign}(W)$ as identity during gradient computation: $\frac{\partial L}{\partial W} \approx \frac{\partial L}{\partial y} \cdot x$
*   **Verification**: Ensure weight updates ($\Delta W$) accumulate and the output $y$ converges to the target.

## 🧪 2. Ternary Attention Mechanism
*   **Concept**: Standard attention uses $Q, K, V$. In a ternary transformer, these projections $W_q, W_k, W_v$ are quantized to ternary values.
*   **Implementation**: 
    1.  Quantize $W \to \{-1, 0, 1\}$ before projection.
    2.  Perform attention calculation: $\text{softmax}(\frac{Q K^T}{\sqrt{d}}) V$.
    3.  Backpropagate through the projection matrices using the STE approach.
*   **Verification**: Check if the attention map (weights) evolves to focus on relevant "tokens" (or scalar indices) as training progresses.

## 📊 3. Minimal Experiment: 1-Layer Ternary Linear
*   **Target**: $y = W x$ where $W$ is ternary and optimal.
*   **Dataset**: Simple synthetic linear mapping.
*   **Success Metric**: Loss $\to 0$ in $< 500$ steps.

## 🧠 Strategic Reasoning: Why this is the "Key"?
If this 1-layer experiment fails:
- The compute substrate is broken.
- The gradient logic is invalid.
- You have no LLM, only rules.

If it succeeds:
- You have proof that your system can "learn" in the strict neural sense.
- You have a verified foundation to scale to a full Transformer.

---

## 🛠 Next Implementation Steps
1.  **Initialize `albert-compute`**: Setup `candle` or `burn` with a custom `TernaryLinear` layer.
2.  **Run Experiment**: Execute the linear layer convergence test.
3.  **Validate Weights**: Inspect the weights post-training—they MUST contain only -1, 0, and 1.
EOF
