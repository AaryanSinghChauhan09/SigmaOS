# SigmaOS Architecture Overview

> Quick navigation: [Kernel](Kernel) · [HAL](HAL) · [Networking](Networking) · [Security-Model](Security-Model) · [System-Daemons](System-Daemons)

---

## System Layers

```
┌─────────────────────────────────────────────────────────────────┐
│  USER SPACE (Ring 3 / EL0)                                      │
│  PWAs · Zenith Desktop · profession apps · sigma-ai LLM         │
├─────────────────────────────────────────────────────────────────┤
│  BROWSER SHELL (browser profile)                                │
│  Custom Chromium + navigator.sigmaos.* API                      │
├─────────────────────────────────────────────────────────────────┤
│  SYSTEM DAEMONS (Ring 3, pledge-restricted)                     │
│  sigmad-health · sigmad-pkg · sigmad-netd · sigmad-vault        │
├─────────────────────────────────────────────────────────────────┤
│  SYSCALL INTERFACE                                              │
│  sigma_pledge · sigma_unveil · seccomp-BPF · AVC MAC            │
├─────────────────────────────────────────────────────────────────┤
│  KERNEL (Ring 0 — freestanding, no glibc)                       │
│  Scheduler · Memory · Security · Network · Filesystem           │
│  IPC · IRQ/APIC · cgroups · namespaces · eBPF                   │
├─────────────────────────────────────────────────────────────────┤
│  SOVEREIGN HAL — x86_64 · ARM64 · RISC-V                        │
├─────────────────────────────────────────────────────────────────┤
│  HARDWARE — CPU · NVMe · GPU · NIC · USB · TPM2 · UEFI          │
└─────────────────────────────────────────────────────────────────┘
```

---

## Core Subsystems

### Scheduler
- MLFQ (4 queues + aging), CFS clone (vruntime/red-black tree)
- EDF for RTOS profile, NUMA-aware placement
- sigma-ai predictive pre-warming (Phase H)

### Memory
- Buddy allocator (2^n frames), Slab allocator (kmalloc)
- 4-level paging (x86_64 PML4), ASLR 42-bit per VMA, W^X

### Security
- sigma_pledge / sigma_unveil (OpenBSD-inspired)
- AVC O(1) MAC, Zero-trust SPIFFE, TPM2 attestation
- PQC: Kyber-1024 KEM + Dilithium-5 signatures

### Networking
- TCP/IP · TLS 1.3+Kyber · DNS/DoH · DHCP · WPA3 · Firewall

### Filesystem
- VFS → SigmaFS (native CoW) / Ext4 / FAT32 / Tmpfs / dm-verity

### HAL
- PCI/PCIe MSI-X, ACPI (MADT/SRAT/DSDT), UEFI runtime

---

## Shard System

600+ atomic capability modules (`suites/S001–S500+`) — each independently testable and deployable. Shards are merged into profiles at build time via CMake feature flags.

---

## 8 Deployment Profiles

| Profile | Branch | Use Case |
|---------|--------|---------|
| Standalone | `release/standalone` | Developer laptops, workstations |
| Browser | `release/browser` | Chromebook-style thin clients |
| Microkernel | `release/microkernel` | Servers, hypervisors, research |
| Mobile | `release/mobile` | Raspberry Pi, ARM64 tablets |
| RTOS | `release/rtos` | Industrial control, robotics |
| Dual-Boot | `release/dual-boot` | Alongside Windows/Linux |
| Cloud | `release/cloud` | AWS/Azure VMs, BharatCloud |
| Distributed | `release/distributed` | Multi-node clusters |

---

## Key Directories

| Path | Purpose |
|------|---------|
| `kernel/` | Microkernel core |
| `arch/` | x86_64, arm64, riscv64 code |
| `drivers/` | SDF hardware drivers |
| `hal/` | Hardware abstraction |
| `fs/` | Filesystems |
| `net/` | Network stack |
| `security/` | Security subsystems |
| `crypto/` | PQC primitives (Kyber, Dilithium) |
| `suites/` | 600+ capability shards |
| `include/` | All header files |
| `docs/` | Extended documentation |
| `wiki_repo/` | This wiki |

---

*Full spec: [ARCHITECTURE.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/ARCHITECTURE.md) · [Kernel internals](Kernel) · [Development Roadmap](Development-Roadmap)*


---
## Merged from Architecture Overview.md
# Architecture Overview

1 > A ground-truth description of the sovereign 7-layer lattice architecture.

---
1 SigmaOS is structured as a hierarchical lattice, ensuring that each layer has a strictly defined responsibility and zero-dependency upward.
1 graph TD
    L10[Layer 10: Sovereign Nexus - Enterprise Suite] --> L9
    L9[Layer 9: Ecosystem Abstraction - S99] --> L8
    L8[Layer 8: Sovereign Claw AI Automation] --> L7
    L7[Layer 7: Sovereign AI & Orchestration] --> L6
    L6[Layer 6: Zenith UI & Morphic Shell] --> L5
    L5[Layer 5: Sovereign Package Ecosystem] --> L4
    L4[Layer 4: Capability-Gated Security] --> L3
    L3[Layer 3: Sovereign Virtual Filesystem] --> L2
    L2[Layer 2: Genesis Kernel & Scheduling] --> L1
    L1[Layer 1: Universal Hardware Abstraction]

    style L10 fill:#f96,stroke:#333,stroke-width:4px
    style L9 fill:#fcf,stroke:#333,stroke-width:4px
