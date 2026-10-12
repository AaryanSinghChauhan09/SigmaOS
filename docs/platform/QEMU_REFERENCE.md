# QEMU Reference and Execution Specification

## Environment Requirements

To run SigmaOS within QEMU for boot-to-shell validation and integration testing:

- **QEMU Binary**: `qemu-system-x86_64` (v7.0+)
- **Memory**: Minimum 512 MB RAM (`-m 512M` or `-m 2G` for desktop tests)
- **CPU**: Standard x86_64 or host passthrough (`-cpu QEMU64` or `-cpu host`)
- **Acceleration**: KVM enabled where available (`-enable-kvm` or `-machine accel=kvm:tcg`)

---

## Recommended Launch Command

```bash
qemu-system-x86_64 \
    -kernel target/x86_64-unknown-none/release/sigmaos_kernel \
    -m 1G \
    -smp 2 \
    -serial stdio \
    -display none \
    -device virtio-net-pci,netdev=net0 \
    -netdev user,id=net0 \
    -device virtio-blk-pci,drive=hd0 \
    -drive file=build/sigmaos_disk.img,format=raw,id=hd0,if=none \
    -no-reboot
```

---

## Boot Smoke Test Validation

The automated boot smoke test harness (`scripts/ci/qemu_boot_smoke.sh`) executes QEMU in headless console mode and asserts:

1. Kernel initialization messages appear on the serial console.
2. Subsystem readiness markers (`[OK] Scheduler`, `[OK] Paging`, `[OK] VFS`) are logged.
3. Interactive shell prompt (`sigma# `) or diagnostic prompt is reached within 30 seconds.
