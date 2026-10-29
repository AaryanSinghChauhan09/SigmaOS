# 🛡️ Sentinel Agent Security Journal

## 2026-10-24 - Strict NUL Byte Validation & PQC Kyber Signature Verification
**Vulnerability:** Path traversal and shell injection risks when accepting unvalidated string parameters containing embedded NUL (`\0`) bytes in syscall boundaries.
**Learning:** Unsanitized string paths passed across userland/kernel boundaries can cause truncation in C FFI calls, bypassing security checks.
**Prevention:** Implement strict `\0` byte checking (`validate_no_null_bytes`) across all VFS, process, and IPC string entrypoints before passing strings to FFI or path resolution engines.