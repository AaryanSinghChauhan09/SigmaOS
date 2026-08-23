# 🏗️ SigmaOS Architecture Overview

SigmaOS is built around a **sovereign microkernel** architecture prioritizing security, zero dependencies, and multi-architecture portability.

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     USER APPLICATIONS                     │
│  (Zenith Desktop, sigma-pkg, sigma-tools, Web Browser)   │
├─────────────────────────────────────────────────────────┤
│                    SYSTEM SERVICES                        │
│  SigmaInit │ SigmaNet │ SigmaAudio │ SigmaX11 │ Wayland  │
├─────────────────────────────────────────────────────────┤
│                    SECURITY LAYER                         │
│  SigmaMAC │ SigmaDAC │ SigmaRBAC │ pledge/unveil │ Audit  │
├─────────────────────────────────────────────────────────┤
│                   KERNEL SUBSYSTEMS                       │
│  Scheduler │ Memory │ IPC │ VFS │ Network │ Devices       │
├─────────────────────────────────────────────────────────┤
│              HARDWARE ABSTRACTION LAYER (HAL)             │
│       x86_64        │     AArch64      │    RISC-V 64    │
├─────────────────────────────────────────────────────────┤
│                       HARDWARE                            │
│    CPU │ RAM │ Storage │ Network │ GPU │ USB │ PCIe      │
└─────────────────────────────────────────────────────────┘
```

---

## Microkernel Design

SigmaOS uses a **hybrid microkernel** approach:

| Layer | Components | Design Philosophy |
|-------|-----------|-------------------|
| **Ring 0 (Kernel)** | Scheduler, Memory, IPC, Interrupts | Minimal, verified |
| **Ring 1 (Drivers)** | Device drivers, HAL | Isolated via capabilities |
| **Ring 2 (Services)** | Init, Network, Audio | Restartable, sandboxed |
| **Ring 3 (Userland)** | Apps, shell, GUI | Full pledge/unveil isolation |

---

## Key Subsystems

### 🧠 SigmaKernel (Microkernel Core)
- Capability-based access control at the kernel level
- IPC via ALPC (Asynchronous Local Procedure Calls)
- Virtual memory with demand paging and CoW
- SMP with per-CPU scheduler queues

### 📅 CachyBoreScheduler
- BORE (Burst-Oriented Response Enhancer) algorithm
- Interactive IPC scoring for desktop responsiveness
- P/E core affinity for hybrid Intel/ARM CPUs
- EEVDF (Earliest Eligible Virtual Deadline First) integration

### 🔒 Security Stack
```
PostQuantumTls (Kyber-1024 + Dilithium-5)
    └── SELinuxEngine (AVC caching + policy)
        └── SigmaMAC (Bell-LaPadula MAC)
            └── SigmaRBAC (Role-based policies)
                └── SigmaDAC (POSIX ACLs)
                    └── pledge/unveil (per-process)
                        └── Capabilities (bounding sets)
```

### 📦 Package Management
```
sigma-pkg (CLI)
    └── SerpentMossEngine (transaction engine)
        ├── PackageSnapshotRollback (Btrfs/ZFS snapshots)
        └── UniversalOopSystem (multi-format parser)
            ├── .pkg (native)
            ├── .deb (Debian)
            ├── .rpm (Fedora)
            └── ALPM (Arch)
```

---

## Memory Architecture

```
Virtual Address Space (x86_64):
┌──────────────────────────────┐ 0xFFFFFFFFFFFFFFFF
│   Kernel Space (256 TB)      │
│   Direct mapped, no-exec     │
├──────────────────────────────┤ 0xFFFF800000000000
│   Non-canonical hole         │
├──────────────────────────────┤ 0x00007FFFFFFFFFFF
│   Userspace (128 TB)         │
│   ASLR enabled               │
│   per-process capabilities   │
└──────────────────────────────┘ 0x0000000000000000
```

---

## Boot Sequence

```
Power On → UEFI Firmware
    → Secure Boot verification
    → SigmaInit bootloader (GRUB or systemd-boot)
    → Kernel decompression + ELF loading
    → SigmaKernel init (BSP core)
    → Memory manager initialization
    → AP (Application Processor) SMP startup
    → VFS mounting (SigmaFS root)
    → SovereignInitSupervisor PID 1
    → Service dependency graph resolution
    → Desktop environment launch (ZenithCompositor)
    → Login manager (SigmaSession)
```

---

## Multi-Architecture HAL

| Arch | HAL Status | Boot | SMP | Interrupts |
|------|-----------|------|-----|-----------|
| x86_64 | ✅ | UEFI + BIOS | ✅ APIC | ✅ IDT |
| AArch64 | 🔄 | UEFI + U-Boot | 🔄 GIC | 🔄 GICv3 |
| RISC-V 64 | 🔄 | SBI (OpenSBI) | 🔄 PLIC | 🔄 PLIC |

---

*See also: [Kernel Internals](Kernel-Internals) | [Boot Process](Boot-Process-Architecture) | [Components Master Table](Components-Master-Table)*
