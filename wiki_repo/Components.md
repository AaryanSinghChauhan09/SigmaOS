# 🔧 SigmaOS Components

This page provides a comprehensive table of all SigmaOS components, their implementation language, description, and current status.

## Status Legend
| Status | Meaning |
|--------|---------|
| ✅ Active | Fully implemented and tested |
| 🔬 Experimental | Implemented but not production-ready |
| 📋 Planned | Designed but not yet implemented |
| 🚧 In Progress | Currently being developed |

---

## Core Kernel

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| Sigma Kernel | Rust/C | Core monolithic-hybrid kernel | ✅ Active |
| EEVDF Scheduler | Rust/C | Earliest Eligible Virtual Deadline First | ✅ Active |
| BORE Scheduler | Rust | Burst-Oriented Response Enhancer (CachyOS-style) | ✅ Active |
| MLFQ Scheduler | Rust | Multi-Level Feedback Queue | ✅ Active |
| NUMA Scheduler | Rust | NUMA-topology-aware scheduling | ✅ Active |
| Work-Stealing Queue | Rust | SMP load balancing via work stealing | ✅ Active |
| Memory Manager | Rust | Paging, segmentation, buddy allocator | ✅ Active |
| Virtual Memory Manager | Rust | VMM with CoW, KSM, demand paging | ✅ Active |
| IPC Bus | Rust | High-speed inter-process communication | ✅ Active |
| VFS Layer | Rust | Virtual file system abstraction | ✅ Active |
| eBPF Runtime | C | Kernel extension runtime | 🔬 Experimental |
| Process Manager | Rust | Process creation, lifecycle, signals | ✅ Active |
| Thread Manager | Rust | POSIX thread model, futex | ✅ Active |
| System Call Interface | C/Rust | Linux-compatible syscall layer | ✅ Active |
| Crash Reporter | Rust | Panic handler and core dumps | 📋 Planned |
| Live Patch Engine | C | Kernel live patching | 📋 Planned |

## Memory Management

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| Buddy Allocator | Rust | Physical memory zone allocator | ✅ Active |
| Slab Allocator | Rust | Object cache allocator | ✅ Active |
| Paging Engine | Rust | x86_64 4-level, ARM64 paging | ✅ Active |
| Segmentation | Rust | GDT, LDT, protected mode segments | ✅ Active |
| kswapd | Rust | Background memory reclamation daemon | ✅ Active |
| Predictive Prefetcher | Rust | AI-driven memory prefetch | 📋 Planned |
| KSM (Kernel Same-page Merging) | Rust | Shared page deduplication | ✅ Active |
| CoW (Copy-on-Write) | Rust | Fork-on-write page sharing | ✅ Active |

## Package Management

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| sigma-pkg | Rust | Native SigmaOS package manager | ✅ Active |
| AUR Layer | Rust/C++ | Arch User Repository compatibility | 🚧 In Progress |
| Flatpak | C | Flatpak runtime integration | ✅ Active |
| AppImage | C | AppImage mount & execution | ✅ Active |
| Snap Bridge | Rust | Snap daemon bridge | 📋 Planned |
| Nix Integration | Nix/Rust | Nix expression evaluation | 🔬 Experimental |
| dpkg/APT Bridge | Rust | Debian package compatibility | 🚧 In Progress |
| DNF/RPM Bridge | Rust | Red Hat/Fedora compatibility | 🚧 In Progress |
| Portage Bridge | Rust | Gentoo source-based builds | 🔬 Experimental |
| Content-Addressed Store | Rust | Reproducible package storage | ✅ Active |
| Package Signing | Rust | GPG + Dilithium PQC signatures | ✅ Active |

## Security Subsystem

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| Sentinel | Rust | Threat detection & behavioral IDS | ✅ Active |
| SELinux | C | Mandatory access control | ✅ Active |
| AppArmor | C | Application profile sandboxing | 🚧 In Progress |
| Secure Boot | C/Rust | UEFI secure boot chain | ✅ Active |
| TPM 2.0 | Rust | Hardware root of trust, PCR sealing | ✅ Active |
| Kernel Hardening | C | KSPP-inspired hardening | ✅ Active |
| Audit | C/Rust | System call audit trail | ✅ Active |
| Pledge/Unveil | Rust | OpenBSD-inspired capability restrictions | ✅ Active |
| Seccomp-BPF | C | Syscall filtering | ✅ Active |
| Post-Quantum Crypto | Rust | Kyber KEM, Dilithium signatures | ✅ Active |
| PQC-TLS 1.3 | Rust | Quantum-resistant TLS | ✅ Active |
| Vulnerability Scanner | Rust | CVE detection and remediation | ✅ Active |
| Penetration Testing Engine | Rust | Simulated pentest framework | 🔬 Experimental |
| Rootkit Detector | Rust | Kernel integrity verification | ✅ Active |
| PKI/Certificate | Rust | Certificate management | ✅ Active |
| AnonSurf | Rust | Tor-based anonymization | 🔬 Experimental |

## AI Subsystem (S-AI)

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| Orchestrator | Rust | Multi-agent task orchestrator | ✅ Active |
| LLM Router | Rust/Python | Model routing & load balancing | ✅ Active |
| Local LLM Engine | Rust | llama.cpp, Ollama integration | ✅ Active |
| Sigma Copilot | Rust | CLI + GUI assistant | ✅ Active |
| Neural Power Manager | Rust | AI-driven power optimization | 🔬 Experimental |
| Predictive Prefetcher | Rust | AI memory access prediction | 📋 Planned |
| AI Crash Analyzer | Python/Rust | Core dump AI analysis | 📋 Planned |
| Model Marketplace | Rust | PQC-signed model registry | ✅ Active |
| Developer Platform | Rust | AI development workspace | ✅ Active |
| Sovereign Planner | Rust | Multi-step task planning agent | ✅ Active |

