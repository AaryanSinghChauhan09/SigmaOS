# 🔬 SigmaOS v15.0 Zenith — Core Edition

> **The sovereign microkernel, bare. No GUI. Pure silicon computation.**

[![Release](https://img.shields.io/badge/release-v15.0--zenith--core-red)](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-core)
[![Architecture](https://img.shields.io/badge/arch-x86__64%20%7C%20ARM64%20%7C%20RISC--V-green)](https://github.com/AaryanSinghChauhan09/SigmaOS)
[![Mode](https://img.shields.io/badge/mode-headless%20%7C%20server%20%7C%20embedded-darkblue)](https://github.com/AaryanSinghChauhan09/SigmaOS)

---

## 📋 Overview

**SigmaOS Zenith Core** is the headless, GUI-free edition of the Sovereign Lattice Microkernel. It exposes the raw kernel ABI directly — no desktop, no window manager, no display server. Just the microkernel, the sovereign shell, and your workload.

This edition is designed for **servers, CI/CD nodes, embedded systems, container hosts, and kernel developers** who need the full power of the Sovereign Lattice without any graphical overhead.

| Property | Value |
|---|---|
| Edition | Zenith Core |
| Version | v15.0.0 |
| Kernel | Sovereign Lattice Microkernel v15.0 |
| Architecture | x86_64, ARM64, RISC-V (experimental) |
| Interface | CLI only — OmniShell + sigma-pkg |
| Boot Mode | UEFI / Legacy BIOS / PXE Network Boot |
| Target | Servers, CI nodes, embedded, kernel developers |
| Desktop | None (headless) |
| Memory Footprint | ~128 MB at idle |

---

## ⚡ Key Features

### 🧬 Sovereign Lattice Microkernel — Full Exposure

- **Direct Kernel ABI Access**: Syscall table fully documented and accessible via C, C++, Rust wrappers
- **600-Shard Architecture**: Load/unload kernel shards at runtime via `sigma-shard`
- **Real-Time Kernel Extensions**: Deterministic scheduling extensions for RT workloads
- **Sovereign Syscall Table**: 48 custom syscalls with PQC-attested execution
- **Live Kernel Patching**: Hot-patch kernel shards without reboot via `sigma-livekernel`
- **DTrace-Compatible Tracing**: Low-overhead kernel event tracing via SovereignDTrace
- **eBPF-Equivalent**: SovereignBPF for safe kernel-space compute without recompilation

### 🔒 Security — Maximum Hardening

- **S-ARMOR Mandatory Access Control**: Process isolation at hardware boundary
- **PQC Key Daemon**: Background Dilithium-5/Kyber-1024 key rotation
- **IMA Kernel Module Verification**: Every loaded shard hash-verified against PQC signature
- **Namespace Isolation**: Full namespace support (PID, NET, MNT, UTS, USER, IPC)
- **cgroup v2**: Resource limiting and accounting at kernel level
- **Audit Log**: Kernel-level audit trail with tamper-evident signatures

### 🚀 Server/Headless Performance

- **2 MB Kernel Image**: Minimal footprint for embedded deployment
- **<100ms Boot**: Headless SSB boot with minimal shard set
- **TCP/IP Stack**: Full SovereignNetStack with DPDK-equivalent memory-mapped I/O
- **Sovereign VFS**: Distributed file system replication across up to 8 storage nodes
- **NVMe-Direct**: Pass-through NVMe access for database workloads
- **NUMA Topology Awareness**: Memory and thread placement optimized per NUMA domain

### 📡 Remote Management

- **SSH Server**: Built-in sovereign SSH daemon (PQC-encrypted sessions)
- **REST API**: Kernel management REST API via `sigma-api-gateway` shard
- **Telemetry**: Prometheus-compatible metrics exposure via `sigma-telemetry`
- **Remote Shard Deployment**: Push shards to remote nodes via `sigma-pkg push`

---

## 💻 System Requirements

| Component | Minimum | Recommended |
|---|---|---|
| CPU | x86_64 (64-bit), ARM64, RISC-V64 | Multi-core server CPU |
| RAM | 512 MB | 4 GB+ |
| Storage | 4 GB | 20 GB+ NVMe |
| Network | 100 Mbps Ethernet | 10 GbE+ |
| Boot | UEFI, Legacy BIOS, or PXE | UEFI 2.4+ |
| GPU | Not required | Not required |
| Display | Not required | Not required |

---

## 🛠️ Installation Guide

### Method A — Physical / VM Installation

```bash

# Download Core ISO

curl -LO https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/v15.0-zenith-core/SigmaOS-v15.0-Zenith-Core-x86_64.iso

# Flash to USB

sudo dd if=SigmaOS-v15.0-Zenith-Core-x86_64.iso of=/dev/sdX bs=4M status=progress && sync
```

Boot from USB → Select **"Install SigmaOS Core (Headless)"**

Minimal partition layout:

```bash
/dev/sda1  →  512MB   EFI (UEFI only)
/dev/sda2  →  2GB     Swap
/dev/sda3  →  rest    /  (root, SLF)
```

### Method B — PXE Network Boot (Server Farm Deployment)

```bash

# On TFTP/PXE server — configure pxelinux:

# pxelinux.cfg/default:
label sigmaos-core
  kernel sigmaos-v15.0-core-vmlinuz
  initrd sigmaos-v15.0-core-initrd.img
  append root=/dev/nfs nfsroot=192.168.1.1:/exports/sigmaos ip=dhcp quiet

# On NFS server:

sudo mkdir /exports/sigmaos
sudo tar -xf SigmaOS-v15.0-Core-rootfs.tar.gz -C /exports/sigmaos
sudo exportfs -a
```

### Method C — Container / VM Image

```bash

# Docker/Podman (Core userspace image)

docker pull ghcr.io/aaaryansinghchauhan09/sigmaos-core:v15.0-zenith

# QEMU KVM (full kernel in VM)

qemu-system-x86_64 \
  -m 2G \
  -drive file=SigmaOS-v15.0-Core.qcow2,format=qcow2 \
  -enable-kvm \
  -cpu host \
  -smp 4 \
  -nographic \
  -serial mon:stdio
```

### Method D — Embedded / ARM64 Deployment

```bash

# Flash to SD card / eMMC (Raspberry Pi 4, Jetson Nano, etc.)

sudo dd if=SigmaOS-v15.0-Core-arm64.img of=/dev/mmcblk0 bs=4M status=progress && sync

# Configure for your SBC in /boot/config.sigma:

SOC=bcm2711
UART=ttyAMA0
BAUD=115200
```

---

## 🔧 Core Functions Reference

### sigma-shard — Kernel Shard Manager

```bash
sigma-shard list                        # List all loaded kernel shards

sigma-shard load s-netstack-core        # Load a shard into kernel

sigma-shard unload s-netstack-core      # Unload a shard gracefully

sigma-shard status s-vfs-core           # Check shard operational status

sigma-shard reload s-scheduler          # Hot-reload a shard

sigma-shard verify <shard>              # PQC-verify shard signature

sigma-shard hotpatch <shard> <patch>    # Apply a live patch to running shard

```

### sigma-diag — System Diagnostics

```bash
sigma-diag cpu                          # CPU topology, temperature, frequency

sigma-diag mem                          # Memory usage, NUMA topology, buddy state

sigma-diag net                          # Network interface statistics

sigma-diag sched                        # Scheduler runqueue inspection

sigma-diag vfs                          # VFS shard status and mount points

sigma-diag security                     # IMA measurements, PQC key status

sigma-diag full                         # Complete diagnostic report

```

### sigma-livekernel — Hot Patching

```bash
sigma-livekernel --patch <patch.slp>    # Apply live kernel patch

sigma-livekernel --rollback             # Roll back last live patch

sigma-livekernel --list-patches         # Show applied patches

sigma-livekernel --verify <patch.slp>   # Verify patch PQC signature

```

### Syscall Reference (Core Syscalls)

```c
// Process management
sigma_sys_fork()            // Fork process
sigma_sys_exec(path, argv)  // Execute binary
sigma_sys_exit(code)        // Exit process
sigma_sys_wait(pid, status) // Wait for child

// File system
sigma_sys_open(path, flags)         // Open file → fd
sigma_sys_read(fd, buf, size)       // Read from fd
sigma_sys_write(fd, buf, size)      // Write to fd
sigma_sys_close(fd)                 // Close fd
sigma_sys_stat(path, stat_buf)      // File metadata

// Memory
sigma_sys_mmap(addr, size, prot, flags, fd, offset)
sigma_sys_munmap(addr, size)
sigma_sys_brk(addr)                 // Heap expansion

// Networking
sigma_sys_socket(domain, type, protocol)
sigma_sys_bind(fd, addr, len)
sigma_sys_listen(fd, backlog)
sigma_sys_accept(fd, addr, len)
sigma_sys_connect(fd, addr, len)
sigma_sys_send(fd, buf, size, flags)
sigma_sys_recv(fd, buf, size, flags)

// Security
sigma_sys_pqc_sign(data, len, key)      // Sign with Dilithium-5
sigma_sys_pqc_verify(data, sig, pubkey) // Verify PQC signature
sigma_sys_set_sarmor_policy(pid, policy) // Apply S-ARMOR policy
```

### sigma-api-gateway — REST API (when enabled)

```bash

# Start kernel management API

sigma-shard load s-api-gateway
sigma-api-gateway --port 8443 --tls

# Example API calls

curl https://localhost:8443/api/v1/shards          # GET all shards

curl -X POST https://localhost:8443/api/v1/shards/s-netstack/reload  # Reload shard

curl https://localhost:8443/api/v1/metrics          # Prometheus metrics

curl https://localhost:8443/api/v1/syscall-table    # Full syscall reference

```

---

## 📊 Kernel Shard Catalog (Core Edition)

| Shard | Purpose | Auto-Loaded |
|---|---|---|
| `s-kernel-core` | Base microkernel | ✅ Always |
| `s-scheduler-cfs` | Completely Fair Scheduler | ✅ Always |
| `s-mm-buddy` | Buddy page allocator | ✅ Always |
| `s-mm-slab` | Slab object allocator | ✅ Always |
| `s-vfs-core` | Virtual File System | ✅ Always |
| `s-netstack-core` | TCP/IP networking | ✅ Default |
| `s-security-ima` | Integrity Measurement | ✅ Default |
| `s-security-pqc` | PQC crypto daemon | ✅ Default |
| `s-ssh-daemon` | SSH server | ⬜ Optional |
| `s-api-gateway` | REST management API | ⬜ Optional |
| `s-telemetry` | Prometheus metrics | ⬜ Optional |
| `s-dtrace` | Kernel event tracing | ⬜ Optional |
| `s-bpf-engine` | SovereignBPF runtime | ⬜ Optional |
| `s-livekernel` | Hot-patch engine | ⬜ Optional |

---

## 🆘 Support & Resources

- **Release Page**: [v15.0-zenith-core](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-core)
- **Kernel Developer Handbook**: [Kernel-Developer-Handbook](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Kernel-Developer-Handbook)
- **Syscall Reference**: [SYSCALLS.md](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/SYSCALLS)
- **Architecture Deep-Dive**: [Architecture.md](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture)

---

*SigmaOS v15.0 Zenith Core — The sovereign microkernel, unobstructed.*
