# ternlang-consensus

Triadic Byzantine Fault Tolerance (TBFT) for distributed systems.

Modern cloud infrastructure runs on binary consensus algorithms like Raft and Paxos. When a node experiences high latency, it simply times out (a binary `Failure`), triggering chaotic leader elections and "split-brain" database corruption.

`ternlang-consensus` introduces the `State 0` (Hold) vote natively into the network layer. 

## The Advantage
If a node needs more time to verify a transaction, it casts a `0`. The network intentionally enters a "Deliberation" state instead of crashing. By eliminating binary timeouts, TBFT achieves **zero-downtime consensus** in high-latency environments (e.g., satellite constellations, global edge networks).
