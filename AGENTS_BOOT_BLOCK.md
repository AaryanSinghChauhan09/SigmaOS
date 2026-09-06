# SigmaOS AI Agent Boot Block Management Directive (`AGENTS_BOOT_BLOCK.md`)

This document specifies technical directives, security validation rules, and configuration guidelines for AI agents managing boot block structures, bootloaders, and firmware startup sequences in SigmaOS.

---

## 1. Core Principles for Boot Block Management

Bootloader and boot block components (including UEFI entry generators, systemd-boot configuration builders, GRUB config synthesizers, and measured boot modules) require strict verification:

1. **Firmware & Partition Table Integrity:**
   - Boot entry generation via `SigmaBootloaderEngine` must support Multiboot2, GRUB2, and systemd-boot loader entry formats (`/loader/entries/*.conf`).
   - Dual-boot entries (`DualBootOsEntry`, `BootEntry`, `BootConfiguration`) must safely probe and validate paths (`/EFI/Microsoft/Boot/bootmgfw.efi`, `/boot/vmlinuz-sigma`, `/boot/initramfs-sigma.img`).

2. **Cryptographic Measured Boot Verification:**
   - Measured boot streams must record firmware stage hashes into TPM PCR registers (`TPM_PCR_4`).
   - Secure Boot signature verification must validate kernel images and initramfs blobs against trusted public keys before execution.

3. **Atomic Multi-Stage Boot Supervision:**
   - Multi-supervisory init systems (`sigma-init`) must transition safely through boot runlevel stages (Stage 1 OneTimeInit, Stage 2 MultiUser, Stage 3 Shutdown).
   - Boot configurations must maintain fail-safe default fallback entries in case primary kernel parameters or image checksums fail.

4. **Zero-Dependency `#![no_std]` Compatibility:**
   - Bootloader helpers in core kernel layers must maintain zero-dependency `#![no_std]` compliance using native string builders and vector primitives.

---

## 2. Pre-Commit Boot Block Verification Checklist

Before committing bootloader or boot block modifications, AI agents must verify:
- [ ] UEFI systemd-boot and GRUB2 configuration string generators output valid syntax.
- [ ] Dual-boot detection routines safely handle missing or corrupted partition tables.
- [ ] Measured boot TPM measurement routines correctly update PCR registers without panics.
- [ ] `./run_sigma_tests.sh` executes with 100% test pass rate.