## Desktop Environment

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| Zenith Compositor | Rust | DRM/KMS Wayland compositor | ✅ Active |
| Sigma Shell | Rust | Custom DE shell | ✅ Active |
| Palette Theme Engine | Rust | System-wide theming | 🚧 In Progress |
| Sigma Panel | Rust | Taskbar, dock, status area | ✅ Active |
| Sigma Launcher | Rust | Application launcher | ✅ Active |
| Wayland Protocol | C | Display protocol server | ✅ Active |
| XWayland | C | X11 compatibility layer | ✅ Active |
| Sigma Display Manager | Rust | Login/session manager | ✅ Active |
| Dr460nized Theme | CSS/Rust | Garuda-inspired dark theme | ✅ Active |
| Cinnamon-style Layout | Rust | Cinnamon-inspired taskbar | ✅ Active |
| Tiling Manager | Rust | BSP/MasterStack tiling | ✅ Active |
| Terminal Emulator | Rust | GPU-accelerated terminal | ✅ Active |

## Networking

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| Network Manager | C/Rust | Connection & interface management | ✅ Active |
| TCP/UDP Stack | Rust | Full TCP state machine, UDP | ✅ Active |
| eBPF Firewall | C | XDP/TC-level packet filtering | ✅ Active |
| WireGuard | C/Rust | Modern VPN tunnel | ✅ Active |
| DNS Resolver | Rust | Local caching DNS (DoH ready) | ✅ Active |
| NetworkBolt | Rust | Zero-copy TCP/UDP optimizer | 📋 Planned |
| Zero-Trust Agent | Rust | Identity-based network access | ✅ Active |
| WiFi Stack | C | 802.11ax (WiFi 6) support | ✅ Active |
| Bluetooth | C | BT 5.3 stack | ✅ Active |
| AnonSurf/Tor | Rust | Traffic anonymization | 🔬 Experimental |

## Filesystems

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| Btrfs | C | Default root filesystem with snapshots | ✅ Active |
| ext4 | C | Legacy compatibility filesystem | ✅ Active |
| XFS | C | High-performance filesystem | ✅ Active |
| ZFS | C | Advanced pooled storage | 📋 Planned |
| EROFS | C | Read-only compressed filesystem | ✅ Active |
| OverlayFS | C | Union mount for containers | ✅ Active |
| NTFS | C | Windows filesystem read/write | ✅ Active |
| procfs | Rust | /proc virtual filesystem | ✅ Active |
| sysfs | Rust | /sys kernel object interface | ✅ Active |
| Sigma FS Watcher | Rust | inotify replacement | 📋 Planned |
| Auto Snapshots | Rust | Btrfs automated snapshots | ✅ Active |

## System Services

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| sigma-init (systemd-compat) | Rust | Init system with systemd units | ✅ Active |
| runit Init Bridge | Rust | Void Linux runit compatibility | ✅ Active |
| D-Bus | C | System/session message bus | ✅ Active |
| Polkit | C | Privilege escalation control | ✅ Active |
| logind | C | Login session manager | ✅ Active |
| sigma-journal | Rust | Structured logging daemon | ✅ Active |
| cron | C | Task scheduler | ✅ Active |
| Time Sync (NTP) | Rust | NTP/PTP client | ✅ Active |
| Print Support | C | CUPS printing integration | 📋 Planned |
| CUPS | C | Common Unix Printing System | 📋 Planned |

## Developer Tools

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| sigma-sdk | Rust | Development toolkit | ✅ Active |
| sigma-dbg | C/Rust | Debugger interface | ✅ Active |
| sigma-profile | Rust | Performance profiler | 📋 Planned |
| sigma-trace | Rust | eBPF-based tracer | 🔬 Experimental |
| CI/CD Pipeline | YAML | GitHub Actions workflows | ✅ Active |
| Dev Container | Docker | Containerized dev environment | ✅ Active |
| Build System | Make/Rust | Cargo + custom build scripts | ✅ Active |

## Hardware & Architecture Support

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| UEFI Bootloader | C/Rust | System bootloader (EFI stub) | ✅ Active |
| BIOS/Legacy Boot | C | Legacy MBR boot support | ✅ Active |
| ACPI | C | Power and hardware management | ✅ Active |
| GPU/Mesa | C/C++ | Open-source graphics drivers | ✅ Active |
| Vulkan | C | Graphics API support | ✅ Active |
| USB | C | Universal Serial Bus stack | ✅ Active |
| NVMe | C | PCIe NVMe storage driver | ✅ Active |
| SATA/AHCI | C | SATA storage driver | ✅ Active |
| x86_64 | ASM/C | Primary architecture support | ✅ Active |
| ARM64/AArch64 | ASM/C | ARM 64-bit support | ✅ Active |
| RISC-V 64 | ASM/C | RISC-V experimental support | 🔬 Experimental |

## Virtualization

| Component | Language | Description | Status |
|-----------|----------|-------------|--------|
| QEMU/KVM Bridge | Rust | Hardware-accelerated VMM | ✅ Active |
| vCPU Execution Loop | Rust | Virtual CPU management | ✅ Active |
| QMP Engine | Rust | QEMU Machine Protocol | ✅ Active |
| virtio Drivers | C/Rust | Paravirtual device drivers | ✅ Active |
| Container Runtime | Rust | OCI-compatible container runtime | ✅ Active |
| cgroups v2 | C | Resource isolation | ✅ Active |
| Namespaces | C | PID, net, mount, UTS isolation | ✅ Active |
| DistroSandbox | Rust | Landlock + seccomp sandbox | ✅ Active |
