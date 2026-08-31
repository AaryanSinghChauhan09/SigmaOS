# SigmaOS Security Policy

## Vulnerability Reporting

**DO NOT** open a public GitHub issue for security vulnerabilities.

Instead use one of:

1.  **GitHub Private Vulnerability Reporting** — click *Security → Report a vulnerability* in this repo.
2.  **Email** — security@sigmaos.dev (PGP key fingerprint in SECURITY.md).

We aim to acknowledge reports within **48 hours** and issue a patch within **14 days** for critical issues.

***

## Coding Standards for Security

### 1. No Panics in Production Code

| ❌ Forbidden | ✅ Required |
|------------|-----------|
| `.unwrap()` without explanation | `.unwrap()` with `// SAFETY:` comment proving it cannot fail |
| `.expect("msg")` in non-test code | `?` operator, `match`, or `if let` |
| `unreachable!()` in hot paths | Explicit exhaustive match |

### 2. Unsafe Code Rules

Every `unsafe` block **must** be preceded by a `// SAFETY:` comment explaining:

*   Why the operation is valid.
*   What invariant guarantees soundness.
*   Who is responsible for maintaining that invariant.

```rust
// SAFETY: `ptr` was obtained from `Box::into_raw` above and is still
//         exclusively owned — no aliases exist at this point.
unsafe { drop(Box::from_raw(ptr)); }
```

Unsafe blocks must be reviewed by **≥ 2 maintainers** before merge.

### 3. Input Validation

All data that crosses a trust boundary **must** pass through
`crate::security::input_validation` before use:

```rust
use crate::security::input_validation::{validate_path, validate_username};

fn open_config(path: &[u8]) -> Result<(), SigmaError> {
    validate_path(path)?;
    // ...proceed only after validation succeeds
}
```

Validation rules:

*   Reject NUL bytes in strings.
*   Enforce length limits (`MAX_PATH_LEN`, `MAX_FILENAME_LEN`, etc.).
*   Reject path traversal (`..` components).
*   Validate character sets per domain (username, hostname, env key…).

### 4. No Hardcoded Secrets

The following are **never** permitted in source code:

*   Passwords, passphrases
*   API keys or tokens
*   Cryptographic keys or seeds
*   Salt or IV values that are fixed/reused

Use `crate::security::secrets` for all secret management. Secrets are
loaded at runtime from secure storage, never compiled in.

### 5. Integer Safety

Use checked arithmetic everywhere an overflow is conceivable:

```rust
// ❌ May panic or wrap in debug/release respectively:
let total = count * size;

// ✅ Explicit overflow handling:
let total = crate::security::input_validation::safe_mul(count, size)
    .ok_or(SigmaError::Overflow)?;
```

Prefer: `checked_add`, `checked_mul`, `checked_sub`, `saturating_*`.

### 6. Memory Safety

*   Prefer safe Rust at all times.
*   No raw pointer arithmetic outside `klib` and `kernel/mm`.
*   Use `klib::vec::Vec<T>::get(i)` (returns `Option<&T>`) instead of
    direct `data[i]` indexing in kernel code.

***

## Security Features Enabled by Default

| Feature | Description | Module |
|---------|-------------|--------|
| ASLR | High-entropy address randomisation | `kernel::mm` |
| W^X | Memory pages are write OR execute, never both | `kernel::mm` |
| Stack canaries | Detect stack smashing | `kernel::mm` |
| MAC | Mandatory Access Control | `security::mac` |
| Audit | Security event logging | `security::audit` |

## Optional Hardening

| Feature | Module | Enable with |
|---------|--------|-------------|
| pledge() | `security::pledge_impl` | Call after setup |
| Capsicum capability mode | `security::capsicum` | `caps.enter()` |
| sigma\_unveil | `security::sigma_unveil` | `unveil_finalize()` |
| Qubes domain isolation | `security::qubes_isolation` | Config flag |
| Process sandbox | `security::sandbox` | Per-process |

***

## Automated Scanning

GitHub Code Scanning (CodeQL) and `cargo-audit` run automatically on
every push to `main` and every pull request (see
`.github/workflows/codeql.yml` and `.github/workflows/security-audit.yml`).

All **critical** and **high** severity alerts **must** be resolved before
a PR may be merged.

***

## PR Security Checklist

*   \[ ] No hardcoded secrets.
*   \[ ] No `.unwrap()` without `// SAFETY:` comment.
*   \[ ] All `unsafe` blocks have `// SAFETY:` comments.
*   \[ ] `input_validation` used for all user-supplied data.
*   \[ ] Checked arithmetic for all size/index calculations.
*   \[ ] No path traversal possible.
*   \[ ] Security tests added where applicable.
*   \[ ] `CHANGELOG.md` updated.
