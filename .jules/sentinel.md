## 2026-10-16 - Case-Insensitive URL Scheme & Backslash Parser Differential Security
**Vulnerability:** `validate_url` rejected uppercase schemes (e.g. `HTTP://` / `Https://`) while allowing backslashes (`\`) which WHATWG URL parsers normalize to slashes (`/`), leading to URL parser differential, open redirect, and SSRF vulnerabilities (CWE-601 / CWE-918).
**Learning:** Strict RFC 3986 scheme comparison requires ASCII case-insensitivity (`eq_ignore_ascii_case`). Furthermore, backslashes in URL paths allow host-path confusion when downstream components decode or parse URLs differently (WHATWG vs RFC 3986).
**Prevention:** Always parse scheme prefixes case-insensitively and explicitly reject backslash characters in web input validation to prevent parser differential attacks.

## 2026-10-17 - MS-DOS Reserved Device Name Bypass in Filename Input Validation
**Vulnerability:** `validate_filename` allowed MS-DOS reserved device names (`CON`, `PRN`, `AUX`, `NUL`, `COM1`..`COM9`, `LPT1`..`LPT9`, `CLOCK$`) with or without file extensions (e.g., `CON`, `aux.json`, `COM1.txt`), leading to special device file access, file system hijacking, denial-of-service, and arbitrary device I/O vulnerabilities (CWE-667 / CWE-706).
**Learning:** Windows and compatibility subsystems map filenames whose stems (up to the first dot) match reserved device names directly to physical or virtual hardware devices regardless of extension or directory structure.
**Prevention:** Always extract the filename stem and perform case-insensitive comparison against reserved device names (`eq_ignore_ascii_case`) in filename validation functions.
