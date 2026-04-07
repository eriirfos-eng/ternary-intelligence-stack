# T-POSIX v1.0 Standard (Triadic Operating System Interface)
**Status:** Published Standard | **Sponsor:** RFI-IRFOS

## 1. Overview
T-POSIX redefines the interface between software processes and the kernel. It replaces binary error codes with triadic signals.

## 2. Process Lifecycle
A T-POSIX process exists in one of three scheduler states:
1. **Executing (+1):** Actively consuming cycles.
2. **Deliberating (0):** Suspended in hardware equilibrium. No context-switching overhead.
3. **Terminated (-1):** Process purged.

## 3. Legacy Obsolescence
T-POSIX provides a `binary-compat` layer that wraps legacy Linux syscalls. Any binary process returning `errno` is automatically analyzed by the `MoE-Audit` daemon and mapped to the closest triadic signal, exposing the "Safety Gaps" in legacy binary software.
