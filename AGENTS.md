# AGENTS.md — AI Agent Guidelines, Versioning, Circular Buffers, Cache Memory, Process, Hardware Fitting, Backend, Loading, Processor & Network Management for SigmaOS

This document provides instructions, rules, and procedures for AI agents working in the SigmaOS repository, specifically regarding **Version Handling**, **Release Channels**, **Circular Buffer & Ring Buffer Management**, **Cache Memory Management**, **Process Lifecycle & Signal ABI Management**, **Hardware Fitting & Driver Auto-Binding**, **Network Stack & eBPF Management**, **Processor Subsystem Management**, **Backend Management**, **Bootloader & Driver Loading**, **Multi-Distro Packaging Parity**, and **Core Subsystem Changes**.

---

## 1. Core Principles & Philosophy

* **Zero External Dependencies:** SigmaOS kernel and core userland maintain a 100% self-sufficient `#![no_std]` Rust architecture. Do NOT introduce third-party external crates to `Cargo.toml`.
* **Semantic Versioning (SemVer 2.0.0):** All core components follow `MAJOR.MINOR.PATCH` versioning scheme.
* **Always Verify Code Changes:** Run `./run_sigma_tests.sh` to ensure all 13 test execution steps (unit, integration, python verification, multi-distro adapters) pass cleanly after making modifications.

---

## 2. Versioning Standards & Rules for AI Agents

When modifying, releasing, or updating versions in SigmaOS:

### 2.1 Core Repository & Cargo Version
* Core package version is declared in `Cargo.toml` (`version = "0.1.0"`).
* **MAJOR (x.0.0):** Incompatible API/ABI or kernel architecture changes (e.g., breaking KABI stability).
* **MINOR (0.x.0):** New backward-compatible kernel subsystems, drivers, or distro parity features.
* **PATCH (0.0.x):** Backward-compatible bug fixes, performance optimizations, or security patches.

---

## 3. Circular Buffer & Lock-Free Ring Buffer Rules for AI Agents

When implementing or modifying ring buffers in `src/klib/ring_buffer.rs`, `src/klib/ringbuf.rs`, or `src/media/sovereign_video_player.rs`:

1. **Power-of-Two Capacity Rule:**
   Ring buffer capacities MUST be powers of two ($2^k$) to perform $O(1)$ index wrapping via bitwise AND `idx & (capacity - 1)`.
2. **Atomic Head/Tail Pointer Synchronization:**
   Single-producer single-consumer (SPSC) ring buffers MUST synchronize `head` and `tail` offsets using `AtomicUsize` with `Acquire`/`Release` memory orderings.
3. **Media Zero-Copy Ring Buffers (`VlcLightweightMediaPipeline`):**
   Real-time video/audio frame buffers MUST manage zero-copy frame recycling without memory allocation during playback.

---

## 4. Cache Memory Architecture, LRU Eviction & Package Cache Rules for AI Agents

When modifying cache memory engines, key-value stores, or package cache trimmers:

1. **Key Invalidation Invariant:**
   `SovereignCacheEngine::set` MUST purge pre-existing entries with matching keys via `self.entries.retain(|e| e.key != key)` before inserting new values.
2. **TTL Expiration:**
   Items with non-zero TTLs MUST be evaluated against system uptime and purged automatically upon access.

---

## 5. Process Lifecycle, Signal ABI Translation & Supervision Rules for AI Agents

When modifying process management, signal handlers, or pseudo-terminals:

1. **State Machine Transitions (`SovereignProcessLifecycleController`):**
   Ensure process state changes (`Created`, `Ready`, `Running`, `Blocked`, `Stopped`, `Zombie`, `Terminated`) execute under thread-safe synchronization.
2. **Process $O(1)$ Name Lookup:**
   Process structs MUST store `name_len: u8` initialized during creation to guarantee $O(1)$ `Process::name()` performance.

---

## 6. Hardware Fitting, Driver Auto-Binding & Device Adaptation Rules for AI Agents

When writing, probing, or modifying hardware device drivers (`src/drivers/`):

