# SigmaOS Security Test Plan

## Continuous & Recurring Security Verification

Security testing is divided into automated CI checks and recurring scheduled reviews:

### 1. Automated Continuous Checks
- **`cargo audit`**: Executed on PRs and weekly schedule to flag vulnerabilities in dependencies.
- **Unsafe Code Audit**: Analyzes every `unsafe` block in the codebase, requiring matching `// SAFETY:` invariants documentation.
- **Secret Scanning**: Scans for leaked keys, tokens, or plaintext credentials.
- **Action Pinning Audit**: Verifies all GitHub Actions are pinned to full immutable 40-character commit SHAs.

---

### 2. Scheduled Recurring Audits

| Cadence | Security Operation | Focus Areas | Output Artifact |
|---|---|---|---|
| Weekly | Security Advisory Triage | Review RustSec, CVE databases, Linux/BSD advisories | `security(...)` PR or issue |
| Monthly | Fuzzing & Regression Campaign | Syscall entry points, ELF parser, VFS path traversal, network headers | Crash reports & fix PRs |
| Quarterly | Privilege Boundary Review | Audit capability escalation points and IPC channel isolation | Security audit checklist |
| Semiannual | Supply Chain & SBOM Audit | Comprehensive software bill of materials and license validation | SBOM JSON artifact |
