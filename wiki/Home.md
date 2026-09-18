# 🏛️ SigmaOS Master Wiki — Arch & Omarchy Edition

Welcome to the **SigmaOS Documentation Portal**, modeled after the world-class **Arch Linux Wiki Principles** (conciseness, technical accuracy, completeness, user empowerment) and the **Omarchy Linux Desktop Philosophy** (curated, keyboard-driven Wayland tiling experience with agentic AI steering).

SigmaOS is a sovereign, secure, bare-metal desktop operating system built in zero-dependency safe Rust (`#![no_std]`). It combines atomic image-based system updates (`sysupdate`), sub-second Copy-on-Write (CoW) rollbacks, a universal package engine (`sigpkg`), and the Zenith Wayland compositor.

---

## 🎯 The SigmaOS Philosophy

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │                           SOVEREIGN WORKFLOW                           │
  ├────────────────────────────────────────────────────────────────────────┤
  │ boot ➔ install ➔ login ➔ Zenith desktop ➔ sigpkg ➔ update ➔ rollback   │
  └────────────────────────────────────────────────────────────────────────┘
```

1. **Simplicity & Zero-Dependency:** Self-contained architecture built natively in safe Rust. No external C runtime dependencies or bloated scripting bridges.
2. **Modern Wayland Desktop:** Keyboard-driven Zenith Wayland tiling compositor with GPU-accelerated terminal (Ghostty), Fastfetch system diagnostic banner, and Hyprpaper wallpaper management.
3. **Omarchy Agentic Steering:** AI agentic triad steering integrated into desktop workflow:
   - **Bolt ⚡:** Fast-path app launcher and automated task runner (`tdl`, `herdr-run`).
   - **Palette 🎨:** Dynamic color scheme & UI theme manager.
   - **Sentinel 🛡️:** Sandboxing guard and capability permission auditor.
4. **Universal Package Management (`sigpkg`):** Native signed `.sigpkg` format with built-in Arch `PKGBUILD` / `AurHelper` (Yay/Paru) compilation, mirror ranking (`Reflector`), and containerized adapters (`.deb`, `.rpm`, `.apk`, `.xbps`, `.hpkg`, `.txz`).
5. **Transactional State & Sub-Second Rollback:** Dual-root A/B system deployments with Copy-on-Write (CoW) snapshots (<1ms rollback time) and NixOS declarative state generation profile management.

---

## 🗂️ Master Wiki Category Portals

```
   ┌────────────────────────────────────────────────────────────────────────┐
   │                        SIGMAOS DOCUMENTATION                           │
   ├───────────────────┬────────────────────┬───────────────────────────────┤
   │  1. Installation  │  2. Desktop Workflow│  3. Package Management        │
   │  4. System Admin  │  5. Security       │  6. Performance & Kernel      │
   │  7. Rollback      │  8. Hardware       │  9. Developer & API           │
   └───────────────────┴────────────────────┴───────────────────────────────┘
