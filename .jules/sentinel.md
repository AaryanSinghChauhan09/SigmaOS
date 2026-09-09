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
