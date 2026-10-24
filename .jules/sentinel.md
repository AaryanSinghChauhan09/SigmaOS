## 2025-05-15 - Input Validation NUL-Byte Consistency
**Vulnerability:** Inconsistent NUL-byte handling in input validation routines (CWE-626) where some routines returned `ValidationError::InvalidChars` instead of `ValidationError::NullByte`.
**Learning:** Checking for `b == 0` early in all byte-slice input validation functions guarantees consistent security error classification across usernames, hostnames, environment keys, and IP addresses.
**Prevention:** Always perform an explicit `b == 0` byte slice check first in all `src/security/input_validation.rs` functions.
