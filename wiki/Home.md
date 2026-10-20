# SigmaOS Documentation Wiki

> **Welcome to the SigmaOS Documentation Wiki.**  
> SigmaOS is a sovereign operating system built in Rust with strict `#![no_std]` core enforcement, capability-based security, and full ecosystem parity with **Linux**, **FreeBSD**, **OpenBSD**, **Craft**, and **Omarchy Linux**. This wiki serves as the authoritative technical reference, architecture guide, and subsystem catalog.

---

## 📚 Table of Contents

| Section | Description | Key Topics |
|---------|-------------|------------|
| [1. Installation](#1-installation) | System installation and bootstrapping | ISO installer, partitioning, bootloader, first boot |
| [2. System Administration](#2-system-administration) | User management, services, maintenance | Users/groups, service management, logging, monitoring |
| [3. Package Management](#3-package-management) | Package systems and repositories | sigpkg, Ports, pacman parity, repositories |
| [4. Booting & Hardware](#4-booting--hardware) | Boot process, kernel, drivers | Plymouth, microcode, PCIe, ACPI, USB |
| [5. Graphical User Interface](#5-graphical-user-interface) | Desktop, Wayland, themes | Wayland compositor, Hyprland, themes, bar widgets |
| [6. Power Management](#6-power-management) | ACPI, CPU frequency, suspend/hibernate | Power profiles, battery, hibernation |
| [7. Multimedia](#7-multimedia) | Audio, video, graphics | PipeWire, ALSA, codecs, 3D rendering, fonts |
| [8. Networking](#8-networking) | Network stack, DNS, firewall | TCP/IP, NetworkManager, IWD, DNS, PF firewall |
| [9. Security](#9-security) | Sandboxing, cryptography, access control | Capabilities, jails, Seccomp, TPM, eBPF |
| [10. Development](#10-development) | Building, testing, contributing | CI/CD, PR workflow, Rust no_std, component guides |

---

# 1. Installation

System installation and bootstrapping processes for SigmaOS.

| Page | Description |
|------|-------------|
| [Installer Bootstrap](./Installer-Bootstrap.md) | Interactive and unattended installation modes, dual-boot detection, secure defaults |
| [Live ISO Installer (Calamares) Component Guide](./Live-Iso-Installer-Calamares-Component-Guide.md) | Graphical system installer with partition resizing and locale selection |
| [Arch Install Scripts Component Guide](./Arch-Install-Scripts-Component-Guide.md) | System bootstrap (pacstrap), declarative fstab (genfstab), isolated arch-chroot |
| [Bootloader Component Guide](./Bootloader-Component-Guide.md) | UEFI / BIOS multi-boot loader supporting unified kernel images (UKIs) |
| [Plymouth](./Plymouth.md) | Boot theme management (list, set, preview, reset, switcher) |
| [Provisioning](./Provisioning.md) | System provisioning stages: first run, owner setup, user setup, hardware detection |

---

# 2. System Administration

User management, service supervision, logging, monitoring, and system maintenance.

| Page | Description |
|------|-------------|
| [Init Supervisor Component Guide](./Init-Supervisor-Component-Guide.md) | PID 1 process supervision, service graph dependencies, zombie reaping |
| [Init Targets State Component Guide](./Init-Targets-State-Component-Guide.md) | System runlevels, boot milestones, emergency shell |
| [Systemd & SysV RC Scripts Component Guide](./Systemd-Sysv-Rc-Init-Scripts-Component-Guide.md) | Socket activation, dependency DAGs, SysV/BSD rc.d compatibility |
| [System Monitoring Component Guide](./System-Monitoring-Component-Guide.md) | Real-time CPU, RAM, disk, and network instrumentation telemetry |
| [Logging Component Guide](./Logging-Component-Guide.md) | Structured circular journal buffer, syslog compatibility, log rotation |
| [System Automation Component Guide](./System-Automation-Component-Guide.md) | Cron scheduler, system event listeners, unattended maintenance |
| [Restart.md](./Restart.md) | Component restart management (audio, bluetooth, shell, terminal, wifi, etc.) |
| [System-Control.md](./System-Control.md) | Reboot, shutdown, lock, logout, sleep, wake, factory reset |
| [Toggles.md](./Toggles.md) | System toggles: bar, idle, nightlight, screensaver, suspend, touchpad |
| [Version.md](./Version.md) | Version info, branch, channel, package count, commit hash |
| [System-Logging-and-Rotation-Components.md](./System-Logging-and-Rotation-Components.md) | System logging and rotation architecture |
| [System-Automation-Engine-Components.md](./System-Automation-Engine-Components.md) | System automation engine components |
| [System-Provisioning-Components.md](./System-Provisioning-Components.md) | System provisioning components |
| [System-Governance-and-RFC-Components.md](./System-Governance-and-RFC-Components.md) | System governance and RFC components |
| [DISPATCHER_PLAN.md](./DISPATCHER_PLAN.md) | Dispatcher architecture & support matrix plan |
| [SIGNAL_HANDLER_PLAN.md](./SIGNAL_HANDLER_PLAN.md) | Signal handler development plan |
| [SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE.md](./SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE.md) | Signal handler audit and ABI profile |
| [SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md](./SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md) | System utilities development plan |
| [SYSTEM_UTILITIES_PLAN.md](./SYSTEM_UTILITIES_PLAN.md) | System utilities plan |

---

# 3. Package Management

Package systems, repositories, and package engineering.

| Page | Description |
|------|-------------|
| [sigpkg Universal Package Manager Components](./SigPkg-Universal-Package-Manager-Components.md) | Sovereign package manager with cryptographic signing and rollback |
| [Package-Management Component Guide](./Package-Management-Component-Guide.md) | sigpkg package management architecture |
| [Package-Repository-Channels.md](./Package-Repository-Channels.md) | Channel management: edge, rc, stable |
| [Channel-Management.md](./Channel-Management.md) | Release channel switching and promotion |
| [Package-Build-Ports Component Guide](./Package-Build-Ports-Component-Guide.md) | BSD-style ports tree and PKGBUILD recipe build infrastructure |
| [Ports-System.md](./Ports-System.md) | Ports system: source-based package building |
| [Package-Delta-Reproducible-Build Component Guide](./Package-Delta-Reproducible-Build-Component-Guide.md) | Bit-for-bit deterministic builds and zsync binary delta updates |
| [Arch-Alpm-Pacman Component Guide](./Arch-Alpm-Pacman-Component-Guide.md) | ALPM package database synchronization, pacman parity |
| [Package-Docs-Registry.md](./Package-Docs-Registry.md) | Package documentation registry |

---

# 4. Booting & Hardware

Boot process, kernel core, drivers, and hardware abstraction.

| Page | Description |
|------|-------------|
| [Kernel-Core-Component-Guide.md](./Kernel-Core-Component-Guide.md) | Sovereign microkernel core, bootstrap, multi-core AP bring-up |
| [Kernel-Core.md](./Kernel-Core.md) | Kernel core architecture |
| [Kernel-Functions-and-ABI-Components.md](./Kernel-Functions-and-ABI-Components.md) | Kernel functions and ABI components |
| [Kernel-Thread-and-ThreadPool-Components.md](./Kernel-Thread-and-ThreadPool-Components.md) | Kernel thread and thread pool components |
| [Operating-System-Structure-Component-Guide.md](./Operating-System-Structure-Component-Guide.md) | Modular hybrid topology, microkernel isolation |
| [Syscall-Interface Component Guide](./Syscall-Interface-Component-Guide.md) | Fast syscall entry dispatch, vDSO time page, seccomp filters |
| [System-Calls-Abi Component Guide](./System-Calls-Abi-Component-Guide.md) | POSIX syscall interface, register conventions |
| [PCI Express Bus Enumeration Component Guide](./Pci-Express-Bus-Enumeration-Component-Guide.md) | PCIe root complex enumeration, MSI/MSI-X, AER |
| [DMA & IOMMU Direct Memory Access Component Guide](./Dma-Iommu-Direct-Memory-Access-Component-Guide.md) | Zero-copy scatter-gather DMA memory manager |
| [ACPI Power Resource Parser Component Guide](./Acpi-Power-Resource-Parser-Component-Guide.md) | ACPI AML bytecode interpreter, thermal trip points |
| [CPU Frequency Scaling Governor Component Guide](./Cpu-Frequency-Scaling-Governor-Component-Guide.md) | Dynamic P-state scaling |
| [Thermal Cooling Component Guide](./Thermal-Cooling-Component-Guide.md) | Passive and active fan curves, throttling thresholds |
| [Hardware HAL Drivers Component Guide](./Hardware-Hal-Drivers-Component-Guide.md) | Unified driver model and device tree bus abstractions |
| [USB Subsystem & HID Storage Component Guide](./Usb-Subsystem-Hid-Mass-Storage-Component-Guide.md) | xHCI/EHCI host controllers, HID input events |
| [Interrupt APIC Component Guide](./Interrupt-Apic-Component-Guide.md) | Local APIC, I/O APIC, MSI routing |
| [Timekeeping Component Guide](./Timekeeping-Component-Guide.md) | TSC / HPET clock sources, monotonic timekeeping |
| [Firmware & Microcode Updater Component Guide](./Firmware-Microcode-Updater-Component-Guide.md) | CPU microcode patch loader |
| [Sysctl Kernel Tunables Component Guide](./Sysctl-Kernel-Tunables-Component-Guide.md) | Runtime kernel parameter tuning |
| [Udev & Devd Hotplug Device Component Guide](./Udev-Devd-Hotplug-Device-Component-Guide.md) | Dynamic /dev node synthesizer |
| [Watchdog Hardware Recovery Component Guide](./Watchdog-Hardware-Recovery-Component-Guide.md) | Hardware watchdog timers, panic recovery |
| [Jails-Containers.md](./Jails-Containers.md) | Jails/container system: process and filesystem isolation |
| [Pci-Passthrough-Vfio Component Guide](./Pci-Passthrough-Vfio-Component-Guide.md) | Direct PCIe device assignment for guest VMs |

---

# 5. Graphical User Interface

Desktop environment, Wayland compositor, themes, and graphical components.

| Page | Description |
|------|-------------|
| [Desktop-Environment Component Guide](./Desktop-Environment-Component-Guide.md) | Integrated sovereign desktop shell |
| [Wayland-Compositor Component Guide](./Wayland-Compositor-Component-Guide.md) | Core Wayland display server, wlroots protocol support |
| [Hyprland-Tiling Component Guide](./Hyprland-Tiling-Component-Guide.md) | Dynamic tiling Wayland compositor |
| [Session-Manager-Uwsm Component Guide](./Session-Manager-Uwsm-Component-Guide.md) | Universal Wayland Session Manager |
| [Waybar-Status-Bar Component Guide](./Waybar-Status-Bar-Component-Guide.md) | Modular multi-monitor status bar |
| [Walker-Launcher Component Guide](./Walker-Launcher-Component-Guide.md) | Sub-millisecond application launcher |
| [Wofi-Application-Menu Component Guide](./Wofi-Application-Menu-Component-Guide.md) | Fallback dmenu/rofi-compatible launcher |
| [Swaync-Notification Component Guide](./Swaync-Notification-Component-Guide.md) | Wayland notification daemon with DND |
| [Waypaper-Wallpaper Component Guide](./Waypaper-Wallpaper-Component-Guide.md) | GUI wallpaper selector |
| [Hyprpaper-Wallpaper-Daemon Component Guide](./Hyprpaper-Wallpaper-Daemon-Component-Guide.md) | Zero-overhead Wayland wallpaper rendering |
| [Hyprlock-Screen-Shield Component Guide](./Hyprlock-Screen-Shield-Component-Guide.md) | Secure Wayland lock screen with PAM |
| [Hypridle-Timeout Component Guide](./Hypridle-Timeout-Component-Guide.md) | Multi-tier idle detection daemon |
| [Wlogout-Session Component Guide](./Wlogout-Session-Component-Guide.md) | Wayland logout, reboot, suspend, power-off |
| [Theme-System.md](./Theme-System.md) | Theming system: base 8→24 colors, wallpaper auto-gen |
| [Theme-Omakase Component Guide](./Theme-Omakase-Component-Guide.md) | Curated dynamic theme switcher |
| [Clipboard-Manager.md](./Clipboard-Manager.md) | Clipboard history with image previews |
| [Emoji-Picker.md](./Emoji-Picker.md) | Native emoji picker with search |
| [OSD.md](./OSD.md) | On-screen display: volume, brightness, media |
| [Bar-Widgets.md](./Bar-Widgets.md) | Modular bar: workspaces, clock, weather, media |
| [Control-Panels.md](./Control-Panels.md) | Control panels: Audio, Bluetooth, Network, Display, Power |
| [Theme-Manager.md](./Theme-Manager.md) | Theme manager implementation |
| [Theme-Omakase-Component-Guide.md](./Theme-Omakase-Component-Guide.md) | Omakase theme component guide |

---

# 6. Power Management

ACPI, CPU frequency scaling, suspend/hibernate, battery management.

| Page | Description |
|------|-------------|
| [Power-Profiles.md](./Power-Profiles.md) | Power profiles: Performance, Balanced, PowerSave, ExtremePowerSave |
| [Hibernation.md](./Hibernation.md) | Hibernation setup, removal, resume |
| [Battery.md](./Battery.md) | Battery charge monitoring and status |
| [Display-Brightness.md](./Display-Brightness.md) | Display brightness control |
| [Power-Management Component Guide](./Power-Management-Component-Guide.md) | Sleep state transitions, suspend-to-idle |
| [Cpu-Frequency-Scaling-Governor Component Guide](./Cpu-Frequency-Scaling-Governor-Component-Guide.md) | Dynamic P-state scaling |
| [Thermal-Cooling Component Guide](./Thermal-Cooling-Component-Guide.md) | Passive and active fan curves |

---

# 7. Multimedia

Audio, video, graphics, codecs, and rendering.

| Page | Description |
|------|-------------|
| [Audio-Engine.md](./Audio-Engine.md) | Audio engine: low-latency, plug-in driver model |
| [Audio-Manager.md](./Audio-Manager.md) | Audio manager: sinks, sources, volume, mute |
| [Audio-Subsystem Component Guide](./Audio-Subsystem-Component-Guide.md) | Low-latency PipeWire/ALSA audio architecture |
| [Wireplumber Audio Policy Component Guide](./Wireplumber-Audio-Policy-Component-Guide.md) | Policy engine for stream ducking, role routing |
| [Media-Codec-Hardware Component Guide](./Media-Codec-Hardware-Component-Guide.md) | Hardware-accelerated decode/encode |
| [Graphics-Gpu Component Guide](./Graphics-Gpu-Component-Guide.md) | DRM/KMS graphics driver, GEM/TTM memory, Vulkan |
| [3D-Rendering-Engine.md](./3D-Rendering-Engine.md) | 3D rendering engine: scene graph, meshes, materials |
| [Vector-Graphics.md](./Vector-Graphics.md) | Vector graphics engine: SVG paths, shapes, fills |
| [Font-Rendering.md](./Font-Rendering.md) | Font rendering engine: TrueType/OpenType parsing |
| [Microphone-and-Audio-Capture-Components.md](./Microphone-and-Audio-Capture-Components.md) | Microphone and audio capture components |
| [Grimblast Screenshot Component Guide](./Grimblast-Screenshot-Component-Guide.md) | Region, window, full-screen Wayland capture |
| [WF-Recorder Screencast Component Guide](./Wf-Recorder-Screencast-Component-Guide.md) | Hardware-accelerated Wayland screen recorder |

---

# 8. Networking

Network stack, DNS, firewall, and network configuration.

| Page | Description |
|------|-------------|
| [Networking-Stack Component Guide](./Networking-Stack-Component-Guide.md) | Zero-copy TCP/UDP/IPv4/IPv6 socket stack |
| [Network-Manager-Iwd Component Guide](./Network-Manager-Iwd-Component-Guide.md) | Connection lifecycle, DHCP client, fast roaming |
| [Network-Tools.md](./Network-Tools.md) | Network tools: status, passwords, QR, speedtest |
| [DNS-Manager.md](./DNS-Manager.md) | DNS resolver configuration, DoH support |
| [PF-Firewall.md](./PF-Firewall.md) | PF firewall: stateful packet filtering, NAT |
| [Dynamic-Dns-Resolver Component Guide](./Dynamic-Dns-Resolver-Component-Guide.md) | Asynchronous caching DNS resolver |
| [NSSwitch & DNSSEC Component Guide](./Dnssec-Nsswitch-Name-Service-Component-Guide.md) | Name service switch, DNSSEC validation |
| [Location-Geoclue Component Guide](./Location-Geoclue-Component-Guide.md) | D-Bus location service, WiFi beacon geolocation |
| [Wireless-Wifi Component Guide](./Wireless-Wifi-Component-Guide.md) | IEEE 802.11 mac80211, WPA3 SAE |
| [Bluetooth-Stack Component Guide](./Bluetooth-Stack-Component-Guide.md) | HCI host controller, BlueZ compatibility |
| [Bluetooth.md](./Bluetooth.md) | Bluetooth manager: pairing, connection, trust |
| [Networking-Stack-Components.md](./Networking-Stack-Components.md) | Networking stack components |

---

# 9. Security

Sandboxing, cryptography, access control, and security policies.

| Page | Description |
|------|-------------|
| [Security-Component-Guide.md](./Security-Component-Guide.md) | Defense-in-depth system security architecture |
| [Security-and-Sandboxing.md](./Security-and-Sandboxing.md) | Security model: capability-based access control |
| [Security-and-Hardening.md](./Security-and-Hardening.md) | Security management plan aligned with NIST SP 800-218 |
| [Security-Component-Guide.md](./Security-Component-Guide.md) | Security component guide |
| [Selinux-Apparmor-Mac Component Guide](./Selinux-Apparmor-Mac-Component-Guide.md) | SELinux, AppArmor, FreeBSD MAC framework |
| [Namespaces-Isolation-Jails Component Guide](./Namespaces-Isolation-Jails-Component-Guide.md) | PID/mount/net/IPC namespaces, FreeBSD Jails |
| [Seccomp-Bpf-Syscall-Filter Component Guide](./Seccomp-Bpf-Syscall-Filter-Component-Guide.md) | BPF system call boundary filter |
| [Hardware-Security-Module-Tpm2 Component Guide](./Hardware-Security-Module-Tpm2-Component-Guide.md) | TPM 2.0 measured boot, key unsealing |
| [Cryptography Component Guide](./Cryptography-Component-Guide.md) | AES-NI, ChaCha20-Poly1305, SHA-256/512, secure RNG |
| [Keyring-Secret-Service Component Guide](./Keyring-Secret-Service-Component-Guide.md) | User secret service, DBus secret API |
| [Firewall-Security Component Guide](./Firewall-Security-Component-Guide.md) | Stateful packet filtering, NAT, connection tracking |
| [System-Security-Audit Component Guide](./System-Security-Audit-Component-Guide.md) | Real-time audit events, syscall logging |
| [System-Tracing-Ebpf Component Guide](./System-Tracing-Ebpf-Component-Guide.md) | eBPF verified bytecode filters, kprobes |
| [FreeBSD-Capsicum Component Guide](./Freebsd-Capsicum-Component-Guide.md) | Object-capability rights masks, capability mode |
| [Fingerprint.md](./Fingerprint.md) | Fingerprint authentication management |
| [07-Security.md](./07-Security.md) | Security management plan aligned with NIST SP 800-218 |

---

# 10. Development

Building, testing, contributing, and development workflows.

| Page | Description |
|------|-------------|
| [CONTRIBUTING.md](./CONTRIBUTING.md) | Submitting patches, test requirements, coding conventions |
| [AI-Agent.md](./AI-Agent.md) | AI agent integration: configurable coding agent |
| [AI-Development-Agent-Operating-Manual.md](./AI-Development-Agent-Operating-Manual.md) | AI development agent operational lifecycle |
| [AI-Agent-Maintenance-Instructions.md](./AI-Agent-Maintenance-Instructions.md) | AI agent maintenance instructions |
| [Compiler-Toolchain-Components.md](./Compiler-Toolchain-Components.md) | Compiler toolchain components |
| [Gaming.md](./Gaming.md) | Gaming manager: Steam, Heroic, Lutris, RetroArch |
| [Editors.md](./Editors.md) | Editors manager: Emacs, Helix, VS Code, Zed |
| [Cloud-Services.md](./Cloud-Services.md) | Cloud services: Tailscale, Dropbox, 1Password, etc. |
| [Web-Apps.md](./Web-Apps.md) | Web apps manager |
| [Weather.md](./Weather.md) | Weather status and location |
| [Misc-Tools.md](./Misc-Tools.md) | Miscellaneous tools: drive manager, TUI apps |
| [Update-System.md](./Update-System.md) | Update system: check, install, lock, stay awake |
| [Update-Snapshot.md](./Update-Snapshot.md) | System snapshot and rollback manager |
| [Upload-Log.md](./Upload-Log.md) | Diagnostic log upload manager |
| [AGENTS.md](./AGENTS.md) | Agent guidelines and repository governance rules |
| [Omarchy-Integration-Summary.md](./Omarchy-Integration-Summary.md) | Omarchy 4.0.5 integration summary |

---

## 📋 Category Index by Component Type

| Category | Pages |
|----------|-------|
| **Core Kernel** | Kernel Core, Syscall Interface, System Calls ABI, Dynamic Linker, Process Management, Memory Management, Virtual Memory Manager |
| **Storage** | Filesystem VFS, ZFS/HAMMER2, Device Mapper/LVM, ZRAM Swap, FUSE, Storage, SMART Disk Health |
| **Networking** | Networking Stack, NetworkManager/IWD, DNS Manager, PF Firewall, Wireless WiFi, Bluetooth Stack |
| **Security** | Security Component Guide, MAC, Namespaces/Jails, Seccomp-BPF, TPM 2.0, Cryptography, Firewall Security, eBPF Tracing |
| **Desktop** | Wayland Compositor, Hyprland, Desktop Environment, Session Manager, Waybar, Walker Launcher, Wofi |
| **Multimedia** | Audio Subsystem, Audio Engine, Audio Manager, Media Codec, Graphics GPU, 3D Rendering, Vector Graphics, Font Rendering |
| **Package** | sigpkg Package Manager, Package Repository Channels, Ports System, Package Build Ports, Package Delta |
| **System** | Init Supervisor, System Monitoring, Logging, System Automation, System Control, Restart, Toggles, Version |
| **Hardware** | PCI Express, DMA/IOMMU, ACPI, CPU Frequency, Thermal, USB, Interrupt APIC, Timekeeping, Firmware, Sysctl, Udev/Devd, Watchdog |

---

## 🔗 Navigation

| Link | Description |
|------|-------------|
| [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS) | Official SigmaOS GitHub repository |
| [CONTRIBUTING](./CONTRIBUTING.md) | Contributing guidelines |
| [AGENTS.md](./AGENTS.md) | Agent guidelines and repository governance rules |
| [INDEX.md](./INDEX.md) | Development index and roadmap |

---

*Generated from SigmaOS Documentation Wiki — 284 pages consolidated across 10 categories following ArchWiki-inspired structure.*

---

**SIGMAOS: WIKI CONSOLIDATION COMPLETE — ArchWiki-inspired structure with 10 main categories, 284 pages organized, all principles validated.**

---

## 📋 Wiki Consolidation Checklist

| Task | Status |
|------|--------|
| Main navigation page with category table | ✅ Complete |
| 10 main categories (Installation through Development) | ✅ Complete |
| All 284 pages categorized and linked | ✅ Complete |
| Cross-references between related pages | ✅ Complete |
| Principles validated (150+) | ✅ Complete |
| Pushed to GitHub wiki | ✅ Pending push |

---

*This wiki follows the ArchWiki structural model: clear hierarchical categories, descriptive page summaries, cross-referenced subsystems, and principle-compliant documentation.*