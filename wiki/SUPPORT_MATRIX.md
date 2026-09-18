# SigmaOS Supported Hardware Matrix

## Tier 1: Primary Release Targets
- **Target Platform**: QEMU / KVM x86_64 Virtual Machine (UEFI Boot, VirtIO Display, VirtIO Net, VirtIO Block, VirtIO Input).
- **CPU Architectures**: x86-64-v3, x86-64-v4 (AVX-512, BMI2 instruction sets).
- **RAM Minimum**: 2048 MB.
- **Storage**: 10 GB NVMe / VirtIO SCSI.

## Tier 2: Physical Reference Hardware
- **Graphics**: Intel Integrated Graphics (HD / UHD / Iris Xe), AMD Radeon Integrated (RDNA2/RDNA3).
- **Network**: Realtek PCIe Gigabit Ethernet, Intel Wi-Fi 6 (AX200 / AX201).
- **Input**: Standard USB HID Keyboard & Mouse, Precision Touchpad.

## Tier 3: Long-Term Research Platforms
- **ARM64**: Apple Silicon (QEMU virt), Raspberry Pi 4/5.
- **RISC-V**: QEMU virt RISC-V 64.
