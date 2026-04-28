# Σ SIGMAOS SOVEREIGN API REFERENCE (v20.0)

This document provides a comprehensive technical reference for the **SigmaOS Micro-Shard API**. All functions listed here are zero-dependency primitives engineered for bare-metal silicon control.

---

## 🧩 SOVEREIGN LIBC (ZENITH)

The Sovereign LibC is a modular, zero-dependency implementation of essential C primitives, optimized for direct hardware interaction and memory sharding.

### 📡 I/O PRIMITIVES

- `sigma_printf(const char* format, ...)`: Industrial-grade formatted output with support for `%s`, `%d`, `%u`, `%llu`, `%llx`, `%p`, and improved `%f` (bare-metal float handling).
- `sigma_print(const char* str)`: Direct syscall-backed string print.
- `sigma_log(const char* msg)`: Labeled system logging shard.

### 🧵 STRING PRIMITIVES

- `sigma_strlen(const char* s)`: ASM-optimized string length calculation.
- `sigma_streq(const char* s1, const char* s2)`: Sovereign string equality check.
- `sigma_atoi(const char* s)`: String to integer conversion shard.

### 🧠 MEMORY MANAGEMENT

- `sigma_malloc(sigma_size_t size)`: Bump-pointer slab allocator (128 MB default shard).
- `sigma_free(void* ptr)`: Per-process shard cleanup (no-op by design for speed).

---

## 🧩 CORE KERNEL API

### `kprintf(const char* fmt, ...)`

Industrial-grade formatted output to the primary VGA buffer and serial UART.

- **Parameters**: `fmt` (Format string), `...` (Arguments).
- **Mode**: Sovereign Sync.

---

## 📡 INTER-PROCESS COMMUNICATION (IPC)

### `ipc_send(u64 receiver, u32 type, void* data, u32 size)`

Sends an asynchronous message to a target micro-shard.

- **receiver**: Target shard ID.
- **type**: Message type identifier.
- **data**: Pointer to message payload.
- **size**: Payload size (max 256 bytes).

---

## 💾 STORAGE & FILESYSTEM

### `summon_shard(const char* name, void* buffer, u32 size)`

Linguistically summons a dormant shard from the filesystem into the active lattice.

---

## 🔒 SECURITY & TRUST

### `secure_boot_verify(void* shard_data, u32 size, shard_sig_t* sig)`

Performs a cryptographic audit of a shard binary before activation.

### `rbac_check(u64 shard_id, bool_t net_req, bool_t fs_req)`

Enforces zero-trust capability grants at the shard level.

---

## 💎 SOVEREIGN OOP FRAMEWORK (C++)

SigmaOS supports high-level OOP sharding with zero standard library dependencies.

### 🏛️ SIGMA OBJECT MODEL

- **`SigmaOS::SigmaObject`**: The base class for all sovereign shards. Provides a unified `type_name()` interface for neural synchronization.
- **Zero-Dependency `new` / `delete`**: The framework provides global overrides for `new` and `delete` that interface directly with the `sigma_malloc` and `sigma_free` shards.

### 🧩 MODULAR SHARD CLUSTERS (v51.0)

- **Morphic UI Designer**: Industrial-grade glassmorphism rasterization engine for real-time Zenith UI sharding.
- **Morphic Automation Engine**: Declarative Aether recipes for silicon-native automation and neural balancing.
- **Cold Storage Lattice**: Decentralized state persistence shard using IPFS-native content addressing.
- **Theme Personalizer**: Dynamic aesthetic sharding for kernel-level personalization and color orchestration.
- **Scholar Zenith**: Modular NCERT simulation shards for Physics, Chemistry, Biology, and Math.
- **Collective Conscious**: Neural synchronization agents for mesh-level awareness.

---

## 🛡️ SECURITY & NETWORKING SHARDS

### 🔐 SECURITY LATTICE (PQC)

- **`SovereignLatticePQC`**: Post-Quantum Cryptography shard.
- **`generate_sovereign_key()`**: Generates high-entropy lattice keys using hardware-level `RDRAND` entropy.
- **`encrypt(const char* plaintext)`**: Quantum-resistant vector transformation for secret sharding.

### ☁️ CLOUD MAESTRO (NET)

- **`CloudMaestro`**: Sovereign cloud orchestrator.
- **`SyncWithGlobalLattice()`**: Atomic state synchronization across mesh nodes.
- **`ShowCloudMatrix()`**: Displays the current RDMA-synchronized cloud node topology.

---

## 🎨 ZENITH UI FRAMEWORK (v15.0)

The **Zenith Experience Layer** is the graphical summit of SigmaOS, providing a "satisfactory feeling" comparable to modern premium operating systems (Windows 11, macOS, Android).

### Core Aesthetics

- **Frosted Glass (Glassmorphism)**: Advanced CSS backdrop-filters for real-time silicon-native transparency.
- **Vibrant Accents**: High-contrast Neon Cyan and Deep Dark Blue background palettes.
- **Spring Physics**: All UI interactions utilize kernel-native motion sharding for fluid 120Hz responsiveness.

### Integrated Components

- **Zenith Dashboard**: Real-time silicon diagnostics widget cluster (CPU/RAM/Security).
- **Notification Matrix**: Live kernel-log projection from the Aether Orchestrator.
- **Shard Launcher**: Graphical entry point for modular apps (Scholar Zenith, Cloud Maestro).

---

## 🏛️ VFS SHARD (v2.0 - OOP SOVEREIGNTY)

The **Sovereign VFS** has been refactored into a high-performance C++ object model, enabling modular filesystem sharding and type-safe node management.

- **`VfsNode`**: Abstract base class for all filesystem objects (files, directories, device nodes).
- **`SovereignVFS`**: The core orchestration layer for node registration and discovery.
- **`RegisterNode(VfsNode* node)`**: Mounts a new shard into the virtual filesystem lattice.
- **`FindNode(const char* name)`**: Recursively searches the active lattice for a target node.

## 👤 PERSONALIZATION & IDENTITY

The **Persona Shard** provides declarative system identity and aesthetic orchestration.

- **`SovereignPersona`**: Manages the user's declarative profile, theme, and cognitive latency.
- **`UpdateTheme(ZenithTheme theme)`**: Hot-swaps the kernel's aesthetic policy across the 500-shard lattice.
- **`SyncWithLattice()`**: Synchronizes the user's identity across distributed silicon nodes.

---

## 🚀 DEPLOYMENT & SYNC

- **Relative Path Sovereignty (v60.0)**: Absolute include resolution parity across all shards.
- **Industrial Build Orchestrator**: PowerShell-based automation with integrated lattice hardening.
- **Repository**: Fully synchronized with GitHub remote (Industrial Grade).
- **Parity**: 100% Zero-Dependency / Bit-Perfect execution.
