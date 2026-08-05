# SigmaOS Security Policy

## Reporting a Vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Email: security@sigmaos.dev (or open a private GitHub Security Advisory at https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories/new)

We will acknowledge receipt within 48 hours and provide a fix timeline within 7 days for critical issues.

---

## Security Architecture

SigmaOS implements a layered, defense-in-depth security model inspired by the best of Linux security subsystems:

### Mandatory Access Control (MAC)
- **SELinux-compatible** type enforcement (`src/security/selinux.rs`)
- **AppArmor-style** path-based profiles (`src/security/mac.rs`)
- **Sigma-Pledge** capability reduction (OpenBSD pledge-inspired, `src/security/sigma_pledge.rs`)
- **Sigma-Unveil** filesystem revelation (OpenBSD unveil-inspired, `src/security/sigma_unveil.rs`)

### Isolation & Sandboxing
- **Qubes-style** VM-based isolation domains (`src/security/qubes_isolation.rs`)
- **OCI container** runtime with seccomp-like filtering (`src/container/oci_runtime.rs`)
- **Namespaces**: PID, network, mount, IPC, user (`src/virtualization/namespaces.rs`)
- **Capability tokens** with bitmask overlap protection (`src/security/capability_token.rs`)

### Cryptography (Post-Quantum Ready)
- Native Rust implementations — **no OpenSSL dependency**
- AES-128/256, SHA-256, HMAC-SHA256, PBKDF2 (`src/kernel/crypto/mod.rs`)
- ChaCha20-Poly1305 (`src/crypto/`)
- Dilithium PQC signatures (`src/crypto/pqc_dilithium.rs`)
- LUKS2-compatible encrypted volumes (`src/crypto/`)

### Boot Security
- UEFI Secure Boot chain (`bootloader/sigma_boot_efi.rs`)
- TPM 2.0 integration for measured boot (`src/tpm/module.rs`)
- Verified boot with signature checking (`src/boot/verified.rs`)

### Network Security
- TLS stack (native, no OpenSSL) (`src/net/`)
- WireGuard-compatible VPN (`src/security/vpn.rs`)
- Fail2ban-style intrusion detection (`src/security/intrusion.rs`)
- SSSD offline credential caching (`src/compatibility/sssd.rs`)

### Audit & Forensics
- Tamper-evident audit log (`src/security/audit.rs`)
- Forensics toolkit (`src/security/forensics.rs`)
- Defensive audit trail (`src/security/defensive_audit.rs`)

---

## Known Security Alerts & Remediation

### Critical (Recently Fixed)

| Alert | File | Status |
|---|---|---|
| `rust/access-invalid-pointer` | `src/boot/uefi.rs` | ✅ Fixed — Added null checks and bounds validation for raw pointer operations |
| `rust/hard-coded-cryptographic-value` | `src/crypto/primitives.rs` | ✅ Fixed — Enhanced entropy collection with multiple hardware and software sources |

### High

| Alert | File | Remediation |
|---|---|---|
| `js/xss-through-dom` | UI files | Replacing `innerHTML` with `textContent` / DOM API |
| `js/prototype-pollution` | `state-manager.js` | Using `Object.create(null)` for configuration objects |

### Medium (Clippy Lints)

Most `clippy::new_without_default`, `unused_imports`, and `dead_code` warnings are suppressed via `#![allow(...)]` in module files. These will be progressively fixed with proper `impl Default` implementations.

---

## Security Hardening Checklist

- [x] No hardcoded cryptographic keys in production code paths
- [x] All crypto uses OS entropy (`#[no_std]` compatible entropy mixing)
- [x] Bitmask overlap privilege escalation fixed (`src/security/capability.rs`)
- [x] Secrets cleared from memory after use (`src/security/cleaner.rs`)
- [x] PKI certificate validation chain (`src/security/pki.rs`)
- [x] Password hashing uses domain-separated PBKDF2 (`src/crypto/kdf.rs`)
- [x] Full audit of raw pointer derefs in bootloader — null checks and bounds validation added
- [ ] Complete JS XSS remediation in web UI (in progress)

---

## Supported Versions

| Version | Supported |
|---|---|
| `main` branch | ✅ Active security fixes |
| Tagged releases | ✅ Critical fixes backported |
| Old branches | ❌ No longer maintained |
