# SigmaOS Security

> **Last Updated**: 2026-07-14
> **Version**: 2.0

## Reporting Vulnerabilities

**Do NOT** open a public issue for security vulnerabilities. All security reports must be treated with strict confidentiality.

### How to Report

Send encrypted email to: **security@sigmaos.dev**

Include the following information:

- Vulnerability description and category (kernel, cryptography, sandbox, network, etc.)
- Impact assessment (CVSS score if available)
- Reproduction steps
- Proof of concept (in a responsible disclosure format)
- Affected versions and components
- Suggested fix (if any)

### PGP Key

```
Key ID: 0x1234567890ABCDEF
Fingerprint: 1234 5678 90AB CDEF 1234 5678 90AB CDEF 1234 5678
```

## Supported Versions

| Version | Supported |
| ------- | --------- |
| latest (main) | ✅ |
| v0.x | ✅ |
| < v0.1 | ❌ |

## Security Architecture Overview

SigmaOS is built ground-up with a security-first philosophy:

### Post-Quantum Cryptography (PQC)
- **Key Encapsulation**: Kyber-1024 (NIST FIPS 203 / ML-KEM)
- **Signatures**: Dilithium-5 (NIST FIPS 204 / ML-DSA)
- **Hashing**: BLAKE3 / SHA3-256 throughout
- All inter-shard IPC is signed at capability-gate time

### Capability-Based Security
- Programs receive explicit capability tokens, not legacy Unix UIDs
- Capabilities are unforgeable, delegatable, and revocable
- Hardware-enforced 64-bit capability model with MPAM/MPK integration

### Mandatory Access Control (MAC)
- SELinux-inspired LSM hooks in the Sigma Security Shard (S-SEC)
- Fine-grained policy enforcement for file access, network, and IPC
- Default-deny for all unspecified operations

### Sandboxing & Isolation
- MicroVM-based process isolation via Firecracker integration
- seccomp-BPF-style syscall filtering (Sigma Silo)
- Namespaces for PID, Network, Mount, and IPC isolation

### Secure Boot Chain
- UEFI Secure Boot with Dilithium-5 signed bootloader
- Measured boot with TPM 2.0 attestation
- Immutable verified kernel image via dm-verity equivalent

## Bug Bounty

| Severity | Reward |
| -------- | ------ |
| Critical (kernel RCE, PQC bypass) | ₹1,00,000 – ₹5,00,000 |
| High (privilege escalation, sandbox escape) | ₹25,000 – ₹1,00,000 |
| Medium (DoS, info leak) | ₹5,000 – ₹25,000 |
| Low (minor hardening bypass) | ₹1,000 – ₹5,000 |

## Disclosure Policy

- We follow **Responsible Disclosure** (90-day coordinated disclosure window)
- Reporter credit in the security advisory (opt-in)
- CVE assignment via MITRE coordination
- Public advisory published after patch release

## Security Updates

Security patches are released on the `main` branch as soon as they are ready. There is no scheduled patch cadence — critical vulnerabilities are patched immediately.

Subscribe to the GitHub Security Advisories on the repository to receive notifications.

## References

- [CONTRIBUTING.md](CONTRIBUTING.md) — General contribution guidelines
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — Community standards
- [Architecture Overview](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture-Overview) — System architecture
- [Security Model](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Security) — Detailed security documentation
