# 🚀 SigmaOS Wiki

Welcome to the **SigmaOS** official wiki — your comprehensive guide to the next-generation operating system built from scratch in **Rust** and **C**.

## 🔗 Quick Navigation

| Page | Description |
|------|-------------|
| [[Home]] | This page |
| [[Components]] | Full component inventory with status table |
| [[Architecture]] | System architecture, layers & design philosophy |
| [[Installation]] | Install SigmaOS (ISO, container, source) |
| [[Development]] | Contributor guide, setup & workflow |
| [[Security]] | Security model, SELinux, AppArmor, TPM |
| [[AI-Subsystem]] | S-AI multi-agent system & LLM routing |
| [[Roadmap]] | Feature milestones and quarterly plans |
| [[Linux-Distro-Ideas]] | Features inspired by major Linux distributions |
| [[Linux-BSD-Parity]] | Linux & BSD feature parity status |
| [[Package-Management]] | sigma-pkg, AUR, Flatpak, AppImage, Snap |
| [[Networking]] | TCP/UDP stack, eBPF firewall, WireGuard |
| [[Filesystems]] | Btrfs, ext4, ZFS, OverlayFS, EROFS |
| [[Boot-Process]] | UEFI, secure boot, sigma-init |
| [[Scheduler]] | EEVDF, CachyOS BORE, NUMA, work-stealing |
| [[Memory-Management]] | Paging, segmentation, CoW, kswapd |
| [[Virtualization]] | QEMU/KVM VMM, vCPU, containers |
| [[Desktop-Environment]] | Zenith DE, Wayland, Sigma Shell |
| [[FAQ]] | Frequently asked questions |
| [[Contributing]] | How to contribute to SigmaOS |
| [[Changelog]] | Release notes and version history |

## 📋 About SigmaOS

SigmaOS is a next-generation OS built from the ground up targeting:
- 🔒 **Full Sovereignty** — complete hardware control, zero telemetry
- ⚡ **Extreme Performance** — EEVDF+BORE scheduler, eBPF-native kernel, NUMA-aware work stealing
- 🤖 **AI-Native** — built-in S-AI multi-agent orchestrator with local LLM routing
- 🛡️ **Defence-in-Depth Security** — SELinux + AppArmor + Sentinel + post-quantum cryptography
- 🌐 **Broad Compatibility** — AUR, Flatpak, AppImage, .deb, .rpm, Snap support

## 🏗️ Repository Structure

```
SigmaOS/
├── src/              # Rust/C source code
│   ├── kernel/       # Core kernel (EEVDF, memory, IPC)
│   ├── ai/           # S-AI multi-agent system
│   ├── security/     # SELinux, AppArmor, Sentinel
│   ├── network/      # TCP/UDP, eBPF, WireGuard
│   ├── container/    # OCI containers, namespaces
│   ├── boot/         # UEFI, secure boot, TPM
│   └── distro/       # Linux/BSD distro parity
├── wiki/             # Wiki documentation files
├── docs/             # Technical specifications
└── scripts/          # Build and CI scripts
```

## 🔗 Repository
[github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
