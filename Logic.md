# SigmaOS Logic & File Relationships

This page explains the core architectural logic and file relationships within the 600-shard SigmaOS Sovereign Lattice.

## Core Architectural Philosophy
SigmaOS is built on the **Sovereign Lattice** architecture. It is a zero-dependency, bare-metal operating system.
Key principles:
1.  **Zero-Dependency**: No external dependencies (e.g., standard C/C++ libraries like STL, libc). All primitives are custom-built.
2.  **Modular Sharding**: The system is composed of hundreds of isolated "shards" (singletons) that handle specific responsibilities.
3.  **Strict Polymorphism**: Interfaces use abstract base classes (`SigmaObject`) without dynamic casting or RTTI.

## The Tiered Header Structure
The entire kernel resolves dependencies through a strictly ordered inclusion hierarchy to prevent circular dependencies.

### 1. `SovereignBareMetal.h`
**Purpose:** Defines absolute hardware-level primitives (`sigma_u8` through `sigma_u64`, `sigma_size_t`) mapped directly to compiler intrinsics.
**Relationship:** This is the foundational file. Every other file in the kernel implicitly depends on these types.

### 2. `sigma_types.h`
**Purpose:** Defines composite types, POSIX aliases, boolean constants (`SIGMA_TRUE`/`SIGMA_FALSE`), and status codes.
**Relationship:** Safely includes `SovereignBareMetal.h`. Used across all C and C++ subsystems.

### 3. `sigma_libc.h` & `SovereignLibC.cpp`
**Purpose:** Zero-dependency, wait-free LibC primitives for the kernel (`sigma_memset`, `sigma_strlen`, `sigma_strcat`, etc.).
**Relationship:** Included by the central lattice header to provide basic memory/string operations without external linking.

### 4. `sigma_log.h`
**Purpose:** Provides wait-free, circular shard logging and machine-state tracing macros (`sigma_log_info`, `sigma_log_warn`, `sigma_log_err`).
**Relationship:** Included by the central lattice header. All kernel shards use these macros for telemetry and diagnostics.

### 5. `SigmaOOP.hpp`
**Purpose:** Defines the `SigmaObject` base class for polymorphic dispatch in C++ shards.
**Relationship:** Included only when compiled with C++. Serves as the foundational base for all sovereign shards.

### 6. `SovereignLattice.h` (The Master Header)
**Purpose:** The single-include umbrella for all kernel-space shards.
**Relationship:** Includes all the tier 1-5 headers in the correct order. **Every kernel shard (`.cpp` and `.hpp`) must include ONLY this header to resolve system dependencies.**

## Key Kernel Subsystems (Shards)

*   **SovereignDiag (`SovereignDiag.hpp` / `SovereignDiag.cpp`)**: Monitors silicon-level health and anomaly detection. Singleton pattern.
*   **SovereignMonitor (`SovereignMonitor.hpp` / `SovereignMonitor.cpp`)**: High-performance telemetry engine utilizing wait-free circular logging and eBPF-style silicon probes. Singleton pattern.
*   **SovereignSandbox (`SovereignSandbox.hpp` / `SovereignSandbox.cpp`)**: Provides zero-trust execution environments for kernel shards to prevent lateral movement. Singleton pattern.
*   **SovereignPQC (`SovereignPQC.hpp` / `SovereignPQC.cpp`)**: Post-Quantum Cryptography engine for lattice-based shard verification. Singleton pattern.
*   **SovereignSnap (`SovereignSnap.hpp` / `SovereignSnap.cpp`)**: Immutable application packaging and snapshot management subsystem. Singleton pattern.
*   **SovereignVulkanLoader (`SovereignVulkanLoader.cpp`)**: GPU driver binding and Vulkan 1.3 ICD surface initialisation shard. Direct silicon coupling (no virtual dispatch).

## Interaction Logic
1.  **Initialization**: Each shard is a Meyers Singleton (e.g., `SovereignDiag::getInstance()`). They are initialized via a C-Bridge function (e.g., `diag_init()`) called during the kernel boot sequence.
2.  **C-Bridge**: To allow cross-language compatibility (C and assembly interacting with C++ shards), `extern "C"` wrappers are provided at the end of each shard's `.cpp` file. These wrappers call the methods on the singleton instances.
3.  **Isolation**: Shards do not directly include each other. They interact through the global `SovereignLattice.h` and communicate via predefined interfaces to maintain strict modularity.
