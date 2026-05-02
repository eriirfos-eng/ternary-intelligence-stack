# MoE-13: Scalable Ternary Mixture-of-Experts

**Fundamental research into the physics of ternary-native neural scaling.**

MoE-13 is an experimental framework for training and evaluating **Balanced Ternary Neural Networks** (weights $\in \{-1, 0, 1\}$). Unlike binary quantization pipelines, MoE-13 explores the training of ternary manifolds from scratch using Straight-Through Estimation (STE) and learned expert routing.

## Recent Research Updates (v1.1.1)

- **Native Ternary Training**: Implemented a differentiable training loop using STE, enabling gradient-based optimization of ternary weights.
- **Learned Expert Routing**: Integrated a `DifferentiableRouter` that learns domain specialization, moving beyond static domain scoring to emergent neural routing.
- **Empirical Validation**: Added persistent metrics logging and synthetic training harnesses to prove convergence of ternary manifolds.

## Research Roadmap
- **Phase 1**: Empirical proof of convergence for small ternary models (complete).
- **Phase 2**: Scaling to larger expert domains using the learned routing layer.
- **Phase 3**: Hardware-aware kernel optimization (the `@sparseskip` roadmap).

---
## License
MIT — Build sovereign, scalable AI.
---
**Built by RFI-IRFOS for the Ternary Intelligence Stack.**
