# AGENTS.md — AI Agent Guidelines, Versioning, Hardware Fitting, Backend, Loading, Processor & Network Management for SigmaOS

This document provides instructions, rules, and procedures for AI agents working in the SigmaOS repository, specifically regarding **Version Handling**, **Release Channels**, **Hardware Fitting & Driver Auto-Binding**, **Network Stack & eBPF Management**, **Processor Subsystem Management**, **Backend Management**, **Bootloader & Driver Loading**, **Multi-Distro Packaging Parity**, and **Core Subsystem Changes**.

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

### 2.2 Release Channel Configurations
SigmaOS maintains two distinct release streams:
1. **Sigma Stable (`sigma-stable.toml`):** Long-term support (LTS) releases, enterprise stability, strict KABI guarantees.
2. **Sigma Rolling (`sigma-rolling.toml`):** Bleeding-edge features, continuous integration, fast-path package updates.

AI agents updating release configs MUST maintain alignment across `sigma.toml.example`, `sigma-stable.toml`, and `sigma-rolling.toml`.

### 2.3 Universal Package Version Translation (`sigpkg`)
`sigpkg` translates multi-distro version formats into canonical SigmaOS package versions (`crate::sigpkg::Version`):
* **Debian / Ubuntu (`.deb`):** Epoch + Version + Revision (`[epoch:]upstream_version[-debian_revision]`).
* **Fedora / RHEL (`.rpm`):** Epoch + Version + Release (`[epoch:]version-release.dist`).
* **Arch Linux (`.pkg.tar.zst`):** Version + Package Release (`pkgver-pkgrel`).
* **Alpine Linux (`.apk`):** Upstream Version + Package Revision (`version-r<pkgrel>`).
* **Gentoo Linux (`.ebuild`):** Version + Subslot + Revision (`version-r<rev>`).
* **Haiku OS (`.hpkg`):** Version + Package Revision (`version-revision`).

When parsing or creating packages across distros, AI agents MUST preserve version epoch and revision fields to ensure correct dependency resolution order.

---

## 3. Hardware Fitting, Driver Auto-Binding & Device Adaptation Rules for AI Agents

When writing, probing, or modifying hardware device drivers (`src/drivers/`):

1. **Bus Signature Probing:**
   Driver probe routines MUST evaluate Vendor ID (VID), Product ID (PID), and interface class codes before claiming attachment, failing fast if signatures do not match.
2. **Hardware Capabilities Registration:**
   Devices MUST register supported capabilities with `UniversalSandboxCapabilityMatrix` prior to device activation.
3. **Contiguous Memory Fitting:**
   DMA ring buffers for hardware drivers MUST use contiguous allocation (`cma_contiguous_memory_reservation_glue` or `dma_ring_buffer_allocator`).
4. **Hot-Unplug Cleanup Safety:**
   Driver cleanup functions MUST safely flush DMA rings and unregister IRQs without triggering kernel panics upon device removal.

---

## 4. Network Stack, eBPF/XDP & PQC Security Rules for AI Agents

When modifying networking drivers, eBPF filters, or VPN subsystems:

1. **Kernel Bypass eBPF/XDP Processing:**
   Use zero-copy DMA ring buffers (`process_xdp_zero_copy_packet`). Ensure XDP actions explicitly return `XDP_PASS`, `XDP_DROP`, `XDP_TX`, or `XDP_REDIRECT`.
2. **Post-Quantum VPN Security:**
   PQC VPN sessions (`SovereignPqcVpnFirewall`) MUST establish peer keys via hybrid post-quantum key encapsulation before data transmission.
3. **Cilium BPF CNI Policies:**
   Ensure CNI network drivers (`SovereignCiliumBpfNetworkEngine`) enforce L3/L4 identity policies under `#![no_std]`.
4. **Network Frame Boundary Validation:**
   All packet parsers MUST validate IPv4/IPv6 header length fields to prevent buffer overread vulnerabilities.

---

## 5. Processor Topology, CPU Scheduling & Multi-Core Rules for AI Agents

