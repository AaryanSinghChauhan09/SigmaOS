## 2026-10-16 - Case-Insensitive URL Scheme & Backslash Parser Differential Security
**Vulnerability:** `validate_url` rejected uppercase schemes (e.g. `HTTP://` / `Https://`) while allowing backslashes (`\`) which WHATWG URL parsers normalize to slashes (`/`), leading to URL parser differential, open redirect, and SSRF vulnerabilities (CWE-601 / CWE-918).
**Learning:** Strict RFC 3986 scheme comparison requires ASCII case-insensitivity (`eq_ignore_ascii_case`). Furthermore, backslashes in URL paths allow host-path confusion when downstream components decode or parse URLs differently (WHATWG vs RFC 3986).
**Prevention:** Always parse scheme prefixes case-insensitively and explicitly reject backslash characters in web input validation to prevent parser differential attacks.
