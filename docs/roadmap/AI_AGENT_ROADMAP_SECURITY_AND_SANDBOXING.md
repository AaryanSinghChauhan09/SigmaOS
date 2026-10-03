# SigmaOS AI Agent Roadmap: System Security, Sandboxing & Post-Quantum Cryptography

## Overview & Zero-Trust Defense In Depth
This document specifies the operational roadmap for AI engineering agents working on the security, privilege management, and cryptographic subsystems of SigmaOS. The goal is to combine the security principles of OpenBSD, FreeBSD, Linux LSMs, and modern Post-Quantum Cryptography (PQC) into a unified, zero-dependency Safe Rust architecture.

---

## 1. Process Isolation & Unprivileged Sandboxing

### Linux & BSD Inspiration Sources
- **Linux Landlock LSM (v5):** File path, network port, and IPC restrict access control for unprivileged processes without root capabilities.
- **OpenBSD Pledge & Unveil:** Restrict system call capabilities (`pledge("stdio rpath wpath cpath")`) and file system access views (`unveil("/usr/lib", "r")`).
- **FreeBSD Capsicum & Capability Mode (`procdesc`):** Descriptor-centric privilege separation and process descriptor file handles.

### AI Agent Execution Directives
1. **Capsicum & Capability Tokens:**
   - Maintain strongly-typed capability rights (`HashSet<CapRight>`) in `src/security/capsicum.rs` and `src/security/capability.rs`.
   - Implement `procdesc` process handle management preventing global PID namespace enumeration.
2. **Pledge & Unveil Enforcer:**
   - Validate syscall pledge bitmasks (`src/security/pledge.rs`) and path-traversal hardened unveil verification (`src/security/sovereign_portable_sandbox_engine.rs`).
3. **Landlock LSM Parity:**
   - Extend path restriction logic to enforce strict read, write, execute, and append rules for sandbox containers.

---

## 2. Hardware-Backed Privilege & Memory Protection

### Hardware Inspiration Sources
- **Intel Memory Protection Keys (MPK / PKEY):** Page-grained PKRU register domain switching without page-table TLB flushes.
- **Hardware Privilege Tokens:** YubiKey FIDO2, TPM 2.0 PCR key unsealing, smartcards, and biometric fingerprint sensors.

### AI Agent Execution Directives
1. **Intel MPK Engine Integration:**
   - Utilize `ProtectionKey`, `PkeyAccessRights`, `PkruRegister`, and `SovereignIntelMpkEngine` (`src/security/memory_protection.rs`) for nanosecond-level memory domain switching.
2. **Omarchy-Inspired Hardware Authentication:**
   - Extend `SovereignOmarchyHardwareAuthenticationEngine` (`src/security/hardware_privilege.rs`) with automated YubiKey hot-unplug screen auto-lock triggers and Dilithium-5 signed TPM 2.0 PCR unsealing.

---

## 3. Post-Quantum Cryptography & Identity Verification

### Modern Cryptographic Standards
- **NIST PQC Standards (2024+):** ML-KEM (Kyber-1024) key encapsulation and ML-DSA (Dilithium-5) digital signatures.
- **OpenBSD Signify & Minisign:** Light-weight signify signature verification for package releases and system updates.

### AI Agent Execution Directives
1. **Zero-Dependency Native Cryptography:**
   - Maintain pure Rust `#![no_std]` implementations in `src/crypto/` (`post_quantum.rs`, `pqc_dilithium.rs`, `aegis_vault.rs`).
2. **Dual-Key Signature Chain Verification:**
   - Enforce hybrid classical (Ed25519/Signify) + Post-Quantum (Dilithium-5) signature verification for all `SigmaPkg` release channels and kernel modules.
