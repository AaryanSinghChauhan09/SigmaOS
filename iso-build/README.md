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

## Current State

The current ISO is a **demonstration/boot stub** that shows:
- SigmaOS boot process via GRUB
- Kernel initialization stub
- Basic root filesystem structure
- Package management infrastructure
- SigmaOS shell with basic commands

### Limitations

- Kernel is a stub (C program placeholder)
- No actual kernel subsystems (memory, process, filesystem, etc.)
- No real package manager (configuration only)
- No GUI or desktop environment
- Limited shell functionality

### Next Steps for Full Implementation

1. **Real Kernel**
   - Integrate actual Linux kernel or custom kernel
   - Implement kernel subsystems
   - Add device drivers

2. **Package Management**
   - Implement actual package manager
   - Set up package repository
   - Add package building infrastructure

3. **Userspace**
   - Add real shell (bash, zsh, or custom)
   - Add core utilities
   - Add system libraries

4. **Desktop Environment**
   - Add display server (Wayland/X11)
   - Add desktop environment (GNOME, KDE, etc.)
   - Add window manager

5. **Integration**
   - Integrate SigmaOS component repositories
   - Add compatibility layers for various distributions
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
