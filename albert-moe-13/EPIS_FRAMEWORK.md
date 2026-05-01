# TIS Architecture: EPIS Framing

This document clarifies the official framing of the MoE-13 system within the Ternary Intelligence Stack (TIS).

## Definition of EPIS
An **Engineered Partitioned Inference System (EPIS)** is defined by:
1. **Explicit Architecture:** Routing and expert partitioning are design-time decisions.
2. **Determinism:** The mapping of input space to expert space is fixed by the RDL and Router.
3. **Substrate Separation:** The compute engine (ternary substrate) is decoupled from the routing logic, serving only as the execution backend.

## Why this is valuable
Unlike "black-box" emergent MoE systems, an EPIS provides:
*   **Predictability:** Ideal for mission-critical or regulated domains (e.g., finance, safety-critical systems).
*   **Performance:** Deterministic routing ensures constant-time execution paths, facilitating optimized hardware resource allocation.
*   **Auditability:** Because partitioning is engineered, we can analytically define which expert handles which category of data, fulfilling transparency mandates.

## Path to Emergent Specialization
The transition from EPIS to Emergent MoE requires moving from "engineered partitioning" to "stochastic learning." The Roadmap in `README.md` details the necessary transition: shifting from hard-coded routing/RDL to differentiable, learning-based manifold co-adaptation.
