# Cryptography Component Guide

> **Related portals:** [Security, Sandboxing & Cryptography](./Home.md) &bull; **Maintainer Agent:** [`CRYPTOGRAPHY_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/CRYPTOGRAPHY_AGENT.md) &bull; **Status:** Canonical Sovereign Component

---

## Contents
1. [Overview & Architecture](#1-overview--architecture)
2. [Technical Specifications & Protocols](#2-technical-specifications--protocols)
3. [Multi-Language Implementations](#3-multi-language-implementations)
4. [Configuration & Usage](#4-configuration--usage)
5. [Diagnostics & Troubleshooting](#5-diagnostics--troubleshooting)
6. [See Also](#6-see-also)

---

## 1. Overview & Architecture
The **Cryptography** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Cryptography`
- **Subsystem Category**: Security, Sandboxing & Cryptography
- **Autonomous Agent**: [`CRYPTOGRAPHY_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/CRYPTOGRAPHY_AGENT.md)
- **Linux Inspiration**: Linux crypto API, AF_ALG sockets, getrandom() syscall, kernel keyring service.
- **BSD Inspiration**: OpenBSD arc4random and bcrypt, FreeBSD crypto framework (opencrypto), hardware TPM integration.

### Architectural Design
Provides  high-performance cryptographic primitives (AES-GCM, ChaCha20-Poly1305, Ed25519, SHA-256, BLAKE3), hardware acceleration (AES-NI), and secure RNG.

**Current Implementation Status**: Software SHA-256, ChaCha20-Poly1305, and basic CSPRNG implemented. AES-NI hardware instruction acceleration and TPM 2.0 key sealing need completion.
**Active Enhancement Objectives**: Hardware-accelerated AES-NI and SHA-NI assembly kernels, TPM 2.0 PCR sealed storage engine, constant-time side-channel audited cryptographic implementations.

---

## 2. Technical Specifications & Protocols
- **Memory Safety**: Clean abstraction layers preventing memory corruption and buffer overflow exploits.
- **Concurrency Model**: Fine-grained thread synchronization using standard mutexes, atomics, and channels.
- **Fault Isolation**: Sandboxed process model ensuring failures do not compromise other subsystems.

---

## 3. Multi-Language Implementations
- **Rust Engine**: Primary safe abstractions, state machines, and system integration.
- **Zig Accelerator**: Zero-allocation high-throughput data buffers and SIMD mathematical routines.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

### Standard Library Authorization
In accordance with updated SigmaOS governance, this subsystem drops all restrictions against standard library usage. Standard libraries (`std::sync::Arc`, `std::sync::Mutex`, `std::collections`, `libc`, and dynamic allocators) are authorized and actively utilized as required.

---

## 4. Configuration & Usage

```toml
# /etc/sigmaos/cryptography.toml
[subsystem]
enabled = true
log_level = "info"
performance_mode = "balanced"
sync_interval_ms = 1000
```

---

## 5. Diagnostics & Troubleshooting
To verify the operation and health of this component:

```bash
# Check subsystem daemon status
$ sigma-ctl status cryptography

# Inspect real-time log stream
$ journalctl -u cryptography -f --no-pager
```

---

## 6. See Also
- [Main Portal: Security, Sandboxing & Cryptography](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
