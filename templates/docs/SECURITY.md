# Security Policy — COMPONENT_NAME

---

## Supported Versions

| Version | Supported |
|---------|-----------|
| latest `main` | ✅ Yes |
| < latest release | ⚠️ Security fixes backported for 6 months |
| > 2 releases behind | ❌ No |

---

## Reporting a Vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

### Preferred Channel

Email: **security@sigmaos.dev** *(PGP key: <!-- link to keyserver -->)*

### What to Include

Please provide as much of the following as possible:

- **Component name and version** (`COMPONENT_NAME --version`)
- **Description** of the vulnerability and its potential impact
- **Steps to reproduce** (minimal proof-of-concept if available)
- **Affected configurations** (OS, architecture, feature flags)
- **Suggested fix** (optional but appreciated)

### Response Timeline

| Stage | Target |
|-------|--------|
| Acknowledgement | 48 hours |
| Initial assessment | 5 business days |
| Fix available (critical) | 14 days |
| Fix available (high) | 30 days |
| Fix available (medium/low) | Next scheduled release |
| Public disclosure | After fix is released (coordinated) |

We follow responsible disclosure. Reporters who follow this process will be
credited in the release notes (unless they prefer to remain anonymous).

---

## Secure Coding Standards

All contributions to `COMPONENT_NAME` must satisfy the following standards.

### Memory Safety

- `#![forbid(unsafe_code)]` is enforced at the compiler level.
- Any exception requires an **Architecture Decision Record (ADR)** approved by
  a maintainer, and the `unsafe` block must be wrapped in a `// SAFETY:` comment
  explaining why it is correct.
- All buffer accesses use Rust's checked slice indexing or iterator APIs.
- No raw pointer arithmetic except in explicitly justified kernel/driver code.

### Input Validation (Defensive Programming)

- Every public API validates all inputs at the boundary before processing.
- Reject invalid input early (fail-fast). Never silently discard bad data.
- String inputs are validated for length, encoding, and expected format.
- Numeric inputs are checked for overflow (enabled in all profiles except release
  — mitigated by using `checked_*` arithmetic in security-sensitive paths).
- Path inputs are canonicalised and checked for traversal attempts before use.

### Least Privilege

- Components request only the OS capabilities they require.
- No ambient authority: capabilities are injected, never ambient globals.
- File I/O is limited to the minimum required paths.
- No network access unless the component's purpose explicitly requires it.

### Secure Defaults

- Default configuration is the most restrictive configuration.
- Features that expand the attack surface (e.g., `async`, `metrics`) are opt-in
  feature flags disabled by default.
- No debug endpoints, dev-mode backdoors, or hardcoded credentials — ever.

### Cryptography

- Do not implement cryptographic primitives. Use audited crates:
  - Hashing: `sha2`, `blake3`
  - Symmetric encryption: `aes-gcm` (via `ring` or `RustCrypto`)
  - Asymmetric / signatures: `ed25519-dalek`, `p256`
  - TLS: `rustls` (not OpenSSL unless specifically justified)
- Use constant-time comparison for secrets (`subtle` crate).
- Never log secrets, tokens, keys, or passwords — not even at `TRACE` level.

### Dependency Management

- Dependencies are pinned to exact or tightly bounded versions in `Cargo.toml`.
- Run `make audit` (`cargo audit`) before every release.
- New dependencies must be reviewed for:
  - Active maintenance (last commit < 12 months ago)
  - Licence compatibility (MIT, Apache-2.0, BSD preferred)
  - Absence of known CVEs
  - Minimal transitive dependency footprint
- Prefer crates from the RustCrypto, Tokio, or Serde ecosystem over unknowns.

### Concurrency

- All shared mutable state protected by `Mutex`, `RwLock`, or atomics.
- Lock ordering is documented in ARCHITECTURE.md and strictly followed.
- No `unsafe` cell sharing across thread boundaries.
- Prefer lock-free and immutable designs where feasible.

### Error Handling

- Errors are never silently swallowed (no `let _ = result;` for fallible ops).
- Error messages must not contain secrets, internal paths, or stack traces in
  production (log them at DEBUG/TRACE, return sanitised messages to callers).
- All error types are `#[non_exhaustive]` to allow adding variants without
  breaking downstream callers.

### Logging and Observability

- Production log output must never contain:
  - Passwords, API keys, tokens, or private keys
  - Personally identifiable information (PII)
  - Full file system paths that reveal system layout
- Sanitise user-supplied data before including it in log messages.
- Use structured logging where possible to avoid injection via log messages.

---

## Threat Model

### Assets

| Asset | Sensitivity | Protection Mechanism |
|-------|------------|---------------------|
| Component configuration | Medium | File permissions, no secrets in config |
| Runtime state | Medium | Process isolation |
| <!-- asset --> | <!-- L/M/H --> | <!-- mechanism --> |

### Adversary Assumptions

- **Local attacker** with user-level access to the system.
- **Remote attacker** with network access to exposed interfaces (if any).
- **Malicious input** via any API boundary (files, IPC, CLI args).
- We do **not** defend against: hardware attacks, physical access, or
  a compromised kernel.

### Attack Surface

| Surface | Exposed To | Mitigation |
|---------|-----------|------------|
| CLI arguments | Local user | clap validation, length limits |
| Config file | Local user | Schema validation on load |
| <!-- surface --> | <!-- who --> | <!-- mitigation --> |

---

## Security Checklist for Contributors

Before opening a security-relevant PR:

- [ ] No `unsafe` blocks without `// SAFETY:` justification and ADR
- [ ] All inputs validated at the boundary
- [ ] No secrets or PII in logs, errors, or test fixtures
- [ ] No hardcoded credentials or tokens
- [ ] `cargo audit` shows zero high/critical vulnerabilities
- [ ] New dependencies reviewed (maintenance, licence, CVEs)
- [ ] Error messages sanitised for production
- [ ] Constant-time comparison used where secrets are compared
- [ ] File paths validated against traversal

---

## Past Security Advisories

| ID | Severity | Component | Fixed In | Summary |
|----|---------|-----------|----------|---------|
| <!-- SIGMA-SA-YYYY-NNN --> | <!-- Critical/High/Medium/Low --> | <!-- component --> | <!-- version --> | <!-- one-line summary --> |

*(No advisories yet.)*

---

## References

- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [RustSec Advisory Database](https://rustsec.org/)
- [CWE Top 25 Most Dangerous Software Weaknesses](https://cwe.mitre.org/top25/)
- [ANSSI Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- SigmaOS ARCHITECTURE.md — threat model detail
- SigmaOS CONTRIBUTING.md — secure coding principles
