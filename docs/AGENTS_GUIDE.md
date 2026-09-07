# SigmaOS AI Agent Comprehensive Developer Guide

This document details the multi-agent collaboration architecture and development standards for AI engineering agents working on SigmaOS.

## 1. Agent Personas & Operational Roles

* **🛡️ Sentinel (Security Guardian):** Focuses on vulnerability remediation (SSRF, path traversal, buffer safety), CI workflow supply chain security (commit SHA action pinning, token permissions), and input validation.
* **🎨 Palette (UX & Accessibility Specialist):** Focuses on user interaction, ARIA keyboard accessibility, desktop styling custom properties, and UI responsiveness.
* **⚡ Bolt (Performance Engineer):** Focuses on micro-optimizations, zero-copy page splicing, stack-allocated formatting primitives, memory footprint reduction, and latency profiling.

## 2. Multi-Distro Feature Parity Standards

SigmaOS synthesizes best-of-breed innovations from major Linux and BSD distributions:
* **Fedora / RedHat:** `FedoraMirrorManager2Engine`, `FedoraSharedSystemManager`, `RpmOstreeDeployEngine`, `systemd-offline-update`.
* **Debian / Ubuntu:** `UbuntuAppArmorEngine`, `MultiArchAptPinningResolver`, `GStreamerPulseAudioPipeline`.
* **Arch Linux:** `PacmanContribSuite`, `ArchWikiKnowledgeBaseEngine`, `YayParuAdapter`, `ArchTestingRepository`.
* **NixOS:** `NixOsFlakesEngine`, `NixOsDeclarativeConfigEngine`, `SovereignNixGcEngine`.
* **FreeBSD / OpenBSD / NetBSD:** `FreeBsdJailSandboxEngine`, `FreeBsdCapsicumEngine`, `OpenBsdUnveilFilter`, `NetBsdPkgsrcEngine`, `MpvFreeBsdSndioEngine`.

## 3. Kernel Class Operation Vtable Architecture

When extending or creating kernel drivers and subsystems:
* Refer to `docs/AGENTS_CLASS_OPERATION_MANAGEMENT_GUIDE.md` for class operation vtable patterns (`FileOperations`, `VnodeOps`, `SchedClass`, `NetDeviceOps`, `BlockDeviceOps`).
* Ensure zero heap allocations inside vtable methods, atomic class registration, and `#[repr(C)]` FFI compatibility.

## 4. Concurrency & Readers/Writers Management

When handling concurrent read-write shared resources:
* Refer to `docs/AGENTS_READERS_WRITERS_MANAGEMENT_GUIDE.md` for Readers/Writers synchronization rules (`AtomicRwLock`, RCU lock-free reading, writer-preference locks).
* Avoid writer starvation and never import standard library mutexes/rwlocks in core `#![no_std]` modules.

## 5. Data Confidentiality & Confidential Computing

When handling sensitive buffers, keys, or enclave memory:
* Refer to `docs/AGENTS_CONFIDENTIALITY_MANAGEMENT_GUIDE.md` for zeroization standards, constant-time algorithms, and confidential computing (AMD SEV-SNP / Intel TDX) guest state isolation.
* Enforce volatile zeroization on drop and ensure no unencrypted secret memory spills into crash dumps.

## 6. Peterson's Algorithm Mutual Exclusion

When implementing software-based two-thread mutual exclusion:
* Refer to `docs/AGENTS_PETERSON_ALGORITHM_MANAGEMENT_GUIDE.md` for `PetersonLock` implementation patterns, atomic `flag`/`turn` variables, `Ordering::SeqCst`, and explicit memory fences.

## 7. Context Providers Architecture

When capturing, updating, or querying execution and system environment state:
* Refer to `docs/AGENTS_CONTEXT_PROVIDERS_MANAGEMENT_GUIDE.md` for zero-allocation process context snapshots (`TaskControlBlock`, PCID/TLB), shell auto-completion contexts (`ShellContext`, `ContextualCompleter`), and Model Context Protocol (MCP) schemas (`KimiCodeAgent`).

## 8. Reactive & Compliance Dashboards

When managing telemetry, visual dashboards, or privacy/compliance scores:
* Refer to `docs/AGENTS_DASHBOARD_MANAGEMENT_GUIDE.md` for system monitor widgets (`UnifiedDashboard`), visual network security (`VisualDashboardManager`), privacy lockdown presets (`PrivacyDashboard`), and compliance alert tracking.

## 9. On-Demand Techniques Architecture

When managing demand paging, Copy-on-Write, or dynamic GPU offloading:
* Refer to `docs/AGENTS_DEMAND_TECHNIQUES_MANAGEMENT_GUIDE.md` for `DemandPagingSwapEngine` fault handling, zeroed frame allocations, and `NvidiaOnDemand` GPU offload profiles.

## 10. Process Abort & Core Dump Management

When handling process aborts, fatal signals, or core dumps:
* Refer to `docs/AGENTS_ABORTING_PROCESSES_MANAGEMENT_GUIDE.md` for `abort_process` isolation rules, core dump metadata generation, and orphan child reparenting.

## 11. Development Workflow & Verification Protocol

1. **Pre-Flight Verification:** Run `./run_sigma_tests.sh` to establish baseline test status.
2. **Implementation:** Modify source files in `src/`, adding companion unit tests in `#[cfg(test)] mod tests` blocks.
3. **Module Export Verification:** Re-export new public structs in parent `mod.rs` files (`src/compatibility/mod.rs`, `src/media/mod.rs`, etc.).
4. **Documentation Synchronization:** Update `docs/` and run `./scripts/sync_wiki.sh` if markdown documentation or wiki specs are updated.
5. **Post-Flight Verification:** Execute `./run_sigma_tests.sh` to confirm 100% test pass rate.
