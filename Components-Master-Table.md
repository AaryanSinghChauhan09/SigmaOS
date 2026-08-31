# 🔩 SigmaOS Components — Master Table

This wiki page is the **authoritative reference** for all SigmaOS components. It shows every component's implementation status, source file, and the Linux/BSD/OS project it was inspired by.

> 📄 This page was automatically generated from [SigmaOS-Components-Master-Table.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/SigmaOS-Components-Master-Table.md) in the main repository.

---

## Legend

| Icon | Meaning |
|------|---------|
| ✅ | Fully implemented and tested |
| 🔄 | In progress / partially implemented |
| 📋 | Planned / not yet started |
| 🧪 | Experimental / proof-of-concept |

---

## Kernel Subsystem

| Component | Description | Status | Source File | Inspired By |
|-----------|-------------|--------|-------------|-------------|
| **SigmaKernel** | Microkernel core with IPC and capability model | ✅ | `src/kernel/` | seL4, Mach |
| **CachyBoreScheduler** | BORE + EEVDF hybrid CPU scheduler | ✅ | `src/kernel/scheduler.rs` | CachyOS, Linux CFS |
| **SigmaMemoryManager** | Physical/virtual memory manager with buddy allocator | ✅ | `src/kernel/memory.rs` | Linux mm, FreeBSD vm |
| **SigmaSMP** | Symmetric multiprocessing and core pinning | ✅ | `src/kernel/smp.rs` | Linux SMP |
| **SigmaHAL** | Hardware Abstraction Layer (x86_64, AArch64, RISC-V) | 🔄 | `src/kernel/architecture.rs` | UEFI, ARM TF-A |
| **SigmaIPC** | ALPC/Pipe inter-process communication channels | ✅ | `src/kernel/ipc.rs` | Windows ALPC, seL4 |
| **SigmaSyscall** | System call dispatch table and validation | ✅ | `src/kernel/syscall.rs` | Linux syscall ABI |
| **SigmaPCIScanner** | PCIe ECAM config access, BAR decoding, MSI/MSI-X | 🔄 | `src/kernel/pci_scanner.rs` | Linux PCI subsystem |
| **SigmaPageFault** | Page fault handler and demand paging | ✅ | `src/kernel/paging.rs` | Linux page fault |

---

## Security Subsystem

| Component | Description | Status | Source File | Inspired By |
|-----------|-------------|--------|-------------|-------------|
| **SigmaMAC** | Mandatory Access Control (Bell-LaPadula + SELinux) | ✅ | `src/security/mac.rs` | SELinux, AppArmor |
| **SigmaDAC** | Discretionary Access Control with POSIX ACLs | ✅ | `src/access/control.rs` | POSIX ACL |
| **SigmaRBAC** | Role-Based Access Control with policy engine | ✅ | `src/security/rbac.rs` | RBAC standard |
| **OpenBsdPledgeUnveilSentinel** | OpenBSD-style pledge/unveil syscall restriction | ✅ | `src/security/pledge.rs` | OpenBSD pledge |
| **SigmaCapability** | Linux-compatible capability bounding sets | ✅ | `src/security/capability.rs` | Linux capabilities |
| **PostQuantumTls** | Kyber-1024 + Dilithium-5 TLS 1.3 stack | ✅ | `src/crypto/pqc_dilithium.rs` | liboqs |
| **SigmaCrypto** | Symmetric and asymmetric crypto primitives | ✅ | `src/crypto/` | LibreSSL, Ring |
| **SELinuxEngine** | Full SELinux AVC caching and policy engine | ✅ | `src/security/selinux.rs` | SELinux reference |
| **SigmaSecureBoot** | UEFI Secure Boot + measured boot chain | 🔄 | `src/boot/secure_boot.rs` | shim, UEFI spec |
| **SigmaAudit** | Kernel-level audit subsystem with log streaming | ✅ | `src/audit/` | Linux audit |

---

## Package Management

