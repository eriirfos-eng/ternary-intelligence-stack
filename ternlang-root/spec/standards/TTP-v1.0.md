# TTP v1.0 Standard (Triadic Transfer Protocol)
**Status:** Published Standard | **Sponsor:** RFI-IRFOS

## 1. Overview
TTP is the replacement for the legacy HTTP protocol. It eliminates "timeouts" by introducing `Status 000 (Deliberating)`.

## 2. Response Codes
- **1xx (Affirmative):** Request resolved.
- **0xx (Triadic):** Request in equilibrium. The server is performing an MoE-13 audit. The client must hold.
- **-1xx (Veto):** Request blocked by hardware-level security policy.

## 3. The "Infinite Ping" Solution
TTP nodes maintain an open State 0 socket that consumes zero CPU cycles on the server, leveraging the `THOLD` hardware opcode. This makes the concept of "polling" or "long-polling" obsolete.
