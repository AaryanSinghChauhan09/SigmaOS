# Logic: The Sovereign Architecture of SigmaOS

This page explains the core architectural logic, file relationships, and the "Source of Truth" build system within the 600-shard SigmaOS Sovereign Lattice.

## 1. Core Architectural Philosophy

SigmaOS is an **Industrial-Grade, AI-Native Sovereign Operating System**. It follows a **Modular Sharding** strategy to ensure stability, security, and hardware sovereignty.

### Universal Principles

- **Zero-Dependency**: No reliance on standard libraries (`libc`, `libstdc++`). All primitives are silicon-direct.

- **Sovereign Lattice**: A modular mesh of "shards" where each component is an isolated, PQC-attested singleton.

- **Formal Verification Ready**: Code is structured to allow mathematical proof of correctness in critical shards.

- **Universal Sharding**: Everything from drivers to AI personas is a "shard" with a standardized lifecycle.

---

## 2. The Build Logic (SHARDS.manifest)

- **Logic**: The `Makefile` reads `SHARDS.manifest` (audited and normalized) to discover every source file.

- **Relationship**: If a file is not in the manifest, it is not part of the "Singularity". All includes are normalized to root-relative paths for industrial stability.

- **Layers**: The manifest is organized into logical layers (HAL, Core, AI, Industrial, UI) to manage dependency flow.

---

## 3. Foundational Header Hierarchy (The "Core Logic")

To prevent circular dependencies, SigmaOS uses a strictly ordered header lattice:

| Header | Level | Purpose | Relationship |
| :--- | :--- | :--- | :--- |

| `sigma_kernel_types.h` | Base | Fundamental Silicon Types (`sigma_u8`, etc.) | The absolute foundation for all code. |
| `sigma_types.h` | Layer 1 | Status codes, POSIX aliases, Boolean constants. | Includes `sigma_kernel_types.h`. |
| `sigma_libc.h` | Layer 2 | Custom memory and string primitives (`sigma_memcpy`). | Includes `sigma_types.h`. |
| `sigma_log.h` | Layer 2 | Machine-state tracing and industrial logging. | Used by all shards for telemetry. |
| `SigmaOOP.hpp` | Layer 3 | Base class `SigmaObject` for C++ polymorphism. | Foundation for all object-oriented shards. |

| `SovereignLattice.h`| Master | The "Universal Inclusion" header. | Includes all above; entry point for shards. |

---

## 4. Directory Logic & Relationships

Each directory in the SigmaOS repository serves a distinct logical purpose:

### `kernel/` (The Sovereign Brain)

- **`core/`**: Essential services: HAL, PMM/VMM, IPC, AI (Claw Stack), and Security.

- **`shards/`**: Industrial and professional modules (e.g., `medical/`, `finance/`, `engineering/`).

- **`hal/`**: Hardware Abstraction Layer for x86_64, ARM64 (RPi5), and RISC-V.

### `drivers/` (Hardware Sovereignty)

- **`linux_distros/`**: Ported and hardened drivers from the Linux ecosystem to ensure hardware parity.

- **`gpu/`**: Custom Vulkan-native drivers for high-performance Zenith UI.

### `modules/ui/zenith/` (The UX Layer)

- **Logic**: A C-native, GPU-accelerated compositor.

- **Relationship**: Connects the `SovereignSnap` application logic to the physical display via Vulkan.

### `userland/` (The Sovereign Ecosystem)

- **Logic**: System-level agents and utilities (e.g., `update-agent`, `logd`).

- **Relationship**: Operates in the `SovereignSandbox` to ensure kernel-space protection.

### `tools/` (The Industrial Toolchain)

- **Logic**: Custom tools for packaging (`sigma-pkg`), fixing (`sigma-fix`), and debugging.

- **Relationship**: Used by developers to maintain the repository and deploy the OS.

### `WIKI/` & `docs/` (The Knowledge Graph)

- **Logic**: Comprehensive documentation ensuring the system remains professional and accessible.

- **Relationship**: Syncs with GitHub Wiki and Pages to provide a global source of truth.

---

