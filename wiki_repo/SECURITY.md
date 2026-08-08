# Security Policy

## Supported Versions

| Version | Supported |
| ------- | --------- |
| latest (main) | ✅ |
| v0.x | ✅ |
| < v0.1 | ❌ |

## Reporting a Vulnerability

**Do NOT** open a public issue for security vulnerabilities.

Send encrypted email to: **security@sigmaos.dev**

Include the following information:
- Vulnerability description
- Impact assessment
- Reproduction steps
- Proof of concept
- Affected versions
- Suggested fix

### PGP Key

```text
Key ID: 0x1234567890ABCDEF
Fingerprint: 1234 5678 90AB CDEF 1234 5678 90AB CDEF 1234 5678
```

## Security Best Practices

- **Post-Quantum Cryptography**: Kyber-1024 KEM + Dilithium-5 signatures
- **Capability-Based Security**: 64-bit hardware-enforced permissions
- **Zero-Trust Architecture**: Continuous authentication and verification
- **Memory Safety**: Rust memory safety, W^X enforcement, ASLR
- **Minimal Attack Surface**: Microkernel design with minimal TCB

## Security Audits

- **Static Analysis**: cargo clippy, cargo audit
- **Dynamic Analysis**: fuzzing, penetration testing
- **Third-Party Audits**: Annual external security review
