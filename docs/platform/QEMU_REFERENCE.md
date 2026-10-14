# QEMU Reference Guide & CI Runner Configuration

## Running QEMU Locally

### Basic x86_64 Serial Console Boot
```bash
qemu-system-x86_64 \
    -m 2048M \
    -nographic \
    -kernel iso_root/boot/vmlinuz \
    -initrd iso_root/boot/initrd.img \
    -append "console=ttyS0 quiet sigmaos.boot_to_shell=1"
```

### Display / Graphic Window Execution
```bash
qemu-system-x86_64 \
    -m 4096M \
    -vga std \
    -cdrom build/sigmaos-x86_64.iso
```

---

## CI Smoke Test Runner

CI execution is driven by `scripts/ci/qemu_boot_smoke.sh` and validated using `tests/boot/boot_to_shell_test.py`.

### Artifact Captures
On completion or timeout, CI saves the following diagnostic artifacts:
- `reports/boot/qemu_x86_64.log`: Full serial console output.
- `reports/boot/qemu_x86_64.json`: Boot timing and diagnostic event report.