```

---

### 🚀 Portal 1: Installation & Initial Setup
- **[[Installation-Guide]]**: Complete Arch-style installation guide for bare-metal targets, QEMU/KVM, and VirtualBox (`sigpstrap`, disk partitioning, rootfs setup).
- **[[Bootloader-and-Secure-Boot|bootloader]]**: `SigmaBootloaderEngine` setup supporting systemd-boot and GRUB EFI configurations with TPM measured boot.
- **[[Virtual-Filesystem-and-Storage|filesystem]]**: Sovereign VFS layout (`/sovereign/store`), bcachefs/ZFS CoW pools, and volatile tmpfs overlays.

---

### 🎨 Portal 2: Zenith Desktop & Omarchy Workflow
- **[[Zenith-Desktop-and-Omarchy-Workflow]]**: In-depth guide to Zenith Wayland compositor, tiling layout keybindings, and desktop customization.
- **Agentic Steering Triad:**
  - **Bolt ⚡:** Fast-path app execution and automated task orchestration.
  - **Palette 🎨:** Dynamic theme Provider and WCAG 2.1 AAA high-contrast switching.
  - **Sentinel 🛡️:** Exec guard capability permission dialogs and sandboxing.
- **Omakase Workstation Presets (`sigomarchy`):** Curated configurations for Ghostty terminal, Fastfetch sysinfo banner, Neovim, Tmux, and Waybar/Quickshell.

---

### 📦 Portal 3: Universal Package Management (`sigpkg`)
- **[[Package-Management-and-Sigpkg]]**: Package management facade (`sigpkg`), query syntax, dependency resolution (SAT solver), and transaction checkpoints.
- **[[Arch-PKGBUILD-and-AUR-Helper|ARCH_LINUX_PARITY_FEATURES]]**: Native parsing of Arch Linux `PKGBUILD` recipes, Yay/Paru-style `AurHelper`, AST security scanning, and chroot builds.
- **[[Multi-Distro-Adapter-Pipeline|UNIVERSAL_PACKAGE_SYSTEM_IMPLEMENTATION_PLAN]]**: Containerized wrappers (`apx`) translating `.deb`, `.rpm`, `.apk`, `.xbps`, `.hpkg`, and `.txz` foreign packages into native `.sigpkg`.
- **Mirror Ranking & Maintenance:** `ReflectorMirrorRanker` latency sorting, `paccache_clean` cache pruning, and non-locking index updates.

---

### 🎛️ Portal 4: System Administration & Services
- **[[System-Administration-and-Services]]**: Service supervision manifests (`SovereignServiceManifest`), `systemd` / `OpenRC` / `Runit` unit parity, and parallel dependency startup.
- **[[User-Permissions-and-Doas|SECURITY]]**: Privilege delegation via OpenBSD `doas` (`SovereignOpenBsdDoas`), PAM authentication, and user boundaries.
- **[[Network-Configuration-and-WireGuard|networking]]**: Wayland network applet, static/DHCP IP management, WPA3 wireless profiles, and WireGuard VPN tunnels.
- **[[System-Logging-and-Journalctl|syslog]]**: `SovereignJournalLogger` zero-allocation binary logging and `journalctl` query interface.

---

### 🛡️ Portal 5: Security, Sandboxing & Hardening
- **[[Security-Sandboxing-and-Hardening]]**: Multi-layer sandboxing combining OpenBSD `pledge`/`unveil`, Linux Landlock v5, and FreeBSD Capsicum descriptor delegation.
- **Zorin Exec Guard Security:** Default-deny capability permission model intercepting untrusted execution requests.
- **Hardware Enclave Isolation:** AMD SEV-SNP and Intel TDX confidential enclave isolation with constant-time cryptographic zeroization.
- **Post-Quantum Integrity:** Dilithium-5 signatures and Kyber-1024 KEM key exchange for livepatching and package attestations.

---

### ⚡ Portal 6: Performance Tuning & Kernel Architecture
- **[[Performance-Tuning-and-Kernel]]**: CachyOS BORE (Burst-Oriented Response Enhancer) desktop task scheduler for low-latency interactivity.
- **CPU Governors (`CachyOsAutoFreqEngine`):** Dynamic power profile switching (Performance, Balanced, Powersave) and Energy Performance Preference (EPP) tuning.
- **Memory Reclamation (`DemandPagingSwapEngine`):** Multi-Generational LRU (`MGLRU`) page frame reclamation, Copy-on-Write (`CoW`) faulting, KSM deduplication, and zram compressed swap.
- **eBPF & XDP Networking:** Zero-copy XDP packet filtering and lock-free ring buffers (`BPF_MAP_TYPE_RINGBUF`).

---

### 🔄 Portal 7: Maintenance, State & System Recovery
- **[[Maintenance-and-Rollback-Engine]]**: Sub-millisecond Copy-on-Write transaction snapshots and rollbacks (openSUSE Snapper parity).
- **Declarative State Graph (`DeclarativeStateGraph`):** Atomic system generation profiles (NixOS profile parity).
- **ZFS Boot Environments:** FreeBSD `bectl` boot environment selection integrated into bootloader.
- **Offline Updates (`FedoraOfflineUpdateEngine`):** Staged update execution during reboot (`systemd-offline-update` parity).

---

### 🛠️ Portal 8: Hardware Support Matrix
- **[[Hardware-Support-Matrix|SUPPORT_MATRIX]]**: Verified hardware compatibility table for CPUs, GPUs, NVMe storage, WiFi/Bluetooth, touchpads, and monitors.
- **Device Drivers:** `EdidMonitorDdcDisplayDriver`, `PcSpeakerInternalAudioDriver`, `UvcWebcamVideoCameraDriver`, `IntelBtUsbBluetoothDriver`, `NvmePCIeHostControllerDriver`.

---

### ⌨️ Quick CLI Cheatsheet

| Command | Subsystem | Action Description |
|---------|-----------|--------------------|
| `sigpkg install <pkg>` | Package Manager | Resolves dependencies and installs package |
| `sigpkg aur build <pkg>` | AUR Helper | Downloads, audits PKGBUILD diff, and compiles package in chroot |
| `sigpkg rankmirrors` | Mirror Manager | Benchmarks and ranks package mirror latency |
| `sigpkg rollback <id>` | Transaction Engine | Reverts system state to snapshot `<id>` in < 1ms |
| `sigomarchy preset apply <name>` | Workstation Manager | Applies Omakase desktop config preset (Ghostty, Neovim, Waybar) |
| `doas <cmd>` | Privilege Escalation | Executes command with delegated capabilities per rule |
| `sysctl <key>=<val>` | Kernel Tuner | Queries or modifies kernel MIB configuration tree |
| `journalctl -u <service>` | System Logger | Filters binary journal logs by service unit |

---

*Documentation maintained according to Arch Linux Wiki standards and Omarchy Linux desktop principles. Verified by `./run_sigma_tests.sh`.*
