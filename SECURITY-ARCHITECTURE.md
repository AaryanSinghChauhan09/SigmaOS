# SigmaOS — Security Architecture

## Overview

SigmaOS implements a comprehensive, defense-in-depth security architecture inspired by OpenBSD, Qubes OS, SELinux, and modern post-quantum cryptographic standards.

---

## Security Layers

```
┌─────────────────────────────────────────────┐
│  APPLICATION LAYER                          │
│  pledge() · unveil() · sandbox              │
├─────────────────────────────────────────────┤
│  MANDATORY ACCESS CONTROL (MAC)             │
│  SELinux policies · AppArmor profiles       │
├─────────────────────────────────────────────┤
│  DISCRETIONARY ACCESS CONTROL (DAC)         │
│  POSIX permissions · ACLs · capabilities    │
├─────────────────────────────────────────────┤
│  CRYPTOGRAPHIC LAYER                        │
│  Kyber-1024 KEM · Dilithium-5 signatures    │
│  PQC-TLS 1.3 · PKI · TPM 2.0 sealing       │
├─────────────────────────────────────────────┤
│  KERNEL SECURITY                            │
│  W^X memory · KASLR · securelevels         │
│  Audit logging · Vulnerability scanning     │
└─────────────────────────────────────────────┘
```

---

## 1. Process Sandboxing — `pledge()` and `unveil()`

### pledge() — Syscall Restriction
Inspired by OpenBSD, `pledge()` restricts a process to only the syscalls it declares it needs:

```rust
// Example: restrict a network process to only networking + IO
process.pledge(&["inet", "rpath", "stdio"])?;
// After this point, any other syscall kills the process with SIGABRT
```

**Module:** [`src/security/pledge.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/security/pledge.rs)

**Pledge groups available:**
| Group | Allowed Operations |
|-------|--------------------|
| `stdio` | Read/write to already-opened files |
| `rpath` | Read from filesystem |
| `wpath` | Write to filesystem |
| `cpath` | Create files/directories |
| `inet` | IPv4/IPv6 networking |
| `unix` | Unix domain sockets |
| `exec` | Execute new processes |
| `proc` | Process management |
| `id` | Change UID/GID |
| `tty` | Terminal operations |

### unveil() — Filesystem Restriction
Restrict which filesystem paths a process can access:

```rust
// Only allow read access to /etc and write to /tmp
process.unveil("/etc", "r")?;
process.unveil("/tmp", "rw")?;
process.unveil_finalize()?; // no more paths allowed after this
```

**Module:** [`src/security/unveil.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/security/unveil.rs)

---

## 2. Post-Quantum Cryptography

### Key Exchange — Kyber-1024 (KEM)
SigmaOS uses CRYSTALS-Kyber for all key encapsulation:
- **Security level:** NIST Level 5 (equivalent to AES-256)
- **Algorithm:** Module-LWE based lattice cryptography
- **Quantum resistant:** Yes — secure against Shor's algorithm

```
Alice                              Bob
  |                                 |
  |--[Kyber public key]------------>|
  |                                 |
  |<-[encapsulated ciphertext]------|
  |                                 |
  Both derive shared secret (ss)    |
```

### Digital Signatures — Dilithium-5
All kernel modules, packages, and boot components are signed with Dilithium-5:
- **Security level:** NIST Level 5
- **Algorithm:** Module-LWE + Module-SIS lattice signatures
- **Verification:** Checked at load time for all kernel modules

### PQC-TLS 1.3
Network connections use a hybrid PQC+classical TLS 1.3:
- Key exchange: X25519 + Kyber-1024 (hybrid)
- Authentication: ECDSA P-384 + Dilithium-5 (hybrid)
- Cipher: AES-256-GCM or ChaCha20-Poly1305

**Module:** [`src/security/pki.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/security/pki.rs)

---

## 3. Mandatory Access Control (MAC)

### SELinux-style Policies
**Module:** [`src/security/audit.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/security/audit.rs)

- Every process has a security context: `user:role:type:level`
- Policy rules define allowed transitions between types
- Default-deny: anything not explicitly allowed is denied
- Policy enforcement happens in the kernel before each syscall

