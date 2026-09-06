## 2025-05-18 - IPv4 Octal Parser Differential SSRF Vulnerability
**Vulnerability:** IPv4 input validation allowed multi-digit octets with leading zeros (e.g., `010.0.0.1` or `192.168.01.1`), which can lead to octal/decimal parser differential and SSRF security bypass attacks.
**Learning:** Legacy C network routines (`inet_aton`) interpret leading zero octets as octal numbers (e.g. `010` = 8), while decimal-only string matchers parse them as decimal `10`. This discrepancy allows attackers to bypass IP blocklists and WAF filters.
**Prevention:** In input validation routines for IPv4 addresses, explicitly detect and reject multi-digit octets starting with `0` (`octet_len > 1 && octet_has_leading_zero`) to enforce strict, unambiguous decimal IPv4 format.
