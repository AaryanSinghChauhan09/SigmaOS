# SigmaOS Security Test Plan

This document defines the security audit routines, static analysis passes, fuzzing schedules, and regression testing policies for SigmaOS.

## Security Test Taxonomy

1. **Static Security Analysis**: Executed on every pull request via `.github/workflows/security.yml` and `cargo clippy`.
2. **Capability Token Verification**: Standalone test suite in `tests/security/capability_test.rs` validating capability revocation and handle bounds.
3. **Fuzzing Sweeps**: Monthly fuzzing runs targeting VFS path parsers (`fuzz/fuzz_targets/vfs_path_parser.rs`) and shell lexers (`fuzz/fuzz_targets/shell_lexer_parser.rs`).
4. **Crypto Signature Verification**: Standalone PQC signature verifier tests covering Dilithium-5 and FALCON signature validation routines.

---

## Automated Security Command Reference

```bash
# Execute capability security test suite
cargo test --test capability_test

# Execute SAST and dependency vulnerability audits
cargo audit
```
