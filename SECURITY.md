# 🔐 Security Policy — SigmaOS

## Supported Versions

SigmaOS follows a rolling release model. Only the latest commit on `main` is supported.

| Version | Supported |
|---------|----------|
| `main` (latest) | ✅ Fully Supported |
| Older commits | ❌ Not Supported |

---

## 🚨 Reporting a Vulnerability

**Please do NOT report security vulnerabilities via GitHub Issues.**

To responsibly disclose a vulnerability in SigmaOS:

1. **Email**: Send details to `security@sigmaos.dev` (or open a GitHub Security Advisory)
2. **Include**:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)
3. **Response time**: We aim to respond within **48 hours** and patch within **7 days** for critical issues.

---

## 🛡️ Security Model

SigmaOS is built with security-first principles:

### Memory Safety
- Written entirely in **safe Rust** — no `unsafe` blocks except where absolutely required
- Zero buffer overflows, use-after-free, or data races by design
- Rust's borrow checker eliminates entire classes of vulnerabilities

### Capability-Based Security
- **Sentinel subsystem** enforces capability tokens for all system calls
- Processes receive only the capabilities they explicitly need
- Inspired by OpenBSD's `pledge()` and `unveil()` syscalls

### Path Traversal Protection
- All path components validated against `..`, `.`, `/`, `\`, `:` boundaries
- Prevents directory traversal attacks (CVE-class vulnerabilities)
- Implemented in `src/security/input_validation.rs`

### Cryptography
- Post-quantum cryptography: **ML-KEM (Kyber)**, **ML-DSA (Dilithium)**
- Classical: **Ed25519**, **AES-256-GCM**, **SHA3-256/512**
- No hardcoded secrets, salts, or cryptographic keys in source code

### CI/CD Security
- All GitHub Actions use pinned SHA hashes for actions
- `permissions: contents: read` on all workflows
- OSV Scanner runs on every PR
- Secrets stored only in GitHub Secrets, never in code

### Container Isolation
- OCI containers run in capability-isolated namespaces
- Seccomp profiles block unnecessary syscalls
- KVM-based VMs for stronger isolation

---

## 🏆 Security Hall of Fame

We are grateful to security researchers who have responsibly disclosed vulnerabilities:

| Researcher | Issue | Date |
|-----------|-------|------|
| *(None yet — be the first!)* | — | — |

---

## 📋 Known Security Mitigations

| Threat | Mitigation |
|--------|------------|
| Buffer overflow | Rust type system + bounds checking |
| Use-after-free | Rust ownership model |
| Data races | Rust `Send`/`Sync` traits |
| Privilege escalation | Capability-based access control |
| Path traversal | Input validation in `src/security/input_validation.rs` |
| Supply chain attacks | Pinned CI actions, Cargo lock file |
| Hardcoded secrets | Runtime key derivation only |
| Container escapes | Seccomp + capability namespaces |

---

*SigmaOS Security Team*
