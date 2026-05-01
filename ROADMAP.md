# 🚀 SigmaOS Sovereign Roadmap

> **Current Status: v28.0 Zenith — Kernel Architecture Hardened**
> The OOP singleton (`SovereignEngine`) refactoring is complete. The kernel
> is modular and structurally sound. The following milestones close the gap
> between the current kernel prototype and a fully usable OS distribution.

---

## ✅ v28.0 Zenith — COMPLETE

- [x] 600-shard `SovereignEngine` OOP singleton architecture
- [x] Bare-metal memory allocator (QBMP bump allocator)
- [x] COM1 serial output for kernel debugging
- [x] QEMU boot target (`make qemu`)
- [x] CI/CD pipeline (GitHub Actions: build + `cppcheck`)
- [x] Static analysis hardening (`-Wall -Wextra -Werror`)
- [x] MMU paging, IPC, process scheduler, syscall gate
- [x] Sandbox (CIB), Crypto (HASI), Entropy (QREP) engines

---

## 🔧 v29.0 — Boot & Userland Foundation _(Next Priority)_

These items address the core gap: SigmaOS must boot reliably and run programs.

### 🥾 Bootloader

- [ ] **GRUB2 integration**: Finalize `iso_root/` GRUB config so `make zenith-iso` produces a bootable ISO
- [ ] **Multiboot2 header**: Ensure the kernel binary is recognized by GRUB/QEMU directly
- [ ] **Boot verification**: Confirm serial output appears on `make qemu` with no manual flags

### 🐚 Minimal Shell (`sigma_sh`)

- [ ] **sigma_sh**: A minimal interactive shell reading from keyboard input
  - Commands: `help`, `echo`, `clear`, `halt`, `ls`, `exec`
  - Runs as the first userland process after kernel init
- [ ] **stdio routing**: Hook `sigma_write` → serial + framebuffer so shell output is visible

### 📁 Filesystem

- [ ] **`SovereignVFS` read/write**: Complete `open`, `read`, `write`, `close` syscalls
- [ ] **RAM-disk (initrd)**: Bundle a minimal root filesystem into the ISO for testing
- [ ] **Directory listing**: `ls` support in `sigma_sh`

### 📦 Package System (S-PKG)

- [ ] **`.sab` manifest format**: Define a simple Sovereign App Bundle spec (JSON-like)
- [ ] **`spkg install`**: Shell command to load a `.sab` shard at runtime
- [ ] **App registry**: Maintain a list of installed shards in `SovereignConfigEngine`

---

## 🔒 v30.0 — Security & Stability

### Users & Permissions

- [ ] **Multi-user model**: UID/GID for processes, capability-based access control
- [ ] **`sigma_identity` hardening**: Link `SovereignIdentityEngine` to filesystem permissions
- [ ] **Privilege separation**: Ring 3 userland processes, Ring 0 kernel-only shards

### Security Modules

- [ ] **`SovereignSEL` (Sovereign Enforcement Layer)**: Mandatory Access Control (MAC) policy engine
  - Inspired by SELinux label-based access control
  - Policy loaded at boot from a sovereign manifest
- [ ] **`SovereignCapability`**: Per-process capability bitmask for system call gating
- [ ] **`SovereignAuditEngine` integration**: Log all policy violations to audit ring buffer

### Error Handling & Recovery

- [ ] **Kernel panic screen**: Structured panic output with register dump to serial
- [ ] **`SovereignRecover` hot-swap**: Live shard replacement without full reboot
- [ ] **Watchdog timer**: Hardware timer that triggers recovery if kernel hangs

---

## 🌐 v31.0 — Networking & Ecosystem

### Networking Stack

- [ ] **`SovereignNetStack` TCP/IP**: Complete IPv4 stack (ARP, ICMP, TCP, UDP)
- [ ] **`sigma_sh` networking**: `ping`, `wget` commands in the shell
- [ ] **Firewall shard**: Packet filtering rules managed via `SovereignConfigEngine`
- [ ] **DHCP client**: Auto-IP via NIC driver at boot

### Driver Ecosystem

- [ ] **NIC driver**: Expand beyond stub — implement RTL8139 or VirtIO-net for QEMU
- [ ] **Storage driver**: VirtIO-blk or ATA PIO for disk access in QEMU
- [ ] **USB HID**: Basic keyboard driver via PS/2 fallback for bare-metal testing

---

## 📚 v32.0 — Documentation & Community

- [ ] **Man pages**: Simple built-in help system for `sigma_sh` commands
- [ ] **`HACKING.md`**: Step-by-step guide to writing a new kernel shard
- [ ] **QEMU demo GIF**: Animated terminal capture showing boot → shell → program
- [ ] **Architecture diagram**: Visual SVG of the 600-shard lattice topology
- [ ] **Issue templates**: Bug report and feature request GitHub templates

---

## 🛡️ System Stability Matrix

| Component           | Status           | Verified Via         |
| :------------------ | :--------------- | :------------------- |
| Kernel Boot         | ✅ Boots in QEMU | `make qemu` + serial |
| Memory (QBMP)       | ✅ Stable        | Assertions           |
| MMU / Paging        | ✅ Stable        | Unit stubs           |
| IPC (WFAE)          | ✅ Stable        | Singleton verified   |
| Process Sched       | ✅ Stable (PATS) | Context switch log   |
| Syscall Gate        | ✅ Stable        | C-linkage ABI        |
| Sandbox (CIB)       | ✅ Stable        | Container tests      |
| Shell (`sigma_sh`)  | ⚠️ Planned v29.0 | —                    |
| Filesystem (VFS)    | ⚠️ Stub          | `SovereignVFS`       |
| Networking (TCP/IP) | ⚠️ Stub          | `SovereignNetStack`  |
| Package Manager     | 🔴 Not started   | —                    |
| Multi-user          | 🔴 Not started   | —                    |

---

_Σ SIGMAOS: Honest Engineering. Absolute Trajectory._
