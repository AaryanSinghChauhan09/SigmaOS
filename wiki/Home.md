# SigmaOS Wiki

SigmaOS is an operating-system development project with kernel, storage, networking, security, driver, desktop, installer, and package-management components. This wiki documents current source areas and future proposals; a component name or design model does not mean the feature is integrated or supported.

**Repository**: https://github.com/AaryanSinghChauhan09/SigmaOS  
**Build status**: Refer to the repository's current CI checks; this page does not assert a passing build.
**Default branch**: `main` (the repository currently has one branch)

---

## Pages

| # | Page | Description |
|---|------|-------------|
| 00 | [Project Overview](00-Home) | Project goals and architecture |
| 01 | [Installation](01-Installation) | Boot media, installer, first boot |
| 02 | [Getting Started](02-Getting-Started) | First steps after installation |
| 03 | [Configuration](03-Configuration) | Declarative system configuration (NixOS-inspired) |
| 04 | [Kernel](04-Kernel) | Kernel source areas and architecture |
| 05 | [Filesystems](05-Filesystems) | Filesystem source areas and support boundaries |
| 06 | [Networking](06-Networking) | Network components and provider status |
| 07 | [Security](07-Security) | Security interfaces, enforcement, and provider limits |
| 08 | [Desktop](08-Desktop) | Zenith and desktop components |
| 09 | [Packaging](09-Packaging) | SigmaPkg interfaces and status |
| 10 | [Development](10-Development) | Build system, testing, contributing |
| 11 | [Roadmap](11-Roadmap) | 30-month development roadmap |
| 12 | [Contributing](12-Contributing) | How to contribute |
| 13 | [Agents](13-Agents) | AI agent framework (Bolt/Palette/Sentinel) |
| 14 | [Future Development](14-Future-Development) | Long-term plans |
| 15 | [Architecture Decisions](15-Architecture-Decisions) | ADRs |
| 16 | [Self-Sufficiency Encyclopedia](16-Self-Sufficiency-Encyclopedia) | Component catalog |
| 17 | [Package Management](17-Package-Management) | sigpkg and package subsystem |
| 18 | [Audio and Graphics](Audio-and-Graphics) | Audio stack (PipeWire/Intel HDA) and graphics (DRM/KMS) |
| 19 | [USB Devices](USB-Devices) | USB stack, xHCI, HID, and Audio Class drivers |
| 20 | [System Management](System-Management) | System management, power, thermal, diagnostics |
| 21 | [Linux Mint & Omarchy Ecosystem](Mint-and-Omarchy-Supremacy) | Warpinator P2P transfer, Timeshift snapshots, Omarchy provisioner, Web2App sandboxing |
| 22 | [IPC](IPC) | Inter-Process Communication: pipes, sockets, futex, shared memory |
| 23 | [Memory Management](Memory-Management) | Buddy allocator, slab, page cache, page fault handler, CoW |
| 24 | [AI and Agent Runtime](AI-and-Agent-Runtime) | On-device AI, crash analysis, predictive I/O, NL shell |
| 25 | [Compositor](Compositor) | Wayland compositor, tiling WM, input routing, direct scanout |
| 26 | [Installer](Installer) | Guided installer: LUKS2, Btrfs, TPM2, TUI/GUI/headless |
| 27 | [Init and Services](Init-and-Services) | PID 1, service graph, socket activation, AI failure recovery |
| 28 | [Container Runtime](Container-Runtime) | OCI runtime, rootless containers, cgroup v2, seccomp |
| 29 | [Scheduler](Scheduler) | CFS, RT, Deadline, AI-predictive scheduler, timer wheel |
| 30 | [Package Management](Package-Management) | sigma-pkg, .spkg format, SAT resolver, Arch/Flatpak compat |
| 31 | [Compatibility Layers](Compatibility-Layers) | Arch, Debian, Fedora, Mint, Omarchy compatibility engines |
| 32 | [Drivers](Drivers) | NVMe, AHCI, HDA, USB, PCI, Framebuffer, Bluetooth, Wi-Fi |
| 33 | [Boot](Boot) | SigmaEFI bootloader, Secure Boot, TPM2, measured boot |
| 34 | [Performance](Performance) | Smart optimizer, io_uring, BBR3, XDP, benchmarks |
| 35 | [Omarchy Gaming Performance Suite](Omarchy-Gaming-Performance-Suite) | Gaming governor, HUD telemetry, developer stacks |
| 36 | [Mint Tools Supremacy](Mint-Tools-Supremacy) | MintStick, Bulky, MintReport, Nemo Actions, XApps |
| 37 | [Power Management](Power-Management) | ACPI, battery, thermal, suspend/hibernate, sigma-ai governor |
| 38 | [Shell and Userspace](Shell-and-Userspace) | sigma-sh, coreutils, terminal emulator, session manager |
| 39 | [GPU and Graphics](GPU-and-Graphics) | DRM/KMS, Vulkan, OpenGL, HDR, VRR, multi-monitor |
| 40 | [Storage](Storage) | SigmaFS, Btrfs, ext4, NVMe, RAID, snapshots, io_uring |
| 41 | [Theming and Customization](Theming-and-Customization) | TOML theme system, AI accent gen, dotfile versioning |
| 42 | [Diagnostics and Crash Reporting](Diagnostics-and-Crash-Reporting) | kdump, AI crash analysis, watchdog, tracing, logs |
| 43 | [Filesystems (detailed)](Filesystems) | Detailed filesystem reference |
| 44 | [Networking (detailed)](Networking) | Detailed networking reference |
| 45 | [Security and Hardening](Security-and-Hardening) | Detailed security reference |
| 46 | [Virtualization and Containers](Virtualization-and-Containers) | Detailed virtualization reference |
| 47 | [Mint & Omarchy Launch Supremacy](Mint-and-Omarchy-Launch-Supremacy) | Kernel regression watchdog, locale IME, Cinnamon applet sandbox, Omarchy declarative rules & launch certification |
| 48 | [Mint & Omarchy Hardware & Audio Supremacy](Mint-and-Omarchy-Hardware-and-Audio-Supremacy) | Driver manager with MOK enrollment, 1.33ms studio audio DSP, handheld TDP optimizer & fuzzy launcher |
| 49 | [Mint & Omarchy Pinnacle Ecosystem](Mint-and-Omarchy-Pinnacle-Ecosystem) | Backup & software migration engine, Proton prefix manager, biometric screen locker & 1:1 kinetic gestures |
| 50 | [Mint & Omarchy Apex Mastery](Mint-and-Omarchy-Apex-Mastery) | Blur-free fractional scaling, sub-5ms content indexing, zero-polling IPC event bus & 10-foot gamepad navigation |

