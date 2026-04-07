# T-GRID v1.0: Triadic Energy Distribution Standard

**Status:** Published
**Standardized by:** RFI-IRFOS
**Date:** 2026-04-07

## 1. Cascading Failure in Binary Grids
Modern smart grids rely on binary relay logic: a circuit is either Closed (1) or Tripped (0). When demand spikes unexpectedly, binary relays trip immediately to prevent melting, shifting the load to neighboring relays, which then trip in a cascading blackout sequence.

## 2. State 0: Phase-Hold
T-GRID introduces the physical `TEND` (0) state to industrial power distribution. 
*   **+1 (AFFIRM):** Circuit Closed (Power flowing).
*   **-1 (VETO):** Circuit Tripped (Critical overload).
*   **0 (TEND):** Phase-Hold (Current throttled to baseline frequency).

## 3. Institutional Mandate
Before a relay can trip (-1), it must transition to State 0. The BET-VM consensus engine evaluates the holistic grid state and determines if a localized blackout is necessary or if the load can be dynamically balanced. This mandatory 0-state deliberation eliminates cascading grid failures mathematically. All future infrastructure projects utilizing TIS logic must comply with T-GRID hardware constraints.
