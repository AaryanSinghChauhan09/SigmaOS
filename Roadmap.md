# SigmaOS Zenith: LFS-Style Development Roadmap

This roadmap details the progressive stages of building and verifying the sovereign SigmaOS microkernel, drawing inspiration from Linux From Scratch (LFS).

---

## 🚀 Step-by-Step Build Plan (LFS-Style)

### Stage 1: Bootloader + Minimal Kernel (Process & Memory Mgmt)
* [x] **Bootloader**: Multiboot2 compliant bootloader entry code (`boot.asm`) and GRUB menu configs.
* [x] **Process Scheduler**: Deterministic process dispatcher.
* [x] **Memory Management**: Sovereignty allocator for physical frames and virtual memory paging.
* [x] **Device Drivers**: UART Serial (COM1), PS/2 Keyboard, and VGA output drivers.

### Stage 2: Init System + Userland Shell
* [x] **Init Process (PID 1)**: `/init/init.c` script execution sequence and Runlevel mapping.
* [x] **Interactive Shell**: `/usr/sh.c` console parser resolving commands.
* [x] **Core Utilities**: Integrated `ls`, `pwd`, `clear`, and `echo` implementations.

### Stage 3: File System Support
* [x] **VFS Layer**: Virtual File System abstraction (`/fs/vfs.c`) handling files and directories.
* [x] **Ext4 Driver**: Parsing of block structure and inode indices (`/fs/ext4.c`).
* [x] **FAT32 Driver**: Boot Record analysis and cluster chains (`/fs/fat32.c`).

### Stage 4: Networking Stack
* [x] **Loopback Interface**: Virtual loopback driver (`/net/loopback.c`) routing packets.
* [x] **TCP/IP Stack**: TCP 3-way handshake simulation and socket bindings (`/net/tcp_ip.c`).
* [x] **DNS Resolver**: Host name mapping entries (`/net/dns.c`).

### Stage 5: Package & Build System
* [x] **Modular Makefiles**: Root `Makefile` listing separate targets.
* [x] **Junction Links**: Creation of directory symlinks from `userland/` and `networking/` to maintain compatibility.

### Stage 6: Documentation & Validation
* [x] **Module READMEs**: Clean README documents for every layer.
* [x] **Design Specs**: Detailed API specs and syscall mapping references.
* [x] **Test Verification**: 100% green unit test outcomes.

---

## ⚡ Next Steps
- Integrate hardware networking drivers (e.g. Intel e1000) into the `/net/` stack.
- Expand file system write path safety with journaling support.
- Automate complete toolchain bootstrapping for gcc cross-compilation within the build scripts.
