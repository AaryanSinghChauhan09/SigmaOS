# SigmaOS Components Table

This wiki page provides a comprehensive table of all major SigmaOS components, their status, and key details.

## Core OS Components

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 1 | **Kernel Core** | `src/kernel/main.rs` | Main kernel entry point, bootstrapping, HAL init | ✅ Active | Linux monolithic kernel |
| 2 | **Scheduler (MLFQ)** | `src/kernel/sched/sigma_mlfq.rs` | Multi-level feedback queue scheduler | ✅ Active | Linux CFS + FreeBSD ULE |
| 3 | **Thermal Scheduler** | `src/kernel/sched/sigma_thermal_sched.rs` | Thermally-aware CPU scheduling | ✅ Active | CachyOS BORE scheduler |
| 4 | **Task Manager** | `src/kernel/sched/task.rs` | Task lifecycle and state management | ✅ Active | Linux task_struct |
| 5 | **Process Signals** | `src/kernel/proc/signals.rs` | POSIX signal delivery & masking | ✅ Active | Linux signals |
| 6 | **Kernel Components** | `src/kernel/component.rs` | Modular kernel component registry | ✅ Active | Zircon microkernel |

## Memory Management

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 7 | **Memory Zones** | `src/memory/zone.rs` | DMA/Normal/High zone allocator | ✅ Active | Linux NUMA zones |
| 8 | **kswapd Daemon** | `src/memory/kswapd.rs` | Background memory reclamation | ✅ Active | Linux kswapd |
| 9 | **cgroups v2** | `src/memory/cgroups.rs` | Control groups for resource limiting | ✅ Active | Linux cgroups v2 |
| 10 | **Buddy Allocator** | `src/buddy.rs` | Buddy system page allocator | ✅ Active | Linux buddy allocator |
| 11 | **Slab Allocator** | `src/slab.rs` | Object-level slab cache allocator | ✅ Active | Linux SLUB/SLAB |
| 12 | **Virtual Memory** | `src/mm/` | Virtual address space management | ✅ Active | Linux VMM |

## Boot & Init

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 13 | **UEFI Bootloader** | `src/boot/uefi.rs` | UEFI-based secure boot implementation | ✅ Active | GNU GRUB / systemd-boot |
| 14 | **SigmaBoot** | `src/sigma-boot/` | Custom OS bootloader | ✅ Active | rEFInd |
| 15 | **Init System** | `src/init/` | PID 1 and service initialization | ✅ Active | systemd / runit |
| 16 | **Installer** | `src/installer/` | OS installation framework | ✅ Active | Calamares |

## Filesystem & Storage

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 17 | **Filesystem Core** | `src/filesystem/` | VFS layer and filesystem abstraction | ✅ Active | Linux VFS |
| 18 | **Storage Manager** | `src/storage/` | Block device and storage abstraction | ✅ Active | Linux block layer |
| 19 | **FS klib** | `src/klib/fs.rs` | Kernel-space filesystem utilities | ✅ Active | Linux fs/ internal |
| 20 | **Compression** | `src/compression/` | LZ4/Zstd/Zlib compression subsystem | ✅ Active | NixOS store compression |

## Security

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 21 | **Security Core** | `src/security/mod.rs` | Security subsystem coordinator | ✅ Active | Linux LSM framework |
| 22 | **Qubes Isolation** | `src/security/qubes_isolation.rs` | VM-based domain isolation | ✅ Active | Qubes OS |
| 23 | **Vulnerability Scanner** | `src/security/vulnerability.rs` | Runtime vulnerability detection | ✅ Active | OpenBSD pledge/unveil |
| 24 | **PKI Manager** | `src/security/pki.rs` | Certificate management & key store | ✅ Active | OpenSSL / BoringSSL |
| 25 | **Pledge** | `src/security/pledge.rs` | OpenBSD-style syscall restriction | ✅ Active | OpenBSD pledge |
| 26 | **Unveil** | `src/security/unveil.rs` | Filesystem path restriction | ✅ Active | OpenBSD unveil |
| 27 | **Secrets Manager** | `src/security/secrets.rs` | Kernel-space secret storage | ✅ Active | Linux keyring |
| 28 | **Audit Logger** | `src/security/audit.rs` | Security event auditing | ✅ Active | Linux audit / SELinux |
| 29 | **Capability Enforcer** | `src/security/capability_enforcer.rs` | POSIX capabilities enforcement | ✅ Active | Linux capabilities |
| 30 | **Post-Quantum Crypto** | `src/crypto/` | Kyber KEM + Dilithium signatures | ✅ Active | NIST PQC standards |
| 31 | **TPM Integration** | `src/tpm/` | TPM 2.0 attestation & sealing | ✅ Active | Linux TPM driver |

