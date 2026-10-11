# SigmaOS Security Testing & Verification Plan

This document outlines automated security testing procedures, fuzzing cadences, SAST static analysis, and vulnerability response workflows.

---

## Security Verification Cadence

1. **Continuous Integration (CI)**:
   - Dependency vulnerability audit via `cargo audit`.
   - Unsafe code pattern audit (verifying `// SAFETY:` justifications).
   - Hardcoded secret detection.
   - GitHub Actions workflow SHA-pinning check.
2. **Weekly Fuzzing**:
   - `cargo-fuzz` runs against syscall parsers, VFS path normalizers, ELF loader, package manifests, and network packet handlers.
3. **Monthly Security Triage**:
   - Triage fuzzing findings, CVE bulletins, and dependency alerts.

---

## Fuzz Target Inventory

- `fuzz/fuzz_targets/ipc_parser.rs`: Fuzzes IPC magic parsing and capability token decoding.
- `fuzz/fuzz_targets/vfs_path_parser.rs`: Fuzzes VFS path normalization and path traversal safety.
- `fuzz/fuzz_targets/package_manifest.rs`: Fuzzes `sigpkg` package header and dependency resolution.

---

## Reporting Security Vulnerabilities

Please report security issues to `security@sigmaos.org` using standard PGP encrypted mail. Do not open public issues for zero-day security vulnerabilities before a patch is prepared.
