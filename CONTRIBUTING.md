# Contributing to SigmaOS

Welcome to the SigmaOS industrial development lattice. To maintain silicon sovereignty and architectural purity, all contributors must adhere to the following industrial standards.

## Shard-First Development

- **No Derivatives**: Do not import standard libraries (`libc`, `libstdc++`, `stl`). Use only Sovereign primitives.

- **PQC Attestation**: Every commit must be signed with a PQC-compatible key (Kyber/Dilithium).

- **Isolation**: Ensure your shard has zero unintended side-effects on the global lattice.

## Pull Request Process

1. **Shard Proposal**: Open an issue detailing the shard's mission, absorbed patterns, and industrial utility.

2. **Implementation**: Build your shard in the appropriate directory (`kernel/core/`, `drivers/`, etc.).

3. **Verification**: Your shard must pass the `verify_sovereignty.ps1` audit and the `S-FSCK` consistency check.

4. **Documentation**: Update the Wiki with shard architecture and syscall additions.

## Coding Standards

- Use `sigma_u32`, `sigma_u64` for fixed-width types.

- Follow the `SigmaObject` and `SigmaSingleton` OOP patterns.

- Keep comments professional and mission-focused.

Join us in building the Zenith Singularity.
