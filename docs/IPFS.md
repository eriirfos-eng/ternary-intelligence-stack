# TIS IPFS Standards Library

To ensure the **Ternary Intelligence Stack (TIS)** remains permanently reachable even if centralized web infrastructure is compromised (e.g., the "Binary Dinosaurs"), we maintain a read-only mirror of our standards on IPFS.

## Latest Permanent CID
- **CID**: `(Manual Deployment Required)`
- **Gateway**: [https://ipfs.io/ipfs/Qm...](https://ipfs.io/ipfs/Qm...)

## Included Standards
- **TIS Standards**: Quality and Benchmarking
- **BET-ISA-v1.0**: Balanced Ternary Instruction Set
- **T-HARMONY**: Huawei Harmony OS NDK Bindings
- **T-BIO / T-CAD / T-SEC**: Domain-specific triadic standards

## Deployment
To update the IPFS mirror, run:
```bash
./scripts/ipfs-bridge.sh
```
*Note: Requires a local IPFS node (Kubo) to be running.*
