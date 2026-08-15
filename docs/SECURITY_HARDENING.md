# SigmaOS Security Hardening Guide

## 1. Memory Safety
SigmaOS is written in Rust, which prevents out-of-bounds reads, use-after-free, and double-free vulnerabilities at compile time.

## 2. Capability Tokens
All APIs use capability tokens instead of ambient authority (like traditional UID/GID systems). Ensure you drop capabilities when they are no longer needed.

## 3. Sandboxing Mechanisms
- **Namespaces**: Process, Network, Mount, IPC namespaces isolate resources.
- **Pledge/Unveil**: Granular runtime restrictions dynamically reduce a process's attack surface.

## 4. Cryptography Primitives
- Internal cryptographic routines use verified Rust crypto libraries (e.g., ring, RustCrypto).
- Secure boot chains verify kernel modules before loading.

## 5. Audit Logging
- Syscall auditing logs are stored in a protected, append-only buffer memory structure.
