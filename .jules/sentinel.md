## 2026-07-16 - Environment Variable Sanitization in Graphical Elevation Engines
**Vulnerability:** `GksuSecurityGuard::sanitize_environment` previously checked a hardcoded subset of dangerous variables and relied on `allowed_keys.contains(k)`, allowing bypasses via dynamic linker, interpreter, shell, or system variables, or embedded NUL/control characters.
**Prevention:** Enforce a central strict filter in privilege escalation routines regardless of caller allowed lists.

## 2026-07-16 - PAM Authentication and Crypto Utilities Cleanup
**Vulnerability:** Duplicate declarations and broken placeholder logic in `pam.rs` and `crypto_utils.rs` prevented PAM module compilation and caused parameter mismatch bugs.
**Prevention:** Verify `mod.rs` re-exports for new/refactored security subsystems and test standalone modules.
