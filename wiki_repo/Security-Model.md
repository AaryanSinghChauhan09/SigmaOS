# SigmaOS Security Model
**Version:** 1.0 | **Status:** Formal Specification | **Classification:** Public

---

## 1. Overview

SigmaOS employs a **layered, capability-based security architecture** rooted in formal verification (Ada/SPARK), zero-trust network isolation, and a sovereign cryptographic stack. No component implicitly trusts another — every interaction is explicitly authorized.

> **Core Principle:** *Deny by default. Allow by explicit capability grant.*

---

## 2. Threat Model

### 2.1 Attacker Capabilities

| Threat Actor | Capabilities | Mitigations |
|---|---|---|
| Remote attacker | Network exploitation, phishing | Zero-trust network, sigma-shield firewall |
| Malicious package | Supply chain compromise | Ed25519 + SHA-256 verification, reproducible builds |
| Compromised userland | Privilege escalation | Capability-based sandbox, syscall audit |
| Physical attacker | Boot-time tampering | Secure boot + TPM attestation, rollback protection |
| Insider / rogue process | Side-channel, memory attacks | Hardened allocator, ASLR, stack canaries |

### 2.2 Assets to Protect
- Kernel integrity (immutable after boot)
- User credentials and secrets (`sigma-crypto-vault`)
- Network communications (E2E encrypted)
- Package supply chain (cryptographic chain of trust)
- Filesystem data (VFS RBAC + optional encryption)

---

## 3. Capability-Based Sandbox

### 3.1 Design

SigmaOS implements a **Capsicum-inspired capability model**. Every process receives a minimal capability token at spawn time. No ambient authority — capabilities must be explicitly delegated.

```
┌─────────────────────────────────────────────────────────┐
│              PROCESS CAPABILITY TOKEN                    │
│                                                          │
│  cap_fs:    [read:/home/user, write:/home/user/docs]     │
│  cap_net:   [tcp:443, udp:53]                            │
│  cap_ipc:   [zenith-compositor, sigma-audio-daemon]      │
│  cap_dev:   []    ← no device access                     │
│  cap_syscall: [read, write, mmap, futex, exit]           │
└─────────────────────────────────────────────────────────┘
```

### 3.2 Capability Types

| Capability | Description |
|---|---|
| `cap_fs` | Filesystem read/write/exec per path |
| `cap_net` | Network access (protocol, port ranges) |
| `cap_ipc` | IPC bus access to named services |
| `cap_dev` | Hardware device access |
| `cap_syscall` | Permitted syscall whitelist |
| `cap_spawn` | Right to create child processes |
| `cap_time` | Access to monotonic/wall clock |

### 3.3 Sandbox Profiles

```toml
# /etc/sigmaos/sandbox/sigma-browser.toml
[sandbox]
profile = "network-isolated"
cap_fs = ["/home/user/Downloads:rw", "/tmp/sigma-browser:rw"]
cap_net = ["tcp:80", "tcp:443"]
cap_ipc = ["zenith-compositor", "sigma-audio"]
cap_dev = []
cap_syscall = ["minimal"]  # ~30 syscalls vs 400+ on Linux
seccomp_bpf = true
landlock = true
```

---

## 4. Syscall Audit Framework

### 4.1 Architecture

The **sigma-audit** daemon intercepts all syscalls via a kernel-level BPF hook, logs them to a tamper-evident append-only ledger, and can block or alert on policy violations.

```
Process → [syscall] → Kernel Interceptor → BPF Filter
                                               │
                              ┌────────────────┴───────────────┐
                              │  ALLOW (whitelist hit)          │
                              │  DENY + SIGKILL (blacklist hit) │
                              │  LOG + ALERT (anomaly detected)  │
                              └────────────────────────────────┘
                                               │
                                     Audit Ledger (append-only)
                                     /var/log/sigma-audit.log
```

### 4.2 Audit Log Format

```
[2026-01-01T00:00:00Z] pid=1234 comm=sigma-browser syscall=open
  path=/etc/passwd flags=O_RDONLY
  DECISION=DENY (cap_fs restriction: /etc not in allow list)
  ALERT=true severity=HIGH
```

### 4.3 Policy-as-Code

Audit policies are written in a declarative TOML DSL:

```toml
# /etc/sigma-audit/policy.toml
[[rule]]
name = "block-ptrace"
match.syscall = "ptrace"
match.process_except = ["sigma-debugger"]
action = "deny_sigkill"
severity = "CRITICAL"

[[rule]]
name = "log-network-calls"
match.syscall = ["connect", "bind", "sendto"]
action = "log"
severity = "INFO"
```

---

## 5. Secure Boot & Rollback Protection

