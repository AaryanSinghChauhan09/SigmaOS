# Σ security/secure_boot — Cryptographic Secure Boot

Ensures that every component loaded during the SigmaOS boot sequence is
**cryptographically verified** against the Sovereign Trust Root before execution.

## Boot Chain of Trust

```
UEFI Firmware (OEM key)
   └─ SigmaOS UEFI shim (signed with Sovereign Root CA)
         └─ sigma-boot (Rust UEFI bootloader)
               └─ Kernel image verification (Ed25519 + SHA-512)
                     └─ initramfs verification (BLAKE3)
                           └─ kernel_main()
```

## Verification Algorithm

1. Load component into memory.
2. Compute BLAKE3 hash of the raw bytes.
3. Verify Ed25519 signature (from the Sovereign Root CA public key embedded
   in the bootloader).
4. If verification fails → halt with error code and log to TPM event log.
5. If verification passes → transfer execution.

## Rollback Protection

Each verified component carries a **monotonic version counter** stored in the
TPM NV index. Downgrade attacks are rejected:

```c
if (component_version < tpm_read_nv(NV_MIN_VERSION)) {
    secure_boot_halt("Rollback attack detected");
}
```

## API Interface

```c
// Verify a binary image before loading
int secure_boot_verify(const uint8_t *image, size_t len,
                       const uint8_t *sig, size_t sig_len);

// Update the minimum version counter in TPM NV
int secure_boot_update_version(uint32_t component_id, uint32_t new_version);

// Read the TPM event log (for audit purposes)
int secure_boot_read_event_log(sigma_tpm_event_t *out, size_t *count);

// Initialise secure boot subsystem
void init_security_secure_boot(void);
```

## Key Management

| Key | Purpose | Storage |
|---|---|---|
| Sovereign Root CA | Signs all official SigmaOS releases | HSM / offline |
| Platform Key (PK) | UEFI Secure Boot anchor | UEFI NVRAM |
| Key Exchange Key (KEK) | Update DB / DBX | UEFI NVRAM |
| Signing Key | Per-component CI signing | GitHub OIDC → HSM |

## Roadmap

- [ ] Ed25519 signature verification (UEFI phase)
- [ ] BLAKE3 hash chain (initramfs → kernel → rootfs)
- [ ] TPM 2.0 PCR extension and measurement log
- [ ] Rollback counter in TPM NV
- [ ] Measured Boot report (for remote attestation)
- [ ] Post-quantum upgrade path (Dilithium3 signatures)

## Related Modules

- [`modules/security/access_control`](../access_control/README.md) — Runtime MAC
- [`modules/security/isolation`](../isolation/README.md) — Process sandbox
