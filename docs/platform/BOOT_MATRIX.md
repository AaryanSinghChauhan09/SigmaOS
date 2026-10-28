# SigmaOS Platform Boot Matrix

## Overview
The Boot Matrix records boot-to-shell validation results across target architectures, virtualized hypervisors, and bootloaders.

---

## Boot Matrix

| Platform / Hypervisor | Architecture | Bootloader | Result | Last Verified | Evidence Log |
|---|---|---|---|---|---|
| QEMU System x86_64 | x86_64 | Limine 8.x | PASS | Monthly CI | `reports/boot/qemu_x86_64.log` |
| QEMU KVM x86_64 | x86_64 | GRUB2 | PASS | Monthly CI | `reports/boot/qemu_kvm.log` |
| QEMU System AArch64 | aarch64 | EDK2 / UEFI | PASS | Monthly CI | `reports/boot/qemu_aarch64.log` |
| QEMU System RISC-V 64 | riscv64 | OpenSBI | PASS | Monthly CI | `reports/boot/qemu_riscv64.log` |
