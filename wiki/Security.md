# SigmaOS Security

## Overview

SigmaOS implements a comprehensive security model based on capability-based access control, secure boot, and cryptographic primitives. This document describes the security architecture and features.

## Security Architecture

### Capability-Based Security

SigmaOS uses a capability-based security model where processes are granted specific capabilities rather than relying on a global root user. This follows the principle of least privilege.

**Features:**
- Fine-grained permissions for each operation
- No global root user
- Capability inheritance for child processes
- Capability revocation support

**Implementation:** `security/mac.rs`

### Mandatory Access Control (MAC)

SigmaOS implements a SELinux/AppArmor-style MAC system with sandboxing support.

**Features:**
- Process sandboxing with profiles
- Filesystem access control
- Network access control
- IPC restrictions

**Implementation:** `security/mac.rs`

## Secure Boot

### Overview

SigmaOS Secure Boot provides a chain of trust from firmware to kernel, ensuring that only signed and verified code runs on the system.

**Implementation:** `boot/sigma_secureboot.rs`

### Key Management

Secure Boot uses a hierarchical key management system:

- **PK (Platform Key):** The root of trust for the platform
- **KEK (Key Exchange Key):** Keys used to sign keys in db/dbx
- **db (Signature Database):** Database of authorized signatures
- **dbx (Forbidden Signature Database):** Database of forbidden signatures

### Supported Algorithms

- RSA-2048 with SHA-256
- RSA-4096 with SHA-512
- ECDSA P-256 with SHA-256
- ECDSA P-384 with SHA-384

### Verification Process

1. **Bootloader Verification:** Verifies the PE/COFF signature of the bootloader
2. **Kernel Verification:** Verifies the kernel signature against db database
3. **Module Verification:** Verifies kernel modules against db and dbx databases

### C-ABI Exports

```c
// Initialize Secure Boot manager
int secure_boot_init(SecureBootState state);

// Get/Set Secure Boot state
SecureBootState secure_boot_get_state(void);
int secure_boot_set_state(SecureBootState state);

// Key management
int secure_boot_add_key(KeyType db_type, const KeyInfo* key);
int secure_boot_remove_key(KeyType db_type, const uint8_t* fingerprint);
int secure_boot_list_keys(KeyType db_type, KeyInfo* keys, uint32_t max_keys, uint32_t* key_count);

// Verification
int secure_boot_verify_bootloader(const uint8_t* bootloader_path);
int secure_boot_verify_kernel(const uint8_t* kernel_path);
int secure_boot_verify_module(const uint8_t* module_path);

// Key generation
int secure_boot_generate_key(SignatureAlgorithm algorithm, KeyInfo* private_key, KeyInfo* public_key);
int secure_boot_sign(const uint8_t* data, uint32_t data_len, const KeyInfo* private_key, SignatureInfo* signature);
```

## TPM Integration

### Overview

SigmaOS integrates with TPM (Trusted Platform Module) 2.0 for measured boot and secure key storage.

**Implementation:** `boot/sigma_secureboot.rs`

### TPM Device Structure

```rust
pub struct TpmDevice {
    pub device_id: u32,
    pub manufacturer: [u8; 4],
    pub version: u32,
    pub pcr_count: u32,
    pub pcr_registers: [u8; 24 * 20], // 24 PCRs, 20 bytes each
    pub initialized: bool,
}
```

### PCR (Platform Configuration Register) Management

PCRs are used to store measurements of boot components:

- **PCR 0:** Bootloader hash
- **PCR 1:** Kernel hash
- **PCR 2-7:** Additional measurements
- **PCR 8-15:** Application measurements
- **PCR 16-23:** Reserved for future use

### TPM Operations

#### PCR Extend
Extends a PCR register with new measurement data:
```c
int tpm_pcr_extend(uint32_t pcr_index, const uint8_t* data, uint32_t data_len);
```

#### PCR Read
Reads the current value of a PCR register:
```c
int tpm_pcr_read(uint32_t pcr_index, uint8_t* pcr_value);
```

#### Seal
Encrypts data bound to specific PCR values:
```c
int tpm_seal(const uint8_t* data, uint32_t data_len, uint32_t pcr_mask, 
             uint8_t* sealed_data, uint32_t* sealed_len);
```

#### Unseal
Decrypts data if PCR values match:
```c
int tpm_unseal(const uint8_t* sealed_data, uint32_t sealed_len, uint32_t pcr_mask,
               uint8_t* data, uint32_t* data_len);
```

