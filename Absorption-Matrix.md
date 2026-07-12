# SigmaOS Absorption Matrix

### The definitive map of open-source projects → SigmaOS sovereign replacements

> Every external tool SigmaOS absorbs means one fewer dependency. The goal: a complete sovereign environment where no external software is required.

---

## Legend

| Status | Meaning |
|---|---|
| ✅ Implemented | Core functionality exists in SigmaOS |
| 🔄 In Progress | Actively being built |
| 🎯 Planned | On the roadmap |
| 💡 Inspired By | Design/ideas absorbed, not a direct replacement |

---

## 🔧 System Utilities

| External Tool | SigmaOS Sovereign Replacement | Status | Priority | Inspired By |
|---|---|---|---|---|
| GNU Coreutils | `sigma-core-utils` (Rust) | 🔄 In Progress | P0 | BusyBox, uutils/coreutils |
| BusyBox | `sigma-core-utils` (Rust) | 🔄 In Progress | P0 | BusyBox |
| Bash / Zsh / Fish | `sigma-sh` (Rust) | 🔄 In Progress | P0 | Fish shell, elvish |
| systemd | `sigma-init` (Rust) | 🎯 Planned | P1 | OpenRC, s6, dinit |
| OpenRC | `sigma-init` (Rust) | 🎯 Planned | P1 | runit |
| syslog / journald | `sigma-log` (Rust) | 🎯 Planned | P1 | — |
| cron | `sigma-cron` (Rust) | 🎯 Planned | P2 | — |
| sudo | `sigma-priv` (capability-based) | 🎯 Planned | P1 | doas |
| man pages | `sigma-doc` | 🎯 Planned | P2 | tldr, tealdeer |

---

## 📂 File Systems & Storage

| External Tool | SigmaOS Sovereign Replacement | Status | Priority | Inspired By |
|---|---|---|---|---|
| ext4 | `SovereignFS` (journaling, POSIX) | 🎯 Planned | P0 | xv6, Minoca OS |
| btrfs | `SovereignFS` (snapshots, CoW) | 🎯 Planned | P1 | btrfs, ZFS |
| ZFS | `sigma-zfs` integration | 🎯 Planned | P2 | OpenZFS |
| LVM | `sigma-volume` | 🎯 Planned | P2 | — |
| mdadm (RAID) | `sigma-raid` | 🎯 Planned | P2 | — |
| LUKS | `sigma-crypt` (dm-crypt sovereign) | 🎯 Planned | P1 | LUKS2 |
| VirtIO drivers | `sigma-virtio` | 🎯 Planned | P1 | Hermit-rs, Unikraft |
| NVMe driver | `sigma-nvme` | 🎯 Planned | P0 | Linux NVMe |
| USB/HID stack | `sigma-usb` | 🎯 Planned | P1 | — |

---

## 🖥️ Developer Tools

| External Tool | SigmaOS Sovereign Replacement | Status | Priority | Inspired By |
|---|---|---|---|---|
| GCC / Clang | `sigma-cc` (Rust/Zig frontend) | 🎯 Planned | P1 | LLVM, zig cc |
| CMake / Meson | `sigpkg build` (Rust) | 🔄 In Progress | P1 | Zig build system |
| Make / Ninja | `sigma-make` (Rust) | 🎯 Planned | P2 | just, ninja |
| Git | `SigmaVCS` | 🎯 Planned | P1 | jj (Jujutsu), fossil |
| GDB | `sigma-debug` | 🎯 Planned | P2 | — |
| Valgrind | `sigma-memcheck` | 🎯 Planned | P2 | — |
| strace / perf | `sigma-trace` | ✅ Implemented | P0 | eBPF |
| Docker | `sigma-container` | 🎯 Planned | P1 | nanos, gvisor |
| Kubernetes | `sigma-orchestrator` | 🔄 In Progress | P1 | Unikraft, nomad |
| QEMU / KVM | `sigma-hypervisor` | 🎯 Planned | P2 | — |
| Vagrant | `sigma-vm` | 🎯 Planned | P3 | — |

---

## 🌐 Networking & Internet

| External Tool | SigmaOS Sovereign Replacement | Status | Priority | Inspired By |
|---|---|---|---|---|
| OpenSSH | `sigma-ssh` (Rust) | 🎯 Planned | P0 | russh, Dropbear |
| curl / wget | `sigma-fetch` (Rust) | 🎯 Planned | P0 | — |
| Firefox / Chromium | `sigma-browse` | 🔄 In Progress | P1 | Ladybird, NetSurf |
| Tor Browser | `sigma-anon` | 🎯 Planned | P2 | Whonix |
| WireGuard | `sigma-vpn` (native) | 🔄 In Progress | P0 | WireGuard-rs |
| OpenVPN | `sigma-vpn` | 🎯 Planned | P2 | — |
| nmap | `sigma-scan` | 🎯 Planned | P2 | — |
| Wireshark | `sigma-capture` | 🎯 Planned | P3 | — |
| iptables / nftables | `sigma-shield` (BPF) | ✅ Implemented | P0 | eBPF, XDP |
| dnsmasq | `sigma-dns` (DoH) | ✅ Implemented | P0 | — |

---

## 📦 Package Management

