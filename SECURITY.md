# Security Policy

## Overview

SigmaOS is an experimental Rust operating system tracking component maturity as **Proposed, Prototype, Integrated, or Supported**.
We prioritize security across the entire development lifecycle following NIST's Secure Software Development Framework (SSDF) [NIST SP 800-218] and mature Linux/BSD distribution practices.

Detailed policies, threat models, and the 16-PR Security Roadmap are documented in [`docs/SECURITY.md`](docs/SECURITY.md).

---

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities through public GitHub issues or public pull requests.**

### Private Reporting Channel

To privately report a security vulnerability or report suspected security bugs:
- **GitHub Private Vulnerability Reporting:** Use the "Report a vulnerability" button on the GitHub repository security tab (if enabled).
- **Email / Confidential Contact:** Submit vulnerability details to `security@sigmaos.org` or contact the designated repository security maintainers.

### What to Include in a Vulnerability Report

1. **Component & Version:** Affected subsystem, file, git commit SHA, or release version.
2. **Vulnerability Type:** Syscall boundary breach, memory protection bypass, crypto flaw, privilege escalation, denial of service, etc.
3. **PoC / Steps to Reproduce:** Minimal reproduction script, command line sequence, or payload (keep strictly confidential).
4. **Impact Analysis:** Assessed impact, required privileges, and proposed fix/mitigation if known.

### Disclosure Coordination & SLA

- **Acknowledgement:** We aim to acknowledge receipt of private vulnerability reports within **3 business days**.
- **Triage & Analysis:** Maintainers analyze exploitability, affected releases, and scope within **10 business days**.
- **Coordinated Release:** A fix will be developed in a private branch, verified with regression tests, and published alongside a Security Advisory and fixed release version.

---

## Baseline Security Guidelines

- **Evidence-Based Security Claims:** A capability is only considered "Supported" if it has passed runtime, security, and recovery tests on real or virtualized target hardware.
- **Minimal Targeted Security PRs:** Security patches are kept focused and separate from major refactorings or feature changes.
- **Two-Reviewer Gate for Privileged Code:** Changes to kernel boundaries, crypto, boot trust, sandboxing, or security policy require two qualified maintainers.
- **Artifact Integrity & SBOM:** Release binaries are published with cryptographic signatures, checksums, and SBOM metadata.

For the full security plan, see [`docs/SECURITY.md`](docs/SECURITY.md).
