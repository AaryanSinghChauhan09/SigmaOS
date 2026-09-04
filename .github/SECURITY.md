# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| `main` (rolling) | ✅ Active |
| Pre-release tags | ⚠️ Best-effort |
| Historical commits | ❌ Unsupported |

## Reporting a Vulnerability

**DO NOT open a public GitHub Issue for security vulnerabilities.**

Use **one** of these private channels:
- **GitHub Security Advisories** (preferred): https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories/new
- **Email**: aaryansinghchauhan09@gmail.com — subject line: `[SECURITY] SigmaOS CVE Report`

### Response Timeline
| Severity | Acknowledgement | Fix Target |
|----------|----------------|------------|
| Critical (CVSS 9-10) | 24 hours | 7 days |
| High (CVSS 7-8.9) | 48 hours | 14 days |
| Medium (CVSS 4-6.9) | 72 hours | 30 days |
| Low (CVSS < 4) | 7 days | Next release |

### What to Include
1. Description of the vulnerability
2. Affected file(s) and line numbers
3. Steps to reproduce
4. Potential impact (CIA triad)
5. Suggested fix (optional)
6. CVE ID if already assigned

## Scope

### In-Scope
- All Rust source code under `src/`
- GitHub Actions workflows under `.github/workflows/`
- Cryptographic implementations in `src/crypto/`, `src/security/`
- Package manager (`sigma-pkg`) privilege handling
- Kernel syscall interfaces
- Network stack (`src/network/`)
- Authentication and identity (`src/auth/`)

### Out-of-Scope
- Third-party crates (report to their maintainers)
- Known issues already tracked in public issues
- Theoretical vulnerabilities without a working PoC
- Social engineering or phishing attacks

## Security Architecture

SigmaOS implements defence-in-depth with:
- **Pledge + Unveil** sandbox model (OpenBSD-inspired) for all userland
- **SELinux MAC** enforcing mode for system services
- **eBPF-enforced** network policy at the NIC level
- **Secure Boot + TPM attestation** for kernel integrity
- **Rust memory safety** — no unsafe code without documented `SAFETY:` comments
- **Zero external runtime dependencies** in the kernel core

## Disclosure Policy

SigmaOS follows **Coordinated Vulnerability Disclosure (CVD)**:
1. Reporter submits privately
2. Maintainers acknowledge and assess
3. Fix developed on private branch
4. Advisory drafted and assigned CVE/GHSA
5. Fix and advisory published simultaneously
6. Reporter credited (with permission)

## Security Contacts

- Primary: [@AaryanSinghChauhan09](https://github.com/AaryanSinghChauhan09)
- Email: aaryansinghchauhan09@gmail.com