## Networking

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 32 | **Network Core** | `src/network/` | TCP/IP stack implementation | ✅ Active | Linux net/ |
| 33 | **TCP/UDP Stack** | `src/network/tcp_udp.rs` | Transport layer protocols | ✅ Active | Linux TCP stack |
| 34 | **Network Analyzer** | `src/network/analyzer.rs` | Packet inspection & analysis | ✅ Active | Wireshark / tcpdump |
| 35 | **Wireless** | `src/wireless/` | Wi-Fi and wireless management | ✅ Active | wpa_supplicant |
| 36 | **Bluetooth** | `src/bluetooth/` | Bluetooth stack | ✅ Active | BlueZ |

## Package Management

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 37 | **Package Core** | `src/package/mod.rs` | Universal package management | ✅ Active | APT/DNF/Pacman |
| 38 | **Universal Package** | `src/package/universal.rs` | Cross-format package adapter | ✅ Active | Flatpak / AppImage |
| 39 | **Package Store** | `src/package/store.rs` | Local package database | ✅ Active | dpkg database |
| 40 | **SigmaPkg** | `src/sigpkg/mod.rs` | Native SigmaOS package manager | ✅ Active | Pacman / Portage |
| 41 | **MakePkg** | `src/sigpkg/makepkg.rs` | Package build system | ✅ Active | Arch AUR makepkg |
| 42 | **Recipe System** | `src/sigpkg/recipe.rs` | Declarative package recipes | ✅ Active | NixOS nix expressions |
| 43 | **Universal Adapter** | `src/sigpkg/universal_adapter.rs` | .deb/.rpm/.pkg.tar adapter | ✅ Active | Alien package converter |

## AI & Machine Learning

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 44 | **AI Orchestrator** | `src/ai/orchestrator.rs` | Multi-agent AI task routing | ✅ Active | k8s operator pattern |
| 45 | **ML Inference** | `src/ml/inference.rs` | On-device ML inference engine | ✅ Active | ONNX Runtime |
| 46 | **ML Training** | `src/ml/training.rs` | Local model fine-tuning | ✅ Active | MLflow |
| 47 | **NLP Engine** | `src/nlp/` | Natural language processing | ✅ Active | spaCy / Hugging Face |
| 48 | **AI Daemon** | `src/ai/` | Background AI system daemon | ✅ Active | Novel design |

## Desktop & UI

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 49 | **Desktop Environment** | `src/desktop/` | Full desktop session management | ✅ Active | GNOME / KDE Plasma |
| 50 | **Graphics Core** | `src/graphics/` | GPU-accelerated rendering | ✅ Active | Mesa / Wayland |
| 51 | **GPU Subsystem** | `src/gpu/` | GPU driver framework | ✅ Active | Mesa DRI |
| 52 | **HAL (GPU)** | `src/hal/` | Hardware abstraction layer | ✅ Active | Linux DRM/KMS |
| 53 | **UI Framework** | `src/ui/` | Native UI toolkit | ✅ Active | GTK4 / Qt |
| 54 | **Input System** | `src/input/` | Keyboard, mouse, touchscreen | ✅ Active | libinput |
| 55 | **Touchscreen** | `src/touchscreen/` | Multi-touch gesture handling | ✅ Active | libinput gestures |
| 56 | **Accessibility** | `src/accessibility/` | a11y support, screen reader | ✅ Active | AT-SPI2 |

## Drivers & Hardware

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 57 | **Driver Framework** | `src/driver/framework.rs` | Universal driver model | ✅ Active | Linux driver model |
| 58 | **Driver Core** | `src/drivers/` | Built-in hardware drivers | ✅ Active | Linux kernel drivers |
| 59 | **USB Stack** | `src/usb/` | USB HCD and device drivers | ✅ Active | Linux USB |
| 60 | **Audio** | `src/audio/` | Audio subsystem (ALSA-like) | ✅ Active | ALSA / PipeWire |
| 61 | **Camera** | `src/camera/` | Camera device support | ✅ Active | V4L2 |
| 62 | **Sensor Manager** | `src/sensor/` | Hardware sensors (temp, accel) | ✅ Active | IIO subsystem |
| 63 | **Power Management** | `src/power/` | ACPI/DPMS/sleep states | ✅ Active | Linux ACPI |
| 64 | **Thermal Manager** | `src/thermal/` | Thermal zone management | ✅ Active | Linux thermal |
| 65 | **Bluetooth HW** | `src/bluetooth/` | Bluetooth controller drivers | ✅ Active | BlueZ hciattach |

## Compatibility Layers

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 66 | **Compatibility Core** | `src/compatibility/mod.rs` | Multi-distro compatibility layer | ✅ Active | WSL2 / Proton |
| 67 | **AntiX Compat** | `src/compatibility/antix.rs` | AntiX/MX Linux compatibility | ✅ Active | AntiX Linux |
| 68 | **Distro Parity** | `src/distro/` | Distro-specific feature parity | ✅ Active | Multiple distros |

## Containerization & Virtualization

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 69 | **Container Runtime** | `src/container/runtime.rs` | Native container execution | ✅ Active | containerd / runc |
| 70 | **SigmaKube** | `src/orchestration/sigmakube.rs` | Kubernetes-like orchestration | ✅ Active | Kubernetes |
| 71 | **Virtualization** | `src/virtualization/` | VM management | ✅ Active | KVM / QEMU |
| 72 | **Virt Manager** | `src/virt/` | Virtual machine controller | ✅ Active | libvirt |