## Component Future-Development Roadmaps

These pages describe proposed work inspired by Linux distributions and BSD systems. They separate design ideas from capabilities already implemented in SigmaOS.

| Component | Roadmap |
|---|---|
| Kernel, scheduling, and memory | [Kernel roadmap](Future-Development-Kernel-and-Scheduling) |
| Storage and filesystems | [Storage roadmap](Future-Development-Storage-and-Filesystems) |
| Networking and security | [Networking and security roadmap](Future-Development-Networking-and-Security) |
| Device drivers and desktop | [Drivers and desktop roadmap](Future-Development-Drivers-and-Desktop) |
| Packaging, installation, and updates | [Packaging and installation roadmap](Future-Development-Packaging-and-Installation) |
| Init and service supervision | [Init and services roadmap](Future-Development-Init-and-Services) |
| IPC and userspace interfaces | [IPC and userspace roadmap](Future-Development-IPC-and-Userspace) |
| Virtualization and containers | [Virtualization roadmap](Future-Development-Virtualization-and-Containers) |
| Power and timekeeping | [Power and time roadmap](Future-Development-Power-and-Time) |

---

## AI Agent Maintenance Instructions

This wiki follows [Arch Linux wiki style](https://wiki.archlinux.org/title/Help:Style):
- **One page per topic** — no duplicates
- **Flat Markdown** — no nested headers beyond H3
- **Factual and implementation-focused** — link to source files, not external URLs
- **Each page has a Maintenance section** with instructions for future AI agents
- When a `.md` file in the repo is fully implemented, transfer it here and delete the source file
- Always use Rust (`#![no_std]`), Zig, or Nim for new implementations — never C/C++
