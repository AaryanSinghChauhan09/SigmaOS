#ifndef SIGMA_BOOTLOADER_H
#define SIGMA_BOOTLOADER_H

#include <stdint.h>

// SigmaOS Genesis Bootloader (S01)
// Absorbing rapid-boot paradigms of systemd-boot and flexibility of GRUB
// Integrated natively with the Zero-Trust Secure Boot Verification protocol.

void genesis_init_uefi_hooks(void);
void genesis_verify_kernel_signature(void);
void genesis_decompress_kernel(void);
void genesis_handoff_to_orchestrator(void);

#endif // SIGMA_BOOTLOADER_H
