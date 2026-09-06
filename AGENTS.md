# AGENTS.md — AI Agent Guidelines, Versioning, HTML Dependency Reduction, Compile-Time Defenses, Clock, Circular Buffers, Cache Memory, Process, Hardware Fitting, Backend, Loading, Processor & Network Management for SigmaOS

This document provides instructions, rules, and procedures for AI agents working in the SigmaOS repository, specifically regarding **Version Handling**, **Release Channels**, **HTML Dependency Reduction & Text-First Architecture**, **Compile-Time Defenses & Build Hardening**, **Clock & Timer Management**, **Circular Buffer & Ring Buffer Management**, **Cache Memory Management**, **Process Lifecycle & Signal ABI Management**, **Hardware Fitting & Driver Auto-Binding**, **Network Stack & eBPF Management**, **Processor Subsystem Management**, **Backend Management**, **Bootloader & Driver Loading**, **Multi-Distro Packaging Parity**, and **Core Subsystem Changes**.

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

## 3. HTML Dependency Reduction & Text-Based Interface Rules for AI Agents

When creating or modifying documentation, dashboards, or user interfaces:

1. **Text-First & Terminal Preference:**
   Prioritize Markdown (`DocFormat::Markdown`), AsciiDoc (`DocFormat::AsciiDoc`), or ANSI terminal output over HTML web rendering.
2. **HTML Entity Escaping (`escape_html`):**
   If HTML string output is necessary, ALL dynamic string parameters MUST be sanitized via `escape_html` in `src/docs/mod.rs` to neutralize XSS vectors (`<`, `>`, `&`, `"`, `'`).
3. **Progressive Enhancement:**
   Web interfaces MUST support zero-JS progressive enhancement fallbacks without requiring dynamic HTML DOM injection.

---

## 4. Compile-Time Defenses & Build Hardening Rules for AI Agents

When modifying build settings, profile options, or feature flags:

1. **`#![no_std]` Zero-Dependency Invariant:**
   Maintain 100% self-sufficient core Rust implementations. Do NOT add external dependencies to `Cargo.toml`.
2. **`panic = "abort"` Unwind Protection:**
   Both `dev` and `release` profiles MUST use `panic = "abort"` to prevent stack unwinding exploit primitives.

---

## 5. Clock Algorithm & Timer Management Rules for AI Agents

When modifying clock page replacement or timekeeping subsystems:

1. **Clock Page Replacement Hand-Pointer Traversal:**
   Page frame eviction MUST traverse physical memory frames in a circular queue. Clear reference bits from `1` to `0` for second-chance evaluation before evicting unreferenced pages.

---

## 6. Circular Buffer & Lock-Free Ring Buffer Rules for AI Agents

When implementing or modifying ring buffers in `src/klib/ring_buffer.rs`, `src/klib/ringbuf.rs`, or `src/media/sovereign_video_player.rs`:

1. **Power-of-Two Capacity Rule:**
   Ring buffer capacities MUST be powers of two ($2^k$) to perform $O(1)$ index wrapping via bitwise AND `idx & (capacity - 1)`.

---

## 7. Cache Memory Architecture, LRU Eviction & Package Cache Rules for AI Agents

When modifying cache memory engines, key-value stores, or package cache trimmers:

1. **Key Invalidation Invariant:**
   `SovereignCacheEngine::set` MUST purge pre-existing entries with matching keys via `self.entries.retain(|e| e.key != key)` before inserting new values.

---

## 8. Process Lifecycle, Signal ABI Translation & Supervision Rules for AI Agents

When modifying process management, signal handlers, or pseudo-terminals:

1. **State Machine Transitions (`SovereignProcessLifecycleController`):**
   Ensure process state changes (`Created`, `Ready`, `Running`, `Blocked`, `Stopped`, `Zombie`, `Terminated`) execute under thread-safe synchronization.

---

## 9. Hardware Fitting, Driver Auto-Binding & Device Adaptation Rules for AI Agents

When writing, probing, or modifying hardware device drivers (`src/drivers/`):

1. **Bus Signature Probing:**
   Driver probe routines MUST evaluate Vendor ID (VID), Product ID (PID), and interface class codes before claiming attachment.

---

## 10. Network Stack, eBPF/XDP & PQC Security Rules for AI Agents

When modifying networking drivers, eBPF filters, or VPN subsystems:

1. **Kernel Bypass eBPF/XDP Processing:**
   Use zero-copy DMA ring buffers (`process_xdp_zero_copy_packet`). Ensure XDP actions explicitly return `XDP_PASS`, `XDP_DROP`, `XDP_TX`, or `XDP_REDIRECT`.

---

## 11. Processor Topology, CPU Scheduling & Multi-Core Rules for AI Agents

When modifying CPU scheduling, task management, or ISA optimization:

1. **ISA Level Auto-Detection (`src/klib/isa.rs`):**
   Support x86-64 microarchitecture levels (`v1`..`v4`). Route vectorized operations via `vectorized_memcpy` dynamically based on detected features.

---

## 12. Backend Subsystem & Server Engine Rules for AI Agents

When modifying backend services in `src/open_source_obsoletion.rs`, `src/open_source_os_gap_closure.rs`, or `src/automation/system_level.rs`:

1. **Zero-Dependency Native Backend Engines:**
   Maintain native parity for embedded DBs (`SovereignEmbeddedDb`), web servers (`SovereignWebServer`), in-memory caches (`SovereignCacheEngine`), message brokers (`SovereignMessageBroker`), secret vaults (`SovereignSecretVault`), object stores (`SovereignDistributedStorage`), and orchestrators (`SovereignK8sOrchestratorEngine`).

---

## 13. Checklist for AI Agents

1. **Update Manifests & Documentation** when bumping versions, adding drivers, or modifying HTML/UI logic.
2. **Run Standalone Subsystem Tests:**
   ```bash
   rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/open_source_os_gap_closure.rs
   ```
3. **Execute Full Pipeline:** Run `./run_sigma_tests.sh` and ensure all test steps pass.
4. **Follow Conventional Commits:**
   `docs(agents): add HTML dependency reduction guide` or `fix(docs): escape HTML special characters`.

---

## 14. Detailed Documentation References

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
* [`docs/AGENTS_CLOCK_ALGORITHM_MANAGEMENT.md`](docs/AGENTS_CLOCK_ALGORITHM_MANAGEMENT.md)
* [`docs/AGENTS_COMPILE_TIME_DEFENSES_MANAGEMENT.md`](docs/AGENTS_COMPILE_TIME_DEFENSES_MANAGEMENT.md)
* [`docs/AGENTS_REDUCING_HTML_DEPENDENCY.md`](docs/AGENTS_REDUCING_HTML_DEPENDENCY.md)
* [`docs/RELEASE_CADENCE.md`](docs/RELEASE_CADENCE.md)
* [`docs/package-manager.md`](docs/package-manager.md)
