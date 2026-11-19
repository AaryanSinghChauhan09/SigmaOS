# SigmaOS ISO Build and Testing

## Overview

This directory contains the SigmaOS ISO build infrastructure and the generated bootable ISO image.

## Building the ISO

The ISO is built using the `build.sh` script:

```bash
./build.sh
```

This script:
- Checks for required dependencies (genisoimage, xorriso, mksquashfs)
- Creates the directory structure
- Compiles the kernel stub from C source
- Generates initramfs
- Creates the root filesystem with package management
- Builds the bootable ISO image

## ISO Contents

The generated ISO (`output/sigmaos.iso`) includes:

### Boot Configuration
- GRUB configuration with 3 boot options:
  - SigmaOS (normal boot)
  - SigmaOS (Recovery Mode)
  - SigmaOS (Verbose)

### Kernel
- Compiled C kernel stub (`kernel-stub.c`)
- Placeholder for actual kernel implementation

### Initramfs
- Basic init script
- Mounts /proc, /sys, /dev
- Provides minimal userspace

### Root Filesystem (SquashFS)
- Full FHS directory structure
- SigmaOS identity files (`/etc/os-release`, `/etc/hostname`)
- Package manager configuration (`/etc/sigmaos/pacman.conf`)
- Package mirror configuration
- Package database structure
- SigmaOS shell (`/bin/sigmaos-shell`)

## Testing the ISO

### Prerequisites

You need one of the following virtualization platforms:

1. **QEMU/KVM** (recommended)
   ```bash
   sudo apt-get install qemu-system-x86 qemu-kvm libvirt-daemon-system libvirt-clients bridge-utils virt-manager
   ```

2. **VirtualBox**
   ```bash
   sudo apt-get install virtualbox
   ```

3. **VMware Workstation/Player**
   - Download from VMware website

### Running in QEMU

```bash
qemu-system-x86_64 \
  -cdrom output/sigmaos.iso \
  -m 2048 \
  -boot d \
  -enable-kvm
```

### Running in VirtualBox

1. Create a new VM
2. Set OS type: Linux, Other Linux (64-bit)
3. Memory: 2048 MB (minimum)
4. Storage: Use the ISO as optical drive
5. Start the VM

### Running in VMware

1. Create a new VM
2. Select "Linux" as guest OS
3. Mount the ISO as CD/DVD drive
4. Start the VM

## Architecture Tracks

SigmaOS development is organized into two distinct, deliberate tracks:

### Track A: Sovereign Arch-based Desktop Live ISO (Primary Distro Target)
- **Profile Location**: `iso/archiso/`
- **Build Script**: `scripts/build_archiso.sh`
- **Kernel & Drivers**: Production Linux LTS / Zenith kernel with full hardware graphics & networking modules.
- **Installer**: Defensive sovereign installer (`iso_root/installer/install.sh` and `/usr/local/bin/sigmaos-install`) featuring default `DRY_RUN=1` simulation, active root-disk protection, and multi-flag authentication (`--destructive --confirm-disk <DEV>`).
- **Desktop Environment**: Wayland compositor (Hyprland), Waybar status line, native settings daemon, and GTK3 application suite.
- **Package Management**: Arch ALPM / pacman binary format orchestrated by `sigpkg` and `SigmaOS-packages-kit`.

### Track B: Standalone Sovereign Microkernel Research (Experimental Long-Term Target)
- **Source**: `iso-build/kernel-stub.c`, `src/kernel/`
- **Build Script**: `iso-build/build.sh`
- **Goal**: Long-term clean-slate `#![no_std]` Rust microkernel research. Kept isolated from the daily-driver desktop distribution to ensure immediate installability and user stability.

## Safe Testing in QEMU Virtual Machines

Always test installation using disposable virtual disks rather than host bare metal:

```bash
# 1. Create a 32GB disposable virtual disk
qemu-img create -f qcow2 /tmp/sigmaos_test_disk.qcow2 32G

# 2. Boot ISO with disposable drive attached
qemu-system-x86_64 \
  -m 4096 \
  -enable-kvm \
  -drive file=/tmp/sigmaos_test_disk.qcow2,format=qcow2,if=virtio \
  -cdrom out/sigmaos-desktop.iso \
  -boot d

# 3. Inside the live VM, test dry-run installer:
sigmaos-install --dry-run /dev/vda

# 4. Destructive installation is executed only within the virtual machine:
sigmaos-install --destructive --confirm-disk /dev/vda /dev/vda
```
   - Add proper boot configuration

## Component Repositories

SigmaOS has 267+ component repositories covering:
- 20 operating system kernels
- 17 distributions
- 12 init systems
- 5 package managers
- 6 shells
- 2 display servers
- And many more categories

See `COMPONENTS.toml` in the main SigmaOS directory for the complete list.

## Architecture Principles

SigmaOS follows these principles:
- Modular design with separate repositories
- Clear abstraction and encapsulation
- Single responsibility
- Secure defaults
- Comprehensive logging
- Memory safety (Rust-based components)
- No stub code - actual working implementations
- Compatibility with various open source operating systems

## Contributing

To contribute to SigmaOS:
1. Fork the repository
2. Create a branch for your component
3. Implement the component following SigmaOS principles
4. Add comprehensive tests
5. Update COMPONENTS.toml
6. Submit a pull request

## License

MIT License - see LICENSE file in main SigmaOS directory

## Contact

- GitHub: https://github.com/AaryanSinghChauhan09/SigmaOS
- Issues: https://github.com/AaryanSinghChauhan09/SigmaOS/issues
- Wiki: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki
