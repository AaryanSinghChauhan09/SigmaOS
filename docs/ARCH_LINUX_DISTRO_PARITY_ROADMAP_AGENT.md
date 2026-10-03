# Arch Linux Distro Parity & AI Agent Future Development Roadmap

This document defines operational directives for AI agents filling functional and architectural gaps between SigmaOS and Arch Linux, as well as inspiration from FreeBSD, OpenBSD, and NetBSD distributions.

## Core Architectural Objectives & Arch Linux Gap Closure

1. **Pacman & AUR Package Ecosystem Parity**:
   - Transpile Arch Linux PKGBUILD manifests (`makepkg`) and `.pkg.tar.zst` packages into native `.sigpkg` format.
   - Execute pre-install/post-install Pacman hooks with sandboxed eBPF LSM governance.
   - Enforce SAT-based dependency resolution with subslot support.

2. **CachyOS SchedExt (SCX) & BORE Scheduling**:
   - Integrate CachyOS-inspired eBPF BORE v2 scheduler governor (`SovereignSchedExtBoreV2Governor`) into kernel dispatch.
   - Enable dynamic core frequency scaling and real-time interactive boost for desktop workloads.

3. **Archiso & Bootloader Hooks**:
   - Support Arch-style automated installer ISO generation (`archiso`) with GRUB2/systemd-boot dual boot support.
   - Implement zero-dependency initramfs hook generation and early Microcode loading.

## AI Agent Directives

- **Code Style**: Strictly `#![no_std]` bare-metal compatible zero-dependency Rust implementation.
- **Verification**: All features must have corresponding standalone unit tests that execute cleanly via `rustc --test`.
- **Documentation Transfer**: Upon 100% implementation verification, feature specs are mirrored directly to the GitHub Wiki.
