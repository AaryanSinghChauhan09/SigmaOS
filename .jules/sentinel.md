## 2026-07-16 - PAM Authentication and Crypto Utilities Cleanup
**Vulnerability:** Duplicate declarations and broken placeholder logic in `pam.rs` and `crypto_utils.rs` prevented PAM module compilation and caused parameter mismatch bugs (`password` vs `user_token`).
**Learning:** `src/security/pam.rs` was not previously exposed in `src/security/mod.rs`, masking compilation errors from `cargo check` until explicitly added.
**Prevention:** Always verify `mod.rs` re-exports for newly added or refactored security subsystems and test standalone modules using a custom module test harness.
