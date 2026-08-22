# Zero-Dependency Subsystems in SigmaOS

## Overview

A core engineering principle of SigmaOS is **self-sufficiency and minimal external dependencies**. Rather than relying on external dynamic C libraries (`glibc`, `openssl`, `libsystemd`) or third-party precompiled packages, SigmaOS implements native, clean-room Rust equivalents designed for security, auditability, and deterministic builds.

---

## Zero-Dependency Matrix

| Traditional Component | SigmaOS Native Implementation | Zero-Dependency Benefits |
|-----------------------|------------------------------|--------------------------|
| **glibc / musl** | [`src/klib/`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/klib/) + `sigma_libc.h` | Embedded memory allocator, string operations, no runtime heap bloat |
| **systemd** | [`src/init/systemd_init.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/init/systemd_init.rs) + [`src/init/sigma_init.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/init/sigma_init.rs) | Service unit dependency graph, cgroup simulation, socket activation |
| **OpenSSL / libcrypto** | [`src/crypto/`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/crypto/) + [`kernel/crypto/`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/kernel/crypto/) | ChaCha20-Poly1305, Argon2id, BLAKE3, Ed25519, Kyber/Dilithium PQC |
| **libcap / SELinux** | [`src/security/capability.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/security/capability.rs) + [`src/security/mac.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/security/mac.rs) | In-kernel capability check with $O(1)$ bitmask matching |
| **DPKG / APT / RPM** | [`src/sigpkg/`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/sigpkg/) | In-memory DAG solver with cycle elimination and atomic filesystem rollback |
| **libevent / libuv** | [`src/distro/linux_bsd_inspirations.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/distro/linux_bsd_inspirations.rs) | SPSC lock-free ring buffer and `io_uring` kernel emulation |

---

## Architectural Guidelines

1. **No Unaudited External Crates in Core Kernel**: The kernel and core drivers execute in `#![no_std]` environments with dedicated memory allocators (`BuddyAllocator`, `SlabAllocator`).
2. **Deterministic Builds**: Build outputs are bit-for-bit reproducible across independent toolchain invocations.
3. **Safe Memory Abstractions**: All low-level MMIO, DMA, and page table interactions are encapsulated inside rigorously documented safe wrappers with `# Safety` contracts.
