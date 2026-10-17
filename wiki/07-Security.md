# 07 Security and Hardening

SigmaOS Security Management Plan aligned with NIST SP 800-218 (Secure Software Development Framework - SSDF) and mature Linux/BSD distribution security practices.

## Overview

Component maturity across SigmaOS is tracked as **Proposed, Prototype, Integrated, or Supported**.
A unit-tested module alone does not constitute proof of a complete, secure system path.

For the complete policy, baseline rules, 16-PR roadmap, and vulnerability disclosure workflow, refer to:
- [`docs/SECURITY.md`](../docs/SECURITY.md)
- [`SECURITY.md`](../SECURITY.md)
- [Security and Hardening Guide](./Security-and-Hardening.md)

## Baseline Rules

1. **Security status requires evidence:** Supported maturity requires documented runtime, security, and recovery test results on target hardware or hypervisors.
2. **One PR, one reviewable outcome:** Keep security PRs narrowly scoped; separate refactorings from remediation.
3. **Private vulnerability reporting:** Maintain confidential intake channels (`security@sigmaos.org` / GitHub Private Vulnerability Reporting).
4. **Prioritize risk in product context:** Triage exploitability in SigmaOS architecture rather than scanner scores alone.
5. **Human review required:** Privileged boundaries, cryptography, sandboxing, and boot trust require review from two maintainers.
6. **No claim without release evidence:** Release builds must publish commit SHA, build artifacts, advisories, package checksums, and SBOMs.