### 5.1 Boot Chain

```
UEFI Firmware (TPM 2.0 PCR measurements)
     │
     ▼
sigma-bootloader (Ed25519 verified against sovereign root key)
     │
     ▼
Kernel Image (SLSA L3 reproducible build, hash in TPM PCR[8])
     │
     ▼
initramfs (dm-verity integrity check)
     │
     ▼
sigma-init (capability setup, audit daemon start)
     │
     ▼
Userland (profile packages, sandboxed)
```

### 5.2 A/B Partition Rollback

- Two kernel slots: **Slot A** (active) and **Slot B** (backup)
- On update: write to inactive slot → verify hash → flip active pointer atomically
- If boot of new slot fails 3× → automatic rollback to previous slot
- TPM extends PCR[9] with slot change events (audit trail)

### 5.3 Sovereign Root Key

The **sovereign root key** is an Ed25519 keypair:
- Private key: HSM-protected, never leaves signing infrastructure
- Public key: burned into firmware and distributed with the ISO
- Key rotation: requires 3-of-5 multisig from SigmaOS core maintainers

---

## 6. Hardened Memory Allocator

### 6.1 Design Principles (inspired by OpenBSD malloc, hardened_malloc)

- **Magic cookie validation:** Every allocation tagged with `0xSIGMA5A5A`
- **Guard pages:** Unmapped pages before/after each allocation region
- **Randomized base:** ASLR + randomized heap layout per boot
- **Use-after-free detection:** Poison freed memory with `0xDEADBEEF`
- **Double-free detection:** Allocation bitmap tracks state
- **Size segregation:** Separate arenas per size class (8, 16, 32, 64, 128, 256 bytes)

### 6.2 sigma_malloc ABI

```c
// kernel/libc/sigma_malloc.h
void *sigma_malloc(size_t size);          // Always zero-initializes
void  sigma_free(void *ptr);              // Validates cookie + poisons
void *sigma_realloc(void *ptr, size_t);  // Safe resize with validation
void *sigma_calloc(size_t n, size_t sz); // Overflow-checked
```

---

## 7. Sovereign Cryptographic Library

All cryptographic operations use **sigma-crypto** — a formally verified Ada/SPARK library:

| Algorithm | Use Case | Standard |
|---|---|---|
| Ed25519 | Package signing, boot verification | RFC 8032 |
| X25519 | Key exchange (WireGuard, TLS) | RFC 7748 |
| ChaCha20-Poly1305 | Symmetric encryption | RFC 8439 |
| Kyber-1024 | Post-quantum key encapsulation | NIST PQC |
| SHA-256 / SHA-3 | Integrity hashing | FIPS 180-4 |
| Argon2id | Password hashing | RFC 9106 |
| AES-256-GCM | Block cipher (hardware-accelerated) | FIPS 197 |

All modules are SPARK-proven: no buffer overflows, no integer overflows, no use-after-free.

---

## 8. Zero-Trust Network Architecture

```
Internet ──────► sigma-shield (BPF firewall)
                      │
                      ▼
               Network Namespace
               ┌─────────────────────────────┐
               │  sigma-dns (DoH resolver)   │
               │  WireGuard VPN tunnel       │
               │  TLS 1.3 + Kyber-1024       │
               └─────────────────────────────┘
                      │
                      ▼ (only via IPC bus)
               Application Sandbox
               (no direct socket access)
```

- GUI apps: **no** direct network access — all requests proxied via `sigma-netd`
- DNS: DoH only (no cleartext UDP/53 from userland)
- Outbound firewall: default-deny, whitelist per-app

---

## 9. sigma-crypto-vault (Sovereign Credential Manager)

Sovereign replacement for KeePass/Bitwarden:

- Master key: Argon2id-derived from passphrase + hardware token (FIDO2/TPM)
- Vault file: ChaCha20-Poly1305 encrypted, Ed25519-signed
- No cloud sync by default — sovereign storage only
- CLI: `sigma-vault get/set/delete/generate`
- TOTP support built-in

---

## 10. Compliance & Formal Verification

| Standard | Status | Notes |
|---|---|---|
| SLSA Level 3 | 🎯 Planned v1.0 | Reproducible ISO builds |
| SPARK proof coverage | 🔄 In progress | sigma-crypto fully proven |
| FIPS 140-2 | 🎯 Planned | crypto primitives |
| Zero-Trust (NIST 800-207) | ✅ Architecture compliant | Network + IPC isolation |
| Common Criteria EAL4+ | 🎯 Long-term | Formal security evaluation |

---

*Document maintained by SigmaOS Security Team. Report vulnerabilities via [SECURITY.md](../SECURITY.md).*
