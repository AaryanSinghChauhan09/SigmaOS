# Security Policy

## Reporting a Vulnerability

**Please do NOT open public GitHub issues for security vulnerabilities.**

Report security vulnerabilities privately via:
- **GitHub Security Advisories**: https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories/new
- **Email**: aaryansinghchauhan09@gmail.com (subject: `[SECURITY] SigmaOS`)

We aim to acknowledge reports within **48 hours** and provide a fix timeline within **7 days** for critical issues.

### What to Include

1. Description of the vulnerability
2. Steps to reproduce
3. Potential impact assessment
4. Suggested fix (optional)

---

## Security Model

SigmaOS implements a defence-in-depth security model with multiple independent layers:

```
┌──────────────────────────────────────────────────┐
│               APPLICATION LAYER                   │
│  Sandboxed via pledge + unveil (OpenBSD model)    │
├──────────────────────────────────────────────────┤
│               CAPABILITY LAYER                    │
│  Capsicum capability-mode (FreeBSD model)         │
│  Every FD has an explicit rights bitmask          │
├──────────────────────────────────────────────────┤
│         MANDATORY ACCESS CONTROL LAYER            │
│  SELinux type-enforcement policies                │
│  TrustedBSD MAC framework                         │
├──────────────────────────────────────────────────┤
│              ISOLATION LAYER                      │
│  Jails (FreeBSD) + Namespaces (Linux) + cgroups   │
├──────────────────────────────────────────────────┤
│          KERNEL HARDENING LAYER                   │
│  KASLR · W^X · SMEP/SMAP · Retguard · CET        │
├──────────────────────────────────────────────────┤
│           HARDWARE SECURITY LAYER                 │
│  TPM 2.0 · Secure Boot · IOMMU · MTE             │
└──────────────────────────────────────────────────┘
```

---

## Memory Safety Guarantees

### Rust-Enforced Safety

All kernel code is written in Rust with:
- **No memory unsafety by default** — the borrow checker eliminates use-after-free, double-free, and buffer overflows at compile time.
- **`unsafe` blocks are audited** — every `unsafe {}` block in the kernel has an accompanying `// SAFETY:` comment explaining the invariant being upheld.
- **No null pointer dereferences** — `Option<T>` is used throughout; raw pointer access requires explicit unsafe.

### Checked `unsafe` Inventory

To view all unsafe blocks:
```bash
grep -rn 'unsafe {' src/ --include='*.rs' | wc -l
grep -rn 'unsafe {' src/ --include='*.rs'
```

Policy: every `unsafe` block must:
1. Have a `// SAFETY:` comment
2. Justify why the operation is memory-safe despite the compiler not verifying it
3. Be as narrow (small) as possible

---

## Kernel Hardening Features

### Address Space Randomisation
| Feature | Description | Status |
|---------|-------------|--------|
| KASLR | Kernel text/data at random physical base | ✅ Implemented |
| KARL | Kernel relinking (OpenBSD-style) | ✅ Implemented |
| ASLR | Userspace address randomisation | ✅ Implemented |
| PIE | Position-independent executables | ✅ Default |

### Memory Protection
| Feature | Description | Status |
|---------|-------------|--------|
| W^X | Write XOR Execute enforcement | ✅ Enforced |
| SMEP | Supervisor Mode Execution Prevention | ✅ Enabled |
| SMAP | Supervisor Mode Access Prevention | ✅ Enabled |
| Guard pages | Stack overflow detection | ✅ Implemented |
| Canaries | Stack smashing protection | ✅ Retguard |

### Control Flow Integrity
| Feature | Description | Status |
|---------|-------------|--------|
| Retguard | Return-address shadow stack (OpenBSD) | ✅ Implemented |
| CFI | Clang CFI for indirect calls | 🔧 In progress |
| CET | Intel Control-flow Enforcement Technology | 🗓 Planned |
| BTI | ARM Branch Target Identification | 🗓 Planned |

---

## Process Isolation

