# SigmaOS Canonical Subsystem Implementation Status & Matrix (`docs/PROJECT_STATUS.md`)

## Executive Overview

This document serves as the **single source of truth** for the implementation maturity of all major SigmaOS subsystems. Every subsystem claim is categorized into one of five standardized taxonomy levels and linked directly to source code implementations, unit tests, integration test suites, and CI verification jobs.

---

## Subsystem Maturity Taxonomy

| Classification | Meaning | Criteria & Verification Requirement |
| :--- | :--- | :--- |
| `Implemented` | Production-ready / fully functional | Native source code present, 100% unit test pass rate, integration test in CI, QEMU/real hardware verified. |
| `Partially Implemented` | Core logic present; edge cases pending | Basic functionality works and tested; advanced features or full POSIX parity undergoing active development. |
| `Prototype` | Experimental / functional POC | Initial struct and trait definitions compiled; basic unit tests passing; pending full OS integration. |
| `Specification Only` | Architectural blueprint published | Technical design document, RFC, or specification `.md` created; code implementation not started. |
| `Not Started` | Planned future roadmap item | Roadmap item defined; design document and code implementation pending. |

---

## Master Subsystem Status Matrix

### 1. Kernel, Microkernel & Low-Level Foundations

| Subsystem Name | Status | Primary Source File(s) | Verification / Test Suite |
| :--- | :--- | :--- | :--- |
| **Sovereign Modular Kernel System** | `Implemented` | `src/kernel/universal_modular_system.rs` | `./run_sigma_tests.sh` |
| **Boot Foundations & UEFI Protocol** | `Implemented` | `src/kernel/boot_foundations.rs` | `rustc --test src/kernel/boot_foundations.rs` |
| **Memory Management & Paging** | `Partially Implemented` | `src/memory/paging.rs`, `src/kernel/vmm_paging.rs` | `rustc --test src/kernel/vmm_paging.rs` |
| **Preemptive SMP Scheduler** | `Implemented` | `src/kernel/scheduler.rs` | `rustc --test src/kernel/scheduler.rs` |
| **PCI MSI-X Vector Engine** | `Implemented` | `src/drivers/msix_engine.rs` | `rustc --test src/drivers/msix_engine.rs` |
| **Demand Paging & COW** | `Partially Implemented` | `src/kernel/vmm_paging.rs` | `rustc --test src/kernel/vmm_paging.rs` |
| **cgroups v2 Memory Controller** | `Implemented` | `src/memory/cgroups.rs` | `rustc --test src/memory/cgroups.rs` |

### 2. Userland Coreutils, Shell & Compatibility

| Subsystem Name | Status | Primary Source File(s) | Verification / Test Suite |
| :--- | :--- | :--- | :--- |
| **Coreutils Baseline Suite** | `Partially Implemented` | `src/bin/`, `src/functions/` | `pytest tests/` |
| **Sovereign POSIX Shell (`sigma-sh`)** | `Partially Implemented` | `src/shell/` | `rustc --test src/shell/mod.rs` |
| **Dynamic ELF Loader & Relocation** | `Prototype` | `src/compatibility/elf_loader.rs` | `rustc --test src/compatibility/elf_loader.rs` |
| **Linux Syscall Compatibility ABI** | `Partially Implemented` | `src/compatibility/linux_compat.rs` | `pytest tests/` |
| **FreeBSD / OpenBSD POSIX Shims** | `Partially Implemented` | `src/compatibility/bsd_compat.rs` | `pytest tests/` |

### 3. Security, Access Control & Sandboxing

| Subsystem Name | Status | Primary Source File(s) | Verification / Test Suite |
| :--- | :--- | :--- | :--- |
| **Zorin Exec Guard Policy Engine** | `Implemented` | `src/security/exec_guard.rs` | `rustc --test src/security/exec_guard.rs` |
| **OpenBSD Pledge & Unveil Sentinel** | `Implemented` | `src/security/pledge_impl.rs` | `rustc --test src/security/pledge_impl.rs` |
| **Linux Landlock ABI v4 Sandboxing** | `Implemented` | `src/security/landlock.rs` | `rustc --test src/security/landlock.rs` |
| **FreeBSD Capsicum Descriptor Rights** | `Implemented` | `src/security/capsicum.rs` | `rustc --test src/security/capsicum.rs` |
| **TPM 2.0 & PQC Firmitas Attestation** | `Implemented` | `src/security/pqc_measurement.rs` | `rustc --test src/security/pqc_measurement.rs` |

### 4. Storage, File Systems & Updates

| Subsystem Name | Status | Primary Source File(s) | Verification / Test Suite |
| :--- | :--- | :--- | :--- |
| **Dual-Root A/B Update & COW Engine**| `Implemented` | `src/filesystem/cow_snapshot.rs` | `rustc --test src/filesystem/cow_snapshot.rs` |
| **SigmaFS & VFS Mount Manager** | `Implemented` | `src/filesystem/mount_namespace.rs` | `rustc --test src/filesystem/mount_namespace.rs` |
| **Ext4 & ZFS Hybrid Self-Healing** | `Implemented` | `src/filesystem/ext4.rs`, `src/filesystem/zfs.rs` | `./run_sigma_tests.sh` |
| **Async `io_uring` Subsystem** | `Implemented` | `src/kernel/sigma_io_uring.rs` | `rustc --test src/kernel/sigma_io_uring.rs` |

### 5. Zenith Desktop Environment

| Subsystem Name | Status | Primary Source File(s) | Verification / Test Suite |
| :--- | :--- | :--- | :--- |
| **Zenith Wayland Compositor Core** | `Implemented` | `src/desktop/zenith_compositor.rs` | `./run_sigma_tests.sh` |
| **Wayland Multi-Protocol Engine** | `Implemented` | `src/desktop/wayland_protocol.rs` | `rustc --test src/desktop/wayland_protocol.rs` |
| **Omarchy Omakase Layout Engine** | `Implemented` | `src/desktop/omarchy_omakase.rs` | `rustc --test src/desktop/omarchy_omakase.rs` |
| **Zenith File Manager & Miller Columns**| `Implemented` | `src/desktop/filemanager.rs` | `rustc --test src/desktop/filemanager.rs` |
| **Zenith System Control Center** | `Implemented` | `src/desktop/settings.rs` | `rustc --test src/desktop/settings.rs` |

### 6. Universal Package System (`sigpkg`)

| Subsystem Name | Status | Primary Source File(s) | Verification / Test Suite |
| :--- | :--- | :--- | :--- |
| **Native `.sigpkg` Manifest & Solver** | `Implemented` | `src/sigpkg/universal_engine.rs` | `rustc --test src/sigpkg/universal_engine.rs` |
| **Multi-Distro Translation Facade** | `Implemented` | `src/package/universal.rs` | `rustc --test src/package/universal.rs` |
| **Signature & Cryptographic Verifier** | `Implemented` | `src/sigpkg/verifier.rs` | `rustc --test src/sigpkg/verifier.rs` |
| **Post-Install Sandbox Isolator** | `Implemented` | `src/package/sandbox.rs` | `rustc --test src/package/sandbox.rs` |
