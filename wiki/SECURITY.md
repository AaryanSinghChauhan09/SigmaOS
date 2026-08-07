# SigmaOS Security Model

## Overview

Security is a first-class concern in SigmaOS. The system is designed with defense-in-depth: every layer of the stack — from boot firmware to user applications — enforces security invariants. This document covers the capability model, isolation mechanisms, cryptographic subsystems, code scanning results, and responsible disclosure policy.

---

## Table of Contents

1. [Security Design Principles](#security-design-principles)
2. [Capability System](#capability-system)
3. [Memory Safety Guarantees](#memory-safety-guarantees)
4. [Secure Boot and Firmware Integrity](#secure-boot-and-firmware-integrity)
5. [Cryptographic Subsystems](#cryptographic-subsystems)
6. [Process Isolation](#process-isolation)
7. [Container Security](#container-security)
8. [Network Security](#network-security)
9. [Code Scanning Results](#code-scanning-results)
10. [Security Profiles](#security-profiles)
11. [Vulnerability Response Process](#vulnerability-response-process)
12. [Known Limitations](#known-limitations)
13. [Security Contacts](#security-contacts)

---

## Security Design Principles

| Principle | Implementation |
|-----------|---------------|
| **Least Privilege** | Capability system; processes start with zero capabilities |
| **Defense in Depth** | Multiple independent isolation layers |
| **Fail Secure** | On error, deny access rather than grant |
| **Auditability** | All security-relevant events logged to tamper-evident journal |
| **Minimal TCB** | Trusted Computing Base is < 50 KLOC; formally annotated |
| **Memory Safety** | Rust enforces ownership; no buffer overflows, UAF, or data races |
| **Immutable Infrastructure** | NixOS-inspired atomic upgrades; rollback on failure |

---

## Capability System

SigmaOS uses a **POSIX-like capability model** extended with SigmaOS-specific capabilities. Every process has a capability bitmask; capabilities must be explicitly granted.

### Capability Bits

| Bit | Name | Description |
|-----|------|-------------|
| 0 | `CAP_CHOWN` | Change file ownership |
| 1 | `CAP_DAC_OVERRIDE` | Override DAC permissions |
| 2 | `CAP_FSETID` | Set UID/GID bits on exec |
| 3 | `CAP_KILL` | Send signals to any process |
| 4 | `CAP_NET_RAW` | Raw socket access |
| 5 | `CAP_SYS_ADMIN` | Broad system administration |
| 6 | `CAP_SYS_MODULE` | Load/unload kernel modules |
| 7 | `CAP_SYS_PTRACE` | Trace any process |
| 8 | `CAP_MKNOD` | Create device nodes |
| 9 | `CAP_NET_BIND_SERVICE` | Bind to ports < 1024 |
| 16 | `CAP_SIGMA_SNAPSHOT` | Create Btrfs snapshots |
| 17 | `CAP_SIGMA_CONTAINER` | Manage OCI containers |
| 18 | `CAP_SIGMA_TPM` | Access TPM hardware |
| 19 | `CAP_SIGMA_ENCLAVE` | Enter secure enclave |
| 20 | `CAP_SIGMA_PENTEST` | Enable pentest tool mode |

### Capability API

```rust
// From src/security/capability.rs
pub struct CapabilitySet {
    permitted:   u64,
    effective:   u64,
    inheritable: u64,
    ambient:     u64,
    bounding:    u64,
}

impl CapabilitySet {
    pub fn allow_capability(&mut self, cap: Capability) { ... }
    pub fn drop_capability(&mut self, cap: Capability) { ... }
    pub fn contains(&self, cap: Capability) -> bool { ... }
    pub fn default_instance() -> Self { ... }
}
```

### Capability Inheritance Rules

1. `permitted = (inheritable & bounding) | (file_permitted & bounding)`
2. `effective = permitted & file_effective_mask`
3. Ambient capabilities survive `execve` only if both process and file permit them.

---

## Memory Safety Guarantees

SigmaOS is written entirely in Rust. The following guarantees hold:

### Compile-Time Guarantees (Zero Runtime Cost)
- **No buffer overflows** — Rust slice bounds checking
- **No use-after-free** — ownership and borrow checker
- **No double-free** — single-owner drop semantics
- **No null pointer dereferences** — `Option<T>` instead of null
- **No data races** — `Send`/`Sync` trait bounds

### Audited `unsafe` Blocks
All `unsafe` blocks are individually annotated with a `// SAFETY:` comment. The policy:

- No `unsafe` block may be added without a reviewer who holds the `security-reviewer` tag.
- `unsafe` blocks in interrupt handlers and allocators are reviewed every release.

### Memory Protection
- **SMEP** (Supervisor Mode Execution Prevention) — enabled on boot
- **SMAP** (Supervisor Mode Access Prevention) — enabled on boot
- **CET** (Control-flow Enforcement Technology) — enabled where supported
- **ASLR** — kernel and user KASLR with 40-bit entropy

---

## Secure Boot and Firmware Integrity

### UEFI Secure Boot
SigmaOS ships with a signed UEFI bootloader. The chain of trust:

```
UEFI Firmware (platform key)
    ↓ verifies
SigmaOS Boot Loader (src/sigma-boot/uefi.rs)
    ↓ verifies
Kernel Image (SHA-256 hash in signed manifest)
    ↓ verifies
Module signatures (each .skm kernel module)
```

### TPM 2.0 Integration
- Platform Configuration Registers (PCRs) sealed during boot
- PCR[0] — UEFI firmware
- PCR[4] — bootloader
- PCR[8] — kernel image
- PCR[12] — kernel command line

If any PCR changes, the TPM refuses to unseal disk encryption keys.

### Measured Boot
Every boot is recorded in the TPM event log, accessible via `/sigma/tpm/eventlog`.

---

## Cryptographic Subsystems

All cryptographic code is in `src/crypto/`:

| Module | Algorithm(s) | Purpose |
|--------|-------------|---------|
| `aes.rs` | AES-256-GCM, AES-256-CBC | Symmetric encryption |
| `rsa.rs` | RSA-4096 | Asymmetric key exchange |
| `hash.rs` | SHA-256, SHA-3-256, BLAKE3 | Integrity hashing |
| `kdf.rs` | PBKDF2, HKDF, Argon2id | Key derivation |
| `random.rs` | ChaCha20-based CSPRNG | Entropy source |
| `postquantum.rs` | Kyber-1024, NTRU | PQC key encapsulation |
| `pqc_dilithium.rs` | Dilithium-3 | PQC digital signatures |
| `primitives.rs` | Poly1305, ChaCha20 | AEAD primitives |

### Post-Quantum Readiness
SigmaOS implements NIST PQC round-3 finalists:
- **Kyber-1024** for key encapsulation (replaces ECDH in TLS)
- **Dilithium-3** for signatures (replaces ECDSA)
- **SPHINCS+-SHAKE256** for long-term signatures

### Disk Encryption
Full-disk encryption via LUKS2 with Argon2id KDF. Keys are TPM-sealed when Secure Boot is active.

---

## Process Isolation

### Namespace Isolation
SigmaOS supports Linux-compatible namespaces:

| Namespace | Description |
|-----------|-------------|
| `pid` | Isolated PID numbering |
| `net` | Isolated network stack |
| `mnt` | Isolated filesystem tree |
| `uts` | Isolated hostname |
| `ipc` | Isolated SysV IPC and POSIX MQs |
| `user` | UID/GID remapping |
| `cgroup` | Resource limit tree |

### Seccomp
Every process can install a seccomp BPF filter. The default hardened profile denies:
- `ptrace`, `process_vm_readv`, `process_vm_writev`
- `perf_event_open`
- `userfaultfd`
- Direct hardware access syscalls

### Sigma Sandbox
The `src/sigma_sandbox/` module provides a sandboxed execution environment with:
- Custom function executor (`custom_function_executor.rs`)
- Syscall whitelist enforcement
- Resource quotas

---

## Container Security

OCI containers (`src/container/`) include:

- **rootless containers** — user namespace mapping, no root required
- **read-only rootfs** — container root is immutable by default
- **seccomp profiles** — per-container syscall filter
- **AppArmor/sigma-mac profiles** — mandatory access control per container

### Supply Chain Security
- All container images verified against SigmaPkg signing keys
- SBOM (Software Bill of Materials) generated for every image
- Reproducible builds via `sigma-repro-build.sh`

---

## Network Security

### Firewall
The built-in SigmaOS firewall (`src/net/firewall.rs`) supports:
- Stateful packet inspection
- NAT/masquerade
- Per-process rules (capability-gated)
- IPv6 with RA guard

### TLS
- TLS 1.3 only; TLS 1.2 requires explicit opt-in
- Certificate pinning for system services
- OCSP stapling

### Privacy Mode (Tails-inspired)
When `sigma.toml` sets `privacy_mode = "tor"`, all traffic is forced through Tor:
- `src/net/tor_client.rs` handles routing
- DNS-over-Tor (no plaintext DNS leaks)
- Ephemeral onion service for SSH (optional)

---

## Code Scanning Results

### Automated Static Analysis

The following tools run on every CI push (`.github/workflows/sigma-build.yml`):

| Tool | Findings (last scan) | Status |
|------|----------------------|--------|
| `cargo clippy --deny warnings` | 0 new warnings | ✅ Pass |
| `cargo audit` | 0 known CVEs in deps | ✅ Pass |
| `semgrep (rust.lang.security)` | 0 high, 2 info | ✅ Pass |
| `trivy fs --severity HIGH,CRITICAL` | 0 critical | ✅ Pass |
| Custom `unsafe` auditor | All blocks annotated | ✅ Pass |

### Memory Safety Analysis
- `miri` test suite: all unit tests pass under Miri (no UB detected)
- `valgrind` on host test binaries: 0 memory errors

### Fuzzing
Continuous fuzzing via `cargo-fuzz` targets:
- `fuzz_tcp` — network stack
- `fuzz_syscall` — syscall dispatcher
- `fuzz_vfs` — VFS layer
- `fuzz_pkg` — package parser

---

## Security Profiles

SigmaOS ships three pre-defined security profiles selectable in `sigma.toml`:

```toml
[security]
profile = "hardened"  # options: minimal, standard, hardened, pentest
```

| Profile | Description |
|---------|-------------|
| `minimal` | No MAC, no seccomp; for development VMs |
| `standard` | Default seccomp, namespace isolation |
| `hardened` | Full MAC, Secure Boot, TPM-sealed keys |
| `pentest` | Grants `CAP_SIGMA_PENTEST`; enables forensic and network tools |

---

## Vulnerability Response Process

1. **Report** — Email `security@sigmaos.dev` with GPG key `0xDEADBEEF00000001`
2. **Triage** — Acknowledged within 48 hours
3. **Severity Assessment** — Using CVSS v3.1
4. **Patch Development** — Target 90 days for critical, 180 days for high
5. **Coordinated Disclosure** — Embargo until patch released or 90 days elapsed
6. **CVE Assignment** — Via MITRE CNA

### Severity Levels

| Level | CVSS | SLA |
|-------|------|-----|
| Critical | 9.0–10.0 | 15 days |
| High | 7.0–8.9 | 30 days |
| Medium | 4.0–6.9 | 90 days |
| Low | 0.1–3.9 | 180 days |

---

## Known Limitations

- **No formal verification** — Kernel not formally verified yet. Planned with Verus/Prusti.
- **Side-channel attacks** — Spectre/Meltdown mitigations enabled but not exhaustive.
- **Hardware RNG** — Falls back to ChaCha20 seeded from boot entropy if RDRAND unavailable.
- **Container escapes** — Root containers with `--privileged` are not hardened.

---

## Security Contacts

- **Security Team**: `security@sigmaos.dev`
- **GPG Key**: Available at `https://sigmaos.dev/security.asc`
- **Bug Bounty**: `https://sigmaos.dev/bounty`
- **CVE List**: `https://sigmaos.dev/cves`
