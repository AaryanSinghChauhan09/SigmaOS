# SigmaOS Threat Model & Boundary Analysis

This document details threat scenarios, trust boundaries, attacker profiles, and mitigations across the SigmaOS operating system ecosystem.

---

## Trust Boundaries

1. **Userland vs. Kernel Boundary**: Ring 3 application code calling into Ring 0 via syscalls (`Posix450SyscallDispatcher`).
2. **IPC Boundary**: Processes communicating across microkernel / IPC channels using `CapabilityToken` checks.
3. **Driver & Hardware Boundary**: Userland / WDM drivers accessing MMIO registers, DMA channels, and PCI/USB devices.
4. **Network & Package Boundary**: External packet streams and untrusted software package installation via `sigpkg`.

---

## Attacker Profiles

| Profile | Capability / Access | Objective | Primary Mitigation |
|---|---|---|---|
| **Untrusted Application** | Unprivileged Ring 3 execution | Arbitrary code execution in kernel / Ring 0 | Capability tokens, W^X memory, SECCOMP sandboxing |
| **Malicious Driver** | Driver loading / IO permissions | Memory corruption or hardware takeover | NonPaged pool isolation, IOMMU DMA protection |
| **Network Attacker** | External network packet injection | Memory buffer overflow or panic | Bounds-checked packet parsing, Dilithium PQC TLS/VPN |
| **Supply Chain Attacker** | Package repository manipulation | Malicious package payload execution | Post-quantum Dilithium-5 signatures, atomic rollbacks |

---

## Mitigations Matrix

- **Kernel Memory Safety**: Safe Rust abstractions for physical frame allocation, bounds-clamped DMA buffers, and explicit `// SAFETY:` invariants.
- **Syscall Input Validation**: Every pointer, buffer length, and file descriptor passed across syscall boundaries is validated before dereferencing.
- **Supply Chain Verification**: `sigpkg` verifies Dilithium-5 cryptographic signatures and SHA-256 package manifests prior to extraction.
