# 🏛️ SigmaOS Master Wiki Portal (Arch Linux Standards)

Welcome to the **SigmaOS Documentation Portal**, modeled after the **Arch Linux Wiki Principles**: conciseness, technical accuracy, completeness, user empowerment, and timeliness.

SigmaOS is a sovereign, secure, bare-metal operating system built in zero-dependency safe Rust (`#![no_std]`), incorporating the strengths of Linux and BSD distributions.

---

## 🎯 The SigmaOS Philosophy

Inspired by the *Arch Way*, SigmaOS is guided by five core principles:

1. **Simplicity:** Self-contained, zero-external-dependency architecture. Every kernel shard, driver, and userland utility is native Rust.
2. **Modernity:** Next-generation bare-metal kernel primitives: Wayland wire protocol compositor, eBPF XDP networking, CachyOS BORE scheduler, and MGLRU memory management.
3. **Pragmatism:** Universal Linux & BSD distro compatibility. Seamlessly parse, adapt, and run packages from Debian (`.deb`), Arch Linux (`PKGBUILD` / `.pkg.tar.zst`), Fedora (`.rpm`), Alpine (`.apk`), Void (`.xbps`), FreeBSD (`.txz`), and Flatpak/Snap containers.
4. **User-Centricity:** Complete system transparency. Users control privilege delegation, sandboxing, system generations, and hardware Governors without opaque obfuscation.
5. **Versatility & Reliability:** Rolling-release agility combined with sub-millisecond atomic transactional state rollbacks (Snapper CoW, NixOS generations, FreeBSD ZFS boot environments).

---

## 🗂️ Master Category Index

```
   ┌────────────────────────────────────────────────────────────────────────┐
   │                       SIGMAOS WIKI PORTALS                             │
   ├───────────────────┬────────────────────┬───────────────────────────────┤
   │  1. Installation  │  2. System Admin   │  3. Package Management        │
   │  4. Security      │  5. Performance    │  6. Compositor & Desktop      │
   │  7. Maintenance   │  8. Hardware       │  9. Developer & Kernel        │
   └───────────────────┴────────────────────┴───────────────────────────────┘
```

---

## 🛠️ Category 1: Installation & Initial Setup

- **[Installation Guide](INSTALL.md):** Step-by-step installation instructions for bare-metal, QEMU, KVM, and VirtualBox targets.
- **[Bootloader Setup](bootloader.md):** `SigmaBootloaderEngine` setup supporting systemd-boot and GRUB EFI/BIOS bootloader configurations with TPM `PCR_4` measured boot.
- **[Virtual Filesystem Formatting](filesystem.md):** VFS initialization, rootfs layout (`/sovereign/store`), and tmpfs volatile overlays.
- **[Headless & Scripted Provisioning](HEADLESS_PROVISIONING.md):** Headless automated deployments using Kickstart and JSON specification manifests (`FedoraButaneSpec` / `FedoraIgnitionConfig`).

---

## ⚙️ Category 2: System Administration & Services

- **[Init System & Service Supervision](systemd-parity.md):** Declarative service manifests (`SovereignServiceManifest`), `systemd` / `OpenRC` unit parity, parallel dependency ordering, and orphan process reparenting.
- **[Network Configuration](networking.md):** Interface binding, static/DHCP IP assignment, WPA2/WPA3 wireless profiles (`LxqtNetworkManagerTray`), and WireGuard VPN tunnels.
- **[User & Privilege Management](USER_MANAGEMENT.md):** POSIX username boundaries, UID/GID mapping, OpenBSD `doas` capability delegation (`SovereignOpenBsdDoas`), and PAM authentication modules.
- **[System Logging & Observability](syslog.md):** `SovereignJournalLogger` zero-allocation structured binary logging and `journalctl` query shims.

---

## 📦 Category 3: Package & AUR Management

- **[Package Management Architecture](PACKAGE_MANAGEMENT.md):** `sigpkg` universal package manager facade (`UniversalPackageManager`).
- **[PKGBUILD & AUR Build Pipeline](ARCH_LINUX_PARITY_FEATURES.md):** Native parsing of Arch Linux `PKGBUILD` scripts (`pkgname`, `pkgver`, `depends`, `makedepends`, `prepare()`, `build()`).
- **[AUR Helper (`AurHelper`)](aur-helper.md):** Yay/Paru-style package search, PKGBUILD diff AST security inspection (`AurPkgbuildDiffAnalyzer`), and sandboxed chroot compilation.
- **[Mirror Ranking & Maintenance](arch-pacman-engine.md):** `ReflectorMirrorRanker` latency sorting, `paccache_clean` cache pruning, `checkupdates` non-locking index diffing, and `updpkgsums` SHA-256 calculation.
- **[Universal Package Transpilation](universal-adapter.md):** Cross-distro adapter pipeline converting `.deb`, `.rpm`, `.apk`, `.xbps`, `.hpkg`, and `.txz` foreign packages into native `SigmaPkg`.

