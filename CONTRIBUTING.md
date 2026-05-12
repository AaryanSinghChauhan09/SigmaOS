# Contributing to SigmaOS

Welcome, Sovereign. SigmaOS is an industrial-grade operating system lattice built on the principles of **Silicon Sovereignty**, **Neural Intelligence**, and **Zero-Dependency** engineering.

## 🏛 The Sovereign Principles
1. **Zero-STL**: We do not use the standard library. All primitives must be defined in `sigma_libc.h` or `sigma_kernel_types.h`.
2. **Shard Isolation**: Every feature must be a "shard" with a strict interface. No circular dependencies.
3. **PQC-Native**: All inter-shard communication and binary distribution must be Post-Quantum Cryptography attested.

## 🛠 Development Workflow
1. **Sharding**: Identify the resource you are managing. Create a new shard in the appropriate layer.
2. **Registration**: Add your shard source path to `SHARDS.manifest`.
3. **Audit**: Ensure your shard uses `SigmaObject` for OOP parity.

## 🚀 Pull Request Process
- Ensure the build passes locally using `s-cli build`.
- Update the relevant Wiki page (Logic, API, etc.) to reflect your changes.
- Submit your PR with a clear "Industrial USP" (Unique Selling Point) explanation.

---
*Stay Sovereign.*
