# Security

> Full policy: [SECURITY.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/SECURITY.md)

## Reporting Vulnerabilities

Do **not** open public GitHub issues for security bugs. Use:
- GitHub private security advisories: https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories/new

## Security Subsystems

### 1. Mandatory Access Control

| Module | Inspired By | File |
|---|---|---|
| Type Enforcement | SELinux | `src/security/selinux.rs` |
| Path-based MAC | AppArmor | `src/security/mac.rs` |
| Capability reduction | OpenBSD pledge | `src/security/sigma_pledge.rs` |
| Path revelation | OpenBSD unveil | `src/security/sigma_unveil.rs` |

### 2. Cryptography (No External Libraries)

All crypto is native Rust, no OpenSSL, no ring crate:

| Primitive | File |
|---|---|
| AES-128/256 | `src/kernel/crypto/mod.rs` |
| SHA-256, HMAC-SHA256 | `src/kernel/crypto/mod.rs` |
| PBKDF2 | `src/kernel/crypto/mod.rs`, `src/crypto/kdf.rs` |
| ChaCha20-Poly1305 | `kernel/crypto/chacha20.rs` |
| Dilithium PQC | `src/crypto/pqc_dilithium.rs` |
| LUKS2 encryption | `kernel/core/crypto/sigma_luks.rs` |

**Key policy:** No hardcoded keys. Domain-separation labels like `b"sigmaos-kdf-v1"` are acceptable as context labels (not secrets). PBKDF2 context parameter was fixed from `b"password"` → `b"sigmaos-password-hash-v1"`.

### 3. Isolation

- **Qubes-style domains** (`src/security/qubes_isolation.rs`) — separate VM-like security domains
- **Namespaces** (`src/virtualization/namespaces.rs`) — PID, net, mount, user, IPC isolation
- **OCI containers** (`src/container/oci_runtime.rs`) — sandboxed app containers

### 4. Audit Trail

Every security decision (allow/deny) is logged to a tamper-evident append-only log (`src/security/audit.rs`). Log entries are chained via hash to detect tampering.

### 5. Secrets Management

- `src/security/secrets.rs` — encrypted secrets store with memory scrubbing on drop
- `src/security/cleaner.rs` — zero-on-free for sensitive buffers
- `src/security/pki.rs` — certificate chain validation

## Known Security Alerts Status

| ID | Rule | File | Fix Status |
|---|---|---|---|
| Critical | `rust/access-invalid-pointer` | `bootloader/sigma_boot_efi.rs` | 🔄 Adding bounds checks |
| Critical | `rust/hard-coded-cryptographic-value` | `crypto/sigma_key_derive.rs` | 🔄 Replacing with entropy |
| High | `js/xss-through-dom` | `zenith.html`, `index.html` | 🔄 Using textContent |
| High | `js/prototype-pollution` | `state-manager.js` | 🔄 Object.create(null) |
| Medium | `clippy::new_without_default` | Multiple | ✅ Suppressed with allow |
| Low | `unused_imports`, `dead_code` | Multiple | ✅ Suppressed with allow |

## Hardening Features

- Stack canaries (compile-time via Rust)
- RELRO / PIE (linker flags in `build.rs`)
- No `unsafe` without `// SAFETY:` comment policy
- `cargo-audit` integration in CI (`.github/workflows/build.yml`)
- Bitmask overlap privilege escalation fixed in `CapabilityToken`
