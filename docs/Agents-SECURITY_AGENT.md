# Security Agent Guidelines for SigmaOS

This document defines instructions and guidelines for security review, triage, and auditing in SigmaOS.

## Security Engineering Principles
1. **SSDF Alignment:** Follow NIST SP 800-218 across all development stages.
2. **Evidence-Based Maturity:** Maturity levels (`Proposed`, `Prototype`, `Integrated`, `Supported`) reflect verified runtime evidence, not source presence alone.
3. **Privilege Boundary Inspections:** Changes affecting kernel boundaries, cryptography, boot, updates, or sandboxing require two maintainer approvals.
4. **Vulnerability Disclosure:** Keep details of unpatched vulnerabilities private until coordinated disclosure.

## Security Merge Gate Checklist
Agents reviewing security PRs must verify:
- Complete PR template with all 9 standard sections.
- Test evidence covering negative cases (invalid bounds, unauthorized access).
- No secrets or private release credentials in source/logs.
- Maturity status correctly mapped in `FEATURE_STATUS.toml`.
