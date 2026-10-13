# SigmaOS Security Policy

This root security policy governs the SigmaOS repository.

For the full detailed security management framework, PR roadmap, maintenance cadences, component rules, and merge gates, see [`docs/SECURITY.md`](docs/SECURITY.md).

## Reporting a Vulnerability

**Do NOT file public issues or pull requests containing unpatched security vulnerabilities or exploit code.**

- **Private Channel:** Submit report via GitHub Private Vulnerability Reporting or contact security maintainers directly at `security@sigmaos.org`.
- **Required Report Information:**
  - Description of vulnerability and affected component/version.
  - Proof of Concept (PoC) or step-by-step reproduction instructions.
  - Threat model, impact assessment, and potential mitigations.
- **Response Targets:** Initial acknowledgement within 3 business days; initial triage rating within 10 business days.

## Baseline Evidence Rules

- **Evidence Requirement:** Maturity level "Supported" requires documented runtime, security, and recovery test evidence on target hardware or hypervisor.
- **One PR, One Outcome:** Keep security patches minimal and isolated from refactorings.
- **Human Review Gates:** Security-sensitive PRs (privilege boundaries, cryptography, boot/update trust, security policies) require approval from two qualified maintainers.
