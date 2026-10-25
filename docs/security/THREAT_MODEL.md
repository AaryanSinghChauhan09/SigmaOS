# SigmaOS Threat Model

This document outlines the threat vectors, asset classifications, attacker assumptions, and security boundaries for SigmaOS.

## Threat Vectors & Countermeasures

| Threat Vector | Targeted Asset | Mitigation Strategy | Enforcing Component |
|---|---|---|---|
| **Buffer Overflow / Arbitrary Code Execution** | Kernel Memory / Syscall Handlers | Safe Rust `#![no_std]`, W^X page protection, buffer length bounds checks | `src/kernel/syscalls.rs` |
| **Privilege Escalation** | Root Authority / Kernel Capabilities | Capability tokens, non-setuid privilege separation, Capsicum Casper sandboxing | `src/userland/security_sandbox.rs` |
| **Path Traversal / Unauthorized File Access** | Filesystem Data | Unveil path restriction, POSIX file permissions, VFS path normalization | `src/vfs/posix_path.rs` |
| **Cryptographic Tampering / Spoofing** | Packages & Keyrings | Post-quantum Dilithium-5 / FALCON signature verifier, Cosign attestation | `src/crypto/` |
| **Denial of Service / Memory Starvation** | Run Queue / Frame Allocator | Bounded run queues, starvation boosting, bounded swap slots, OOM killer | `src/scheduler/run_queue.rs` |

---

## Attacker Profile Assumptions
- **Local Low-Privilege Attacker**: Has user shell access (`uid=1000`). Attempting to gain kernel privilege or breach process memory isolation.
- **Untrusted Package Payload**: Malicious third-party `.sigpkg` payload attempting path traversal or unauthorized hardware access during installation.
- **Malicious Network Packet**: Remote attacker transmitting malformed Ethernet/IP packets to breach network buffer parsers.