| External Tool | SigmaOS Sovereign Replacement | Status | Priority | Inspired By |
|---|---|---|---|---|
| apt / dpkg | `sigpkg` (Rust) | 🔄 In Progress | P0 | Wolfi OS, apk |
| rpm / yum | `sigpkg` (Rust) | 🔄 In Progress | P0 | — |
| pacman | `sigpkg` (Rust) | 🔄 In Progress | P0 | — |
| Snap / Flatpak | `sigma-sandbox` | 🎯 Planned | P1 | Nanos, gVisor |
| Nix | `sigpkg --reproducible` | 🎯 Planned | P1 | NixOS, Wolfi OS |
| Cargo | `sigpkg` (natively wraps) | ✅ Implemented | P0 | — |
| npm / pip | `sigpkg plugin:lang` | 🎯 Planned | P2 | — |

---

## 🔒 Security

| External Tool | SigmaOS Sovereign Replacement | Status | Priority | Inspired By |
|---|---|---|---|---|
| SELinux | `sigma-sandbox` (capability) | 🎯 Planned | P0 | Capsicum |
| AppArmor | `sigma-sandbox` | 🎯 Planned | P0 | — |
| OpenSSL | `sigma-crypto` (Ada/SPARK) | 🔄 In Progress | P0 | libsodium, rustls |
| GnuTLS | `sigma-crypto` | 🔄 In Progress | P0 | — |
| libsodium | `sigma-crypto` | 🔄 In Progress | P0 | libsodium |
| KeePass | `sigma-vault` | 🎯 Planned | P1 | — |
| Bitwarden | `sigma-vault` | 🎯 Planned | P1 | — |
| Auditd | `sigma-audit` | ✅ Implemented | P0 | BPF audit |
| Fail2ban | `sigma-guard` | 🎯 Planned | P2 | — |
| ClamAV | `sigma-scan` (behavioral) | 🎯 Planned | P3 | — |
| TPM tools | `sigma-tpm` | 🎯 Planned | P1 | tpm2-tools |

---

## 🎨 Productivity & Media

| External Tool | SigmaOS Sovereign Replacement | Status | Priority | Inspired By |
|---|---|---|---|---|
| LibreOffice (Writer) | `sigma-write` | 🎯 Planned | P2 | — |
| LibreOffice (Calc) | `sigma-calc` | 🎯 Planned | P2 | — |
| LibreOffice (Impress) | `sigma-present` | 🎯 Planned | P3 | — |
| VLC / MPV | `sigma-play` | 🎯 Planned | P2 | MPV |
| GIMP | `sigma-paint` | 🎯 Planned | P3 | — |
| Inkscape | `sigma-draw` | 🎯 Planned | P3 | — |
| Evince / Okular | `sigma-view` (PDF) | 🎯 Planned | P2 | — |
| Thunderbird | `sigma-mail` | 🎯 Planned | P2 | — |
| Signal desktop | `sigma-chat` | 🎯 Planned | P2 | Signal protocol |
| Matrix client | `sigma-matrix` | 🎯 Planned | P2 | Matrix.org |
| Obsidian | `sigma-notes` | 🎯 Planned | P3 | — |
| Terminal emulator | `sigma-term` (built into Zenith) | ✅ Implemented | P0 | — |

---

## 🏗️ Core OS Projects Absorbed

| Source Project | What SigmaOS Absorbs | Status |
|---|---|---|
| **Redox OS** | Memory safety, microkernel modularity, Rust `#![no_std]` patterns | 💡 Inspired By |
| **rCore** | Syscall design, RISC-V support, educational clarity | 💡 Inspired By |
| **Unikraft** | Cloud-native unikernel profile architecture | 💡 Inspired By |
| **Nanos** | Single-application sandboxing model | 💡 Inspired By |
| **Hermit-rs** | FS + virtualization integration in Rust | 💡 Inspired By |
| **ReactOS** | Driver compatibility layer design | 💡 Inspired By |
| **ToaruOS** | Driver model, compositor design | 💡 Inspired By |
| **RavynOS** | Device tree patterns, macOS-style API ergonomics | 💡 Inspired By |
| **xv6** | FS design simplicity, educational kernel | 💡 Inspired By |
| **Minoca OS** | Modular FS APIs | 💡 Inspired By |
| **Wolfi OS** | Reproducible builds, SLSA supply chain | 💡 Inspired By |
| **SkiftOS** | GUI and delightful userland UX | 💡 Inspired By |
| **SerenityOS** | Self-sufficient OS philosophy, browser + shell | 💡 Inspired By |

---

## 📊 Absorption Progress Summary

| Category | Total Tools | ✅ Done | 🔄 In Progress | 🎯 Planned |
|---|---|---|---|---|
| System Utilities | 9 | 0 | 2 | 7 |
| File Systems & Storage | 9 | 0 | 0 | 9 |
| Developer Tools | 12 | 1 | 2 | 9 |
| Networking | 10 | 3 | 2 | 5 |
| Package Management | 7 | 1 | 3 | 3 |
| Security | 11 | 2 | 3 | 6 |
| Productivity & Media | 12 | 1 | 0 | 11 |
| **TOTAL** | **70** | **8** | **12** | **50** |

---

### Last updated: 2026-06-30 | Maintained by SigmaOS Contributors | [roadmap.md](../roadmap.md)
