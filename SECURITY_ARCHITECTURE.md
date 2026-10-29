# SigmaOS Security Architecture & Hardening Model

## Core Security Principles

SigmaOS enforces defense-in-depth, least-privilege execution, capability-based access control, and memory safety invariants across the kernel and userland.

1. **Least Privilege by Default:** Every userland process starts with minimal privileges. Capabilities must be explicitly granted and bounded.
2. **Memory Safety Invariants:** All code written in safe Rust. Any `unsafe` block must document invariant safety guarantees and pass security review.
3. **Defense in Depth:** Privilege separation across process boundaries, syscall filtering (pledge/seccomp), path unveils, capability tokens, and Ring 0/Ring 3 isolation.
4. **Fail Closed:** Errors, resource exhaustion, or invalid input parameters immediately reject execution and log security diagnostics.
5. **No Custom Cryptography:** Standardized, peer-reviewed cryptographic primitives and Post-Quantum Cryptography (PQC) Dilithium/Sphincs implementations only.

---

## Security Layers & Subsystem Policies

### 1. Capability Access Control
Defined in `docs/security/CAPABILITY_POLICY.md`. System resources (devices, sockets, files, process creation) require fine-grained bitmask capabilities (`CapabilityToken`) preventing privilege escalation.

### 2. Syscall Sandboxing (Pledge & Unveil)
Inspired by OpenBSD, processes restrict executable syscall classes via `pledge` and restrict visible filesystem hierarchies via `unveil`. Attempts to invoke unpledged syscalls or access unveiled paths result in immediate SIGABRT / SIGKILL and security log generation.

### 3. Threat Model & Security Policy
Defined in `docs/security/THREAT_MODEL.md` and `docs/security/SECURITY_TEST_PLAN.md`. System components categorize attack surfaces (untrusted external inputs, network packets, userland IPC, device drivers) and maintain automated fuzz targets (`fuzz/`) and security regression tests.

### 4. Continuous Security Operations
- **Weekly Advisory Audits:** Dependencies and upstream CVE advisories reviewed continuously.
- **Automated Security Workflow:** `.github/workflows/security.yml` executes static security analysis, capability validation tests, and dependency auditing.
- **Security Review Requirement:** Any PR touching syscalls, privilege boundaries, memory management, cryptography, networking, drivers, or updates requires explicit security review.
