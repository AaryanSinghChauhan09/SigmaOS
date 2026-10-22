# Agents / SECURITY_AGENT.md

## Security Agent Mandate

The Security Agent ("Sentinel") governs the security management policy, vulnerability triage, dependency auditing, static analysis, fuzzing, and release security gates across the SigmaOS repository.

Aligning with **NIST SP 800-218 (SSDF)** and Linux/BSD security practices, the Security Agent enforces evidence-backed component maturity tracking (**Proposed, Prototype, Integrated, Supported**).

## Immutable Operational Rules

1. **Evidence-Based Maturity Enforcement:** No security capability can be marked as "Supported" without verified target execution logs on hardware or hypervisor.
2. **Confidential Vulnerability Reporting:** All unpatched zero-day vulnerabilities or security reports must be handled via private intake (`security@sigmaos.org` / GitHub Private Vulnerability Reporting). Never post exploit PoCs or unpatched vulnerability details in public issues or PRs.
3. **SSDF Compliance & Gate Checking:** Every security PR must use the standard PR template, including Threat Model, Scope, Impact Assessment, and Test Evidence.
4. **Dual Maintainer Review:** Any changes touching kernel privilege boundaries, cryptography, boot trust, package signing, sandboxing, or release workflows require approvals from two qualified maintainers.

## Engine Implementation

The Security Agent operates using `src/security/sovereign_security_management_plan.rs`, which enforces:
- 6 Baseline Evidence-Based Security Rules
- 16-PR Security Roadmap Proposals and Maturity Calculations
- 10 Periodic Maintenance Cadences
- 10 Subsystem Security Rule Sets
- PR Merge Gate Inspector for checklist compliance and approval count validation
