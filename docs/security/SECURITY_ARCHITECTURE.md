# SigmaOS Security Architecture

## Overview

SigmaOS implements a defense-in-depth security model combining capability-based access control, strict memory isolation, post-quantum cryptographic signature verification, and automated continuous security auditing.

## Core Security Pillars

### 1. Capability-Based Access Control
- Process capabilities are explicitly granted via cryptographically unforgeable capability tokens.
- Every system call entrypoint validates capability token scope and process bounds before dispatch.
- Mandatory Access Control policies prevent privilege escalation from unprivileged userspace.

### 2. Memory Isolation & Hardening
- **W^X Execution Policy:** Kernel and user memory regions enforce Write XOR Execute protection.
- **Address Sanitizer (ASan):** Shadow memory maps detect use-after-free, double-free, and redzone buffer overflows (`src/security/address_sanitizer.rs`).
- **Paging & User/Kernel Separation:** 4-level x86_64 page tables enforce user/kernel privilege boundaries with strict page table permissions.

### 3. Supply Chain & Cryptographic Security
- **Post-Quantum Cryptography (PQC):** Package management and kernel update paths mandate Dilithium-5 signatures (`src/package/universal.rs`).
- **Reproducible Builds & SBOM:** Software Bill of Materials and Cosign attestation workflows verify binary integrity (`.github/workflows/reproducible-sbom-cosign.yml`).

### 4. Sandboxing & Process Isolation
- Seccomp-style system call filtering limits available system call interfaces per process namespace.
- Process isolation boundaries prevent cross-namespace unauthorized resource access (`tests/process/namespaces/`).

## Threat Modeling & Policies

- **Threat Model:** Detailed threat vector analysis in [`docs/security/THREAT_MODEL.md`](docs/security/THREAT_MODEL.md).
- **Capability Policy:** Process token requirements in [`docs/security/CAPABILITY_POLICY.md`](docs/security/CAPABILITY_POLICY.md).
- **Security Test Plan:** Verification procedures in [`docs/security/SECURITY_TEST_PLAN.md`](docs/security/SECURITY_TEST_PLAN.md).
- **Security CI Workflow:** Automated security scanning in [`.github/workflows/security.yml`](.github/workflows/security.yml).
