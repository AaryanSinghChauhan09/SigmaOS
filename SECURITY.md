# SigmaOS Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| main (HEAD) | ✅ Security fixes |
| 0.x (pre-release) | ✅ Security fixes |
| Development branches | ⚠️ Best effort |

## Reporting a Vulnerability

**DO NOT** report security vulnerabilities via public GitHub issues.

### Private Disclosure

1. **GitHub Security Advisory**: Use [GitHub's private vulnerability reporting](https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories/new)
2. **Email**: Report to the maintainer via GitHub private message
3. **Response time**: We aim to acknowledge within 48 hours

### What to Include

- Description of the vulnerability
- Affected component(s) (`src/security/`, `src/kernel/`, etc.)
- Steps to reproduce
- Potential impact assessment
- Suggested fix (optional)

## Security Architecture

SigmaOS implements a defense-in-depth security model:

### Layer 1: Process Isolation
- **pledge(2)**: Promise-based syscall restriction (OpenBSD-inspired)
- **unveil(2)**: Filesystem visibility restriction (OpenBSD-inspired)  
- **Capsicum**: Capability-based access control (FreeBSD-inspired)
- **Namespaces**: PID, mount, network namespaces (Linux-inspired)
- **cgroups**: Resource limitation and accounting

### Layer 2: Memory Safety
- Written in Rust (memory-safe by default)
- Custom buddy allocator (`src/klib/buddy_allocator.rs`)
- ASLR (Address Space Layout Randomization)
- Stack canaries in unsafe blocks
- No undefined behavior by construction in safe Rust

### Layer 3: Cryptography
- Post-quantum cryptography (Dilithium, Kyber)
- TLS 1.3 with PQC hybrid mode
- No hard-coded cryptographic keys in production code
- HMAC-based key derivation

### Layer 4: Privilege Separation
- Principle of least privilege
- Separate security contexts per process
- Kernel/userspace strict separation
- No unnecessary `unsafe` blocks

### Layer 5: Code Quality
- GitHub CodeQL scanning on every push
- No use of `unwrap()` in kernel code (explicit error handling)
- Fuzzing with cargo-fuzz
- All security-sensitive modules have comprehensive tests

## Known Security Limitations

- **Experimental OS**: Not production-ready for security-critical deployments
- **Custom allocator**: Less battle-tested than glibc malloc
- **Hardware drivers**: Some drivers may have vulnerabilities

## Security Contacts

- GitHub Security Advisories: Preferred method
- Code scanning: Automated via [GitHub CodeQL](https://github.com/AaryanSinghChauhan09/SigmaOS/security/code-scanning)

## CVE Process

If a vulnerability is confirmed:
1. We will request a CVE via [MITRE](https://cveform.mitre.org/)
2. Fix will be developed in private
3. Coordinated disclosure after patch is ready
4. Credit to researcher in CHANGELOG and release notes

## Security Acknowledgments

We thank all security researchers who responsibly disclose vulnerabilities.

See [CHANGELOG.md](CHANGELOG.md) for security fix history.
