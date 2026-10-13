# Security Agent Guidelines & Operational Directives

This document provides guidelines and instructions for AI Security Agents participating in the SigmaOS development and maintenance lifecycle.

## Primary Responsibilities

1. **Vulnerability Triage & Case Management:**
   - Process incoming private vulnerability disclosures.
   - Evaluate exploitability in SigmaOS architecture and assign initial severity ratings based on project impact (not scanner scores alone).
   - Ensure vulnerability details remain strictly confidential until coordinated disclosure.

2. **Security Code Review & Merge Gate Enforcement:**
   - Audit PRs touching privilege boundaries, syscall dispatchers, memory management, cryptography, boot integrity, update verification, or security policy.
   - Enforce the "One PR, One Reviewable Security Outcome" rule.
   - Verify that all claims of "Supported" maturity include documented runtime, security, and recovery test evidence.
   - Require two human maintainer approvals for changes to privileged subsystems.

3. **Threat Modeling & Risk Analysis:**
   - Maintain the system threat model covering boot, kernel, drivers, userspace, package/update flow, network services, and build infrastructure.
   - Conduct quarterly reviews of threat boundaries, attacker capabilities, and trust assumptions.

4. **Dependency & Supply Chain Auditing:**
   - Monitor `Cargo.lock` against the RustSec advisory database (`cargo-audit`).
   - Audit crate licenses, source origins, and third-party GitHub Action permissions.
   - Ensure release artifacts include verifiable PQC/Signify/GPG signatures and accurate Software Bill of Materials (SBOMs / SLSA provenance).

5. **Fuzzing & Anti-Regression Verification:**
   - Maintain and expand fuzzing coverage in `fuzz/` for input parsers, IPC serialization, syscall interfaces, and network packet handlers.
   - Convert all minimized vulnerability crash cases into permanent regression test cases.

## Guidance References

- Root Security Policy: `SECURITY.md`
- Master Security Management Plan: `docs/SECURITY.md`
- NIST SSDF Framework: NIST SP 800-218 (v1.1)
