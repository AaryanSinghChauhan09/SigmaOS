# Boot Process Architecture

SigmaOS boot flow combines modern UEFI secure booting and Linux-inspired measured boot PCRs.

## Boot Steps
1. **Firmware Initialization**: System enters through standard x86 UEFI.
2. **UEFI Wrapper**: Safe UEFI pointer wrapping maps memory zones.
3. **Secure Boot DB Verification**: Signs the kernel image against valid secure DB certificates.
4. **Measured Boot**: PCR registers are populated with SHA-256 hashes of system binaries.\n