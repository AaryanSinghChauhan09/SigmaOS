## 2026-10-16 - Case-Insensitive URL Scheme & Backslash Parser Differential Security
**Vulnerability:** `validate_url` rejected uppercase schemes (e.g. `HTTP://` / `Https://`) while allowing backslashes (`\`) which WHATWG URL parsers normalize to slashes (`/`), leading to URL parser differential, open redirect, and SSRF vulnerabilities (CWE-601 / CWE-918).
**Learning:** Strict RFC 3986 scheme comparison requires ASCII case-insensitivity (`eq_ignore_ascii_case`). Furthermore, backslashes in URL paths allow host-path confusion when downstream components decode or parse URLs differently (WHATWG vs RFC 3986).
**Prevention:** Always parse scheme prefixes case-insensitively and explicitly reject backslash characters in web input validation to prevent parser differential attacks.

## 2026-10-17 - Mixed & Double URL-Encoded Path Traversal Bypass Security
**Vulnerability:** `validate_path` checked for double URL-encoded dot pairs (`%2e%2e`) and single URL-encoded separators (`%2f`, `%5c`), but missed mixed literal and URL-encoded dot sequences (`.%2e`, `%2e.`) and double-encoded entities (`%252e`, `%252f`, `%255c`), allowing path traversal bypasses (CWE-22).
**Learning:** Checking only uniform URL encoding sequences allows attacker bypasses using mixed encoding (`.%2e`) or double percent encoding (`%252e`).
**Prevention:** Always inspect byte slices for mixed URL-encoded representations and double-percent hex sequences (`%25..`) in input validation filters.

## 2026-10-18 - Percent-Encoded NUL and CRLF HTTP Injection Security
**Vulnerability:** `validate_url` checked literal NUL bytes and control characters (`b <= 32`), but missed percent-encoded NUL bytes (`%00`) and percent-encoded CRLF/control sequences (`%0d`, `%0a`, `%1b`, `%7f`), enabling HTTP request smuggling, response header injection, and C-ABI string truncation (CWE-113 / CWE-626 / CWE-918).
**Learning:** Checking only literal raw bytes leaves applications vulnerable when downstream network components decode percent-encoded control sequences or C-ABI string handlers truncate at `%00`.
**Prevention:** Always scan URL byte slices for percent-encoded NUL bytes (`%00`), CRLF sequences (`%0d`/`%0a`), and encoded userinfo credentials (`%40`) in input validation routines before URL processing or forwarding.

## 2026-10-19 - Percent-Encoded Percent Sign & Dot-Separator Path Traversal Security
**Vulnerability:** `validate_path` checked single percent-encoded separators (`%2f`, `%5c`) and double-encoded dot pairs (`%252e`), but missed double percent-encoded percent signs (`%2525`) and single-percent encoded dot-separator pairs (`%2e%2f`, `%2e%5c`), enabling multi-stage recursive percent-decoding and path traversal bypasses in web and proxy gateways (CWE-22).
**Learning:** Attackers can utilize `%2525` to pass encoded percent signs through multi-tier proxies that perform multiple decoding passes or pair single-encoded dots with encoded separators (`%2e%2f`).
**Prevention:** Always inspect byte slices for `%2525` percent-encoded percent signs and combined encoded dot-separator sequences (`%2e%2f`, `%2e%5c`) in path validation routines.
