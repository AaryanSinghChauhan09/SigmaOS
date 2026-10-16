# Sentinel Agent Journal 🛡️

## 2025-05-18 - OpenBSD Pledge/Unveil Isolation & Hardened Memory Protections
**Learning:** Combining OpenBSD pledge/unveil syscall restrictors with seL4 memory protection boundaries in userland services prevents arbitrary privilege escalation and protects against zero-day exploit chains in package management UDF execution environments.
**Action:** Enforce strict pledge sandbox capabilities (`stdio rpath wpath cpath inet`) on all distro package transpilers and scriptlet engines.

## 2025-05-18 - Privilege Escalation Group Parameter Sanitization
**Vulnerability:** Group membership slices (`groups: &[&str]`) in privilege authorization (`SovereignSudoEngine::authorize`) lacked validation for embedded NUL bytes and ASCII control characters, creating risks of C-ABI string truncation and CWE-150 / CWE-117 log/command injection.
**Learning:** All string slices passed into privilege escalation and authentication routines—including indirect parameters like group membership lists—must be checked for NUL bytes (`\0`) and control characters (`b < 32 || b == 127`).
**Prevention:** Enforce byte-slice iteration checks (`g.as_bytes().contains(&0) || g.bytes().any(|b| b < 32 || b == 127)`) on all slice collections across security enforcement interfaces.

## 2026-03-29 - URL Parser Differential and Scheme Validation
**Vulnerability:** URL input validation routines strictly matching lowercase schemes (`http://`) failed valid RFC 3986 uppercase URLs (`HTTP://`) and permitted backslashes (`\`), exposing downstream components to WHATWG vs RFC 3986 URL parser differentials, Open Redirect (CWE-601), and SSRF (CWE-918).
**Learning:** Scheme checks must be ASCII case-insensitive, and URL validation must explicitly reject backslashes (`\`) to prevent host-path confusion across browser and backend parsers.
**Prevention:** Enforce `eq_ignore_ascii_case` on URL schemes and reject `b'\\'` across all network/URL validation routines.
