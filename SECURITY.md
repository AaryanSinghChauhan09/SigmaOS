# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| main (latest) | ✅ |
| development branches | ❌ (merged into main) |

## Reporting a Vulnerability

**Please do NOT open a public GitHub issue for security vulnerabilities.**

Report security vulnerabilities via [GitHub Security Advisories](https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories).

Include:

*   Description of the vulnerability
*   Steps to reproduce
*   Potential impact assessment
*   Suggested fix (if available)

We aim to respond within **48 hours** and release a patch within **7 days** for critical issues.

## Security Features

See the [Security Wiki page](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Security) for full details.

*   Post-quantum cryptography (Kyber-1024, Dilithium-5)
*   SELinux mandatory access control
*   TPM 2.0 secure boot
*   eBPF-based firewall
*   pledge()/unveil() sandboxing
