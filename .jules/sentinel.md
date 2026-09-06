## 2025-05-18 - IPv4 Octal Parser Differential SSRF Vulnerability
**Vulnerability:** IPv4 input validation allowed multi-digit octets with leading zeros (e.g., `010.0.0.1` or `192.168.01.1`), which can lead to octal/decimal parser differential and SSRF security bypass attacks.
**Learning:** Legacy C network routines (`inet_aton`) interpret leading zero octets as octal numbers (e.g. `010` = 8), while decimal-only string matchers parse them as decimal `10`. This discrepancy allows attackers to bypass IP blocklists and WAF filters.
**Prevention:** In input validation routines for IPv4 addresses, explicitly detect and reject multi-digit octets starting with `0` (`octet_len > 1 && octet_has_leading_zero`) to enforce strict, unambiguous decimal IPv4 format.

---

## AI Agent Security Audit Checklist (Sentinel 🛡️)

When conducting automated security audits or introducing code modifications in SigmaOS, AI agents MUST perform the following checklist verifications:

1. **Syscall & IPC Boundary Audit**:
   - Verify that userland syscall dispatchers in `src/syscall/` enforce capability checks (`src/security/capability.rs`).
   - Validate OpenBSD `pledge` and `unveil` sandboxing enforcement in all shell/CLI tools.

2. **Hardcoded Secret & Credential Scanning**:
   - Audit variable assignments matching `password`, `secret`, `key`, or `token` in `.rs` source files.
   - Ensure all mock/test variables explicitly include `mock`, `test`, `example`, or `TODO`.

3. **Memory Safety & Scrubbing Verification**:
   - Ensure zero raw pointers or unsafe blocks in `#![no_std]` core components without explicit bounds checks.
   - Verify amnesic memory scrubbing logic on deallocation for cryptographic key buffers.

4. **Workflow & Dependency Security**:
   - Verify that all GitHub Actions workflow actions specify fixed tags or commit SHAs and explicitly set toolchains (`with: toolchain: stable`).
