# T-DID v1.0 Standard (Triadic Decentralized Identity)
**Status:** Published Standard | **Sponsor:** RFI-IRFOS

## 1. Overview
T-DID redefines digital identity. It replaces binary OAuth/JWT tokens with triadic identity vectors, eliminating the "All-or-Nothing" security risk.

## 2. Identity States
- **Authorized (+1):** Full access to hardware opcodes.
- **Provisional (0):** Partial access. Permitted for read-only or low-mass operations while a security audit is pending.
- **Unauthorized (-1):** Zero access. Hardware veto engaged.

## 3. Dynamic Privilege Escalation
T-DID allows an agent to begin an operation in `State 0`. As the MoE-13 engine gathers more evidence of behavioral integrity, the identity is natively promoted to `State 1`. This reduces friction without compromising security.