| Component | Description | Status | Source File | Inspired By |
|-----------|-------------|--------|-------------|-------------|
| **SerpentMossEngine** | Atomic package transaction engine with rollback | ✅ | `src/sigpkg/transaction.rs` | serpent-os/moss |
| **SigmaPkg** | Universal package manager CLI with multi-format support | ✅ | `src/sigpkg/` | pacman, apt, dnf |
| **PackageSnapshotRollback** | Btrfs/ZFS-style package snapshot + differential rollback | ✅ | `src/sigpkg/transaction.rs` | snapper, ostree |
| **UniversalOopSystem** | Multi-distro format verification and rollback hooks | ✅ | `src/sigpkg/universal_oop_system.rs` | Nix, Portage |
| **SigmaAUR** | AUR-compatible community package helper | 🔄 | `src/sigpkg/aur.rs` | paru, yay |

---

## Filesystem

| Component | Description | Status | Source File | Inspired By |
|-----------|-------------|--------|-------------|-------------|
| **SigmaFS** | Native B-tree filesystem with CoW and compression | ✅ | `src/fs/` | Btrfs, ZFS, ext4 |
| **SigmaVFS** | Virtual filesystem switch with POSIX semantics | ✅ | `src/fs/vfs.rs` | Linux VFS |
| **SigmaFUSE** | Userspace filesystem framework | 🔄 | `src/fs/fuse.rs` | FUSE, macFUSE |
| **SigmaZFS** | ZFS-compatible snapshot and pool management | ✅ | `src/fs/zfs_compat.rs` | OpenZFS |
| **SigmaEncrypt** | Transparent filesystem encryption (LUKS2) | ✅ | `src/fs/encrypt.rs` | dm-crypt, LUKS2 |

---

## Networking

| Component | Description | Status | Source File | Inspired By |
|-----------|-------------|--------|-------------|-------------|
| **SigmaNet** | Zero-dependency TCP/IP stack | 🔄 | `src/net/` | smoltcp, lwIP |
| **SigmaFire** | Zone-based firewall with stateful packet inspection | ✅ | `src/net/firewall.rs` | firewalld, pf |
| **SigmaVPN** | WireGuard-inspired VPN tunnel | 🔄 | `src/net/vpn.rs` | WireGuard |
| **SigmaZeroTrust** | Zero-trust network access control model | ✅ | `src/net/zero_trust.rs` | BeyondCorp, Tailscale |
| **SigmaDNS** | Encrypted DNS resolver (DoH/DoT) | 🔄 | `src/net/dns.rs` | systemd-resolved |
| **FreeBsdRacctVnetGuard** | RACCT resource accounting + VNET isolation | ✅ | `src/compat/freebsd.rs` | FreeBSD VNET |

---

## Desktop & UI

| Component | Description | Status | Source File | Inspired By |
|-----------|-------------|--------|-------------|-------------|
| **ZenithCompositor** | DRM/KMS Wayland-inspired compositor | ✅ | `src/desktop/zenith.rs` | Enlightenment, Weston |
| **SigmaWM** | Tiling + stacking + floating window manager | ✅ | `src/desktop/wm.rs` | i3, AwesomeWM, KWin |
| **SigmaBar** | Status bar with system monitoring widgets | ✅ | `src/desktop/bar.rs` | polybar, waybar |
| **SigmaLauncher** | Application launcher with fuzzy search | ✅ | `src/desktop/launcher.rs` | rofi, wofi |
| **SigmaTheme** | Declarative theming engine with CSS-like syntax | ✅ | `src/desktop/theme.rs` | GTK themes |
| **SigmaA11y** | Accessibility layer (screen reader, magnifier) | 🔄 | `src/accessibility/` | AT-SPI, Orca |

---

## Drivers

| Component | Description | Status | Source File | Inspired By |
|-----------|-------------|--------|-------------|-------------|
| **SigmaGPU** | AMD/Intel GPU driver with Vulkan support | 🔄 | `src/drivers/gpu/` | Mesa, AMDGPU |
| **SigmaAudio** | ALSA/PipeWire-compatible audio subsystem | ✅ | `src/audio/` | ALSA, PipeWire |
| **SigmaUSB** | USB host controller and device enumeration | ✅ | `src/drivers/usb/` | Linux USB core |
| **SigmaNIC** | Ethernet + WiFi driver framework | ✅ | `src/drivers/net/` | Linux netdev |
| **SigmaBT** | Bluetooth HCI stack | 🔄 | `src/bluetooth/` | BlueZ |
| **SigmaInput** | Keyboard/mouse/touchpad/touchscreen drivers | ✅ | `src/drivers/input/` | Linux input layer |
| **UnifiedDriverFramework** | Universal driver loading with hardware ID matching | ✅ | `src/driver/device.rs` | Windows WDM |

