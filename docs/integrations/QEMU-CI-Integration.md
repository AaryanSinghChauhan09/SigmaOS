# QEMU as CI Device Model for Driver Testing

## Overview

SigmaOS uses QEMU as the primary CI device model for testing drivers, boot sequences, and system smoke tests. The CI matrix covers three architectures (x86_64, ARM64, RISC-V), two firmware types (UEFI, BIOS/U-Boot), and two acceleration modes (KVM, no-KVM/TCG).

---

## Machine Types

| Architecture | QEMU machine | Firmware |
|---|---|---|
| x86_64 | `q35` | OVMF UEFI or SeaBIOS |
| ARM64 | `virt` | EDK2 UEFI (qemu-efi-aarch64) |
| RISC-V 64 | `virt` | OpenSBI + U-Boot |

---

## CI Matrix: 3 Arches × 2 Firmware × (KVM / no-KVM) = up to 12 jobs

GitHub Actions does not have native KVM on all runners, so we use 6 jobs:

| Job | Arch | Firmware | Accel |
|---|---|---|---|
| x86_64-uefi-kvm | x86_64 | OVMF | KVM (self-hosted) |
| x86_64-uefi-tcg | x86_64 | OVMF | TCG (hosted) |
| x86_64-bios-tcg | x86_64 | SeaBIOS | TCG |
| arm64-uefi-tcg | ARM64 | EDK2 | TCG |
| riscv64-uboot-tcg | RISC-V 64 | U-Boot | TCG |
| arm64-uefi-kvm | ARM64 | EDK2 | KVM (self-hosted) |

---

## sigma_qemu.yml (Complete Workflow)

```yaml
name: QEMU CI Matrix

on:
  push:
    branches: [main]
  pull_request:

jobs:
  qemu-matrix:
    name: "${{ matrix.arch }}-${{ matrix.firmware }}-${{ matrix.accel }}"
    runs-on: "${{ matrix.runner }}"

    strategy:
      fail-fast: false
      matrix:
        include:
          - arch: x86_64
            firmware: uefi
            accel: kvm
            runner: self-hosted-kvm
            qemu_machine: q35,accel=kvm
            bios_args: "-bios /usr/share/OVMF/OVMF_CODE.fd"
          - arch: x86_64
            firmware: uefi
            accel: tcg
            runner: ubuntu-22.04
            qemu_machine: q35,accel=tcg
            bios_args: "-bios /usr/share/OVMF/OVMF_CODE.fd"
          - arch: x86_64
            firmware: bios
            accel: tcg
            runner: ubuntu-22.04
            qemu_machine: q35,accel=tcg
            bios_args: ""
          - arch: arm64
            firmware: uefi
            accel: tcg
            runner: ubuntu-22.04
            qemu_machine: virt,accel=tcg
            bios_args: "-bios /usr/share/qemu-efi-aarch64/QEMU_EFI.fd"
          - arch: riscv64
            firmware: uboot
            accel: tcg
            runner: ubuntu-22.04
            qemu_machine: virt,accel=tcg
            bios_args: "-bios /usr/lib/riscv64-linux-gnu/opensbi/generic/fw_jump.bin"
          - arch: arm64
            firmware: uefi
            accel: kvm
            runner: self-hosted-arm64-kvm
            qemu_machine: virt,accel=kvm
            bios_args: "-bios /usr/share/qemu-efi-aarch64/QEMU_EFI.fd"

    steps:
      - uses: actions/checkout@v4

      - name: Install QEMU
        run: |
          sudo apt-get update -q
          sudo apt-get install -y \
            qemu-system-x86 qemu-system-arm qemu-system-misc \
            ovmf qemu-efi-aarch64

      - name: Build kernel (${{ matrix.arch }})
        run: |
          make ARCH=${{ matrix.arch }} sigma-kernel.elf

      - name: Boot smoke test
        run: |
          timeout 120 qemu-system-${{ matrix.arch }} \
            -machine ${{ matrix.qemu_machine }} \
            -m 512M \
            -nographic \
            -serial stdio \
            -device virtio-net-pci,netdev=net0 \
            -netdev user,id=net0 \
            -device virtio-blk-pci,drive=disk0 \
            -drive file=dist/sigma-rootfs.img,if=none,id=disk0,format=raw \
            -device virtio-rng-pci \
            ${{ matrix.bios_args }} \
            -kernel dist/sigma-kernel-${{ matrix.arch }}.elf \
            -append "console=ttyS0 sigma.test=smoke" \
          | tee /tmp/boot-${{ matrix.arch }}.log

      - name: Assert boot success
        run: |
          grep -q "sigma-init: boot complete" /tmp/boot-${{ matrix.arch }}.log
          echo "Boot smoke test passed for ${{ matrix.arch }}-${{ matrix.firmware }}-${{ matrix.accel }}"

      - name: Driver smoke tests (virtio)
        run: |
          grep -q "virtio-net: initialized" /tmp/boot-${{ matrix.arch }}.log
          grep -q "virtio-blk: initialized" /tmp/boot-${{ matrix.arch }}.log
          echo "virtio driver tests passed"

      - name: Upload boot log
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: boot-log-${{ matrix.arch }}-${{ matrix.firmware }}
          path: /tmp/boot-${{ matrix.arch }}.log
```

---

## virtio Device Smoke Tests

The CI kernel boot appends `sigma.test=smoke` which triggers built-in driver smoke tests:

| Device | Test |
|---|---|
| virtio-net | Sends a single ICMP echo to 10.0.2.2 (QEMU gateway) |
| virtio-blk | Reads first 512 bytes of disk0, checks MBR magic |
| virtio-rng | Reads 32 bytes, verifies entropy > 0 |
| virtio-gpu | Writes a test pattern to framebuffer, reads back |

---

## Exit Criteria

- All 6 matrix jobs green on every PR.
- Boot time (kernel start → `sigma-init: boot complete`) < 5s in TCG mode.
- virtio-net, virtio-blk, virtio-rng driver smoke tests pass in all jobs.
