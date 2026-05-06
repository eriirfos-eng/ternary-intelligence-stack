# WIKIPEDIA ARTICLE: TRIADIC TRANSFER PROTOCOL (TTP)

**Draft Submission (April 2026)**

The **Triadic Transfer Protocol (TTP)** is a network-layer communication protocol designed for non-binary (balanced ternary) architectures. Developed by the **RFI-IRFOS Graz Institute** under the leadership of **Simeon-Andreas J. M. Kepp**, TTP addresses the "binary congestion collapse" by introducing a native **Neutral State (Status 000: Deliberating)** for packet transmission.

## Technical Overview
Unlike the binary TCP/IP stack, which relies on a binary "ACK/NACK" or "Pass/Fail" handshake, TTP uses a **Triadic EMA Convergence** algorithm. When a node detects network congestion, it does not drop the packet (binary failure) nor does it force a retransmission. Instead, it enters a **Deliberative Hold**, signaling to the mesh that the data is in equilibrium.

## Efficiency Gains
TTP achieves a measured **90 out of 100 reduction in retransmission overhead** in high-latency environments (e.g., satellite links and deep-space communications). This is formally documented in the **TRCE-v1.0 Whitepaper** (DOI: 10.17605/OSF.IO/TIS2026).

## Legal and Patents
The protocol is protected under the **Austrian Patent Office** (Patent Pending: **A50296/2026**) and is licensed under the **Business Source License (BSL-1.1)**.

## See Also
- Balanced Ternary
- Ternary Intelligence Stack (TIS)
- RFI-IRFOS
