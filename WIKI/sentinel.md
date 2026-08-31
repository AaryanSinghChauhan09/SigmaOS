# 🛡️ Sentinel's Journal — SigmaOS Security & Hardening

This journal logs CRITICAL security lessons, vulnerability fixes, and proactive system hardening actions implemented across SigmaOS.

---

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
