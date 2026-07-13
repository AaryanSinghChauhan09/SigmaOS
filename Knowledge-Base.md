# SigmaOS Arch-Wiki: Knowledge Base

> **SigmaOS Knowledge Base** — A community-maintained reference in the tradition of the Arch Wiki, covering installation, configuration, troubleshooting, subsystem internals, and development.

---

## 📋 Table of Contents

### Getting Started
- [Installation Guide](#installation-guide)
- [Boot Process](#boot-process)
- [First Configuration](#first-configuration)
- [System Update](#system-update)

### Kernel & Core
- [Kernel Architecture](#kernel-architecture)
- [Scheduler (EEVDF)](#scheduler)
- [Memory Management](#memory-management)
- [IPC System](#ipc-system)

### Hardware
- [Driver Framework](#driver-framework)
- [GPU Drivers](#gpu-drivers)
- [Networking Hardware](#networking-hardware)
- [Thermal & Power Management](#thermal--power-management)

### Security
- [Mandatory Access Control](#mandatory-access-control)
- [Secure Boot & TPM](#secure-boot--tpm)
- [Cgroup Isolation](#cgroup-isolation)
- [sigma-shield Firewall](#sigma-shield-firewall)
- [Post-Quantum Cryptography](#post-quantum-cryptography)

### Package Management
- [sigpkg Overview](#sigpkg-overview)
- [Creating Packages](#creating-packages)
- [Package Signing](#package-signing)

### Development
- [Building SigmaOS](#building-sigmaos)
- [Writing Drivers](#writing-drivers)
- [Kernel Hacking Guide](#kernel-hacking-guide)
- [SDK Reference](#sdk-reference)

---

## Installation Guide

### Prerequisites

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | x86_64 (Haswell+) | x86_64 or ARM64 |
| RAM | 512 MB | 4 GB+ |
| Storage | 4 GB | 32 GB+ NVMe |
| Firmware | UEFI or BIOS | UEFI + Secure Boot |

### Installation Methods

#### 1. ISO Boot (Recommended)

```bash
# Burn the ISO to USB
dd if=sigmaos-v0.2-x86_64.iso of=/dev/sdX bs=4M status=progress sync

# Boot from USB, then run the installer
sigma-install --target /dev/sda --profile desktop
```

#### 2. QEMU (Testing/Development)

```bash
qemu-system-x86_64 \
  -enable-kvm \
  -m 2G \
  -drive format=qcow2,file=sigmaos.qcow2 \
  -cdrom sigmaos-v0.2-x86_64.iso \
  -boot d \
  -serial stdio \
  -vga virtio
```

See the [full QEMU guide](Installation.md) for networking and GPU passthrough.

#### 3. Dual Boot

SigmaOS supports dual-boot with Windows and Linux. See [Dual-Boot-Compatibility-Matrix.md](Dual-Boot-Compatibility-Matrix.md).

---

## Boot Process

SigmaOS uses a two-stage boot:

```
UEFI/BIOS
    │
    ▼
sigma-boot (stage 1)          ← Multiboot2 header, memory map
    │
    ▼
sigma-boot (stage 2)          ← Long mode, paging, load kernel ELF
    │
    ▼
kernel/init/sigma_init.rs     ← BSP init, SMP, interrupt controllers
    │
    ▼
sigmad (PID 1)                ← Service manager, socket activation
    │
    ▼
userland                      ← Shell, desktop, applications
```

**Key boot parameters** (passed via GRUB/sigma-boot):

| Parameter | Description |
|-----------|-------------|
| `sigma.profile=desktop` | Load desktop profile |
| `sigma.loglevel=4` | Kernel verbosity (0-7) |
| `sigma.nosmp` | Disable SMP (single CPU) |
| `sigma.pqc=dilithium5` | Force PQC signing algorithm |
| `sigma.audit=1` | Enable tamper-evident audit logging |

---

## Kernel Architecture

SigmaOS uses a **hybrid microkernel** architecture:

- **Ring 0**: Core kernel (scheduler, memory, IPC, interrupt handling)
- **Ring 1**: Trusted drivers (GPU, NVMe, network — elevated but isolated)
- **Ring 3**: Userland (applications, untrusted drivers, POSIX compat layer)

```
┌─────────────────────────────────────────────────────┐
│                    Applications                      │  Ring 3
│   sigma-shell  │  zenith-desktop  │  sigma-browser  │
├─────────────────────────────────────────────────────┤
│              POSIX Compatibility Layer               │  Ring 3
│         (sigma_posix.rs — 50 syscalls, O(log n))    │
├─────────────────────────────────────────────────────┤
│                  Sovereign Syscall ABI               │  Ring 0/3 boundary
├─────────────────────────────────────────────────────┤
│  Scheduler │  VFS  │  IPC  │  Memory  │  Security   │  Ring 0
│  (EEVDF)   │       │       │  (PMM+VMM)│  (MAC+PQC) │
├─────────────────────────────────────────────────────┤
│                Hardware Abstraction Layer            │  Ring 0/1
│     Thermal  │  Power  │  IRQ  │  DMA  │  IOMMU     │
├─────────────────────────────────────────────────────┤
│                    Hardware                          │
└─────────────────────────────────────────────────────┘
```

---

## Scheduler

SigmaOS uses the **EEVDF (Earliest Eligible Virtual Deadline First)** scheduler, similar to Linux 6.6+.

### Features

- Nanosecond-precision virtual runtime tracking
- Per-CPU runqueues with work-stealing
- Real-time FIFO and Round-Robin classes
- AI-assisted workload prediction (optional)
- CPU affinity and NUMA-awareness

### Tuning

```toml
# sigma.toml — scheduler section
[scheduler]
eevdf_slice_ns = 3_000_000      # 3 ms time slice (default)
rt_priority_boost = true         # Boost RT tasks
ai_prediction = false            # Experimental AI scheduler
preempt_mode = "full"           # none | voluntary | full
```

---

## Memory Management

### Page Allocator (PMM)

SigmaOS uses a **buddy allocator** for physical memory management:

- 2^0 to 2^10 page orders (4 KiB to 4 MiB)
- NUMA-aware allocation policies
- Huge page (2 MiB, 1 GiB) support
- Memory hotplug support

### Virtual Memory (VMM)

- 4-level paging (PML4 on x86_64)
- Copy-on-Write (CoW) for process fork
- Memory-mapped files with page cache
- `mmap`, `mprotect`, `mlock` sovereign equivalents

### OOM Handling

When memory pressure is critical, the **Sovereign OOM Daemon** (`sigma-oom`) selects victims based on:
1. cgroup memory limits
2. Process OOM score
3. Process uptime and priority

---

## Driver Framework

All drivers implement the `SigmaDriver` trait:

```rust
pub trait SigmaDriver {
    fn name(&self) -> &str;
    fn probe(&mut self) -> Result<(), DriverError>;
    fn remove(&mut self);
    fn suspend(&mut self) -> Result<(), DriverError>;
    fn resume(&mut self) -> Result<(), DriverError>;
}
```

### Driver Rings

| Ring | Type | Examples |
|------|------|---------|
| Ring 1 | Trusted | NVMe, GPU, NIC |
| Ring 3 | Untrusted | USB accessories, printers |
| Module | Dynamically loadable | Third-party hardware |

---

## Thermal & Power Management

See `kernel/hal/thermal/mod.rs` for the implementation.

### Power Profiles

| Profile | Max CPU | TDP Limit | Use Case |
|---------|---------|-----------|---------|
| `power-saver` | 800 MHz | 15 W | Long battery life |
| `balanced` | 2.4 GHz | 45 W | Default daily use |
| `performance` | 5.2 GHz | 125 W | Compute workloads |
| `silent` | 1.8 GHz | 25 W | Quiet media playback |

### Thermal Trip Points (Default — CPU)

| Temperature | Severity | Action |
|-------------|----------|--------|
| < 70 °C | Normal | No action |
| 70–85 °C | Warning | Reduce boost clocks |
| 85–100 °C | Critical | DVFS throttle to 2.4 GHz |
| > 100 °C | Emergency | Immediate 800 MHz + log |

### Changing Profile

```bash
sigma power set-profile performance
sigma power status
sigma thermal show-zones
```

---

## sigma-shield Firewall

See `kernel/net/firewall/sigma_shield.rs`.

### Default Policy

SigmaOS ships with a **default-drop** egress and ingress policy. Rules must explicitly allow traffic.

### Basic Configuration

```bash
# Allow HTTPS
sigma-shield add --direction ingress --proto tcp --dst-port 443 --action accept

# Rate-limit SSH
sigma-shield add --direction ingress --proto tcp --dst-port 22 --action rate-limit:10

# View rules
sigma-shield list

# View stats
sigma-shield stats
```

### Rule File Format

```toml
# /etc/sigma/firewall.toml
[[rule]]
id = 1
priority = 100
direction = "ingress"
protocol = "tcp"
dst_port = 443
action = "accept"
comment = "Allow HTTPS inbound"

[[rule]]
id = 2
priority = 90
direction = "ingress"
conn_state = "established"
action = "accept"
comment = "Allow established connections"
```

---

## Cgroup Isolation

See `kernel/security/cgroups/mod.rs`.

### Creating a Cgroup

```bash
# Create a new cgroup for a web server
sigma-cg create webserver /

# Limit to 2 CPU cores and 512 MiB RAM
sigma-cg set webserver cpu.quota=200000 cpu.period=100000
sigma-cg set webserver memory.limit=536870912

# Start nginx inside the cgroup
sigma-cg run webserver -- nginx -g "daemon off;"
```

### cgroup v2 Interface

SigmaOS exposes cgroup control via `/sigma/cg/`:

```
/sigma/cg/
├── webserver/
│   ├── cgroup.procs       # PIDs in this group
│   ├── cpu.max            # quota period
│   ├── memory.max         # hard limit
│   ├── io.max             # I/O limits
│   └── net.cls.classid    # network class
```

---

## sigpkg Overview

`sigpkg` is the sovereign package manager for SigmaOS. It does not depend on POSIX libc.

### Basic Commands

```bash
sigpkg install firefox        # Install a package
sigpkg remove firefox         # Remove a package
sigpkg update                 # Update package index
sigpkg upgrade                # Upgrade all packages
sigpkg search "video player"  # Search packages
sigpkg info mpv               # Show package details
sigpkg verify mpv             # Verify cryptographic signature
```

### Package Format (`.spkg`)

```
package.spkg/
├── MANIFEST.toml         # Metadata, version, deps, signatures
├── files/                # Installed files
├── pre_install.sh        # Pre-install hook
├── post_install.sh       # Post-install hook
└── signature.dilithium5  # PQC signature
```

---

## Building SigmaOS

### Prerequisites

```bash
# Install Rust (nightly)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
rustup component add rust-src llvm-tools-preview

# Install build dependencies
sudo apt install cmake ninja-build nasm grub-common xorriso
```

### Build Commands

```bash
# Full build (desktop profile)
just build-desktop

# Minimal core build
just build-core

# Build ISO
just iso

# Run in QEMU
just qemu

# Run tests
just test
```

---

## Post-Quantum Cryptography

SigmaOS implements NIST-standardized PQC algorithms:

| Algorithm | Use | Standard |
|-----------|-----|---------|
| Kyber-1024 | Key encapsulation | FIPS 203 |
| Dilithium5 | Digital signatures | FIPS 204 |
| SPHINCS+-256 | Hash-based signatures | FIPS 205 |
| BLAKE3 | Hashing | — |

All package signatures and kernel module signatures use **Dilithium5**.

### Verifying a Package Signature

```bash
sigpkg verify --key /etc/sigma/pqc/public.dilithium5 package.spkg
```

---

## Troubleshooting

### Kernel Panic at Boot

1. Boot with `sigma.loglevel=7` for verbose output
2. Check serial console output (COM1 at 115200 baud)
3. Inspect `/sigma/logs/kernel.log` from recovery shell

### Package Install Fails

```bash
sigpkg verify <package>   # Check signature
sigpkg doctor             # Run diagnostics
sigpkg clean-cache        # Clear stale cache
```

### Network Not Available

```bash
sigma-net list-interfaces          # Show all NICs
sigma-net dhcp start eth0          # Request DHCP lease
sigma-shield list                  # Check firewall rules
sigma-net diagnose                 # Run connectivity diagnostics
```

---

*This knowledge base is a living document. Contributions welcome via the standard [contributing guide](CONTRIBUTING.md).*
