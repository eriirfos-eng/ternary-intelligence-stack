# TSON v1.0 Standard (Ternary Standard Object Notation)
**Status:** Published Standard | **Sponsor:** RFI-IRFOS

## 1. Overview
TSON is the definitive data serialization format for the post-binary era. It replaces the lossy binary `null` found in JSON with the formal `tend` (State 0) primitive.

## 2. Base Types
- `affirm` (+1): Deterministic positive signal.
- `tend` (0): Deterministic uncertain/pending signal.
- `reject` (-1): Deterministic negative signal.

## 3. Implementation Mandate
Any parser claiming TSON compliance MUST NOT coerce `tend` values into binary booleans or null pointers. Hardware-accelerated TSON parsers must utilize the `BET-ISA` sparse deserialization opcodes to achieve 30% higher throughput than legacy UTF-8 JSON.
