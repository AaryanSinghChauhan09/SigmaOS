# SigmaOS Internal Security Policy

> **Classification**: Internal — Maintainer Use Only
> **Public Policy**: See [/SECURITY.md](../../SECURITY.md)

---

## Incident Response Contacts

| Role | Contact | PGP Key |
| ---- | ------- | ------- |
| Lead Maintainer | aaryansinghchauhan@sigmaos.com | Key ID: 0xABCD1234 |
| Security Lead | security@sigmaos.dev | Key ID: 0x1234ABCD |

---

## CVE Triage Procedure

When a CVE or vulnerability report is received:

1. **Acknowledge** within 24 hours via encrypted channel
2. **Assess** CVSS score and affected components
3. **Triage** into severity bucket:
   - **Critical**: Kernel RCE, PQC bypass → patch within 48 hours
   - **High**: Privilege escalation, sandbox escape → patch within 7 days
   - **Medium**: Info leak, DoS → patch within 30 days
   - **Low**: Hardening bypass → patch in next release
4. **Develop** fix on a private fork or branch
5. **Review** with at least one other maintainer before merge
6. **Coordinate** embargo period with reporter (default 90 days)
7. **Release** patch on `main` and draft a GitHub Security Advisory
8. **Publish** CVE assignment and advisory simultaneously


---

## Private Disclosure Channel

All unpatched CVEs are discussed in:

- **GitHub Private Vulnerability Reporting** (preferred)
- **Encrypted Email**: security@sigmaos.dev (PGP required)


Do **not** discuss unpatched issues in:

- Public GitHub Issues
- Discord/community channels
- Social media


---

## Internal Security Checks

Before each release, the following checks must pass:

- [ ] `cargo audit` — Rust dependency vulnerability scan
- [ ] Static analysis (`clippy` + `codeql-analysis.yml`)
- [ ] Kernel capability boundary tests (`modules/security/access_control/`)
- [ ] PQC signature verification test suite
- [ ] Silo isolation smoke tests (`modules/security/isolation/`)
- [ ] Secure boot chain validation (`modules/security/secure_boot/`)


---

## Component Security Owners

| Component | Owner | Shard |
| --------- | ----- | ----- |
| Kernel Core | Lead Maintainer | S-CORE |
| PQC / Crypto | Security Lead | S-SEC |
| Network Stack | Network Maintainer | S-NET |
| Package Manager | Package Maintainer | sigma-pkg |
| AI Subsystem | AI Lead | S-AI |

---

## Related

- [SECURITY.md (Public)](../../SECURITY.md) — Public security policy
- [CONTRIBUTING.md](contributing.md) — Meta contribution guidelines
- [BUG_BOUNTY.md](../../wiki_repo/BUG_BOUNTY.md) — Public bug bounty program
