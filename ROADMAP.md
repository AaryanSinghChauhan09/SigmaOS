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

- [x] **GRUB2 integration**: Finalize `iso_root/` GRUB config so `make zenith-iso` produces a bootable ISO
- [x] **Multiboot2 header**: Ensure the kernel binary is recognized by GRUB/QEMU directly
- [x] **Boot verification**: Confirm serial output appears on `make qemu` with no manual flags

### 🐚 Minimal Shell (`sigma_sh`)

- [x] **sigma_sh**: A minimal interactive shell reading from keyboard input
  - Commands: `help`, `echo`, `clear`, `halt`, `ls`, `exec`
  - Runs as the first userland process after kernel init
- [x] **stdio routing**: Hook `sigma_write` → serial + framebuffer so shell output is visible

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

- [x] **Multi-user model**: `SovereignMultiUser.cpp` — UID/GID + PQC identity vault
- [x] **`sigma_identity` hardening**: `SovereignIdentity.cpp` linked to filesystem MAC
- [x] **Privilege separation**: Ring 3 userland, Ring 0 kernel enforced by `SovereignSEL`

### Security Modules

- [x] **`SovereignSEL`**: MAC policy engine — zero-trust label-based access control
- [x] **`SovereignCapability`**: Per-process capability bitmask with `DENY ALL` default
- [x] **`SovereignAuditEngine` integration**: Continuous Lattice Auditing ring buffer

### Error Handling & Recovery

- [x] **Kernel panic screen**: `SovereignDiag.cpp` — structured fault localization + serial dump
- [x] **`SovereignHotPatch` hot-swap**: Live shard replacement without full reboot
- [x] **Watchdog timer**: `SovereignWatchdog.cpp` — hardware timer-driven hang recovery

---

## 🌐 v31.0 — Networking & Ecosystem

### Networking Stack

- [x] **`SovereignNetStack` TCP/IP**: Zero-trust IPv4/IPv6 stack with DPI in Ring-0
- [x] **`sigma_sh` networking**: Sovereign custom protocol (SCP) mesh commands
- [x] **Firewall shard**: Packet filtering enforced by `SovereignSEL` MAC engine
- [x] **DHCP client**: SovereignNetStack handles IP auto-configuration

### Driver Ecosystem

- [x] **NIC driver**: `SovereignNICDriver.cpp` — VirtIO-net + RTL8139 PCIe auto-probe
- [x] **Storage driver**: `SovereignStorageDriver.cpp` — VirtIO-blk + ATA PIO LBA
- [x] **USB HID**: `SovereignHWTranspiler.cpp` handles PS/2 + USB HID register mapping

---

## 📚 v32.0 — Documentation & Community

- [x] **Man pages**: `HACKING.md` built-in shard authorship guide
- [x] **`HACKING.md`**: Step-by-step sovereign shard creation guide — complete
- [ ] **QEMU demo GIF**: Animated terminal capture showing boot → shell → program
- [ ] **Architecture diagram**: Visual SVG of the 600-shard lattice topology
- [x] **Issue templates**: Bug report and feature request templates live in `.github/`

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
| Shell (`sigma_sh`)  | ✅ Implemented   | `SovereignShell.cpp` |
| Filesystem (VFS)    | ✅ Implemented   | `SovereignVFS.cpp`   |
| Networking (TCP/IP) | ✅ Implemented   | `SovereignNetStack.cpp` |
| NIC Driver          | ✅ Implemented   | `SovereignNICDriver.cpp` |
| Storage Driver      | ✅ Implemented   | `SovereignStorageDriver.cpp` |
| Package Manager     | ✅ Implemented   | `SovereignPackage.cpp` |
| Multi-user          | ✅ Implemented   | `SovereignMultiUser.cpp` |

---

_Σ SIGMAOS: Honest Engineering. Absolute Trajectory._
