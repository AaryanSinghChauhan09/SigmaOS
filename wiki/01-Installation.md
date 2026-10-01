# Installation

SigmaOS can be installed on bare metal hardware or in virtual machines.

## System Requirements

### Minimum Requirements
- CPU: x86_64 (64-bit) or ARM64
- RAM: 2 GB minimum
- Storage: 20 GB minimum
- Boot: UEFI with Secure Boot support (optional)

### Recommended Requirements
- CPU: Multi-core x86_64 with x86-64-v2 extensions
- RAM: 8 GB or more
- Storage: 64 GB NVMe SSD
- Graphics: GPU with KMS/DRM support

## Installation Methods

### 1. Bare Metal Installation

Download the SigmaOS ISO and boot from USB:

```bash
# Download ISO
wget https://github.com/AaryanSinghChauhan09/SigmaOS/releases/latest/sigmaos.iso

# Write to USB (replace /dev/sdX with your USB device)
sudo dd if=sigmaos.iso of=/dev/sdX bs=4M status=progress && sync
```

Boot from USB and follow the graphical installer.

### 2. Virtual Machine Installation

#### QEMU

```bash
qemu-system-x86_64 \
  -m 4096 \
  -smp 2 \
  -drive file=sigmaos.qcow2,format=qcow2 \
  -enable-kvm \
  -net nic,model=virtio \
  -net user \
  -display gtk
```

#### VirtualBox

Create a new VM with:
- OS Type: Linux, Other 64-bit
- Memory: 4096 MB
- Storage: 64 GB
- Graphics: VMSVGA 3D
- Network: NAT or Bridged Adapter

### 3. Dual Boot Installation

SigmaOS can coexist with other operating systems. The installer includes:

- Automatic partition detection
- LUKS encryption support
- BitLocker conflict detection
- Limine multi-boot scanner

## Installation Steps

1. Boot from the installation media
2. Select language and keyboard layout
3. Partition disk (automatic or manual)
4. Configure encryption (optional but recommended)
5. Select desktop environment (Zenith)
6. Create user account
7. Install system
8. Reboot

## Post-Installation

After installation, update the system:

```bash
sigpkg update
sigpkg upgrade
```

See [Getting Started](02-Getting-Started.md) for first-time configuration.

## AI Agent Maintenance Instructions

- **Bolt ⚡**: Ensure documentation of any new zero-allocation optimizations or performance improvements are added concisely without marketing fluff.
- **Palette 🎨**: Maintain Arch Linux wiki style: clear, factual, one page per topic, using appropriate markdown formatting and tables where necessary.
- **Sentinel 🛡️**: Verify that no hardcoded credentials or unvetted cryptographic algorithms are documented as production-ready. Ensure security limitations are accurately stated.
- **General**: Keep pages up-to-date with current repository capabilities. Remove redundant files when consolidating information.