When modifying CPU scheduling, task management, or ISA optimization:

1. **ISA Level Auto-Detection (`src/klib/isa.rs`):**
   Support x86-64 microarchitecture levels (`v1`..`v4`). Route vectorized operations via `vectorized_memcpy` dynamically based on detected features.
2. **Hybrid Scheduling Policies (`src/scheduler/process.rs`):**
   Maintain EEVDF virtual deadline fairness for background threads and BORE priority boosting for interactive CPU bursts.
3. **Task Descriptor $O(1)$ Name Lookup:**
   `SimpleProcess` and `SimpleKernelTask` MUST store explicit `name_len: u8` fields initialized during creation to ensure $O(1)$ slice lookups.

---

## 6. Kernel, Bootloader & Driver Loading Rules for AI Agents

When modifying boot sequences, driver registration, or scheduler loading:

1. **Multi-Stage Boot Pipeline:**
   Respect the 4-phase boot sequence: Bootloader -> Kernel Initialization -> Dynamic Driver Loading -> Userland Supervisor.
2. **Paging & Memory Map Setup:**
   Ensure virtual memory setup in `src/klib/paging.rs` aligns 4-level PML4 table indexing (`pml4_idx * 512 + pdpt_idx`).

---

## 7. Backend Subsystem & Server Engine Rules for AI Agents

When modifying backend services in `src/open_source_obsoletion.rs`, `src/open_source_os_gap_closure.rs`, or `src/automation/system_level.rs`:

1. **Zero-Dependency Native Backend Engines:**
   Maintain native parity for embedded DBs (`SovereignEmbeddedDb`), web servers (`SovereignWebServer`), in-memory caches (`SovereignCacheEngine`), message brokers (`SovereignMessageBroker`), secret vaults (`SovereignSecretVault`), object stores (`SovereignDistributedStorage`), and orchestrators (`SovereignK8sOrchestratorEngine`).
2. **Key Retention & Cache Invalidation:**
   `SovereignCacheEngine::set` MUST purge pre-existing entries via `self.entries.retain(|e| e.key != key)` before inserting new values to prevent key duplication.

---

## 8. Kernel ABI (KABI) Versioning & Stability

* **Syscall Table (`src/kernel/syscall/table.rs`):** Syscall numbers MUST remain stable across minor versions. Extensions are added above index 500.
* **System Call Table Auditing:** Use `AntiRootkitGuard` and SSDT auditing when adding or modifying system call dispatch handlers.

---

## 9. Checklist for AI Agents

1. **Update Manifests & Documentation** when bumping versions, adding drivers, or modifying hardware fitting logic.
2. **Run Standalone Subsystem Tests:**
   ```bash
   rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/open_source_os_gap_closure.rs
   ```
3. **Execute Full Pipeline:** Run `./run_sigma_tests.sh` and ensure all test steps pass.
4. **Follow Conventional Commits:**
   `feat(driver): add CH340 USB serial bridge driver` or `docs(agents): add hardware fitting guide`.

---

## 10. Detailed Documentation References

For technical specifications, see:
* [`docs/AGENTS_VERSION_HANDLING.md`](docs/AGENTS_VERSION_HANDLING.md)
* [`docs/AGENTS_BACKEND_MANAGEMENT.md`](docs/AGENTS_BACKEND_MANAGEMENT.md)
* [`docs/AGENTS_LOADING_MANAGEMENT.md`](docs/AGENTS_LOADING_MANAGEMENT.md)
* [`docs/AGENTS_PROCESSOR_MANAGEMENT.md`](docs/AGENTS_PROCESSOR_MANAGEMENT.md)
* [`docs/AGENTS_NETWORK_MANAGEMENT.md`](docs/AGENTS_NETWORK_MANAGEMENT.md)
* [`docs/AGENTS_FITTING_MANAGEMENT.md`](docs/AGENTS_FITTING_MANAGEMENT.md)
* [`docs/RELEASE_CADENCE.md`](docs/RELEASE_CADENCE.md)
* [`docs/package-manager.md`](docs/package-manager.md)
