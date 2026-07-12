# Architecture Overview

SigmaOS is structured as a hierarchical lattice, ensuring that each layer has a strictly defined responsibility and zero-dependency upward.

---

## 10-Layer Lattice Architecture

```
Layer 10: Sovereign Nexus - Enterprise Suite
    ↓
Layer 9: Ecosystem Abstraction - S99
    ↓
Layer 8: Sovereign Claw AI Automation
    ↓
Layer 7: Sovereign AI & Orchestration
    ↓
Layer 6: Zenith UI & Morphic Shell
    ↓
Layer 5: Sovereign Package Ecosystem
    ↓
Layer 4: Capability-Gated Security
    ↓
Layer 3: Sovereign Virtual Filesystem
    ↓
Layer 2: Genesis Kernel & Scheduling
    ↓
Layer 1: Universal Hardware Abstraction
```

---

## Layer Descriptions

### Layer 1: Universal Hardware Abstraction (HAL)

Direct silicon interfaces (NVMe, USB, VGA, GPU, TPM2).

**Responsibilities:**
- Hardware enumeration and initialization
- Device driver loading and management
- Interrupt routing and handling
- ACPI table parsing
- UEFI runtime services integration

### Layer 2: Genesis Kernel & Scheduling

IRQ/IDT handling, memory management, and the SHS scheduler.

**Responsibilities:**
- CPU context switching
- Interrupt handling
- System call dispatch
- Memory management (paging, allocation)
- Process/thread scheduling

**SHS Scheduler:**
Merges the stability of Fedora's CFS with the priority-based preemptive scheduling of Windows.

**Timer Interrupt Flow:**
1. Timer Interrupt (IRQ0): Triggers every 1ms (configurable)
2. Context Save: Current registers are saved via inline ASM
3. Selection: The SHS selects the next task based on virtual runtime and AI priority
4. Quantum Enforcement: Budget enforcement via RDTSC
5. Context Restore: Resumes execution of the selected process

### Layer 3: Sovereign Virtual Filesystem

A capability-backed filesystem that treats all resources as handles.

**Responsibilities:**
- VFS (Virtual File System) abstraction
- File descriptor management
- Path resolution
- File locking and synchronization

**Supported Filesystems:**
- SigmaFS (native CoW journaling filesystem)
- Ext4 (read/write with JBD2 ordered journaling)
- FAT32 (for EFI system partitions)
- Tmpfs (RAM-backed ephemeral storage)
- dm-verity (block-level integrity verification)

### Layer 4: Capability-Gated Security

PQC (Kyber/Dilithium) and TPM 2.0 attestation.

**Responsibilities:**
- sigma_pledge (capability declaration and enforcement)
- sigma_unveil (filesystem path restriction)
- AVC (Access Vector Cache) for MAC decisions
- Zero-trust workload identity (SPIFFE)
- Post-quantum cryptography integration
- TPM2 key sealing and attestation

**Security Chain:**
pledge → unveil → AVC O(1) cache → PQC attestation → TPM2

### Layer 5: Sovereign Package Ecosystem

Dependency DAG management via `sigma-pkg`.

**Responsibilities:**
- Package installation and removal
- Dependency resolution
- Package signature verification
- Repository management
- Delta updates

**Snapshot System:**
Combines openSUSE Snapper-style CoW snapshots with Windows-style System Restore checkpoints, allowing for absolute state recovery at any lattice layer.

### Layer 6: Zenith UI & Morphic Shell

Wayland-native compositor with Morphic shaders.

**Responsibilities:**
- Window management and composition
- Input device handling
- Rendering pipeline
- UI toolkit
- Shell integration

**Performance:**
The Zenith compositor utilizes EGL/Vulkan integration to offload UI transformations directly to the GPU, ensuring a fluid 120Hz interface even under heavy computational load.

### Layer 7: Sovereign AI & Orchestration

The high-level intent-to-shard dispatch system.

**Responsibilities:**
- Natural language command processing
- Intent recognition and routing
- Task orchestration
- Predictive resource management

**AI Integration:**
SigmaOS integrates reinforcement learning models directly into the kernel scheduler and memory manager. The AI Watchdog (S09) predicts resource contention and preemptively triggers rollbacks or re-sharding.

### Layer 8: Sovereign Claw AI Automation

Autonomous AI agent gateway for multi-step goal execution.

**Responsibilities:**
- Agent lifecycle management
- Multi-agent coordination
- Goal decomposition
- Task execution and monitoring

### Layer 9: Ecosystem Abstraction (S99)

POSIX-compatible translation layer for legacy Linux binaries.

**Responsibilities:**
- Linux syscall translation
- POSIX API compatibility
- Binary compatibility layer
- Library translation

### Layer 10: Sovereign Nexus - Enterprise Suite

Integrated Enterprise (ERP/CRM) and Productivity (Office) suite.

**Responsibilities:**
- Enterprise application integration
- Productivity suite
- Data synchronization
- Workflow automation

The Nexus layer (S100) aggregates and enhances the USPs of the world's leading enterprise suites.

---

## Key Architectural Features

### Zero-Trust Communication

All Inter-Shard communication in v11.0 is Zero-Trust. Every packet is:
- Cryptographically signed with Dilithium3
- Encrypted with Kyber-1024
- Attested via SPIFFE workload identity
- Validated per-syscall

### Fast Startup

SigmaOS implements a Fast Startup mechanism inspired by Windows. At shutdown, the kernel state and critical driver shards are serialized to a silicon-direct snapshot. During boot, the system bypasses traditional hardware re-init, restoring the lattice in under 0.8s.

### Predictive Memory Management

The memory manager uses a Neural Network (S09) to predict which shards will be needed next based on user intent. Predicted shards are pre-loaded from NVMe to DRAM, reducing effective latency to near-zero.

### Hardware Abstraction Layer

The HAL (S04) implements plug-and-play detection. Drivers are loaded as atomic shards. Fallback drivers ensure basic I/O availability.

### Sovereign Registry

Inspired by the Windows Registry but reimagined for sovereignty, the Sovereign Registry is a centralized, hierarchical configuration lattice.

### Data Science Integration

All sovereign events are logged in structured JSON/CSV formats by the Sovereign Data Science Shard (S17). This data powers the `sigma-top` dashboard and predictive analytics.

---

## Design Principles

1. **Usability First**: Zenith Compositor + `sigma-pkg`
2. **Security Next**: TPM Attestation + PQC Encryption
3. **Resilience**: Self-Healing Snapshots + AI Watchdog
4. **Differentiation**: Adaptive UI + Sovereign AI Assistant

---

*See also: [Architecture.md](Architecture.md) · [Architecture-Deep-Dive.md](Architecture-Deep-Dive.md) · [Kernel Internals](Kernel)*