1. **Bus Signature Probing:**
   Driver probe routines MUST evaluate Vendor ID (VID), Product ID (PID), and interface class codes before claiming attachment.

---

## 7. Network Stack, eBPF/XDP & PQC Security Rules for AI Agents

When modifying networking drivers, eBPF filters, or VPN subsystems:

1. **Kernel Bypass eBPF/XDP Processing:**
   Use zero-copy DMA ring buffers (`process_xdp_zero_copy_packet`). Ensure XDP actions explicitly return `XDP_PASS`, `XDP_DROP`, `XDP_TX`, or `XDP_REDIRECT`.

---

## 8. Processor Topology, CPU Scheduling & Multi-Core Rules for AI Agents

When modifying CPU scheduling, task management, or ISA optimization:

1. **ISA Level Auto-Detection (`src/klib/isa.rs`):**
   Support x86-64 microarchitecture levels (`v1`..`v4`). Route vectorized operations via `vectorized_memcpy` dynamically based on detected features.

---

## 9. Kernel, Bootloader & Driver Loading Rules for AI Agents

When modifying boot sequences, driver registration, or scheduler loading:

1. **Multi-Stage Boot Pipeline:**
   Respect the 4-phase boot sequence: Bootloader -> Kernel Initialization -> Dynamic Driver Loading -> Userland Supervisor.

---

## 10. Backend Subsystem & Server Engine Rules for AI Agents

When modifying backend services in `src/open_source_obsoletion.rs`, `src/open_source_os_gap_closure.rs`, or `src/automation/system_level.rs`:

1. **Zero-Dependency Native Backend Engines:**
   Maintain native parity for embedded DBs (`SovereignEmbeddedDb`), web servers (`SovereignWebServer`), in-memory caches (`SovereignCacheEngine`), message brokers (`SovereignMessageBroker`), secret vaults (`SovereignSecretVault`), object stores (`SovereignDistributedStorage`), and orchestrators (`SovereignK8sOrchestratorEngine`).

---

## 11. Checklist for AI Agents

1. **Update Manifests & Documentation** when bumping versions, adding drivers, or modifying ring buffers.
2. **Run Standalone Subsystem Tests:**
   ```bash
   rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/open_source_os_gap_closure.rs
   ```
3. **Execute Full Pipeline:** Run `./run_sigma_tests.sh` and ensure all test steps pass.
4. **Follow Conventional Commits:**
   `feat(klib): optimize power-of-two ring buffer index wrap` or `docs(agents): add circular buffer guide`.

---

## 12. Detailed Documentation References

For technical specifications, see:
* [`docs/AGENTS_VERSION_HANDLING.md`](docs/AGENTS_VERSION_HANDLING.md)
* [`docs/AGENTS_BACKEND_MANAGEMENT.md`](docs/AGENTS_BACKEND_MANAGEMENT.md)
* [`docs/AGENTS_LOADING_MANAGEMENT.md`](docs/AGENTS_LOADING_MANAGEMENT.md)
* [`docs/AGENTS_PROCESSOR_MANAGEMENT.md`](docs/AGENTS_PROCESSOR_MANAGEMENT.md)
* [`docs/AGENTS_NETWORK_MANAGEMENT.md`](docs/AGENTS_NETWORK_MANAGEMENT.md)
* [`docs/AGENTS_FITTING_MANAGEMENT.md`](docs/AGENTS_FITTING_MANAGEMENT.md)
* [`docs/AGENTS_PROCESS_MANAGEMENT.md`](docs/AGENTS_PROCESS_MANAGEMENT.md)
* [`docs/AGENTS_CACHE_MEMORY_MANAGEMENT.md`](docs/AGENTS_CACHE_MEMORY_MANAGEMENT.md)
* [`docs/AGENTS_CIRCULAR_BUFFER_MANAGEMENT.md`](docs/AGENTS_CIRCULAR_BUFFER_MANAGEMENT.md)
* [`docs/RELEASE_CADENCE.md`](docs/RELEASE_CADENCE.md)
* [`docs/package-manager.md`](docs/package-manager.md)
