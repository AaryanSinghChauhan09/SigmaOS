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

## 5. Development Workflow & Verification Protocol

1. **Pre-Flight Verification:** Run `./run_sigma_tests.sh` to establish baseline test status.
2. **Implementation:** Modify source files in `src/`, adding companion unit tests in `#[cfg(test)] mod tests` blocks.
3. **Module Export Verification:** Re-export new public structs in parent `mod.rs` files (`src/compatibility/mod.rs`, `src/media/mod.rs`, etc.).
4. **Documentation Synchronization:** Update `docs/` and run `./scripts/sync_wiki.sh` if markdown documentation or wiki specs are updated.
5. **Post-Flight Verification:** Execute `./run_sigma_tests.sh` to confirm 100% test pass rate.
