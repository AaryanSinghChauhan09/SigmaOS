# SigmaOS Threat Model

## Asset Classification
1. **Kernel Memory & Page Tables**: Critical integrity and confidentiality; protected via CR3 isolation, SMEP, SMAP, and W^X memory mappings.
2. **Cryptographic Key Material**: Private keys, disk encryption master keys, and package signing certificates; protected against memory exposure and timing side channels.
3. **User Files & IPC Channels**: Confidential user data protected by DAC permissions, capability flags, and mount namespaces.
4. **Hardware Access Ports**: PCI MMIO, DMA buffers, and I/O ports protected via IOMMU (VT-d/AMD-Vi) isolation.

---

## Threat Vectors & Mitigations

| Threat Vector | Potential Impact | SigmaOS Mitigation | Verification Mechanism |
|---|---|---|---|
| Malformed Syscall Arguments | Kernel panic or privilege escalation | Bounds checking, strict pointer validation, copy_from_user safe accessors | Fuzz smoke test, cargo test |
| Malformed ELF / Package Files | Code execution during load/install | Fuzzed ELF parser, memory boundaries, sandbox execution | `fuzz/` targets, dry-run installer |
| Unsafe Rust Invariant Violation | Memory corruption / UAF | Strict unsafe review rule (`[rule.unsafe_justification]`), automated unsafe grep audit | `.github/workflows/security.yml` |
| Supply Chain Dependency Vulnerability | Remote code execution via third-party crates | Pinned toolchains, automated `cargo audit`, weekly dependency review | Weekly chore(deps), cargo-audit CI |
| Side-Channel Leaks (Timing / Cache) | Key recovery | Constant-time cryptographic primitives, cache-line flushing | Cryptographic review, automated lints |