### AppArmor-compatible Profiles
Per-application confinement profiles:
- Restrict file access patterns (glob-based)
- Network access restrictions
- Capability restrictions
- Signal delivery restrictions

---

## 4. Capability System

**Module:** [`src/security/capability_enforcer.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/security/capability_enforcer.rs)

POSIX capabilities divide root privileges into fine-grained units:

| Capability | Purpose |
|------------|---------|
| `CAP_NET_ADMIN` | Configure network interfaces |
| `CAP_SYS_ADMIN` | Various system admin operations |
| `CAP_SYS_PTRACE` | Process tracing (debug) |
| `CAP_SETUID` | Change process UID |
| `CAP_NET_BIND_SERVICE` | Bind port <1024 |
| `CAP_SYS_RAWIO` | Raw I/O access |
| `CAP_MKNOD` | Create device files |
| `CAP_AUDIT_WRITE` | Write to audit log |

Processes start with empty capability sets and must explicitly request what they need.

---

## 5. TPM 2.0 Integration

**Module:** [`src/tpm/`](file:///home/aaryansinghchauhan/SigmaOS/src/tpm/)

- **Secure Boot:** Boot chain measured into TPM PCRs; kernel sealed to known-good measurements
- **Key Sealing:** Encryption keys sealed to TPM; only released if boot chain is unmodified
- **Remote Attestation:** TPM quote proves system integrity to remote verifiers
- **Disk Encryption:** LUKS-equivalent keys stored in TPM, auto-released on verified boot

---

## 6. Qubes OS-Inspired Domain Isolation

**Module:** [`src/security/qubes_isolation.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/security/qubes_isolation.rs)

Each security domain runs in an isolated VM:

```
┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│  Work Qube  │  │Banking Qube │  │ Untrusted   │
│  (trusted)  │  │ (high-sec)  │  │  Qube       │
└──────┬──────┘  └──────┬──────┘  └──────┬──────┘
       │                │                │
       └────────────────┴────────────────┘
                        │
                 ┌──────┴──────┐
                 │  Network VM │
                 │  (isolated) │
                 └─────────────┘
```

- **Template VMs:** Shared read-only base images
- **Disposable Qubes:** Ephemeral VMs destroyed after use
- **Clipboard isolation:** Explicit copy-paste between qubes only
- **Network VM:** All network access isolated to dedicated VM

---

## 7. Kernel Memory Security

### W^X (Write XOR Execute)
No memory page can be simultaneously writable and executable:
- JIT compilers must use mprotect() to toggle W/X
- Stack is non-executable by default
- Heap is non-executable by default

### KASLR (Kernel Address Space Layout Randomization)
- Kernel mapped at random address on each boot
- Module base addresses randomized
- Makes ROP/JOP attacks significantly harder

### Stack Canaries
- Per-function stack canaries detect stack overflows
- Canary values derived from hardware RNG at boot
- Canary mismatch triggers immediate kernel panic

---

## 8. Security Audit Log

**Module:** [`src/security/audit.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/security/audit.rs)

All security-relevant events are logged:
- Process credential changes
- Failed syscall attempts (pledge violations)
- MAC policy denials
- Authentication events
- Privilege escalation attempts
- Network connection attempts

Audit logs are write-once, tamper-evident (Merkle tree chained).

---

## 9. Vulnerability Management

**Module:** [`src/security/vulnerability.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/security/vulnerability.rs)

- Continuous runtime vulnerability scanning
- CVE database integration for installed packages
- Automatic security update notifications
- Kernel live-patching for critical vulnerabilities

---

## Security Checklist for Developers

- [ ] Does your code use `unsafe`? Add `// SAFETY:` comment
- [ ] Does your process need filesystem access? Use `unveil()`
- [ ] Does your process need specific syscalls? Use `pledge()`
- [ ] Are secrets stored securely? Use `src/security/secrets.rs`
- [ ] Is network traffic encrypted? Use PQC-TLS 1.3
- [ ] Are packages signed? Verify Dilithium-5 signature at install

---

*Last updated: 2026-08-23 | SigmaOS Security Team*
