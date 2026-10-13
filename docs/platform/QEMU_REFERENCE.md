# QEMU Reference and Boot Parameters

## Recommended QEMU Command Line
To run SigmaOS in QEMU for smoke testing and interactive debugging:

```bash
qemu-system-x86_64 \
  -M q35 \
  -cpu qemu64 \
  -m 2048M \
  -kernel build/kernel.bin \
  -initrd build/initramfs.img \
  -append "console=ttyS0 quiet" \
  -serial stdio \
  -display none \
  -no-reboot
```

## CI Integration
In GitHub Actions and local CI (`.github/workflows/boot-smoke.yml`), QEMU is executed headless via `scripts/ci/qemu_boot_smoke.sh`.
The boot harness captures stdout/stderr and verifies boot progression using regex markers.
