# Contributing to SigmaOS

Welcome to the **Sovereign Lattice™**! We are building the world's first industrial-grade, profession-aware operating system, and we need your help to scale the lattice.

## 🏛 How to Contribute

### 1. Developing Shards

SigmaOS is modular. All features must be implemented as isolated **shards** within the lattice.

- Use the **Sovereign SDK** for all kernel interactions.
- Ensure zero-dependency APIs.
- All code must be C++17 compliant.

### 2. Professional Profiles

We aim to support 600+ professions. If your profession is missing from `PROFESSION-MAP.md`, please submit a PR with:

- A new tool manifest.
- Resilience strategies for your specific role.
- Integration tests for your professional tools.

### 3. Hardware Drivers

Hardware drivers are implemented as HAL shards.

- Follow the `SovereignDriverFramework` guidelines.
- Use software fallbacks whenever possible to ensure universal "Zenith" compatibility.

## 🔒 Security Requirements

- All commits must be signed.
- New shards must include PQC-attestation metadata.
- No legacy C libraries are allowed; use `SovereignLibC`.

## 🤝 Community

- **Discord**: [Join the SigmaOS Shard](https://discord.gg/sigmaos)
- **Mailing List**: `dev@sigmaos.org`

---
*By contributing, you agree that your code will be licensed under the Sovereign Open Source License.*
