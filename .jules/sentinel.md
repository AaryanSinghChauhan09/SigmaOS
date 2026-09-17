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