---

## Virtualization & Containers

| Component | Description | Status | Source File | Inspired By |
|-----------|-------------|--------|-------------|-------------|
| **QubesIsolationManager** | VM compartmentalization + domain isolation | ✅ | `src/vm/qubes.rs` | Qubes OS |
| **SigmaContainer** | Rootless OCI container runtime | ✅ | `src/container/` | podman, runc |
| **SigmaVM** | KVM/QEMU-enhanced virtual machine manager | 🔄 | `src/vm/` | QEMU, bhyve |
| **SigmaWASM** | WebAssembly runtime for sandboxed execution | 🔄 | `src/wasm/` | Wasmtime |
| **SigmaNamespace** | Linux namespace emulation (PID/NET/MNT/UTS) | ✅ | `src/container/namespace.rs` | Linux namespaces |

---

## Init & Service Management

| Component | Description | Status | Source File | Inspired By |
|-----------|-------------|--------|-------------|-------------|
| **SovereignInitSupervisor** | Parallel init daemon with dependency graph | ✅ | `src/init/` | systemd, s6, runit |
| **SigmaService** | Service unit management with socket activation | ✅ | `src/init/service.rs` | systemd units |
| **SigmaSession** | Login session and seat management | ✅ | `src/auth/session.rs` | elogind, logind |
| **SigmaTimer** | Timer-based service activation | ✅ | `src/init/timer.rs` | systemd.timer |
| **SigmaMount** | Automount and mount unit management | 🔄 | `src/fs/mount.rs` | systemd.mount |

---

## AI & Automation

| Component | Description | Status | Source File | Inspired By |
|-----------|-------------|--------|-------------|-------------|
| **SigmaAI** | AI orchestrator for system optimization | 🔄 | `src/ai/` | Cortex, k8s |
| **SigmaCopilot** | AI-assisted CLI and system configuration | 🔄 | `src/ai/copilot.rs` | GitHub Copilot |
| **SigmaPredict** | Predictive prefetch and memory optimization | 🔄 | `src/ai/predict.rs` | inotify, fwupd |

---

## Developer Tools

| Component | Description | Status | Source File | Inspired By |
|-----------|-------------|--------|-------------|-------------|
| **SovereignVcsEngine** | Zero-dependency version control system | ✅ | `src/vcs/` | git, fossil |
| **SigmaDebugger** | Kernel and userspace debugger | 🔄 | `src/debugger/` | gdb, lldb |
| **SigmaBenchmark** | System benchmarking suite | ✅ | `src/benchmarks/` | sysbench |
| **SigmaBuild** | Build system with LTO and multi-arch support | ✅ | `src/build/` | cmake, meson |
| **BtopSystemMonitor** | btop-inspired terminal resource monitor | ✅ | `src/unimplemented_tools.rs` | btop, htop |
| **FastFetchInfo** | fastfetch-inspired system info display | ✅ | `src/unimplemented_tools.rs` | fastfetch |
| **BatSyntaxViewer** | bat-inspired syntax-highlighted file viewer | ✅ | `src/unimplemented_tools.rs` | bat |
| **FastFileSearchEngine** | fd-inspired fast file search engine | ✅ | `src/unimplemented_tools.rs` | fd, find |
| **EbpfSystemTracer** | eBPF-based system performance tracer | ✅ | `src/unimplemented_tools.rs` | bpftrace |

---

## Summary Statistics

| Subsystem | Total | ✅ Done | 🔄 WIP |
|-----------|-------|---------|--------|
| Kernel | 9 | 7 | 2 |
| Security | 10 | 8 | 2 |
| Package Mgmt | 5 | 4 | 1 |
| Filesystem | 5 | 4 | 1 |
| Networking | 6 | 3 | 3 |
| Desktop & UI | 6 | 5 | 1 |
| Drivers | 7 | 5 | 2 |
| Virtualization | 5 | 3 | 2 |
| Init & Services | 5 | 4 | 1 |
| AI & Automation | 3 | 0 | 3 |
| Dev Tools | 9 | 7 | 2 |
| **TOTAL** | **75** | **50 (67%)** | **20 (27%)** |

---

> 💡 **Want to contribute?** See the [Contributing Guide](Contributing) and pick any 🔄 component!

*Last updated: 2026-08-23 | Auto-generated from main repository*