### pledge() — Syscall Restriction

Processes declare a whitelist of syscall categories at startup. Any syscall outside the declared set kills the process immediately:

```rust
// Example: restrict a network daemon to only network + file I/O
process.pledge(&[PledgeClass::Inet, PledgeClass::Rpath, PledgeClass::Wpath])?;
```

Available pledge classes:
- `stdio` — standard I/O
- `rpath` — read-only filesystem
- `wpath` — write filesystem
- `cpath` — create/delete filesystem entries
- `inet` — IPv4/IPv6 network sockets
- `unix` — Unix domain sockets
- `exec` — execute programs
- `proc` — process management
- `id` — UID/GID changes
- `prot_exec` — mprotect(PROT_EXEC)
- `dns` — DNS resolution only

### unveil() — Filesystem Path Masking

Restricts which filesystem paths a process can access:

```rust
// Reveal only /etc/sigma-config in read-only mode
process.unveil("/etc/sigma-config", UnveilMode::ReadOnly)?;
// After this, any access to other paths returns ENOENT
```

### Capsicum Capabilities

File descriptors carry an explicit rights bitmask. A process cannot gain rights it wasn't given at fd creation:

```rust
let limited_fd = fd.limit_rights(cap_rights![CAP_READ, CAP_SEEK])?;
// Now limited_fd cannot write or execute
```

---

## Cryptographic Standards

### Symmetric Encryption
- **ChaCha20-Poly1305** — default AEAD cipher
- **AES-256-GCM** — hardware-accelerated alternative
- **BLAKE3** — default hash function (faster than SHA-3)

### Asymmetric / Key Exchange
- **CRYSTALS-Kyber** (ML-KEM) — post-quantum key encapsulation
- **CRYSTALS-Dilithium** (ML-DSA) — post-quantum digital signatures
- **X25519** — Diffie-Hellman key exchange (legacy compat)
- **Ed25519** — signing (legacy compat)

### Key Storage
- Secrets never stored in plaintext on disk
- TPM 2.0 PCR-bound key sealing
- Kernel keyring isolation per process namespace

---

## Network Security

- **WireGuard** — all inter-node communication default
- **DNSSEC** — DNS response validation
- **DoH/DoT** — DNS over HTTPS/TLS
- **TLS 1.3 only** — TLS 1.0/1.1/1.2 disabled by default
- **Perfect Forward Secrecy** — enforced on all TLS connections
- **RPKI** — BGP route origin validation

---

## Supply Chain Security

### Package Integrity
- All packages signed with Ed25519 keys
- Content-addressed store — package hash is its install path
- Build reproducibility — identical inputs always produce identical outputs
- SBOM (Software Bill of Materials) generated for every package
- Sigstore transparency log integration (planned)

### Build System
- Hermetic builds — no network access during compilation
- Reproducible builds — timestamps stripped, paths normalised
- `Cargo.lock` pinned — no floating dependency versions

---

## Vulnerability Response Process

1. **Report received** → Acknowledge within 48 hours
2. **Triage** → Assess severity (CVSS score), affected components
3. **Fix development** → Private branch, no public disclosure yet
4. **Testing** → Security regression tests added
5. **Coordinated disclosure** → Notify reporter 7 days before publish
6. **Release** → Patch release + security advisory
7. **Post-mortem** → Root cause analysis added to docs

### Severity Classification

| Severity | CVSS Range | Response SLA |
|----------|-----------|-------------|
| Critical | 9.0 – 10.0 | 24 hours |
| High | 7.0 – 8.9 | 72 hours |
| Medium | 4.0 – 6.9 | 14 days |
| Low | 0.1 – 3.9 | 30 days |

---

## Responsible Disclosure Hall of Fame

_No vulnerabilities reported yet. Be the first!_

---

## Security Audits

SigmaOS welcomes third-party security audits. If you are interested in conducting a security review, contact us at the email above.

Audit reports will be published in `docs/security-audits/` upon completion.
