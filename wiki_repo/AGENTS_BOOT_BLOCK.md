# AI Agent Boot Block Management Architecture (`docs/AGENTS_BOOT_BLOCK.md`)

This guide details the architectural design, EFI/GRUB loader configuration generators, and AI agent monitoring protocols for boot block management in SigmaOS.

---

## 1. Subsystem Architecture

SigmaOS provides unified bootloader management across Linux, BSD, and dual-boot environments:

### A. Bootloader Engine & Configuration (`SigmaBootloaderEngine`, `Bootloader`)
- Located in `src/distro/linux_bsd_distro_gaps.rs` and `src/tools/bootloader.rs`.
- Generates systemd-boot configuration entries (`/loader/entries/*.conf`), GRUB2 configuration blocks (`grub.cfg`), and FreeBSD `/boot/loader.conf` settings.
- Manages dual-boot auto-probing (`auto_detect_dual_boot`) for Windows Boot Manager, Arch Linux, Debian, and FreeBSD.

### B. Measured Boot & TPM Integration
- Integrates with `src/tpm/tpm2_implementation.rs` (`TPM_PCR_4` for Boot Loader measurements).
- Measures firmware, bootloader binaries, and kernel command-line options into TPM PCRs prior to transferring execution control.

### C. Boot Environment & Init Supervision
- `sigma-init` orchestrates multi-supervisor boot sequences (supporting systemd, OpenRC, runit, s6, or dinit).
- Boot environment switching enables atomic rollbacks between boot datasets.

---

## 2. AI Agent Operational Directives

1. **Syntax Validation:** Verify that generated systemd-boot loader entries and GRUB menu entries contain valid `title`, `linux`, `initrd`, and `options` directives.
2. **Path Sanitization:** Prevent path traversal vulnerabilities when reading kernel image or initramfs paths from configuration inputs.
3. **Automated Verification:** Execute `./run_sigma_tests.sh` to confirm bootloader unit tests pass.
