# Core System Roadmap — SigmaOS Foundation

This document outlines the foundational components required to make SigmaOS competitive with established Linux distributions.

---

## Phase 1: Kernel & Drivers

### Current Status
- Custom microkernel architecture (SigmaOS Zenith v15.0)
- Basic hardware support (e1000 NIC, VGA framebuffer)
- Limited driver coverage

### Target State
- **Adopt latest stable Linux kernel (6.8+) as LKMs**
  - Leverage Linux driver ecosystem while maintaining SigmaOS kernel
  - Loadable kernel modules for GPU, Wi-Fi, printer, USB, audio
  - Compatibility layer for Linux syscalls

- **GPU Drivers**
  - NVIDIA (Nouveau + proprietary via LKM)
  - AMD (AMDGPU)
  - Intel (i915)
  - Vulkan/OpenGL acceleration

- **Network Drivers**
  - Intel (e1000e, igb, ixgbe)
  - Realtek (r8169)
  - Broadcom (tg3)
  - Wi-Fi (iwlwifi, ath9k, brcmfmac)

- **Storage Drivers**
  - NVMe (nvme)
  - AHCI SATA (ahci)
  - USB storage (usb-storage)

- **Input Drivers**
  - Keyboard/Mouse (hid-generic, usbhid)
  - Touchpad (synaptics, libinput)
  - Touchscreen (hid-multitouch)

### Implementation Tasks
- [ ] Build Linux kernel module compatibility layer
- [ ] Port 10 most common GPU drivers
- [ ] Port 20 most common network drivers
- [ ] Port storage and input drivers
- [ ] Hardware compatibility testing matrix
- [ ] Driver auto-detection on boot

### Estimated Timeline: 3-4 months

---

## Phase 2: Installer & Bootloader

### Current Status
- Basic installer stub exists
- Manual dual-boot process

### Target State
- **Polished Installer (Calamares-based or custom)**
  - Graphical installer with modern UI
  - Automatic partition detection
  - Dual-boot with Windows/Ubuntu/other
  - VM image creation (VirtualBox, VMware, QEMU)
  - Live USB creation
  - Network installation support

- **Bootloader Options**
  - GRUB2 with SigmaOS branding
  - systemd-boot (UEFI)
  - Secure Boot support
  - Btrfs snapshot boot menu

### Implementation Tasks
- [ ] Design installer UI mockups
- [ ] Implement partition manager
- [ ] Add dual-boot detection and configuration
- [ ] Create VM image builder
- [ ] Secure Boot signing infrastructure
- [ ] Bootloader theme customization

### Estimated Timeline: 2 months

---

## Phase 3: Init System

### Current Status
- Custom init system (minimal)

### Target State
- **Decision: Adopt systemd or Build SigmaOS-native init**
  - Option A: systemd (proven, compatible, complex)
  - Option B: SigmaOS-native init (simpler, sovereign, less compatible)

### Recommended: SigmaOS-Native Init
- Service orchestration with declarative config
- Socket activation
- Dependency management
- Journal logging (with PQC signatures)
- cgroups integration
- DID-based service authentication

### Implementation Tasks
- [ ] Design init system architecture
- [ ] Implement service manager
- [ ] Add socket activation
- [ ] Build logging system with DID signatures
- [ ] Create service migration tools from systemd
- [ ] Documentation for service authors

### Estimated Timeline: 2-3 months

---

## Phase 4: File Systems

### Current Status
- Custom SigmaFS (semantic + time-travel)
- Basic ext4 support
- FAT32 support

### Target State
- **Multi-filesystem support**
  - ext4 (default, stable)
  - Btrfs (snapshots, compression)
  - ZFS (enterprise features)
  - XFS (large files)
  - FAT32/exFAT (compatibility)

- **SigmaFS as overlay**
  - Semantic indexing layer
  - Time-travel versioning
  - Works on top of ext4/Btrfs

### Implementation Tasks
- [ ] Complete Btrfs driver integration
- [ ] Add ZFS on Linux (ZOL) support
- [ ] Implement filesystem selection in installer
- [ ] Add filesystem conversion tools
- [ ] Optimize SigmaFS overlay performance
- [ ] Filesystem repair tools

### Estimated Timeline: 2 months

---

## Phase 5: Hardware Compatibility Testing

### Current Status
- Limited testing on few machines

### Target State
- **Comprehensive hardware database**
  - Tested hardware list (HCL)
  - Automatic hardware detection
  - Driver recommendations
  - Community-contributed reports

### Implementation Tasks
- [ ] Build hardware reporting tool
- [ ] Create HCL database
- [ ] Add hardware detection to installer
- [ ] Community submission portal
- [ ] Automated testing on CI

### Estimated Timeline: 1-2 months

---

## Dependencies

- Package Ecosystem (for driver packages)
- Security (for secure boot, signed drivers)
- User Experience (installer UI)

---

## Success Metrics

- Boot on 95% of consumer hardware (2018+)
- GPU acceleration on 90% of tested GPUs
- Wi-Fi working on 85% of tested adapters
- Installer success rate > 98%
- Dual-boot with Windows 10/11 working
- VM images for VirtualBox, VMware, QEMU

---

## Next Steps

1. Begin Linux kernel module compatibility layer
2. Survey hardware compatibility priorities
3. Design installer UI mockups
4. Decide on init system approach (community vote)

---

## See Also

- [Package Ecosystem Roadmap](Package_Ecosystem.md)
- [Security Roadmap](Security.md)
- [User Experience Roadmap](User_Experience.md)
