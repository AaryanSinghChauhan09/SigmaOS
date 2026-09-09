## 2026-09-09 - Environment Variable Key Option Injection & POSIX Boundary Vulnerability
**Vulnerability:** `validate_env_key` permitted environment variable keys starting with a hyphen (e.g. `-LD_PRELOAD` or `--config`), leading to command-line option flag injection when environment variables are passed to subshell runners or `env` utilities, as well as permitting non-POSIX leading digits and special characters.
**Learning:** Blacklisting only `=` and NUL bytes is insufficient for environment variable key validation because leading hyphens act as CLI option flags when passed to `env` or process launchers, and POSIX IEEE Std 1003.1 strictly restricts variable names to `[a-zA-Z_][a-zA-Z0-9_]*`.
**Prevention:** In environment variable key validation, enforce that the first byte must be an ASCII letter or underscore (`[a-zA-Z_]`) and subsequent bytes must be ASCII alphanumeric or underscore (`[a-zA-Z0-9_]`).

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
