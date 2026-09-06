## 2026-09-06 - IPv6 Compressed Address Validation Bypass Vulnerability
**Vulnerability:** IPv6 validation permitted compressed addresses (`::`) containing 8 or more explicit hexadecimal blocks (e.g. `2001:db8::1:2:3:4:5:6`), which allows invalid IPv6 syntax to pass validation and cause parser differentials or buffer bounds violations downstream.
**Learning:** `::` replaces one or more zero blocks, requiring total blocks to remain under 8. Checking `colons <= 7` alone is insufficient when `::` is present because `::` consumes only 1 colon while expansion adds missing zero blocks.
**Prevention:** In IPv6 string validation routines, track explicit block count (`blocks`) and enforce `if double_colon && blocks >= 8 { return Err(...); }` to guarantee compressed addresses expand to at most 8 16-bit fields.

## 2025-05-18 - IPv4 Octal Parser Differential SSRF Vulnerability
**Vulnerability:** IPv4 input validation allowed multi-digit octets with leading zeros (e.g., `010.0.0.1` or `192.168.01.1`), which can lead to octal/decimal parser differential and SSRF security bypass attacks.
**Learning:** Legacy C network routines (`inet_aton`) interpret leading zero octets as octal numbers (e.g. `010` = 8), while decimal-only string matchers parse them as decimal `10`. This discrepancy allows attackers to bypass IP blocklists and WAF filters.
**Prevention:** In input validation routines for IPv4 addresses, explicitly detect and reject multi-digit octets starting with `0` (`octet_len > 1 && octet_has_leading_zero`) to enforce strict, unambiguous decimal IPv4 format.
