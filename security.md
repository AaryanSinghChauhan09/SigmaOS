# SigmaOS Security Documentation

## Security Model

SigmaOS implements a **defense-in-depth** security model with multiple independent layers:

```
Layer 7: Application Sandboxing (pledge/unveil)
Layer 6: AppArmor MAC Profiles
Layer 5: SELinux Type Enforcement
Layer 4: Capability-Based Access Control (CBAC)
Layer 3: Mandatory Access Control (MAC LSM hooks)
Layer 2: Address Space Isolation (MMU + KASLR)
Layer 1: Verified Boot (Secure Boot + dm-verity)
```

## Post-Quantum Cryptography

SigmaOS uses quantum-resistant algorithms in addition to classical ones:

| Algorithm | Type | Key Size | Use |
|-----------|------|----------|-----|
| Dilithium-5 | PQC Signature | 2592B public | Package/driver signing |
| Kyber-1024 | PQC KEM | 1568B public | Key exchange |
| Ed25519 | Signature | 32B public | General signing |
| X25519 | ECDH | 32B public | Key agreement |
| AES-256-GCM | Symmetric | 256-bit | Encryption |
| ChaCha20-Poly1305 | Symmetric | 256-bit | Encryption |
| SHA-256/SHA-3 | Hash | — | Integrity |
| Argon2id | KDF | — | Password hashing |

## Kernel Security Hardening

### KASLR (Kernel ASLR)
The kernel loads at a random base address each boot. Entropy: 22 bits on x86_64.

### SMEP + SMAP
Prevent kernel from executing/accessing user-space pages unexpectedly.

### KPTI (Kernel Page Table Isolation)
Separates kernel and user page tables to mitigate Meltdown/Spectre variants.

### Stack Canaries
All kernel functions with stack frames include canary values checked on return.

### CFI (Control Flow Integrity) — Planned
Prevent ROP/JOP attacks by validating indirect call targets at runtime.

## Mandatory Access Control (MAC)

### SELinux Integration

```
# Check current SELinux context
sigma-security context

# Set context for a file
sigma-security setfcontext /usr/bin/myapp sigma_exec_t

# Check policy
sigma-security check -- myapp read /etc/passwd
```

### AppArmor Profiles

```
# Load a profile
sigma-security apparmor load /etc/apparmor.d/usr.bin.myapp

# Set to complain mode (log but don't enforce)
sigma-security apparmor complain myapp

# Enforce profile
sigma-security apparmor enforce myapp
```

## Input Validation

All network inputs are validated by `src/security/input_validation.rs`:

### IPv4 Validation
- Rejects leading zeros (prevents octal SSRF bypass: `010.0.0.1` = `8.0.0.1`)
- Rejects octets > 255
- Requires exactly 4 dot-separated octets

### Path Validation
- Rejects `..` path traversal components
- Rejects embedded NUL bytes (C string injection)
- Enforces maximum path length (4096 bytes)

### Integer Overflow Protection
- All arithmetic uses checked/saturating operations
- Explicit overflow checks before array indexing

## Vulnerability Reporting

**Please do NOT open public issues for security vulnerabilities.**

Report security issues privately:
- **Email:** aaryan.singh.chauhan.09@gmail.com (PGP preferred)
- **GitHub Security Advisories:** https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories

See [SECURITY.md](../SECURITY.md) for the full vulnerability disclosure policy.

## Secure Coding Guidelines

1. **No hardcoded credentials** — use environment variables or secure stores
2. **Validate all inputs** — use `src/security/input_validation.rs` validators
3. **Use checked arithmetic** — prefer `checked_add()` over `+`
4. **Minimize `unsafe`** — document every `unsafe` block with safety proof
5. **Zero sensitive data** — overwrite secrets before dropping them
6. **Constant-time comparisons** — use constant-time compare for secrets
7. **Pin dependencies** — use exact versions in Cargo.toml
8. **Follow least privilege** — request minimum capabilities needed
