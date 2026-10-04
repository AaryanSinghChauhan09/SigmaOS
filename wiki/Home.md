# SigmaOS Wiki

SigmaOS is a sovereign, AI-native, bare-metal operating system written in Rust (`#![no_std]`), Zig, and Nim. It targets hardware from 1980s 16-bit PC/AT systems to 2026+ high-performance servers (CXL 3.0, PCIe Gen7, NVMe 2.0).

**Repository**: https://github.com/AaryanSinghChauhan09/SigmaOS  
**Build status**: ✅ 0 errors (Oct 2026)  
**Branches**: `main` only

---

## Pages

| # | Page | Description |
|---|------|-------------|
| 00 | [Home](00-Home) | This page |
| 01 | [Installation](01-Installation) | Boot media, installer, first boot |
| 02 | [Getting Started](02-Getting-Started) | First steps after installation |
| 03 | [Configuration](03-Configuration) | Declarative system configuration (NixOS-inspired) |
| 04 | [Kernel](04-Kernel) | Kernel architecture: scheduling, memory, syscalls |
| 05 | [Filesystems](05-Filesystems) | Ext4+JBD2, Btrfs, ZFS, fscrypt, autofs |
| 06 | [Networking](06-Networking) | TCP/IP, VIMAGE, nftables, QUIC, Netmap |
| 07 | [Security](07-Security) | W^X, pledge, unveil, Capsicum, seccomp, CFI |
| 08 | [Desktop](08-Desktop) | Zenith compositor, UX components |
| 09 | [Packaging](09-Packaging) | sigpkg CLI overview |
| 10 | [Development](10-Development) | Build system, testing, contributing |
| 11 | [Roadmap](11-Roadmap) | 30-month development roadmap |
| 12 | [Contributing](12-Contributing) | How to contribute |
| 13 | [Agents](13-Agents) | AI agent framework (Bolt/Palette/Sentinel) |
| 14 | [Future Development](14-Future-Development) | Long-term plans |
| 15 | [Architecture Decisions](15-Architecture-Decisions) | ADRs |
| 16 | [Self-Sufficiency Encyclopedia](16-Self-Sufficiency-Encyclopedia) | Component catalog |
| 17 | [Merge Summary Oct 2026](17-Merge-Summary-Oct-2026) | Oct 2026 consolidation |
| 18 | [New Components Oct 2026](18-New-Components-Oct-2026) | New modules added |
| 19 | [Package Management](19-Package-Management) | sigpkg, 33+ formats, universal PM |
| 20 | [Branches Merged Oct 2026](20-Branches-Merged-Oct2026) | All branches merged into main |

---

## AI Agent Maintenance Instructions

This wiki follows [Arch Linux wiki style](https://wiki.archlinux.org/title/Help:Style):
- **One page per topic** — no duplicates
- **Flat Markdown** — no nested headers beyond H3
- **Factual and implementation-focused** — link to source files, not external URLs
- **Each page has a Maintenance section** with instructions for future AI agents
- When a `.md` file in the repo is fully implemented, transfer it here and delete the source file
- Always use Rust (`#![no_std]`), Zig, or Nim for new implementations — never C/C++
