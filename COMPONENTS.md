# SigmaOS Components Reference

This document provides a comprehensive overview of all major components in the SigmaOS ecosystem.

## Core Kernel Components

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| Sigma Kernel (σ-kernel) | Rust/C | Hybrid microkernel/monolithic core with eBPF-native design | Active |
| Memory Manager | Rust | NUMA-aware allocator with zRAM+zSwap support | Active |
| Scheduler (EEVDF) | Rust/C | Energy-efficient virtual deadline-first scheduler | Active |
| IPC Bus | Rust | High-speed inter-process communication via io\_uring | Active |
| VFS Layer | Rust | Virtual filesystem with eBPF hooks | Active |
| eBPF Runtime | C/Rust | Native extended Berkeley Packet Filter runtime | Active |
| Crash Reporter | Rust | Kernel panic and oops handler with telemetry | Active |
| Live Patch Engine | Rust | Zero-downtime kernel patching (inspired by kpatch) | Planned |

## Package Management

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| sigma-pkg | Rust | Primary native package manager | Active |
| AUR Compatibility Layer | Python/Rust | Arch User Repository compatibility | Active |
| Flatpak Runtime | C | Sandboxed application delivery | Active |
| AppImage Support | Shell/C | Portable application format | Active |
| Snap Bridge | Rust | Snapcraft package compatibility | Planned |
| RPM Compat Layer | Rust | RedHat package compatibility shim | Planned |
| Nix Integration | Rust | Nix package manager integration | Experimental |

## Security Subsystem

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| Sentinel Security | Rust | Real-time threat detection and mitigation | Active |
| SELinux Module | C | Security-Enhanced Linux MAC policies | Active |
| AppArmor Profiles | C | Application armor mandatory access control | Active |
| Secure Boot Chain | Rust/C | UEFI Secure Boot with custom key management | Active |
| TPM 2.0 Driver | C | Trusted Platform Module integration | Active |
| Kernel Hardening | C | KSPP/Grsecurity-inspired hardening patches | Active |
| Audit Subsystem | Rust | System call audit logging | Active |
| Sandboxing Engine | Rust | Container/process isolation via namespaces+seccomp | Active |

## AI Subsystem (S-AI)

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| S-AI Orchestrator | Rust | Multi-agent AI planner and coordinator | Active |
| Local LLM Router | Rust | Routes queries to optimal local LLM models | Active |
| Neural Power Manager | Python/Rust | AI-driven power management and optimization | Active |
| Predictive Prefetcher | Rust | ML-based file/memory prefetching | Active |
| AI Crash Analyzer | Python | LLM-assisted crash log analysis | Active |
| Sigma Copilot | Rust/Python | System-integrated AI assistant | Planned |
| Federated Learning Client | Python | Privacy-preserving on-device ML | Experimental |

## Desktop Environment

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| Sigma Shell | C++/Rust | Custom Wayland compositor and DE | Active |
| Palette Theme Engine | Rust | Adaptive theming and color management | Active |
| Sigma Panel | C++ | Taskbar/panel with widget support | Active |
| Sigma Launcher | Rust | Application launcher with AI suggestions | Active |
| Wayland Compositor | C/Rust | Based on wlroots with SigmaOS extensions | Active |
| XWayland | C | X11 backward compatibility layer | Active |
| Display Manager | Rust | Login/session manager | Active |
| Notification Daemon | Rust | Desktop notification system | Active |

## Networking Stack

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| Network Manager | Rust | Unified network configuration daemon | Active |
| eBPF Firewall | C/Rust | XDP/eBPF-based firewall and traffic control | Active |
| WireGuard Integration | C/Rust | Built-in VPN with WireGuard kernel module | Active |
| DNS Resolver | Rust | DoH/DoT-capable DNS stub resolver | Active |
| NetworkBolt | Rust | Custom high-performance networking daemon | Active |
| Zero-Trust Agent | Rust | Zero-trust network access controller | Planned |
| Tor Integration | C | Optional anonymization network layer | Experimental |

## File Systems & Storage

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| Btrfs (primary) | C | Copy-on-write filesystem with snapshots | Active |
| ext4 Support | C | Traditional Linux filesystem support | Active |
| XFS Support | C | High-performance journaling filesystem | Active |
| ZFS Layer | C | OpenZFS integration for advanced storage | Active |
| EROFS Support | C | Enhanced read-only compressed filesystem | Active |
| OverlayFS | C | Union filesystem for containers/live OS | Active |
| Sigma FS Watcher | Rust | inotify/fanotify-based filesystem monitor | Active |
| Automated Snapshots | Bash/Rust | Scheduled Btrfs/ZFS snapshot management | Active |

## System Services

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| sigma-init (systemd fork) | Rust/C | Custom init system based on systemd | Active |
| D-Bus Daemon | C | System message bus | Active |
| Polkit | C | Authorization framework | Active |
| logind | C/Rust | Login session management | Active |
| sigma-journal | Rust | Structured system logging (journald fork) | Active |
| cron/at daemon | Rust | Task scheduling daemon | Active |
| Time Sync (NTP/PTP) | C/Rust | Network time synchronization | Active |
| Printer Support | C | CUPS-based printing subsystem | Active |

## Developer Tools

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| sigma-sdk | Rust/Python | Official SigmaOS development SDK | Active |
| sigma-dbg | Rust | System debugger with eBPF tracing | Active |
| sigma-profile | C/Rust | Performance profiler (perf-compatible) | Active |
| CI/CD Integration | YAML | GitHub Actions workflows for SigmaOS | Active |
| Dev Container | Docker | Pre-configured development environment | Active |
| Sigma Build System | Rust/Make | Custom build orchestration | Active |
| Documentation Generator | Python | Auto-doc from code annotations | Planned |

## Hardware Support

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| UEFI Bootloader | Rust | Custom UEFI-compliant bootloader | Active |
| ACPI Subsystem | C | Advanced power/configuration interface | Active |
| GPU Drivers (Mesa) | C | Open-source GPU stack | Active |
| Vulkan Support | C | GPU compute and graphics API | Active |
| USB Stack | C | Full USB 3.x host and device support | Active |
| Bluetooth Stack | C | BlueZ-based Bluetooth subsystem | Active |
| WiFi Drivers | C | mac80211-based wireless stack | Active |
| RISC-V Port | Rust/C | RISC-V 64-bit architecture support | Experimental |
| ARM64 Port | Rust/C | AArch64 architecture support | Active |
| x86\_64 Primary | Rust/C | Primary x86-64 target | Active |
