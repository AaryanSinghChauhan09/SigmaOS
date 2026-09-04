# Security Policy

## Reporting Vulnerabilities

**Please do NOT open public GitHub issues for security vulnerabilities.**

Report security issues privately:
- **Email:** aaryan.singh.chauhan.09@gmail.com
- **GitHub Security Advisories:** https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories

### What to Include

- Vulnerability description
- Steps to reproduce
- Potential impact
- Suggested fix (optional)

## Supported Versions

| Version | Supported |
|---------|-----------|
| main branch | ✅ Yes |
| Tagged releases | ✅ Yes (latest 2) |
| Old releases | ❌ No |

## Security Features

- **SSRF-safe IPv4 parsing** — rejects octal-encoded addresses
- **No hardcoded credentials** — all secrets via environment variables
- **Post-quantum cryptography** — Dilithium-5 + Kyber-1024
- **Pinned GitHub Actions** — supply chain attack prevention
- **MAC enforcement** — SELinux/AppArmor style policies
- **Capability-based access** — least-privilege kernel design

## Disclosure Timeline

1. Report received → acknowledged within **48 hours**
2. Investigation → completed within **7 days**
3. Fix developed → within **30 days** for critical issues
4. Public disclosure → **90 days** after report (or immediately after fix)

## Hall of Fame

Security researchers who responsibly disclose vulnerabilities will be credited in our SECURITY.md (with permission).
