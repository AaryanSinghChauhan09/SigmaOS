# Security Hardening Guide

## Overview

SigmaOS applies **defense-in-depth** security drawing from Linux kernel security subsystems (SELinux, seccomp) and BSD security models (pledge, unveil, Capsicum).

## Memory Safety

### Unsafe Code Policy
All `unsafe` blocks **must** include a `// SAFETY:` comment:

```rust
// SAFETY: idx < self.len is verified above, so self.data.add(idx) is within bounds
unsafe { &*self.data.add(idx) }
```

### Integer Overflow Prevention
```rust
// ❌ Vulnerable
let new_size = self.len * 2;

// ✅ Safe  
let new_size = self.len.checked_mul(2).expect("size overflow");
```

## Security Features by Layer

### Kernel Layer
| Feature | Inspiration | Status |
|---------|------------|--------|
| W^X Enforcement | OpenBSD | ✅ Implemented |
| ASLR | Linux/BSD | ✅ Implemented |
| Stack Canaries | GCC -fstack-protector | ✅ Implemented |
| Secure Memory Zeroing | OpenBSD explicit_bzero | ✅ Implemented |
| Capability System | FreeBSD Capsicum | 🔄 Partial |

### Syscall Filtering
Inspired by Linux seccomp and OpenBSD pledge():

```rust
pub struct SyscallPolicy {
    allowed: &'static [u32],  // Whitelist approach
}
```

### Cryptography
Custom implementations in `src/kernel/crypto/`:
- AES-256-GCM (no external crate)
- SHA-256/SHA-512 (custom implementation)
- ChaCha20-Poly1305 (custom implementation)
- Curve25519 key exchange (custom implementation)

## Code Scanning Results

The CodeQL analysis runs on every push to main. Check:
- [Security tab](https://github.com/AaryanSinghChauhan09/SigmaOS/security/code-scanning)
- [CodeQL workflow](https://github.com/AaryanSinghChauhan09/SigmaOS/actions/workflows/codeql-analysis.yml)

## Vulnerability Reporting

See [SECURITY.md](SECURITY) for the responsible disclosure process.

> ⚠️ Never commit credentials or keys - use `src/kernel/crypto/` key storage APIs.
