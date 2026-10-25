# QEMU Boot Reference & CI Artifact Specifications

## QEMU Execution Commands

### x86_64 Direct ISO Boot
```bash
qemu-system-x86_64 \
    -m 2048M \
    -smp 2 \
    -machine pc \
    -cdrom build/sigmaos.iso \
    -serial stdio \
    -display none \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04
```

### x86_64 Standalone Bootloader Direct Kernel Entry
```bash
qemu-system-x86_64 \
    -m 1024M \
    -smp 1 \
    -kernel boot/vmlinuz-sigma \
    -initrd boot/initramfs.img \
    -append "console=ttyS0 quiet" \
    -serial file:boot_output.log \
    -display none
```

---

## Smoke Test Verification Flow

1. **Invocation**: `scripts/ci/qemu_boot_smoke.sh` launches QEMU with a timeout of 120 seconds.
2. **Output Stream**: Console serial stream is redirected to `boot_output.log`.
3. **Regex Marker Search**: `tests/boot/boot_to_shell_test.py` scans `boot_output.log` for deterministic markers:
   - Kernel Init: `[KERNEL] Initializing memory and scheduler...`
   - PID 1 Init: `[INIT] Starting interactive shell...`
   - Shell Marker: `SIGMAOS BOOT SUCCESS - SHELL READY`

4. **Exit Status**:
   - Exit Code `0`: Marker found within timeout.
   - Exit Code `1`: Timeout reached or crash panic logged (`[PANIC]`).
