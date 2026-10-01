# SigmaOS

SigmaOS is an autonomous, from-scratch, zero-dependency, zero-trust bare-metal operating system built exclusively using modern low-level systems programming languages (Rust `#![no_std]`, Zig, and Nim).

## Overview

SigmaOS is designed to eliminate operating system fragmentation, bloat, and legacy technical debt by absorbing the finest architectural innovations from all existing operating systems and distributions (Ubuntu, Fedora, Arch, NixOS, Debian, Gentoo, Void, Alpine, FreeBSD, OpenBSD, NetBSD, macOS, and Windows) into a single, unified, principle-driven bare-metal platform.

## Key Features

- **Zero-Dependency**: No external package managers, third-party libraries, or predefined wrappers
- **Modern Languages**: Core OS implemented in Rust, Zig, and Nim for systems programming
- **Hardware Support**: From 1980s ISA/IDE/VGA to 2026+ CXL 3.0/PCIe Gen7/NVMe/xHCI
- **Security**: Capability-based sandboxing, kernel mitigations, post-quantum cryptography
- **Performance**: Lock-free structures, zero-copy buffers, sub-80ns context switching
- **Zenith Desktop**: Direct bare-metal rendering without X11/Wayland overhead
- **SigmaPkg**: Universal package manager supporting 29+ Linux/BSD package formats

## Documentation Structure

This wiki is organized in Arch Linux style with one page per topic:

- [Installation](01-Installation.md)
- [Getting Started](02-Getting-Started.md)
- [Configuration](03-Configuration.md)
- [Kernel](04-Kernel.md)
- [Filesystems](05-Filesystems.md)
- [Networking](06-Networking.md)
- [Security](07-Security.md)
- [Desktop](08-Desktop.md)
- [Packaging](09-Packaging.md)
- [Development](10-Development.md)
- [Roadmap](11-Roadmap.md)
- [Architecture Decisions](15-Architecture-Decisions.md)
- [Self-Sufficiency Encyclopedia](16-Self-Sufficiency-Encyclopedia.md)

## AI Agent Maintenance Instructions

- **Bolt ⚡**: Ensure documentation of any new zero-allocation optimizations or performance improvements are added concisely without marketing fluff.
- **Palette 🎨**: Maintain Arch Linux wiki style: clear, factual, one page per topic, using appropriate markdown formatting and tables where necessary.
- **Sentinel 🛡️**: Verify that no hardcoded credentials or unvetted cryptographic algorithms are documented as production-ready. Ensure security limitations are accurately stated.
- **General**: Keep pages up-to-date with current repository capabilities. Remove redundant files when consolidating information.
