# Bug Bounty Program

To ensure the security and integrity of the Sovereign Lattice, the SigmaOS community invites security researchers to participate in our formal **Bug Bounty Program**.

---

## Program Overview

The SigmaOS Bug Bounty Program rewards security researchers who discover and responsibly disclose vulnerabilities in SigmaOS. Our goal is to identify and fix security issues before they can be exploited maliciously.

---

## Scope

### In Scope
- SigmaOS kernel vulnerabilities (privilege escalation, memory corruption, etc.)
- Security bypasses in sigma_pledge, sigma_unveil, or AVC
- Post-quantum cryptography implementation flaws
- TPM2 and Secure Boot vulnerabilities
- Network stack vulnerabilities (TLS 1.3, DNS, DHCP, etc.)
- Filesystem vulnerabilities (SigmaFS, Ext4, dm-verity)
- Virtualization and container escape vulnerabilities
- Package manager signature bypasses
- Zenith Desktop security issues
- System daemon vulnerabilities

### Out of Scope
- Third-party applications running on SigmaOS
- Issues in dependencies that are not SigmaOS-specific
- Social engineering attacks
- Physical attacks requiring physical access
- Denial of service attacks without security impact
- UI/UX issues without security impact
- Theoretical vulnerabilities without PoC

---

## Reward Tiers

Rewards are tiered based on severity as determined by the Sovereign Council:

| Severity | Reward Range | Description |
|----------|--------------|-------------|
| **Critical** | $10,000 - $50,000 | Remote code execution, privilege escalation, cryptographic bypass |
| **High** | $5,000 - $10,000 | Information disclosure, authentication bypass, significant security bypass |
| **Medium** | $1,000 - $5,000 | Limited information disclosure, minor security bypass |
| **Low** | $100 - $1,000 | Minor security issues, best practice violations |
| **Informational** | $50 - $100 | Security best practices, documentation improvements |

---

## Submission Guidelines

1. Submit reports privately via the GitHub "Security" tab or our encrypted mailing list: `security@sigmaos.org`

2. Provide a clear Proof of Concept (PoC) demonstrating the vulnerability

3. Include detailed steps to reproduce the issue

4. Describe the potential impact and affected versions

5. Suggest a fix or mitigation if possible

6. Allow the maintainers 30 days to triage and patch before public disclosure

---

## Rules

- Do not exploit vulnerabilities on production systems
- Do not access or modify user data
- Do not use automated scanners that may cause disruption
- Report vulnerabilities responsibly through proper channels
- Do not publicly disclose vulnerabilities before the 30-day disclosure window
- Do not engage in any illegal activities

---

## Recognition

Researchers who successfully discover and help patch vulnerabilities will be:
- Permanently credited in the `MAINTAINERS.md` file
- Listed in our Security Hall of Fame
- Eligible for SigmaOS contributor badges
- Invited to private security discussions
- Considered for SigmaOS Security Council membership

---

## Hall of Fame

| Researcher | Vulnerabilities | Date |
|------------|----------------|------|
| *Pending first submission* | - | - |

---

## Safe Harbor

SigmaOS commits to:
- Not pursue legal action against researchers who follow these guidelines
- Work with researchers to understand and resolve issues
- Credit researchers for their contributions
- Respond to submissions within 7 days

---

## Additional Information

For questions about the Bug Bounty Program, contact: `bounty@sigmaos.org`

For encryption keys for secure submissions, see: `SECURITY_POLICY.md`

---

*Last Updated: July 2026*
*Program Version: 1.0*
