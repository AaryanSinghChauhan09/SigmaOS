# Security Policy for SigmaOS

## Supported Versions

| Version | Supported |
|---------|-----------|
| main branch | ✅ Active |
| Legacy branches | ❌ Not supported |

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities through GitHub public issues.**

To report a security vulnerability:

1. **Email**: security@sigmaos.dev (or open a private GitHub Security Advisory)
2. **GitHub Security Advisories**: Use [GitHub's private reporting feature](https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories/new)

Include in your report:
- Type of issue (e.g. buffer overflow, privilege escalation, hard-coded credential)
- Full paths of affected source file(s)
- Steps to reproduce
- Proof of concept (if available)
- Impact assessment

We aim to respond within **48 hours** and provide a patch within **14 days** for critical issues.

## Security Design Principles

SigmaOS follows these security principles inspired by OpenBSD and Linux hardening:

### 1. Least Privilege
All kernel subsystems run with minimum required privileges. Capabilities are fine-grained and non-inheritable unless explicitly set.

### 2. No Hard-Coded Secrets
All cryptographic material (keys, IVs, salts, passwords) must be:
- Generated at runtime using the kernel's CSPRNG (`src/security/rng.rs`)
- Never stored as literals in source code
- Validated by CodeQL scanning (rule: `rust/hard-coded-cryptographic-value`)

### 3. Memory Safety
- Rust's ownership model prevents use-after-free and buffer overflows by design
- Unsafe blocks are reviewed and documented with `// SAFETY:` comments
- Custom allocator (`klib/alloc.rs`) includes red-zone detection

### 4. Address Space Layout Randomization (ASLR)
Implemented in `src/security/aslr.rs`. All user-space processes run with randomized base addresses.

### 5. Stack Smashing Protection
Stack canaries are placed by the compiler. Additional runtime checks in `kernel/mm/`.

### 6. Mandatory Access Control (MAC)
Inspired by SELinux/AppArmor. Implemented in `src/security/mac.rs`.

### 7. Secure Boot Chain
Verified boot from stage 1 bootloader through kernel, checking cryptographic signatures at each step.

## CodeQL Alerts

We use GitHub CodeQL to automatically scan for:
- Hard-coded cryptographic values (`rust/hard-coded-cryptographic-value`)
- Unused variables leaking sensitive information (`rust/unused-variable`)
- Potential injection vulnerabilities

View current status: https://github.com/AaryanSinghChauhan09/SigmaOS/security/code-scanning

## Cryptography Guidelines

| Usage | Recommended | Avoid |
|-------|------------|-------|
| Symmetric encryption | AES-256-GCM, ChaCha20-Poly1305 | DES, 3DES, RC4 |
| Hashing | SHA-3, BLAKE3 | MD5, SHA-1 |
| Key exchange | X25519, ECDH (P-256) | RSA < 2048-bit |
| Signatures | Ed25519, ECDSA | DSA with weak params |
| RNG | Kernel CSPRNG | `rand` without entropy |

## Known Mitigations

| Threat | Mitigation |
|--------|-----------|
| Spectre/Meltdown | KPTI enabled; retpoline trampolines |
| ROP attacks | Shadow stack (when CPU supports CET) |
| Heap spray | ASLR + heap randomization |
| Format string bugs | Rust's type system prevents at compile time |
| Integer overflow | Rust checks in debug mode; explicit `wrapping_*` in release |

