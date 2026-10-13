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
# Runbook: Boot and Runtime Validator (`qemu-boot`)

## Purpose
Prove the primary bootable path (firmware -> bootloader -> kernel -> init -> shell -> desktop) in QEMU x86_64 and record boot evidence.

## Daily Smoke Test Procedure
1. Build the ISO or bootable kernel artifact in a clean environment:
   - `make iso` or `scripts/build_iso.sh`
2. Boot QEMU with fixed parameters:
   - Architecture: x86_64
   - Memory: 2048M
   - CPU: qemu64
   - Serial output: redirected to `reports/boot/qemu-x86_64-serial.log`
   - Timeout: 60 seconds
3. Monitor serial output stream:
   - Check for kernel entry message.
   - Check for memory allocator initialization.
   - Check for VFS mount.
   - Check for init process launch.
   - Check for interactive shell prompt.
4. Execute smoke test commands:
   - Send input: `uname -a`, `help`, `exit`
   - Capture responses.
5. Classify failures if boot halts:
   - Category: `bootloader`, `kernel`, `memory`, `vfs`, `init`, `shell`, `timeout`, `panic`
6. Output reports:
   - Write structured JSON report to `reports/boot/qemu-x86_64.json`
   - Update `reports/boot/boot_matrix.json`
