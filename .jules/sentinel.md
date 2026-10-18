## 2026-10-16 - Case-Insensitive URL Scheme & Backslash Parser Differential Security
**Vulnerability:** `validate_url` rejected uppercase schemes (e.g. `HTTP://` / `Https://`) while allowing backslashes (`\`) which WHATWG URL parsers normalize to slashes (`/`), leading to URL parser differential, open redirect, and SSRF vulnerabilities (CWE-601 / CWE-918).
**Learning:** Strict RFC 3986 scheme comparison requires ASCII case-insensitivity (`eq_ignore_ascii_case`). Furthermore, backslashes in URL paths allow host-path confusion when downstream components decode or parse URLs differently (WHATWG vs RFC 3986).
**Prevention:** Always parse scheme prefixes case-insensitively and explicitly reject backslash characters in web input validation to prevent parser differential attacks.

## 2026-10-17 - Mixed & Double URL-Encoded Path Traversal Bypass Security
**Vulnerability:** `validate_path` checked for double URL-encoded dot pairs (`%2e%2e`) and single URL-encoded separators (`%2f`, `%5c`), but missed mixed literal and URL-encoded dot sequences (`.%2e`, `%2e.`) and double-encoded entities (`%252e`, `%252f`, `%255c`), allowing path traversal bypasses (CWE-22).
**Learning:** Checking only uniform URL encoding sequences allows attacker bypasses using mixed encoding (`.%2e`) or double percent encoding (`%252e`).
**Prevention:** Always inspect byte slices for mixed URL-encoded representations and double-percent hex sequences (`%25..`) in input validation filters.
