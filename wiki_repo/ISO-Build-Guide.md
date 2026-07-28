# SigmaOS ISO Build Guide

This guide explains how to build a ready-to-launch SigmaOS ISO file, inspired by Arch Linux ISO structure.

## Overview

SigmaOS ISO build infrastructure follows Arch Linux conventions with:
- Enhanced GRUB bootloader configuration with multiple boot options
- EFI support for modern systems
- ISO metadata files for identification
- Cross-platform build scripts (Linux and Windows)

## Prerequisites

### Linux
- `grub-mkrescue` (preferred) or `xorriso`
- `cargo` and `rustc` for building the kernel
- `bash`

### Windows
- Windows PowerShell
- Either of the following for ISO creation:
  - Windows ADK (for `oscdimg`)
  - `mkisofs` (from Cygwin, Git Bash, or WSL)
  - `xorriso` via WSL
- `cargo` and `rustc` for building the kernel

## ISO Structure

```
iso_root/
├── boot/
│   ├── grub/
│   │   └── grub.cfg          # GRUB configuration (BIOS)
│   └── sigmaos.bin           # Kernel binary
├── EFI/
│   └── BOOT/
│       └── grub.cfg          # GRUB configuration (EFI)
├── installer/
│   └── install.sh            # Installation script
├── .disk/
│   └── info                  # Disk information
├── VERSION                   # Version information
└── README.md                 # ISO documentation
```

## Building the ISO

### Linux

Run the build script:

```bash
./scripts/build-iso.sh
```

The script will:
1. Build the kernel if not already built
2. Copy the kernel binary to the ISO root
3. Generate GRUB configurations
4. Create the ISO with checksums
5. Output: `build/sigmaos-29.0-x86_64.iso`

### Windows

Run the PowerShell build script:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\build-iso.ps1
```

The script will:
1. Build the kernel if not already built
2. Copy the kernel binary to the ISO root
3. Generate GRUB configurations
4. Create the ISO with checksums (using available tools)
5. Output: `build/sigmaos-29.0-x86_64.iso`

## Boot Options

The ISO includes multiple boot options:

- **SigmaOS v29.0 Zenith Foundation (Boot)** - Standard boot mode
- **SigmaOS v29.0 Zenith Foundation (Safe Mode)** - Single user mode for troubleshooting
- **SigmaOS v29.0 Zenith Foundation (Verbose)** - Boot with verbose logging
- **SigmaOS v29.0 Zenith Foundation (Debug)** - Boot with debug enabled
- **System Information** - Display system details
- **Reboot System** - Reboot the machine
- **Power Off System** - Power off the machine

## Customization

### Changing Version

Edit the `ISO_VERSION` variable in the build script:

```bash
# scripts/build-iso.sh
ISO_VERSION="29.0"
```

```powershell
# scripts/build-iso.ps1
$ISO_VERSION = "29.0"
```

### Modifying GRUB Configuration

Edit the GRUB configuration files:

- BIOS: `iso_root/boot/grub/grub.cfg`
- EFI: `iso_root/EFI/BOOT/grub.cfg`

### Adding Kernel Parameters

Add kernel parameters in the GRUB menuentry:

```grub
menuentry "SigmaOS v29.0 Zenith Foundation (Custom)" {
    multiboot2 /boot/sigmaos.bin custom_param=value
    boot
}
```

## Verification

After building, verify the ISO checksums:

```bash
# Linux
sha256sum -c build/sigmaos-29.0-x86_64.iso.sha256
md5sum -c build/sigmaos-29.0-x86_64.iso.md5
```

```powershell
# Windows
Get-FileHash build\sigmaos-29.0-x86_64.iso -Algorithm SHA256
Get-FileHash build\sigmaos-29.0-x86_64.iso -Algorithm MD5
```

## Testing the ISO

### QEMU (Linux)

```bash
qemu-system-x86_64 -cdrom build/sigmaos-29.0-x86_64.iso -m 2G
```

### QEMU (Windows)

```powershell
qemu-system-x86_64.exe -cdrom build\sigmaos-29.0-x86_64.iso -m 2G
```

### VirtualBox

1. Create a new VM
2. Mount the ISO as the optical drive
3. Start the VM

### Physical Hardware

Burn the ISO to a USB drive or DVD:

```bash
# Linux
dd if=build/sigmaos-29.0-x86_64.iso of=/dev/sdX bs=4M status=progress
```

```powershell
# Windows (using Rufus or similar tool)
# Use Rufus to write the ISO to a USB drive
```

## Troubleshooting

### Build Fails - Missing Tools

If ISO creation tools are not found, the script will create a simulated ISO container. To create a bootable ISO:

**Linux:**
```bash
# Debian/Ubuntu
sudo apt-get install grub-pc-bin xorriso

# Arch Linux
sudo pacman -S grub xorriso
```

**Windows:**
- Install Windows ADK for `oscdimg`
- Or install Git Bash for `mkisofs`
- Or install WSL and `xorriso`

### Kernel Build Fails

If the kernel build fails:

```bash
# Clean build artifacts
cargo clean

# Rebuild
cargo build --release --bin sigma_kernel
```

### GRUB Configuration Issues

Ensure GRUB configuration files are properly formatted:

```bash
# Check syntax
grub-script-check iso_root/boot/grub/grub.cfg
```

## Architecture Support

The current ISO build is configured for x86_64. To build for other architectures:

1. Set the `ARCH` variable in the Makefile
2. Ensure cross-compilation tools are installed
3. Build the kernel for the target architecture
4. Update GRUB configuration if needed

## Release Process

To create an official release:

1. Update version numbers in build scripts
2. Update VERSION file
3. Build the ISO
4. Verify checksums
5. Test in QEMU and physical hardware
6. Upload to release assets
7. Update GitHub release notes

## Additional Resources

- [SigmaOS Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)
- [Installation Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Installation)
- [Building from Source](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Building-from-Source)

## Contributing

To improve the ISO build process:

1. Test on different platforms
2. Add support for additional architectures
3. Improve GRUB configuration
4. Add automated testing
5. Submit pull requests

---
Σ SigmaOS - Sovereign, AI-Native Operating System
