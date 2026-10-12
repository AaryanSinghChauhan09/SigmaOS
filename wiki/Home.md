# SigmaOS Documentation Wiki

Welcome to the **SigmaOS Documentation Wiki**, organized following the [Arch Linux Wiki](https://wiki.archlinux.org/) standards. SigmaOS is an ultra-fast, formally grounded, memory-safe sovereign operating system written natively in **Rust, Zig, and Nim**, designed to outperform and obsolete legacy Linux and BSD distributions in performance, stability, and security.

---

## Table of Contents (Arch Linux Wiki Structure)

| Subsystem / Topic | Description | Link |
|---|---|---|
| **Kernel** | Bare metal `_start` assembly, ELF loader, CFS/EEVDF/ULE schedulers, THP defrag & khugepaged, interrupts, KASAN/KMSAN, dispatcher architecture | [Kernel](Kernel.md) / [Dispatcher Plan](DISPATCHER_PLAN.md) |
| **Package Management** | Sovereign Universal Package Format Master, Arch ALPM, Debian dpkg/debconf, Fedora RPM/OSTree, Void xbps, Alpine apk | [Package Management](Package-Management.md) |
| **Security & Sandboxing** | OpenBSD pledge/unveil, FreeBSD Capsicum, Linux Landlock v4, SELinux/AppArmor, Post-Quantum Cryptography (PQC), Secure Vault | [Security & Sandboxing](Security-and-Sandboxing.md) |
| **Storage & Filesystems** | Virtual Filesystem (VFS), NVMe 2.0, AHCI SATA, Ext4, ZFS, Btrfs, FAT32, NTFS, SMB, LUKS2 encrypted partitions | [Storage & Filesystems](Storage-and-Filesystems.md) |
| **Networking** | Sovereign TCP/IP, IPv6 routing, MPTCP, VXLAN, VLAN 802.1Q, kTLS hardware offload, SCTP, OpenBSD PF, Illumos Crossbow | [Networking](Networking.md) |
| **Desktop & Window Management** | Zenith desktop environment, Hyprland Wayland compositor, XWayland bridge, Terminal emulator, Screen locker | [Desktop & Window Management](Desktop-and-Window-Management.md) |
| **Virtualization & Containers** | KVM VMCS hardware virtualization, Firecracker microVMs, FreeBSD Jails isolation, Linux cgroups/namespaces, WASM | [Virtualization & Containers](Virtualization-and-Containers.md) |
| **Hardware & Drivers** | Modern NVMe, AHCI SATA, VirtIO family, PCI/PCIe discovery, CUPS/IPP printing, ACPI/APM power management | [Hardware & Drivers](Hardware-and-Drivers.md) |
| **Init & System Supervision** | Systemd unit engine, OpenRC runlevels, Void runit supervisor, job control, deterministic boot-to-shell harness | [Init & System Supervision](Init-and-System-Supervision.md) |
| **Diagnostics & Tracing** | Ftrace dynamic tracer, Perf PMU hardware performance counters, GDB remote serial stub, kernel crash dump, serial console | [Diagnostics & Tracing](Diagnostics-and-Tracing.md) |
| **System Utilities & Maintenance** | Master system utilities plan, POSIX baseline, 18-PR roadmap, CLI contract helpers, periodic maintenance runner | [System Utilities](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md) |
| **AI Agent Maintenance Guide** | Supreme architectural invariants, permitted languages (Rust, Zig, Nim only), memory safety, zero-dependency policy | [AI Agent Maintenance Guide](AI-Agent-Maintenance-Guide.md) |

---

## Architectural Principles

1. **Stability Over Everything**: Zero breaking changes to public ABI/APIs. Every patch must maintain `cargo check --lib` exit code 0.
2. **Security by Default**: Mandatory compartmentalization using OpenBSD `pledge`/`unveil`, FreeBSD `capsicum`, and Linux `landlock`. Zero trust memory layout.
3. **Supreme Performance**: Zero-allocation hot paths (`windows(N)` byte scans), lock-free atomic queues, Transparent Huge Pages (THP) 2MB/1GB pages.
4. **Zero Dependency Bloat**: No predefined heavy third-party runtimes. Only self-contained, native implementations in **Rust**, **Zig**, and **Nim**.
5. **Universal Distro Parity**: Direct, native compatibility with Arch Linux, Debian, Fedora/RHEL, Alpine, Void, FreeBSD, OpenBSD, and NetBSD workflows.

---

## AI Agent Maintenance Instructions for Future Releases

- **Autonomous Agent Directive**: Any AI agent modifying or extending SigmaOS MUST read and enforce the rules specified in [AI-Agent-Maintenance-Guide](AI-Agent-Maintenance-Guide.md).
- **Topic Uniqueness**: Maintain strictly **one wiki page per topic**. Do not create duplicate, fragmented, or version-suffixed pages.
- **Language Policy**: Implement OS logic exclusively in Rust, Zig, or Nim. Under no circumstances should C, C++, Python, Go, Java, or JavaScript be introduced into the operating system core.
- **Verification Gate**: Every newly added subsystem must be accompanied by unit tests verifiable via standard compiler test flags (`cargo check --lib`, `rustc --test`).
