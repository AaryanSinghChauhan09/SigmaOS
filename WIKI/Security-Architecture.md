# SigmaOS Security Architecture

## Overview

SigmaOS implements a multi-layered, defense-in-depth security model inspired by the best practices from Parrot OS, QubesOS, OpenBSD, and Grsecurity-hardened Linux kernels. All security primitives are implemented as **clean-room, zero-dependency** Rust modules within the SigmaOS kernel.

---

## Security Layers

```
┌─────────────────────────────────────────┐
│         User Applications               │
├─────────────────────────────────────────┤
│   Sigma-sh (Hardened Shell + Sandbox)   │
├─────────────────────────────────────────┤
│   Landlock LSM │ Seccomp-BPF │ AppArmor │
├─────────────────────────────────────────┤
│   Capability Enforcer │ MAC Framework   │
├─────────────────────────────────────────┤
│   SigmaCrypto │ PKI │ Secrets Manager  │
├─────────────────────────────────────────┤
│   Kernel ASLR │ SMEP │ SMAP │ CET      │
└─────────────────────────────────────────┘
```

---

## 1. Mandatory Access Control (MAC)

**Module:** `src/security/mac.rs`

SigmaOS implements a Parrot/SELinux-inspired Mandatory Access Control system:
- **Subjects**: Processes with assigned security labels
- **Objects**: Files, sockets, memory regions with security contexts
- **Policies**: Allow/Deny rules evaluated at every access attempt

```rust
// Example: Check if process can read a file
let allowed = mac_check(subject_label, object_label, Permission::Read);
```

### Policy Types
| Policy | Description | Inspired By |
|--------|-------------|-------------|
| TE (Type Enforcement) | Label-based mandatory rules | SELinux |
| RBAC | Role-based access grouping | Grsecurity |
| MLS | Multi-level security lattice | Common Criteria |

---

## 2. SovereignLandlock LSM

**Module:** `src/distro/linux_bsd_inspirations.rs`

Clean-room implementation of Linux Landlock security module for filesystem sandboxing:

```rust
let mut lsm = SovereignLandlockLsm::new(true); // enforcement mode
lsm.add_rule("/home/user", LandlockAccess::ReadOnly);
lsm.add_rule("/home/user/downloads", LandlockAccess::ReadWrite);
// /etc/shadow → denied (no rule = deny-by-default)
```

**Key Properties:**
- **Deny-by-default**: Any path without an explicit rule is denied
- **Longest-prefix matching**: More specific rules take precedence
- **Hierarchical rules**: Parent rules apply to subdirectories unless overridden

---

## 3. Cryptographic Subsystem

**Module:** `src/crypto/` & `kernel/crypto/`

### Implemented Algorithms (Zero External Dependencies)

| Algorithm | Module | Use Case |
|-----------|--------|----------|
| ChaCha20-Poly1305 | `kernel/crypto/chacha20.rs` | Stream cipher + AEAD |
| AES-256-GCM | `src/crypto/` | Block cipher AEAD |
| Argon2id | `crypto/sigma_key_derive.rs` | Password KDF |
| BLAKE3 | `src/crypto/` | Hash/MAC |
| Ed25519 | `src/security/pki.rs` | Digital signatures |
| X25519 | `src/crypto/` | Key exchange |
| LUKS-compatible | `kernel/core/crypto/sigma_luks.rs` | Disk encryption |

### Key Derivation Policy
- **All KDF operations** use Argon2id with minimum: m=65536, t=3, p=4
- **No hardcoded keys or salts** in production code (test-only exceptions annotated)
- **Key rotation** supported through `SecretsManager`

---

## 4. Capability Enforcement

**Module:** `src/security/capability_enforcer.rs`

POSIX capabilities are enforced at the kernel level:
- CAP_NET_ADMIN — network configuration
- CAP_SYS_PTRACE — process tracing
- CAP_DAC_OVERRIDE — filesystem permission bypass
- CAP_SETUID/SETGID — privilege changes

Processes must explicitly acquire capabilities through the capability broker.

---

## 5. Process Isolation

### Namespaces
SigmaOS implements process isolation via clean-room namespace simulation:
- **PID namespaces**: Process ID isolation between containers
- **Network namespaces**: Virtual network stack per container
- **Mount namespaces**: Independent filesystem view

### eBPF-Inspired Sandboxing

**Module:** `src/distro/linux_bsd_inspirations.rs` — `SovereignEbpfEngine`

Static program verification before execution prevents:
- Division by zero
- Out-of-bounds register access
- Infinite loops (bounded iteration check)
- Invalid jump targets

---

## 6. Audit System

**Module:** `src/security/audit.rs`

Every security-relevant event is logged with:
- Timestamp (monotonic clock)
- Process ID + label
- Action attempted
- Decision (allow/deny) + policy rule matched

---

## 7. Parrot Security Parity

**Module:** `src/security/parrot_linux.rs`

Implements Parrot OS-inspired features:
- **AnonSurf**: System-wide traffic anonymization mode
- **Exploit prevention**: Stack canaries, heap hardening simulation
- **Forensics mode**: Read-only root filesystem with tmpfs overlay
- **Hardening profiles**: `security_profile_apply()` for different threat levels

---

## Code Scanning & Compliance

Security alerts are tracked at:  
https://github.com/AaryanSinghChauhan09/SigmaOS/security/code-scanning

### Alert Categories Fixed
- ✅ Unused variables (prefixed with `_`)
- ✅ Hardcoded crypto values moved to runtime generation (test-only with `#[allow]`)
- ✅ Missing `# Safety` docs on unsafe functions
- ✅ Invalid pointer access patterns reviewed
- ✅ XSS prevention in web_ui components

---

## Threat Model

SigmaOS targets **workstation and server** deployments with:
1. **Physical access attacker**: Disk encryption (LUKS-compatible) + Secure Boot
2. **Network attacker**: Minimal attack surface, port isolation by default
3. **Malicious software**: MAC + Landlock + Capabilities restrict damage
4. **Insider threat**: Audit logs + MLS classification

*See also: [SECURITY_HARDENING_COMPLETE.md](file:///home/aaryansinghchauhan/SigmaOS/SECURITY_HARDENING_COMPLETE.md)*
