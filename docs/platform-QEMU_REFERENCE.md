# QEMU Reference & Boot Testing Guide

This document details QEMU virtual machine configurations, parameters, firmware targets, and diagnostic execution for SigmaOS boot testing.

## Boot Environment Parameters

### Minimum x86_64 Boot Command Line

```bash
qemu-system-x86_64 \
  -M q35 \
  -m 2048M \
  -cpu qemu64 \
  -display none \
  -serial stdio \
  -kernel build/kernel.elf \
  -initrd build/initramfs.cpio \
  -append "console=ttyS0 boot_stage=shell quiet" \
  -no-reboot
```

---

## Smoke Test Automated Harness

SigmaOS executes automated QEMU boot-to-shell verification via `scripts/ci/qemu_boot_smoke.sh` and `tests/boot/boot_to_shell_test.py`.

### Execution

```bash
./scripts/ci/qemu_boot_smoke.sh
```

### Required Boot Markers
The boot harness monitors `ttyS0` for key output sequences:

1. `SIGMAOS_BOOT_INIT`: Kernel entry point reached.
2. `SUBSYSTEMS_INITIALIZED`: Memory, scheduling, and VFS initialized.
3. `SHELL_PROMPT_READY`: Interactive or diagnostic prompt established.

If the shell prompt is not reached within the configured timeout (default: 30 seconds), the test aborts and logs panic diagnostics.