---

## 🛡️ Category 4: Security & Hardening

- **[Multi-Layer Sandboxing](security.md):** Monotonic privilege restriction combining OpenBSD `pledge`/`unveil`, Linux Landlock v5, and FreeBSD Capsicum descriptor delegation (`FreeBsdCapsicumDescriptorDelegate`).
- **[Hardware Enclave Isolation](hardware-isolation.md):** AMD SEV-SNP and Intel TDX confidential enclave isolation, volatile memory zeroization upon drop, and constant-time cryptographic primitives.
- **[Post-Quantum Cryptographic Integrity](pqc-security.md):** Dilithium-5 signatures and Kyber-1024 KEM key exchange for livepatching trampolines and package manifest attestations.
- **[Input Validation Standard](input-validation.md):** RFC 952/1123 hostname rules, POSIX username/env-key boundaries, decimal IPv4 octal SSRF prevention, and colon-bounded path traversal checks.

---

## ⚡ Category 5: Performance Tuning & Optimization

- **[CachyOS BORE Scheduler (`CachyOsBoreScheduler`)](performance.md):** Burst-Oriented Response Enhancer task scheduling for sub-millisecond desktop responsiveness during background multi-core compilation.
- **[CPU Scaling Governors & EPP (`CachyOsAutoFreqEngine`)](auto-freq.md):** Dynamic auto-cpufreq governor switching (Performance, Balance, Powersave) and Energy Performance Preference tuning.
- **[Memory Reclamation & Swap (`DemandPagingSwapEngine`)](memory-management.md):** Multi-Generational LRU (`MGLRU`) page frame reclamation, Copy-on-Write (`CoW`) page faulting, KSM deduplication, and zram compressed memory swap.
- **[eBPF / XDP Kernel Bypass Network Filtering](ebpf-xdp.md):** Lock-free kernel-to-userland event ring buffer (`BPF_MAP_TYPE_RINGBUF`) and zero-copy XDP packet dispatching.

---

## 🎨 Category 6: Compositor & Desktop Ecosystem

- **[Zenith Wayland Compositor](wayland-protocol.md):** Zero-dependency Wayland wire protocol encoder/decoder (`wl_surface`, `xdg_shell` toplevel, `wl_seat` pointer/keyboard event router, and `wl_data_device` clipboard offer manager).
- **[Omarchy Desktop Suite](omarchy.md):**
  - Hyprpaper wallpaper preloading engine (`OmarchyHyprpaperWallpaperEngine`).
  - Ghostty GPU-accelerated terminal config generator (`OmarchyGhosttyTerminalEngine`).
  - Fastfetch ASCII sysinfo banner generator (`OmarchyFastfetchSysinfoEngine`).
- **[GTK & UI Toolkit Fallback (`GtkCssProvider`)](ui-toolkit.md):** Programmatic fallback styling rules natively in Rust for adwaita action rows, buttons, and system dialogs.
- **[Keyboard & Accessibility Engine (`NativeWasmDesktopEngine`)](accessibility.md):** ARIA-compliant DOM event routing, focus state visibility, and keyboard navigation.

---

## 🔄 Category 7: Maintenance & System Recovery

- **[Declarative State Graph (`DeclarativeStateGraph`)](state-management.md):** Atomic system generation snapshots and rollbacks (NixOS profile parity).
- **[Snapper CoW Snapshots (`SovereignPackageRollbackEngine`)](rollback-engine.md):** Instant Copy-on-Write sub-volume state snapshots (openSUSE Snapper parity).
- **[ZFS Boot Environments](freebsd-zfs.md):** FreeBSD `bectl` / `beadm` boot environment snapshot selection and bootloader integration.
- **[Offline System Updates (`FedoraOfflineUpdateEngine`)](systemd-offline-update.md):** Staging offline update packages for reboot execution (`systemd-offline-update` parity).

---

## ⌨️ Quick CLI Cheatsheet

| Command | Subsystem | Action Description |
|---------|-----------|--------------------|
| `sigma-pkg install <pkg>` | Package Manager | Resolves dependencies and installs package |
| `sigma-pkg aur build <pkg>` | AUR Helper | Downloads, audits PKGBUILD diff, and builds package in chroot |
| `sigma-pkg rankmirrors` | Mirror Manager | Benchmarks and ranks package mirror latency |
| `sigma-pkg rollback <id>` | Transaction Engine | Reverts system state to snapshot `<id>` in < 1ms |
| `sigma-sh` | Command Shell | Sovereign command shell with autocomplete and pledge isolation |
| `doas <cmd>` | Privilege Escalation | Executes command with delegated capabilities per rule |
| `sysctl <key>=<val>` | Kernel Tuner | Queries or modifies kernel MIB configuration tree |

---

*Documentation maintained according to the Arch Linux Wiki Principles. All code examples verified by `./run_sigma_tests.sh`.*
