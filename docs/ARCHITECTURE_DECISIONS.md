# SIGMAOS ARCHITECTURE DECISIONS (ADR)

## ADR-001: Zero-Dependency `#![no_std]` Bare-Metal Sovereignty
- **Status**: Accepted
- **Context**: SigmaOS aims to be a zero-trust, zero-dependency bare-metal operating system.
- **Decision**: All kernel and core userland subsystems are implemented in safe systems languages (Rust, Zig, Nim) without reliance on standard C libraries or language std runtimes.

## ADR-002: Direct DRM/KMS Zenith Compositor
- **Status**: Accepted
- **Context**: Display server dependencies like X11 or Wayland introduce heavy abstractions and security risks.
- **Decision**: Zenith renders directly to DRM/KMS hardware display planes with type-safe Rust tiling WM extensions.

## ADR-003: Post-Quantum Cryptography & Zero-Trust Capabilities
- **Status**: Accepted
- **Context**: Standard classical cryptography is vulnerable to quantum attacks.
- **Decision**: Kyber-1024 and Dilithium-5 PQC primitives are integrated into the kernel memory, network, and package layers alongside OpenBSD-inspired capability tokens and Retguard protections.
