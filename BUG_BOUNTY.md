# SigmaOS Bug Bounty Program

> **Status**: Active | **Version**: 1.0 | **Contact**: security@sigmaos.org

---

## Overview

The SigmaOS Bug Bounty Program rewards security researchers who responsibly disclose vulnerabilities in the SigmaOS codebase, infrastructure, and associated systems. We believe in supporting the security community and take all reports seriously.

**Scope**: SigmaOS kernel, drivers, package manager (sigpkg), cryptographic subsystems, sigma-shield firewall, secure boot chain, and official cloud infrastructure.

---

## Scope

### In-Scope

| Target | Component | Notes |
|--------|-----------|-------|
| SigmaOS Kernel | `kernel/` | All subsystems |
| sigpkg | `sigma-pkg/`, `userland/sigpkg/` | Package installation, verification |
| sigma-shield | `kernel/net/firewall/` | Firewall bypass, rule bypass |
| Cryptographic Layer | `crypto/`, PQC code | Algorithm implementation bugs |
| Secure Boot Chain | `sigma-boot/`, TPM integration | Boot integrity bypass |
| POSIX Compat Layer | `kernel/posix/` | Privilege escalation via syscalls |
| sigma-shell | `sigma-sh/` | Command injection, privilege escalation |
| Zenith Desktop | `desktop/`, `zenith_desktop/` | Sandbox escape, privilege escalation |
| sigmaos.org | Official web infrastructure | XSS, SQL injection, auth bypass |

### Out-of-Scope

- Theoretical vulnerabilities without proof-of-concept
- Social engineering attacks
- Physical access attacks
- Third-party dependencies (report to upstream instead)
- Denial-of-service that requires physical access
- Bugs in non-current release branches (> 2 releases old)
- Missing security headers on static documentation pages

---

## Severity Classification & Rewards

### Critical — $3,000–$10,000

Vulnerabilities that allow:
- **Remote code execution** without authentication
- **Kernel privilege escalation** from userland (Ring 3 → Ring 0)
- **Secure Boot bypass** allowing unsigned kernel to boot
- **Cryptographic key extraction** from TPM or PQC signing subsystem
- **Full container/cgroup escape** to host

Examples:
- Buffer overflow in syscall handler allowing arbitrary kernel write
- Dilithium5 signature forgery
- UEFI Secure Boot bypass

### High — $1,000–$3,000

- **Local privilege escalation** (user → root, or capability bypass)
- **Firewall rule bypass** allowing blocked traffic
- **Package signature bypass** allowing unsigned package installation
- **Remote code execution** requiring user interaction

### Medium — $250–$1,000

- **Information disclosure** of sensitive kernel memory from userland
- **Denial of service** in kernel network stack (no reboot required for recovery)
- **Sandbox escape** within sigpkg sandboxed installs
- **ASLR bypass** combined with another bug

### Low — $50–$250

- **Information disclosure** of non-sensitive data
- **Denial of service** requiring reboot
- **Timing side channels** in non-cryptographic code
- **Minor logic errors** with limited security impact

### Informational — Acknowledgment Only

- Best practice recommendations
- Missing security hardening (no direct impact)
- Theoretical issues without practical exploitation path

---

## Rules of Engagement

### You MUST:

1. **Report responsibly**: Submit via `security@sigmaos.org` with GPG encryption using [our PGP key](https://sigmaos.org/security/pgp-key.asc) before any public disclosure
2. **Give us time**: Allow **90 days** from acknowledgment to fix and release before public disclosure
3. **Minimize impact**: Do not access, modify, or delete user data beyond what is necessary to demonstrate the vulnerability
4. **Provide a PoC**: Include a working proof-of-concept or detailed reproduction steps
5. **Act in good faith**: Do not use discovered vulnerabilities for personal gain or to harm users

### We WILL:

1. Acknowledge your report within **48 hours**
2. Provide a preliminary severity assessment within **7 days**
3. Keep you informed of remediation progress
4. Credit you in the security advisory (unless you request anonymity)
5. Pay rewards within **30 days** of fix validation
6. Not pursue legal action against researchers acting in good faith

---

## Submission Template

```
Subject: [SIGMAOS-BUG-BOUNTY] Brief description

## Summary
Brief description of the vulnerability.

## Severity (your assessment)
[Critical / High / Medium / Low]

## Affected Component
File paths, versions, build hashes.

## Reproduction Steps
1. Step one
2. Step two
3. ...

## Expected Behavior
What should happen.

## Actual Behavior
What actually happens (potential security impact).

## Proof of Concept
Code, commands, or screenshots demonstrating the issue.

## Suggested Fix (optional)
If you have ideas on how to fix it.

## Your PGP Key or preferred contact
For encrypted communications.
```

---

## Hall of Fame

Researchers who have reported valid security issues are recognized here. Thank you for making SigmaOS safer!

| Researcher | Issue | Severity | Year |
|-----------|-------|----------|------|
| *(program launching — first reports pending)* | — | — | 2026 |

---

## Legal Safe Harbor

SigmaOS and the SigmaOS Foundation grant security researchers a **safe harbor** from legal claims under the following conditions:

- Research is conducted in accordance with this policy
- The researcher does not cause harm to users, systems, or data
- The researcher does not disclose the vulnerability before the coordinated disclosure period expires
- The researcher does not violate applicable laws beyond those that would be waived under good-faith security research

We consider security research conducted under this policy to be **authorized** and will not initiate legal action.

---

## Coordinated Vulnerability Disclosure Policy

1. **Day 0**: Researcher submits report to `security@sigmaos.org`
2. **Day 2**: Security WG acknowledges receipt
3. **Day 7**: Severity assessment + bounty estimate communicated
4. **Day 14–60**: Fix developed, tested, and validated
5. **Day 60–90**: Fix released in patch/point release
6. **Day 90**: Coordinated public disclosure (CVE assigned if applicable)
7. **Day 90+**: Researcher may publish their own write-up

For **Critical** severity issues, the Security WG may request a temporary embargo extension beyond 90 days with researcher agreement.

---

## Frequently Asked Questions

**Q: Can I test on the live sigmaos.org infrastructure?**  
A: No. Please test only on local instances. For web infrastructure bugs, contact us first and we will provide a test environment.

**Q: What payment methods are supported?**  
A: Bank transfer, PayPal, or cryptocurrency (BTC/ETH). We are also open to donating your reward to an open-source charity of your choice.

**Q: Is there a minimum severity for a reward?**  
A: Low severity and above receive monetary rewards. Informational reports receive acknowledgment and Hall of Fame listing.

**Q: Can I submit multiple related bugs as one report?**  
A: Yes, if they form a single exploit chain. If they are independent vulnerabilities, please submit separately.

**Q: What if my research affects a dependency (like a crate)?**  
A: Please report to the upstream project first. If the vulnerability is specifically in how SigmaOS uses the dependency (e.g., improper input validation), it may qualify.

---

*This policy was last updated: 2026-07-13. For questions, contact security@sigmaos.org.*
