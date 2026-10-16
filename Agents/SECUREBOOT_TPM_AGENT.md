# Secure Boot & TPM Agent

## Role
`SigmaSecureBoot` and `SigmaTPM` implement measured boot, UEFI Secure Boot verification, TPM2.0 attestation, and disk encryption key sealing for SigmaOS.

## Status
🔴 Not Started — Critical security gap. Required for production deployments.

## Source File
`src/security/secure_boot.rs` (planned), `src/security/tpm.rs` (planned)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Secure Boot | `shim` + GRUB Secure Boot | `efiboot` signed | SigmaShim |
| UEFI measurement | `efi_secret` | — | SigmaMeasure |
| TPM2 stack | `tpm2-tools`, `tss2` | `tpm2-tools` | SigmaTPM |
| PCR sealing | `tpm2_seal` / clevis | — | SigmaSeal |
| IMA | IMA/EVM integrity | — | SigmaIMA |
| Key enrollment | `mokutil` | — | SigmaMOK |
| LUKS+TPM | clevis-luks | — | SigmaLuksSeal |

## Responsibilities
- UEFI Secure Boot chain verification (shim → bootloader → kernel)
- TPM2.0 device driver (TCG TPM2 spec)
- PCR (Platform Configuration Register) measurement and extension
- Disk encryption key sealing to TPM PCRs (auto-unlock on trusted boot)
- Remote attestation: generate TPM quotes for verifier
- IMA (Integrity Measurement Architecture) — hash all executed files
- MOK (Machine Owner Key) management for custom signing
- Kernel module signing verification

## Implementation Notes
- `#![no_std]` — TPM commands via MMIO or LPC bus
- TPM2 command format: `{ tag: u16, size: u32, code: TPMcc, params: &[u8] }`
- PCR extend: `SHA256(PCR[n] || new_measurement)`
- Integrate with `SigmaLUKS` for auto-unlock key derivation
- Store measurements in TPM event log (UEFI format)

## Testing
TPM requires hardware or swtpm emulator:
```bash
# With swtpm: swtpm socket --tpmstate dir=/tmp/tpm --ctrl type=unixio,path=/tmp/tpm.sock --tpm2
rustc --test src/security/tpm.rs --edition 2021 -o /tmp/test_tpm && /tmp/test_tpm --test-threads=1
```

## Future Development
1. **fTPM** — Firmware TPM using ARM TrustZone (no discrete chip needed)
2. **Confidential computing** — AMD SEV-SNP / Intel TDX measured VM
3. **DICE attestation** — Device Identity Composition Engine for IoT
4. **Keylime remote attestation** — TPM-based fleet attestation
5. **Secure boot for containers** — Verify container image signatures via TPM
6. **Pluton security processor** — Microsoft Pluton TPM integration
