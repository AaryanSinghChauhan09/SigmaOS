# SigmaOS AI Agent Security Management Directive (`AGENTS.md`)

This document defines operational guidelines, security policies, and verification instructions for autonomous AI engineering agents working on the SigmaOS codebase.

---

## 1. Core Principles for AI Agents

1. **Zero External Third-Party Dependencies:**
   - SigmaOS strictly follows a zero-dependency `#![no_std]` design philosophy.
   - Do NOT add external crates under `[dependencies]` in `Cargo.toml`.
   - Use `alloc::` primitives (`alloc::vec::Vec`, `alloc::string::String`, `alloc::format`) and native `#![no_std]` structures.

2. **Cross-OS Subsystem Interoperability:**
   - Every security or kernel component must maintain compatibility across Linux and BSD distribution modes (`LinuxArch`, `LinuxDebian`, `LinuxFedora`, `LinuxNix`, `FreeBsd`, `OpenBsd`, `NetBsd`, `DragonFlyBsd`, `SolarisIllumos`, etc.).
   - Sandboxing rules must bridge Linux Landlock v5 with FreeBSD Capsicum rights (`FreeBsdCapsicumDescriptorDelegate`) and OpenBSD pledge/unveil (`OpenBsdUnveilAuditor`).

3. **Autonomous Verification:**
   - Always run `./run_sigma_tests.sh` and `pytest` after making modifications.
   - Individual standalone tests can be compiled and verified using `rustc --edition=2021 --test <file_path>`.

---

## 2. Security Management Framework for AI Agents

### A. Access Control & Sandboxing
- **Landlock v5 + Pledge + Unveil + Capsicum:**
  - File path access must be scoped using `SovereignLandlockV5Guard`.
  - System call promises must be constrained using OpenBSD pledge/unveil enforcers.
  - File descriptors must delegate fine-grained rights via FreeBSD Capsicum.

### B. Cryptographic Integrity & Livepatching
- Post-Quantum Cryptography (Dilithium-5 / Kyber-1024) and Ed25519 signature verification must be enforced for package manifests and livepatching trampolines (`KernelPatchVerificationEngine`).
- Differential rollback snapshots (`SigmaDeltaStateSnapshotEngine`, `SovereignPackageRollbackEngine`) must allow sub-1ms state restoration.

### C. Vulnerability & Audit Auditing
- Maintain vulnerability classification (`Vulnerable`, `Fixed`, `Unaffected`) in `SecurityAdvisoryTracker`.
- Perform QA signoff quorum checks (`PackageSignoff`) requiring `qa_tested`, `build_reproducible`, and `security_audited` flags.

### D. Buffer Overflow & Buffer Overrun Management
- Follow technical directives in `AGENTS_BUFFER_OVERFLOW.md`, `AGENTS_BUFFER_OVERRUN.md`, `docs/AGENTS_BUFFER_OVERFLOW.md`, and `docs/AGENTS_BUFFER_OVERRUN.md`.
- Enforce guard page allocations (`alloc_with_guard_page`), stack clash protection (`has_guard_page`), bounds-checked FFI c-string helpers (`cstrlen`), ring buffers, and W^X / DEP policies.

### E. Bitmap Operations & Resource Allocation
- Follow technical directives in `AGENTS_BITMAP_OPERATIONS.md` and `docs/AGENTS_BITMAP_OPERATIONS.md`.
- Utilize lock-free `AtomicBitmap` for page frames, PIDs, and IRQ vector allocations with atomic memory ordering.

### F. Boot Block & Bootloader Management
- Follow technical directives in `AGENTS_BOOT_BLOCK.md` and `docs/AGENTS_BOOT_BLOCK.md`.
- Ensure `SigmaBootloaderEngine` systemd-boot loader entries and GRUB configs enforce measured boot TPM PCR measurements (`TPM_PCR_4`) and path validation.

### G. Circular Buffer Management & Lock-Free IPC
- Follow technical directives in `AGENTS_CIRCULAR_BUFFER.md` and `docs/AGENTS_CIRCULAR_BUFFER.md`.
- Enforce power-of-two capacity alignment, atomic head/tail pointer ordering (`Acquire`/`Release`), and lock-free bounds checking on `RingBuf` and `RingBuffer`.

### H. Clock Interrupt & Timer Management
- Follow technical directives in `AGENTS_CLOCK_INTERRUPT.md` and `docs/AGENTS_CLOCK_INTERRUPT.md`.
- Ensure clock interrupt handlers avoid blocking locks or allocations, and manage `TimerDescriptor` state transitions atomically.

### I. Coarse Parallelism & Threading Management
- Follow technical directives in `AGENTS_THREADING_PARALLELISM.md` and `docs/AGENTS_THREADING_PARALLELISM.md`.
- Ensure multi-threaded tasks respect RCU synchronization epochs (`rcu_epoch`), adaptive thread quanta (`adaptive_thread_quantum_multiplier`), and stack guard isolation (`has_guard_page`).

