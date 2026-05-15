# 🖥️ SigmaOS v15.0 Zenith — Standalone Edition

> **The sovereign bare-metal OS. One machine. Total control. Zero compromise.**

[![Release](https://img.shields.io/badge/release-v15.0--zenith--standalone-blue)](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-standalone)
[![Architecture](https://img.shields.io/badge/arch-x86__64%20%7C%20ARM64-green)](https://github.com/AaryanSinghChauhan09/SigmaOS)
[![License](https://img.shields.io/badge/license-Sovereign--MIT-purple)](https://github.com/AaryanSinghChauhan09/SigmaOS)

---

## 📋 Overview

**SigmaOS Zenith Standalone** is the flagship, full-sovereign bare-metal edition of SigmaOS v15.0. It takes complete ownership of your hardware — no hypervisor, no Windows, no Linux substrate. Pure silicon execution.

This edition is built on the **SigmaOS Unified Core**, featuring the mandatory baseline toolset (Maintenance, Productivity, Creative) and layered with bare-metal drivers for maximum performance.

| Property | Value |
|---|---|
| Edition | Zenith Standalone |
| Version | v15.0.0 |
| Kernel | Sovereign Lattice Microkernel v15.0 |
| Architecture | x86_64, ARM64 |
| Boot Mode | UEFI Secure Boot (SSB Algorithm) |
| Target | Dedicated bare-metal hardware |
| Desktop | Zenith Industrial Desktop (Z-DESKTOP) |
| Security | PQC-Hardened (Dilithium-5, Kyber-1024) |

---

## ⚡ Key Features

### 🔒 Security — S-ARMOR Isolation

- **Post-Quantum Cryptography**: Dilithium-5 signatures + Kyber-1024 key exchange baked into the kernel
- **Zero-Trust IPC**: Every inter-process call is authenticated via S-IPC attestation
- **Integrity Measurement Architecture (IMA)**: Cryptographic hash verification of every loaded binary
- **Verified Boot (SSB)**: Secure Shard Bootstrapping — 600 shards ignited in topological order with PQC attestation
- **S-ARMOR**: Mandatory access control with per-process silicon-level isolation
- **TPM 2.0 Integration**: Hardware root-of-trust binding

### 🚀 Performance — Sovereign Lattice

- **CFS Scheduler (S-CFS)**: Completely Fair Scheduler with hardware-aware priority weighting
- **Buddy Allocator**: 4KB–4MB page management via sovereign buddy system
- **Slab Allocator**: Sub-millisecond object allocation for kernel structures
- **NUMA-Aware Memory**: Topologically-optimal memory placement
- **NVMe-Direct I/O**: Bypasses legacy AHCI for 5x throughput improvement
- **GPU-Accelerated Desktop**: Wayland compositor with hardware-accelerated rendering
- **<2ms Boot to Desktop**: SSB fast-boot mode with shard pre-verification cache

### 🖥️ Zenith Industrial Desktop (Z-DESKTOP)

- **Tiling Window Manager**: Three workspaces (Development / Audit / Monitoring) — distraction-free
- **SovereignSpotlight**: Universal launcher with neural-prediction ranking
- **SovereignShell**: POSIX-compatible shell with pipeline completion and color theming
- **SovereignDash**: Real-time silicon telemetry — CPU, MEM, NET, THERMAL, NEURAL
- **Omni Shell**: Polymorphic shell with C, Rust, and Python inline execution
- **Dark Industrial Theme**: High-contrast workspace for 12-hour professional sessions

### 📦 Package Management — sigma-pkg

- `sigma-pkg install <shard>` — Install professional shards from the Lattice Nexus
- `sigma-pkg remove <shard>` — Decommission shards cleanly
- `sigma-pkg list` — Enumerate all active sovereign shards
- `sigma-pkg sync` — Synchronize local lattice with global repository
- PQC-attested package integrity — every shard verified before installation

### 🌐 Networking — SovereignNetStack

- **S-VPN**: Built-in PQC-hardened VPN tunnel (WireGuard substrate)
- **Zero-Config Mesh**: Automatic peer discovery via Sovereign Mesh Lattice
- **Firewall**: Stateful packet inspection with microsecond latency

---

## 💻 System Requirements

| Component | Minimum | Recommended |
|---|---|---|
| CPU | x86_64 (SSE4.2+) or ARM64 | Intel 12th Gen / AMD Zen 4 / Apple M-series |
| RAM | 4 GB | 16 GB+ |
| Storage | 20 GB NVMe/SSD | 100 GB+ NVMe Gen 4 |
| GPU | VESA framebuffer | Vulkan 1.3 compatible (NVIDIA/AMD/Intel) |
| Firmware | UEFI 2.4+ | UEFI 2.6+ with Secure Boot |
| TPM | Optional | TPM 2.0 (for full PQC binding) |
| Network | Ethernet/Wi-Fi | 802.11ax (Wi-Fi 6) or Gigabit Ethernet |

---

## 🛠️ Installation Guide

### Step 1 — Download the ISO

```bash

# Direct download from the official release

curl -LO https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/v15.0-zenith-standalone/SigmaOS-v15.0-Zenith-Standalone-x86_64.iso

# Verify PQC integrity signature

sigma-verify --dilithium5 SigmaOS-v15.0-Zenith-Standalone-x86_64.iso SigmaOS-v15.0-Zenith-Standalone-x86_64.iso.sig
```

### Step 2 — Flash to USB Drive

```bash

# On Linux/macOS (replace /dev/sdX with your USB device)

sudo dd if=SigmaOS-v15.0-Zenith-Standalone-x86_64.iso of=/dev/sdX bs=4M status=progress && sync

# On Windows (using Rufus or balenaEtcher)

# Select ISO → GPT partition scheme → UEFI target → Flash

```

### Step 3 — BIOS/UEFI Configuration

1. Enter BIOS/UEFI setup (usually `F2`, `DEL`, or `F12` during POST)
2. Set **Boot Mode** to `UEFI` (disable Legacy/CSM)
3. Enable **Secure Boot** (SigmaOS ships with its own Secure Boot keys)
4. Set USB drive as **first boot device**
5. Save and reboot

### Step 4 — Boot the Installer

1. SigmaOS Zenith boot menu appears — select **"Install SigmaOS Standalone"**
2. The **SSB (Secure Shard Bootstrapping)** algorithm verifies all 600 shards before install
3. Select installation target disk
4. Choose partition layout:
   - **Automatic** (recommended) — EFI + Swap + Root + Home
   - **Manual** — Full LVM/GPT control

### Step 5 — Disk Partitioning (Manual)

```bash

# Recommended layout for standalone

/dev/sda1  →  512MB   EFI System Partition (FAT32)
/dev/sda2  →  8GB     Swap (encrypted)
/dev/sda3  →  50GB+   / (root)  — Sovereign Lattice FS (SLF)
/dev/sda4  →  rest    /home     — User data (optional encryption)
```

### Step 6 — Complete Installation

1. Set username, hostname, and timezone
2. Configure PQC key generation (Dilithium-5 identity keypair created on first boot)
3. Enable/disable TPM binding
4. Installation completes in approximately 8–15 minutes
5. Reboot — remove USB when prompted

### Step 7 — First Boot

```bash

# Post-install first-boot commands

sigma-pkg sync                          # Sync shard repository

sigma-pkg install s-desktop-zenith      # Ensure desktop shards are current

sigma-sysconfig --setup-networking      # Configure network interfaces

sigma-sysconfig --enable-pqc-daemon     # Start PQC background services

```

---

## 🧩 Available Shards (Packages)

| Shard ID | Description | Category |
|---|---|---|
| `s-desktop-zenith` | Zenith Industrial Desktop (Z-DESKTOP) | UI |
| `s-shell-omni` | OmniShell polymorphic terminal | Shell |
| `s-pkg-manager` | Sovereign Package Manager | Tools |
| `s-security-pqc` | PQC hardening daemon | Security |
| `s-netstack-vpn` | Built-in S-VPN client | Networking |
| `s-fs-vfs` | Sovereign Virtual File System | Filesystem |
| `s-sched-cfs` | Completely Fair Scheduler | Kernel |
| `s-gpu-vulkan` | Vulkan GPU acceleration layer | Graphics |
| `s-dev-suite` | Development environment (GCC, LLVM, Rust) | Development |
| `s-audit-engine` | Financial/compliance auditing suite | Professional |

---

## 🔧 Core Functions Reference

### Boot Engine

```cpp
boot_init();                           // Initialize SSB algorithm
boot_ignite_lattice();                 // Ignite all 600 shards
boot_enable_fast_boot(true);           // Enable <2ms fast-boot mode
sigma_boot_stage_t stage = boot_get_current_stage();  // Query boot state
sigma_u32 count = boot_get_ignited_count();           // Get ignited shard count
```

### Memory Management

```cpp
void* ptr = sigma_malloc(size);        // Sovereign bump-pointer allocation
sigma_free(ptr);                       // Sovereign deallocation
// Buddy allocator (kernel-level)
buddy_allocate(order);                 // Allocate 2^order pages
```

### Package Manager

```bash
sigma-pkg install <shard-id>           # Install a shard

sigma-pkg remove  <shard-id>           # Remove a shard

sigma-pkg list                         # List installed shards

sigma-pkg sync                         # Sync with Lattice Nexus

sigma-pkg verify <shard-id>           # PQC-verify shard integrity

```

### Scheduler

```cpp
scheduler_init();                      // Initialize S-CFS
scheduler_push(task_fn, priority);     // Schedule a task (0–100 priority)
```

### VFS

```cpp
vfs_init();                            // Initialize Sovereign VFS
vfs_mount_node("192.168.1.100");       // Mount distributed storage node
sigma_u32 fd = vfs_open(path, flags); // Open file
vfs_read(fd, buffer, size);           // Read from file
vfs_write(fd, buffer, size);          // Write to file
vfs_close(fd);                         // Close file descriptor
vfs_write_file(path, data);           // Atomic write + replicate
```

---

## 🆘 Support & Resources

- **Release Page**: [v15.0-zenith-standalone](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-standalone)
- **Wiki Home**: [SigmaOS Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)
- **Issue Tracker**: [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- **Kernel Developer Handbook**: [Kernel-Developer-Handbook](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Kernel-Developer-Handbook)
- **Security Policy**: [SECURITY.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/SECURITY.md)

---

*SigmaOS v15.0 Zenith Standalone — Industrial-Grade Sovereign Computing. Built for those who demand total silicon control.*
