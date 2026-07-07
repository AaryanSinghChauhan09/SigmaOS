# Installer Roadmap

## GUI Installer Design
The SigmaOS installer adopts a Calamares-style intuitive wizard, built entirely on the Zenith Wayland compositor using our native Rust UI toolkit. 

## Secure by Default
- **Secure Boot:** The installer mandates strict Secure Boot chains. The kernel and bootloader are cryptographically signed.
- **Encrypted Home & Full Disk Encryption:** LUKS2 backed by TPM2 attestation is enabled by default. Users must actively opt-out.

## Dual-Boot & Enterprise Imaging
- The installer detects existing EFI partitions and configures systemd-boot/grub alternatives safely.
- A headless, YAML-driven automated installer is provided for enterprise deployment and CI testing environments.
