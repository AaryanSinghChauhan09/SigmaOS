## 2026-09-06 - IPv6 Compressed Over-Length Address Parser Vulnerability
**Vulnerability:** Textual IPv6 input validation permitted compressed addresses with a double colon (`::`) containing 8 or more explicit hex blocks (e.g. `1:2:3:4:5:6:7::8`), causing parser differential vulnerabilities when expanding compressed IP address structures.
**Learning:** Checking only colon count (`colons <= 7`) is insufficient for compressed IPv6 validation because `::` consumes 1 colon while expanding to fill missing blocks. If 8 explicit hex blocks are present alongside `::`, address expansion results in 9 or more blocks, violating IPv6 128-bit structure boundaries.
**Prevention:** In IPv6 address parsers, track explicit non-empty block counts (`blocks`) alongside `double_colon` flags and reject compressed addresses whenever `double_colon && blocks >= 8`.

## 2025-05-18 - IPv4 Octal Parser Differential SSRF Vulnerability
**Vulnerability:** IPv4 input validation allowed multi-digit octets with leading zeros (e.g., `010.0.0.1` or `192.168.01.1`), which can lead to octal/decimal parser differential and SSRF security bypass attacks.
**Learning:** Legacy C network routines (`inet_aton`) interpret leading zero octets as octal numbers (e.g. `010` = 8), while decimal-only string matchers parse them as decimal `10`. This discrepancy allows attackers to bypass IP blocklists and WAF filters.
**Prevention:** In input validation routines for IPv4 addresses, explicitly detect and reject multi-digit octets starting with `0` (`octet_len > 1 && octet_has_leading_zero`) to enforce strict, unambiguous decimal IPv4 format.
