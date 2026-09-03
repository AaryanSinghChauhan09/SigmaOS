# 🛡️ Sentinel's Journal — SigmaOS Security & Hardening

This journal logs CRITICAL security lessons, vulnerability fixes, and proactive system hardening actions implemented across SigmaOS.

***

## 2025-05-18 - IPv4 Octal Parser Differential SSRF Vulnerability

**Vulnerability:** IPv4 input validation allowed multi-digit octets with leading zeros (e.g., `010.0.0.1` or `192.168.01.1`), which can lead to octal/decimal parser differential and SSRF security bypass attacks.
**Learning:** Legacy C network routines (`inet_aton`) interpret leading zero octets as octal numbers (e.g. `010` = 8), while decimal-only string matchers parse them as decimal `10`. This discrepancy allows attackers to bypass IP blocklists and WAF filters.
**Prevention:** In input validation routines for IPv4 addresses, explicitly detect and reject multi-digit octets starting with `0` (`octet_len > 1 && octet_has_leading_zero`) to enforce strict, unambiguous decimal IPv4 format.

## 2024-07-15 - Strict Field Privacy in Security Capability Tokens

**Learning:** Permitting modules to access raw bitmasks (e.g. `self.capabilities.bits`) directly bypasses the capability enforcement gate, creating risks where internal bits can be mutated or read in an unauthorized manner. Enforcing strict field privacy on the internal bitmask of `CapabilityToken` and requiring all drivers to use public getter methods (such as `bits()`) prevents unauthorized bitwise manipulation and preserves the capability delegation contract.
**Action:** Keep core cryptographic and security privilege fields private at all times, exposing them only via read-only getters or explicit, capability-gated validation methods.

## 2024-07-15 - Uncontrolled Error Propagation in Package Managers

**Learning:** Allowing low-level package resolution errors (such as `ResolveError`) to bubble up directly to transaction commit layers using automatic question-mark conversions without wrapping or sanitizing can leak system paths and dependency graph configurations. Wrapping resolution failures into a high-level `TransactionError::DependencyConflict` sanitizes error outputs, prevents system layout leakages, and keeps error diagnostics safe.
**Action:** Proactively sanitize and map internal package/scheduler errors before propagating them to user-space applications to block potential operating system reconnaissance channels.

## 2024-07-16 - Directory Traversal via Unsanitized Sandbox Paths

**Vulnerability:** Path-gated capability authorizations allowed directory traversal sequences like `..` to bypass root boundaries (e.g. `/var/www/../../etc/passwd`), granting raw system files access.
**Learning:** Checking path prefixes with `starts_with` alone is insufficient when dot-dot traversal can resolve paths out of scope. Paths must be canonicalized or sanitized to ensure they do not contain relative components like `..`.
**Prevention:** Reject paths containing directory traversal segments (`../`, `/..`, or starting/ending relative boundaries) before evaluating security rule prefixes.

## 2024-07-16 - Bitmask Overlap Privilege Escalation

**Vulnerability:** Successive `allow_network` port registrations with logical OR operations corrupted bits 16-31, causing unintended port allocations and privilege escalation (e.g. port 80 and 443 producing unauthorized port 507).
**Learning:** Bitwise OR operations on non-disjoint bit fields pollute boundaries, leaking permissions across fields.
**Prevention:** Always mask and clear target bit ranges (e.g. `self.bits &= !(0xFFFF << 16)`) before writing new values to bit-packed integers.

## 2026-07-20 - Unclosed Import Delimiters and Committed Merge Conflicts

**Vulnerability:** Having merge conflict markers committed directly to repository branches leads to immediate parser/compiler termination, acting as an unintentional Denial-of-Service (DoS) on CI pipelines and developer builds.
**Learning:** Delimiter validation checks must be enforced strictly prior to commit stages to prevent broken master/main trunks.
**Prevention:** Integrate pre-commit or pre-push gates that search for conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`) to ensure only fully resolved files enter upstream integration.

## 2025-10-24 - Attested Cryptographic Audits

**Learning:** Normal filesystem logging is susceptible to modifications when a system is compromised.
**Prevention:** Utilizing WORM (write-once-read-many) structures ensures that cryptographic audit logs cannot be modified once they are committed.

## 2026-07-29 - Unresolved Source Conflict Markers as CI Denial-of-Service Vectors

**Vulnerability:** Permitting unmerged git conflicts to be committed to production branches (such as `main`) results in immediate parser/compiler termination, acting as an unintended Denial-of-Service (DoS) on continuous-integration security validation pipelines.
**Learning:** Any committed parser markers stop compiler diagnostics from performing security/CVE audits. Standard static-analysis checks must run a raw conflict scan prior to pull-request merges to protect integration stability.
**Prevention:** Deploy pre-commit hooks that explicitly scan for the exact conflict sequences (`<<<<<<<`, `=======`, `>>>>>>>`) across all source code paths.

## 2026-08-09 - Sanitizing Dynamic Dependency Trees

**Learning:** Unverified third-party libraries downloaded during build stages can introduce hidden supply chain vulnerabilities. Outdated sub-dependencies like `brace-expansion` and `nanoid` must have priority upgrades pinned at the package level to eliminate Regular Expression Denial of Service (ReDoS) and loop hazards.
**Action:** Always scan for nested lockfile overrides and apply semantic versions upgrades strictly.

## 2026-08-10 - Multi-Stage PAM Authentication and BSD Securelevels

**Learning:** Single-factor authentication or static permission checks leave systems vulnerable to brute-force attacks and root level modifications. Implementing Linux-style Pluggable Authentication Modules (PAM) with account lockouts (`pam_tally2`) and pwquality password rules alongside BSD monotonically non-decreasing Securelevels creates a hardened defense-in-depth framework.
**Action:** Enforce multi-stage PAM authentication for user access and gate critical network/storage operations behind BSD Securelevels and capability bitmasks.

## 2026-08-20 - CRLF Sanitization in Structured Log Attributes

**Vulnerability:** Permitting unescaped carriage returns (`\r`) or line feeds (`\n`) in key-value attributes passed to structured syslog entries allows malicious log entries to split log frames and inject spoofed syslog headers or fake log entries.
**Learning:** Unsanitized newlines in log fields break message framing in line-oriented log sinks like rsyslog and systemd-journald.
**Prevention:** Explicitly strip or escape CRLF characters (`\r`, `\n`) from dynamic key and value attributes before adding them to log structures.

## 2026-08-23 - Multi-Layer Packet Inspection with Post-Quantum Signatures and Hash Verification

**Learning:** Combining sliding-window rate limiting, asymmetric post-quantum public key signature checking (Dilithium-5), zero-trust subnet filtering, and deep packet session hash matching prevents spoofing, replay attacks, and denial-of-service vectors at the network layer.
**Action:** Enforce strict 4-stage validation (rate limit -> PQC signature -> subnet check -> payload hash) on all zero-trust network router interfaces.

## 2026-08-29 - Path Traversal Component Separator Enforcement

**Vulnerability:** Checking path traversal sequences (`..`) using only `/` and `\` directory separators allowed bypasses on non-standard URI schemes and Windows drive relative paths (e.g. `C:..\passwd` or `file:../secret.txt`).
**Learning:** Path traversal detection logic must include scheme and drive specifiers (e.g., `:`) alongside directory delimiters (`/` and `\`) when determining path component boundaries.
**Prevention:** Treat colons (`:`) as valid boundary delimiters when evaluating relative dot-dot traversal sequences in path sanitization routines.
