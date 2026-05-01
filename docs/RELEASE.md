# MoE-13 Release Strategy

## Overview
This document outlines the publishing strategy for the MoE-13 ecosystem. We follow a strict separation between stable, public-facing components and the internal execution runtime.

## Public Crates (Publishable)
These crates are intended for external developers and are maintained with strict SemVer compatibility.

- **`moe-sdk`**: The primary entrypoint. Re-exports all necessary tools for building and deploying inference applications.
- **`moe-platform`**: Contains the stable public API (`MoEPlatform::load`, `MoEPlatform::run`).
- **`moe-plugin-sdk`**: The formal contract for extending the platform with third-party providers.

## Internal Crates (Not Published)
These crates contain the sensitive execution substrate and experimental orchestration logic. They remain closed-source to protect the platform's architectural integrity.

- **`moe-core`**: The proprietary ternary inference engine.
- **`moe-runtime`**: Experimental ROL-13 orchestration logic.
- **`moe-ddel`**: Distributed execution primitives and partitioning logic.

## Stability Guarantees
- Public crates follow strict SemVer.
- Internal crates are managed within the monorepo and are subject to breaking changes.
- The platform maintains binary compatibility across versions for signed plugins.
