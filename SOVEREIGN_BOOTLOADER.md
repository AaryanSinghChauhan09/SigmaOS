# Sovereign Bootloader

SigmaOS bypasses legacy dependency on generic bootloaders (like GRUB or systemd-boot) by implementing its own **UEFI Native Sovereign Bootloader**.

Located in `modules/core/boot/sovereign_boot.c`.

## Competitive Advantages (USPs) over Linux Distros

1. **Zero-Trust Cryptographic Boot (macOS/Windows Parity)**:
   - Standard Linux often boots any kernel placed in `/boot`. 
   - SigmaOS embeds an **Ed25519 Public Key** within the bootloader. The kernel payload is mathematically verified against its signature before execution. Tampered or rootkitted kernels are hard-halted.

2. **TPM Measured Boot Integration**:
   - The verified SHA-256 hash of the kernel is recorded into the hardware **Trusted Platform Module (TPM) PCR registers**.
   - This enables **Remote Attestation**, allowing enterprise networks to verify the integrity of a SigmaOS node before granting it access to the Sovereign Mesh Network.

3. **Immutable Handoff State**:
   - Instead of passing raw, unstructured multiboot strings, the bootloader passes a strict `sovereign_handoff_state_t` struct, tightly coupling hardware initialization with the microkernel's Zero-Trust Memory Manager.

4. **Self-Healing Fallback**:
   - If the primary kernel signature fails validation (e.g., failed OTA update), the bootloader automatically pivots to a cryptographically signed recovery snapshot from `SigmaFS`.