## Kernel Library (klib) — Zero-Dependency Custom Implementations

| # | Component | Module Path | Description | Status | Replaces |
|---|-----------|-------------|-------------|--------|---------|
| 73 | **Custom String** | `src/klib/custom_string.rs` | Heap-free string impl | ✅ Active | `std::string::String` |
| 74 | **Custom Vec** | `src/klib/vec.rs` | No-std dynamic array | ✅ Active | `std::vec::Vec` |
| 75 | **Custom HashMap** | `src/klib/` | Hash map without std | ✅ Active | `std::collections::HashMap` |
| 76 | **BTreeMap** | `src/klib/btreemap.rs` | Balanced tree ordered map | ✅ Active | `std::collections::BTreeMap` |
| 77 | **HashSet** | `src/klib/hashset.rs` | No-std hash set | ✅ Active | `std::collections::HashSet` |
| 78 | **Path Utils** | `src/klib/path.rs` | Kernel path manipulation | ✅ Active | `std::path::Path` |
| 79 | **Process Utils** | `src/klib/process.rs` | Process management primitives | ✅ Active | `std::process` |
| 80 | **Time Utils** | `src/klib/time.rs` | Monotonic time management | ✅ Active | `std::time` |
| 81 | **Env Utils** | `src/klib/env.rs` | Environment variable access | ✅ Active | `std::env` |

## Shell & Tooling

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 82 | **Shell REPL** | `src/shell/repl.rs` | Interactive kernel shell | ✅ Active | fish / zsh |
| 83 | **Sigma Tools** | `src/tools/sigmatools.rs` | OS utility toolkit | ✅ Active | coreutils / busybox |
| 84 | **Debugger** | `src/debugger/` | Kernel debugger (KDB-like) | ✅ Active | Linux kgdb |
| 85 | **Diagnostics** | `src/diagnostics/` | System diagnostics framework | ✅ Active | systemd-analyze |
| 86 | **Observability** | `src/observability/` | Metrics, tracing, logging | ✅ Active | eBPF / Prometheus |

## Automation & Scripting

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 87 | **Automation Engine** | `src/automation/script.rs` | System automation scripting | ✅ Active | Ansible / systemd timers |
| 88 | **Workflow Manager** | `src/workflow/` | Task workflow orchestration | ✅ Active | GNU Make / Meson |
| 89 | **Remote Desktop** | `src/remote/desktop.rs` | Remote desktop protocol | ✅ Active | RDP / VNC |

## Resilience & Recovery

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 90 | **Backup System** | `src/resilience/backup.rs` | Automated backup & restore | ✅ Active | Timeshift / rsync |
| 91 | **Crash Reporter** | `src/crash/` | Crash dump collection | ✅ Active | Linux kdump |
| 92 | **Recovery Mode** | `src/recovery/` | System recovery environment | ✅ Active | Linux rescue mode |

## ELF Loader

| # | Component | Module Path | Description | Status | Inspired By |
|---|-----------|-------------|-------------|--------|-------------|
| 93 | **ELF Loader** | `src/loader/elf/mod.rs` | ELF binary loading | ✅ Active | Linux binfmt_elf |
| 94 | **ELF Relocator** | `src/loader/elf/relocation.rs` | ELF relocation processing | ✅ Active | ld-linux.so |

## CI/CD & Build

| # | Component | Module Path | Description | Status | Notes |
|---|-----------|-------------|-------------|--------|-------|
| 95 | **CI Pipeline** | `.github/workflows/sigma-ci.yml` | Automated build & test | ✅ Active | GitHub Actions |
| 96 | **Security Scanning** | `.github/workflows/appknox.yml` | Mobile & static security scan | ✅ Active | Appknox / CodeQL |
| 97 | **SBOM Generation** | `.github/workflows/nowsecure.yml` | Software bill of materials | ✅ Active | NowSecure |
| 98 | **Snyk Scanning** | `.github/workflows/snyk.yml` | Dependency vulnerability scan | ✅ Active | Snyk |
| 99 | **Parallel Build** | `tools/build/parallel.rs` | Parallel compilation framework | ✅ Active | Ninja build |
| 100 | **Static Linker** | `tools/build/static_linker.rs` | Custom static linking tool | ✅ Active | lld / mold |

## Open Source Obsoletion

| # | Component | Module Path | Description | Status | Notes |
|---|-----------|-------------|-------------|--------|-------|
| 101 | **OSS Obsoletion** | `src/open_source_obsoletion.rs` | Replace external deps with native | ✅ Active | Novel design |
| 102 | **Unimplemented Features** | `src/unimplemented_features.rs` | Feature backlog tracker | 🔄 Planned | Internal tracking |

---

## Component Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ Active | Implemented and functional |
| 🔄 Planned | Designed but not yet implemented |
| 🚧 In Progress | Partially implemented |
| ⚠️ Experimental | Implemented but unstable |

---

*Last updated: 2026-08-23 | Auto-generated from SigmaOS main branch*