## 5. Interaction & Synchronization Logic

1. **Shard Lifecycles**: Every shard is a Meyers Singleton. They are initialized by the `SovereignBridge` during Layer 0 bootstrap.

2. **C-Bridge Interop**: High-level C++ logic is exposed to C/Asm via `extern "C"` wrappers at the bottom of each shard's `.cpp` file.

3. **Repository Sync**: Local updates are pushed to the GitHub repo and Wiki simultaneously to maintain parity.

---

## 6. Detailed File Relationships (The 600-Shard Lattice)

Below is a non-exhaustive list of critical shards and their relationships to the system:

| File Path | Role | Logic |
| :--- | :--- | :--- |

| `kernel/core/automation/SovereignAutomation.cpp` | Automation Nexus | Orchestrates industrial-specific healing and rollback logic. |
| `kernel/core/ui/SovereignTerminal.cpp` | Terminal Engine | GPU-accelerated CLI emulator for Zenith. |
| `kernel/core/hal/SovereignNUMA.cpp` | NUMA Logic | Multi-node memory and CPU orchestration. |
| `kernel/core/drivers/SovereignNVMe.cpp` | NVMe Driver | Ultra-low latency industrial storage. |
| `kernel/core/security/SovereignUserAccounts.cpp` | Identity Matrix | Multi-tenant UID/GID management. |
| `kernel/core/hal/SovereignVMM.cpp` | Virtual Memory | Demand-paging and Page-Table orchestration. |
| `kernel/core/drivers/SovereignUSB.cpp` | USB Stack | XHCI/EHCI controller and device discovery. |
| `kernel/core/drivers/SovereignAudio.cpp` | Audio Engine | HDA controller and low-latency synthesis. |
| `kernel/core/system/SovereignCoreUtils.cpp` | Core Utilities | ls, cat, grep, cp primitives. |
| `kernel/shards/finance/SovereignIndianProfessionalTools.cpp` | Professional Tools | GST and Income Tax calculators (FY 2024-25). |
| `kernel/shards/legal/SovereignBNS.cpp` | BNS Legal Shard | Mappings and lookups for Bharatiya Nyaya Sanhita 2023. |
| `kernel/core/hal/SovereignHAL.cpp` | Hardware Entry | Orchestrates bus probing and driver registration. |
| `kernel/core/hal/SovereignPMM.cpp` | Physical Memory | Managed Silicon-Direct page allocation (Bitmap-based). |
| `kernel/core/ipc/SovereignBridge.cpp` | IPC Gateway | The primary bus for inter-shard communication. |
| `kernel/core/ai/SovereignClawGateway.cpp` | AI Master | Entry point for the "Claw" AI automation stack. |
| `kernel/core/security/SovereignPQC.cpp` | Security Master | Enforces lattice-wide PQC signature verification. |
| `kernel/core/system/SovereignProfiles.cpp` | Personality | Dynamically re-tunes the kernel for specific profiles. |
| `modules/ui/zenith/zenith_compositor.c` | UI Core | Bare-metal compositor for the Zenith desktop. |
| `modules/ui/zenith/zenith_theme_engine.c` | UI Persona | Logic for industrial-grade skinning and accessibility. |
| `userland/update-agent.cpp` | Lifecycle | Manages the atomic rolling-update process (PQC-signed). |
| `tools/sigma-pkg.sh` | Ecosystem | The industrial-grade package manager for Sovereign Shards. |

---

## 7. Operational Flow Logic

1. **Hardware Probe**: `SovereignHAL` scans the PCIe/USB/DeviceTree nodes.

2. **Lattice Initialization**: `SovereignBridge` initializes Layer 1-3 shards in the order specified in `SHARDS.manifest`.

3. **Security Attestation**: `SovereignPQC` verifies the integrity of each loaded shard.

4. **UI Singularity**: `ZenithCompositor` takes control of the framebuffer and renders the authenticated UI.

5. **Adaptive Execution**: `SovereignProfiles` adjusts scheduler quotas and memory pressure based on active shards.

---

### Document Version: 15.0.4 - Stabilized

