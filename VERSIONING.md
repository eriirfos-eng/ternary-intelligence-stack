# VERSIONING.md

MoE-13 follows Semantic Versioning (SemVer) for all public crates: `moe-sdk`, `moe-platform`, and `moe-plugin-sdk`.

## Compatibility Rules

### Breaking Changes (Major)
Any change that alters the public API or breaks deterministic execution guarantees:
- Modifying `MoEPlatform::load` or `MoEPlatform::run` signatures.
- Changing `MoePlugin` trait definition.
- Modifying the CMIR binary contract.
- Altering the SPF-13 artifact format.

### Safe Extensions (Minor)
Additions that do not affect existing workflows:
- Adding new `Capability` variants.
- Adding non-breaking methods to `MoePlugin`.
- Adding new `ModelProvider` variants that do not break binary compatibility.

### Patches (Patch)
Bug fixes that do not change public interfaces or execution behavior.

## Deterministic Guarantee
The system guarantees that identical inputs, plugin states, and system configurations will yield bit-perfect reproducible outputs. Any deviation in the output stream constitutes a breaking change.