### J. Microprocessor Operation Management
- Follow technical directives in `AGENTS_MICROPROCESSOR_OPERATIONS.md` and `docs/AGENTS_MICROPROCESSOR_OPERATIONS.md`.
- Enforce multi-architecture context switching (`CpuContextState`), microarchitecture ISA auto-detection (`x86-64-v1`..`v4`), IRQL execution level guards (`DispatchLevel`), and thermal power governance.

### K. Constrained Application Protocol (CoAP) Management
- Follow technical directives in `AGENTS_COAP_MANAGEMENT.md` and `docs/AGENTS_COAP_MANAGEMENT.md`.
- Ensure IoT CoAP resource endpoints (`CoAPResource`), request methods (`CoAPMethod`), and error codes (`CoAPError`) maintain `#![no_std]` compliance and payload bounds safety.

### L. Control Mode Operation Management
- Follow technical directives in `AGENTS_CONTROL_MODE.md` and `docs/AGENTS_CONTROL_MODE.md`.
- Validate terminal control mode notification parsers (`tmux`), enforce `AccessControlMatrix` rights, and manage remote controller session transitions safely.

### M. Comprehensive Access Operations Management
- Follow technical directives in `AGENTS_ACCESS_MANAGEMENT.md` and `docs/AGENTS_ACCESS_MANAGEMENT.md`.
- Manage the complete access lifecycle across LDAP directory services (`LdapAccessClient`), anonymous/authenticated client tiers, direct/relative path canonicalization, memory access protection (`W^X`), read/write permission enforcers (`FileAttributeAccessControl`), RAT remote files, and wireless access points.

---

## 3. Autonomous AI Agent Development Rules

1. **Persona Protocols & Missions:**
   - **Sentinel (Security):** Focus on security vulnerability remediation (XSS, path traversal, injection, memory safety).
   - **Palette (UX):** Focus on micro-UX enhancements, accessibility (ARIA, focus visible), and terminal/GUI polish.
   - **Bolt (Performance):** Focus on O(1) algorithms, lock-free structures, caching, and allocation reduction.

2. **Planning & Review Cycles:**
   - AI agents must request plan reviews using `request_plan_review` before setting or modifying the plan with `set_plan`.
   - Agents must call `request_code_review` and address review feedback before finalizing PR submission.

3. **Critical Learning Journaling:**
   - Maintain critical learnings in `.jules/<persona>.md` (e.g. `.jules/sentinel.md`, `.jules/bolt.md`, `.jules/palette.md`).
   - Log only non-routine, codebase-specific security findings, edge cases, and unexpected performance/UX insights.

---

## 4. Pre-Commit Verification Checklist for AI Agents

Before submitting changes, AI agents must execute:
1. `./run_sigma_tests.sh` to run all atomic Rust unit tests and system tests.
2. Verify standalone builds for modified modules (`rustc --edition=2021 --test <modified_file.rs>`).
3. Call `pre_commit_instructions` tool and complete all required checks.
4. Record key codebase patterns via `initiate_memory_recording`.

---

## 5. Repository Consolidation Status (September 2026)

### A. Completed Consolidation Work
- **Branch Consolidation**: All development branches have been merged into main branch
- **Compilation Status**: Codebase compiles successfully with 0 errors (790 warnings, all non-critical)
- **Test Coverage**: All 140+ atomic tests pass successfully
- **GitHub Synchronization**: Local main branch synchronized with origin/main
- **Wiki Integration**: Future development plan published to GitHub Wiki

### B. Software Engineering Principles Applied
- **Interface Segregation Principle**: Kernel module re-exports organized by specific functionality rather than glob imports
- **Single Responsibility Principle**: Kernel modules organized by functional domains (memory, scheduling, IPC, security)
- **Dependency Inversion**: Core abstractions exposed through traits and interfaces
- **DRY (Don't Repeat Yourself)**: Eliminated duplicate imports and redundant code structures
- **KISS (Keep It Simple, Stupid)**: Simplified module hierarchy and removed ambiguous glob re-exports

### C. Zero-Dependency Architecture Compliance
- **No External Crates**: `[dependencies]` in Cargo.toml remains empty
- **no_std Compatibility**: Kernel modules use `core::` and `alloc::` primitives
- **Cross-Distro Support**: Linux/BSD compatibility layers maintained
- **Memory Safety**: Lock hierarchy enforced (Scheduler -> Memory Manager -> VFS -> Device/Driver)

### D. Current Repository State
- **Active Branch**: main (all branches consolidated)
- **Open Pull Requests**: 17 PRs identified for review
- **GitHub Workflows**: 31 CI/CD workflows in place
- **Documentation**: 486 wiki pages available, future development plan added
- **Build Status**: Stable compilation with comprehensive test coverage

### E. Future Development Roadmap
A comprehensive 2026 development plan has been implemented covering:
- Phase 1: Core Infrastructure Stabilization
- Phase 2: Cross-Distro Subsystem Integration
- Phase 3: Kernel Core Enhancements
- Phase 4: Hardware Driver Expansion
- Phase 5: Security & Compliance
- Phase 6: Documentation & Wiki Management
- Phase 7: CI/CD & Workflow Optimization
- Phase 8: Performance Optimization
- Phase 9: Developer Experience

The roadmap is available in `FUTURE_DEVELOPMENT_PLAN_2026.md` and published to the GitHub Wiki.
