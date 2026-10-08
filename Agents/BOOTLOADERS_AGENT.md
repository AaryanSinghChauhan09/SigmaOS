# Bootloaders Component Agent

## Component Overview
Bootloaders are critical for OS initialization, handling hardware handoff from firmware (BIOS/UEFI) to the kernel.

## Linux Inspiration
- **GRUB 2**: Grand Unified Bootloader with multiboot support, menu system, and module loading
- **systemd-boot**: Simple UEFI bootloader with auto-detection and Btrfs support
- **Limine**: Modern bootloader with multiboot2, long mode, and cross-architecture support

## BSD Inspiration
- **FreeBSD boot0/boot1/boot2**: Three-stage boot process with MBR and GPT support
- **OpenBSD boot**: Simple, secure bootloader with strict validation
- **NetBSD boot**: Cross-architecture bootloader with excellent hardware support

## Current SigmaOS Status
- Partial implementation in `src/boot/` directory
- Multiboot2 and Limine protocol headers defined
- UEFI GOP framebuffer support partially implemented
- Missing: Complete bootloader binary, menu system, and secure boot integration

## Critical Missing Features
1. **Secure Boot Integration**: PK/KEK/db signature verification
2. **Boot Menu System**: Interactive boot parameter selection
3. **Fallback Boot Mechanism**: Automatic recovery on boot failure
4. **Initramfs Loading**: Early userspace initialization
5. **Kernel Module Preloading**: Load critical drivers before root mount
6. **Btrfs/ZFS Boot Support**: Boot from advanced filesystems
7. **Network Booting**: PXE/iPXE support for diskless systems
8. **UEFI HTTP Boot**: Boot from network via UEFI HTTPBoot protocol

## Implementation Priority
1. **HIGH**: Secure Boot verification and UEFI HTTP Boot
2. **HIGH**: Boot menu with parameter editing
3. **MEDIUM**: Initramfs with cryptsetup support
4. **MEDIUM**: Btrfs/ZFS boot with snapshot selection
5. **LOW**: Network booting (PXE/iPXE)

## Key Files to Create/Improve
- `src/boot/secure_boot.rs` - PK/KEK/db verification
- `src/boot/menu.rs` - Interactive boot menu
- `src/boot/initramfs.rs` - CPIO archive loader
- `src/boot/uefi_http_boot.rs` - UEFI HTTPBoot protocol
- `src/boot/fallback.rs` - Automatic recovery mechanism

## Testing Strategy
- QEMU UEFI boot testing with OVMF
- Physical hardware UEFI boot testing
- Secure boot signing and verification testing
- Initramfs extraction and execution testing

## Dependencies
- UEFI protocol bindings
- Cryptographic library (Ed25519/RSA/SHA256)
- Filesystem drivers (FAT32, ext4, Btrfs, ZFS)
- Network stack (for HTTP boot)

## Success Criteria
- Boot from USB with secure boot enabled
- Interactive boot menu with parameter editing
- Initramfs loading and execution
- Automatic fallback on boot failure
- UEFI HTTP boot functionality

## Open Source Competitors Analysis
- **GRUB 2**: Most feature-rich but complex codebase
- **systemd-boot**: Simple but limited menu system
- **Limine**: Modern, clean architecture with good cross-arch support
- **rEFInd**: Graphical boot menu with auto-detection

## Future Enhancements
- Graphical boot menu with themes
- Boot performance profiling
- Boot splash screen with Plymouth
- Measured boot with TPM
- Hypervisor boot (KVM/Xen)