#### Quote
Generates a signed attestation of PCR values:
```c
int tpm_quote(uint32_t pcr_mask, const uint8_t* nonce, uint32_t nonce_len,
              uint8_t* quote, uint32_t* quote_len);
```

### Secure Boot TPM Integration

SigmaOS automatically integrates Secure Boot with TPM:

1. **Boot Measurement:** Extends PCR 0 with bootloader hash
2. **Kernel Measurement:** Extends PCR 1 with kernel hash
3. **Verification:** Verifies PCR values before boot completion

```c
int secure_boot_tpm_integrate(void);
int secure_boot_tpm_verify(void);
```

## Cryptographic Primitives

### Overview

SigmaOS provides essential cryptographic primitives for security operations.

**Implementation:** `crypto/sigma_crypto.rs`

### Hash Functions

#### SHA-256
- 256-bit hash output
- Used for file integrity verification
- Used in TPM measurements
- C-ABI export: `sigma_crypto_sha256()`

#### SHA-512
- 512-bit hash output
- Used for key derivation
- Used in digital signatures
- C-ABI export: `sigma_crypto_sha512()`

### PGP Key Generation

SigmaOS implements PGP-compatible key generation for security@sigmaos.dev and other purposes.

#### Key Pair Structure
```rust
pub struct PgpKeyPair {
    pub public_key: [u8; 32],
    pub private_key: [u8; 64],
    pub key_id: [u8; 8],
    pub created: u64,
}
```

#### Key Generation
Generates Ed25519-like key pairs from identity information:
```c
int sigma_crypto_pgp_generate_key(const PgpIdentity* identity, PgpKeyPair* key_pair);
```

#### Signing
Signs data with the private key using HMAC-SHA512:
```c
int sigma_crypto_pgp_sign(const uint8_t* data, uint32_t data_len,
                          uint8_t* signature, uint32_t* sig_len);
```

#### Verification
Verifies signatures with the public key:
```c
int sigma_crypto_pgp_verify(const uint8_t* data, uint32_t data_len,
                            const uint8_t* signature, uint32_t sig_len,
                            const uint8_t* public_key);
```

#### Export
Exports public key in ASCII-armored PGP format:
```c
int sigma_crypto_pgp_export_public(const PgpKeyPair* key_pair,
                                   uint8_t* output, uint32_t* output_len);
```

## Memory Protection

### Page-Level Protection
- Read/Write/Execute permissions per page
- NX bit enforcement for non-executable pages
- Copy-on-write for fork optimization

### ASLR (Address Space Layout Randomization)
- Randomized base addresses for executables
- Randomized stack locations
- Randomized heap locations

### Stack Canaries
- Stack protection against buffer overflows
- Canary value verification on function return

## Sandbox Isolation

### Process Sandbox
- Filesystem namespace isolation
- Network namespace isolation
- IPC restrictions
- Resource limits (CPU, memory)

### Capability Gating
- Fine-grained capability checks
- No implicit permissions
- Capability inheritance control

## Security Best Practices

### For Developers

1. **Use Capabilities:** Always use capability-based access control
2. **Validate Input:** Validate all user input before processing
3. **Use Secure APIs:** Use secure variants of functions (e.g., strncpy vs strcpy)
4. **Sign Code:** Sign all kernel modules and drivers
5. **Test Security:** Include security testing in CI/CD

### For Users

1. **Enable Secure Boot:** Keep Secure Boot enabled for maximum security
2. **Verify Signatures:** Verify package signatures before installation
3. **Use Sandboxing:** Run untrusted applications in sandboxes
4. **Keep Updated:** Keep the system updated with security patches
5. **Monitor Logs:** Monitor security logs for suspicious activity

## Security Auditing

### Kernel Logging
- Security-related events logged
- Audit trail for security operations
- Tamper-evident logging

### Crash Analysis
- Kernel panic analysis
- Memory dump generation
- Post-mortem security analysis

## Future Enhancements

### Planned Features

1. **Live Patching:** Secure runtime kernel updates
2. **Hardware Security Keys:** Support for U2F/FIDO2
3. **Container Security:** Enhanced container isolation
4. **Network Security:** Advanced firewall and IDS/IPS
5. **Filesystem Encryption:** Full-disk encryption support

## References

- [Kernel Architecture](Kernel-Architecture.md)
- [Driver Development Guide](../drivers/DRIVER_DEVELOPMENT_GUIDE.md)
- [Package Management](Package-Management-Spec.md)

## License

All SigmaOS security components are licensed under MIT License. See [LICENSE](../LICENSE) for details.
