# Runbook: QEMU Boot and Runtime Validator

## Purpose
Build bootable ISO/RAM-disk image, launch QEMU x86_64, capture serial console output, and verify boot-to-shell.

## Procedure
1. Verify bootloader and ISO staging inputs in `iso_root/` and `installer/`.
2. Launch QEMU x86_64 in headless display mode with serial redirection:
   `qemu-system-x86_64 -m 1024 -nographic -serial file:reports/boot/qemu_x86_64.log -drive file=build/sigmaos.iso,format=raw`
3. Parse `qemu_x86_64.log` for expected boot markers:
   - "Firmware initialized"
   - "Kernel memory mapped"
   - "VFS root mounted"
   - "Init spawned"
   - "SigmaOS Shell ready"
4. Record boot duration, panic status, and exit status code.
5. Generate structured boot JSON: `reports/boot/qemu_x86_64.json`.
