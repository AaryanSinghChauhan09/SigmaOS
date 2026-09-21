## 2026-09-21 - Multi-Dot Segment Path Traversal Bypass in Path Validation
**Vulnerability:** `validate_path` in `src/security/input_validation.rs` checked for `..` path traversal sequences using local 2-character lookaheads around directory separators (`path[i - 1] == '/'`). When an attacker supplied multi-dot segments like `...` or `....` (e.g. `/.../etc/passwd`), the lookahead condition `path[i - 1] == '/'` evaluated to false because `path[i - 1]` was a dot (`.`), allowing multi-dot path traversal attempts to bypass path validation rules.
**Learning:** Checking for `..` path traversal with fixed character lookups relative to delimiters fails on multi-dot variations (`...`, `....`), which can cause parser differentials or VFS resolution anomalies depending on how path normalization routines collapse dot segments.
**Prevention:** In filesystem path validation routines, inspect every path segment bounded by directory separators (`/`, `\`, `:` or path boundaries). Reject any segment consisting solely of dots with a length of 2 or more (`seg_len >= 2 && !segment_has_non_dot`).

## 2026-09-20 - Fixed-Buffer Truncation Sandboxing Bypass in Unveil Access Validation
**Vulnerability:** `PledgeManager::validate_unveil_access` in `src/security/pledge.rs` checked for URL-encoded traversal patterns (`%2e%2e`, `%2f`, `%5c`) by copying the path into a fixed 512-byte stack buffer (`let mut buf = [0u8; 512]`). Any path exceeding 512 bytes was truncated during the copy, allowing attackers to place URL-encoded traversal sequences past byte index 512 and bypass unveil sandboxing controls.
**Learning:** Checking security constraints by copying inputs into fixed-size stack buffers introduces truncation vulnerabilities where malicious payloads placed beyond the buffer length evade validation checks.
**Prevention:** Perform security constraint checks directly over borrowing slices without allocation or fixed-length copying. In URL-encoded pattern validation, inspect raw byte slices across the full input length.

## 2026-09-19 - Path Prefix Confusion Sandboxing Bypass in Unveil Sandboxing Engines
**Vulnerability:** `check_unveil` in `src/security/landlock_sovereign.rs`, `UnveilEntry::covers` in `src/security/sigma_unveil.rs`, and `check_unveil` in `src/distro/linux_bsd_ultimate_synthesis.rs` used naive `path.starts_with(rule_path)` prefix matching without verifying directory component boundaries, allowing sandboxed processes to access unauthorized sibling directories (e.g., unveiling `/etc` permitted access to `/etc_secret` or `/etc_shadow`).
**Learning:** Checking path prefixes with `starts_with` without validating trailing path separators (`/` or `\`) or exact equality allows path prefix confusion bypasses where an attacker appends characters to a permitted prefix to access restricted sibling paths.
**Prevention:** In filesystem sandboxing and unveil access rules, enforce strict path boundary checks: match if `path == rule_path`, `rule_path == "/"`, or `path.starts_with(rule_path)` where `rule_path` ends with a separator or `path` contains a path separator at `rule_path.len()`.

## 2026-09-17 - ASCII Control Character Injection in Filename and Path Validation
**Vulnerability:** `validate_filename` and `validate_path` permitted ASCII control characters (`b < 32 || b == 127`, such as `\n`, `\r`, `\t`, ESC `\x1b`, DEL `\x7f`), leading to log forgery/injection (CWE-117), ANSI escape sequence terminal hijacking (CWE-150), and shell script argument/line splitting when filenames or paths are logged, printed, or processed by utilities.
**Learning:** Checking only for directory separators (`/`, `\`) or NUL bytes (`\0`) is insufficient for filename and path validation because non-printable control characters and newline characters split log streams, trigger terminal commands, and break script parsers.
**Prevention:** In filename and path validation routines, explicitly check for and reject all ASCII control characters (`b < 32 || b == 127`), and map NUL bytes in `validate_filename` to `ValidationError::NullByte`.

## 2026-09-10 - Loose Environment Variable Key Validation Vulnerability
**Vulnerability:** `validate_env_key` allowed environment variable names starting with digits, hyphens, or containing special characters (e.g., `-LD_PRELOAD`, `123_ENV`, `FOO-BAR`), leading to command-line argument injection and environment variable parser differential vulnerabilities when passed to subprocesses or shell helpers.
**Learning:** Checking only for `=` and NUL bytes (`b == 0 || b == b'='`) is insufficient for environment key validation because POSIX / IEEE Std 1003.1 restricts variable names to `[a-zA-Z_][a-zA-Z0-9_]*`. Non-conforming keys can cause shell execution anomalies or option parsing errors in system utilities.
**Prevention:** Strictly enforce POSIX environment key rules: ensure non-empty keys have an initial byte in `[a-zA-Z_]` and subsequent bytes in `[a-zA-Z0-9_]`.

## 2026-09-08 - Hostname Option Injection and Label Boundary Security Vulnerability
**Vulnerability:** `validate_hostname` permitted hostnames starting with a hyphen (e.g. `-oProxyCommand=...` or `-rf`), leading to command-line option injection when hostnames are passed to network or shell utilities, as well as permitting malformed labels (empty labels `..` or label lengths >63).
**Learning:** Checking only character set membership (`is_ascii_alphanumeric() || b == '-' || b == '.'`) is insufficient for hostname validation because hyphens at the start of labels act as option flags in CLI tool invocations, and RFC 952/1123 imposes strict per-label length (1..=63 octets) and formatting rules.
**Prevention:** In hostname validation, split hostnames into dot-separated labels, reject empty labels, enforce `label.len() <= 63`, and explicitly disallow leading and trailing hyphens on each label (`label[0] != '-' && label[label.len() - 1] != '-'`).

## 2026-09-06 - IPv6 Compressed Over-Length Address Parser Vulnerability
**Vulnerability:** Textual IPv6 input validation permitted compressed addresses with a double colon (`::`) containing 8 or more explicit hex blocks (e.g. `1:2:3:4:5:6:7::8`), causing parser differential vulnerabilities when expanding compressed IP address structures.
**Learning:** Checking only colon count (`colons <= 7`) is insufficient for compressed IPv6 validation because `::` consumes 1 colon while expanding to fill missing blocks. If 8 explicit hex blocks are present alongside `::`, address expansion results in 9 or more blocks, violating IPv6 128-bit structure boundaries.
**Prevention:** In IPv6 address parsers, track explicit non-empty block counts (`blocks`) alongside `double_colon` flags and reject compressed addresses whenever `double_colon && blocks >= 8`.

## 2025-05-18 - IPv4 Octal Parser Differential SSRF Vulnerability
**Vulnerability:** IPv4 input validation allowed multi-digit octets with leading zeros (e.g., `010.0.0.1` or `192.168.01.1`), which can lead to octal/decimal parser differential and SSRF security bypass attacks.
**Learning:** Legacy C network routines (`inet_aton`) interpret leading zero octets as octal numbers (e.g. `010` = 8), while decimal-only string matchers parse them as decimal `10`. This discrepancy allows attackers to bypass IP blocklists and WAF filters.
**Prevention:** In input validation routines for IPv4 addresses, explicitly detect and reject multi-digit octets starting with `0` (`octet_len > 1 && octet_has_leading_zero`) to enforce strict, unambiguous decimal IPv4 format.