1 1

1. **Hardware Abstraction (HAL)**: Direct silicon interfaces (NVMe, USB, VGA).

2. **Genesis Kernel**: IRQ/IDT handling, memory management, and the SHS scheduler.

3. **Sovereign VFS**: A capability-backed filesystem that treats all resources as handles.

4. **Security Lattice**: PQC (Kyber/Dilithium) and TPM 2.0 attestation.

5. **Package Layer**: Dependency DAG management via `sigma-pkg`.

6. **Zenith UI**: Wayland-native compositor with Morphic shaders.

7. **AI Orchestrator**: The high-level intent-to-shard dispatch system.

8. **Sovereign Claw**: Autonomous AI agent gateway for multi-step goal execution.

9. **Ecosystem Abstraction (S99)**: POSIX-compatible translation layer for legacy Linux binaries.

10. **Sovereign Nexus**: Integrated Enterprise (ERP/CRM) and Productivity (Office) suite.

---
1 The Nexus layer (S100) aggregates and enhances the USPs of the world's leading enterprise suites:
1 1
1 1
1 1
1 1

---
1 1

SHS merges the **stability of Fedora's CFS**with the**priority-based preemptive scheduling of Windows**.

1 1
1 1

Combines **openSUSE Snapper-style CoW snapshots**with**Windows-style System Restore checkpoints**, allowing for absolute state recovery at any lattice layer.

1 All Inter-Shard communication in v11.0 is **Zero-Trust**. Every packet is:
1 1

---
1 1

SigmaOS implements a **Fast Startup**mechanism inspired by Windows. At shutdown, the kernel state and critical driver shards are serialized to a silicon-direct snapshot. During boot, the system bypasses traditional hardware re-init, restoring the lattice in**under 0.8s**.

1 The memory manager uses a **Neural Network (S09)** to predict which shards will be needed next based on user intent. Predicted shards are pre-loaded from NVMe to DRAM, reducing effective latency to near-zero.

1 The Zenith compositor utilizes **EGL/Vulkan** integration to offload UI transformations directly to the GPU, ensuring a fluid 120Hz interface even under heavy computational load.

1. **Timer Interrupt (IRQ0)**: Triggers every 1ms (configurable).

2. **Context Save**: Current registers are saved via inline ASM.

3. **Selection**: The SHS selects the next task based on virtual runtime and AI priority.

4. **Quantum Enforcement**: Budget enforcement via RDTSC.

5. **Context Restore**: Resumes execution of the selected process.

---
1 SigmaOS integrates reinforcement learning models directly into the kernel scheduler and memory manager. The **AI Watchdog** (S09) predicts resource contention and preemptively triggers rollbacks or re-sharding.

1 All sovereign events are logged in structured **JSON/CSV**formats by the**Sovereign Data Science Shard** (S17). This data powers the `sigma-top` dashboard and predictive analytics.

1 The **HAL** (S04) implements plug-and-play detection. Drivers are loaded as atomic shards. Fallback drivers ensure basic I/O availability.

1 1

Inspired by the Windows Registry but reimagined for sovereignty, the **Sovereign Registry** is a centralized, hierarchical configuration lattice.

1 1

---
1 1
1 1
1 1
1 1. **Usability First**: Zenith Compositor + `sigma-pkg`.

1. **Security Next**: TPM Attestation + PQC Encryption.

2. **Resilience**: Self-Healing Snapshots + AI Watchdog.

3. **Differentiation**: Adaptive UI + Sovereign AI Assistant.


---
## Merged from Architecture_Overview.md
# Architecture Overview

1 1
1 graph TD
    L1[Layer 1: Bare-Metal Silicon & TPM]
    L2[Layer 2: HAL & Quantum Watchdog]
    L3[Layer 3: Sovereign Core Kernel]
    L4[Layer 4: Shard Orchestrator & Sandbox]
    L5[Layer 5: Sovereign Package Manager & AI Daemon]
    L6[Layer 6: Zenith Wayland Compositor]
    L7[Layer 7: Sovereign Applications & ML Hub]

    L1 --> L2
    L2 --> L3
    L3 --> L4
    L4 --> L5
    L5 --> L6
    L6 --> L7
1 1

Communication between layers is enforced by the **Sovereign IPC Bus**. No driver can directly access the kernel space without an encrypted capability token.
1 1

The `SovereignHAL` provides a strict interface for hardware interaction. Direct I/O port mapping is prohibited unless verified by the Hardware Attestation TPM driver during boot.
