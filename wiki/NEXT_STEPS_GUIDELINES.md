# SigmaOS Next Steps Guidelines & Operational Execution Handbook

## Executive Guidelines
This document establishes the official operational guidelines, execution rules, and domain-by-domain action plan for continuous development on **SigmaOS**. All engineering efforts must strictly align with these guidelines and be committed directly to the `main` branch, adhering to the repository directive against creating pull requests.

---

## 1. Single Primary User Journey
SigmaOS adheres strictly to an opinionated, bootable desktop distribution workflow inspired by Omarchy Linux:

```
ISO Boot → Live Media → Installer Wizard → First Boot Login → Zenith Desktop → App Launcher → Package Install (sigpkg) → Theme Customization → Atomic OS Update → Rollback Safety Net
```

### Mandated Guidelines:
1. **Zero Simulated Success Paths:** Every installer stage, package operation, and update process must execute real system operations or validate actual hardware state.
2. **Standard Library in Userland:** Standard Rust (`std`) is canonical for Zenith desktop, package manager (`sigpkg`), installer, and core userland utilities.
3. **`no_std` Microkernel Isolation:** Bare-metal kernel code (`src/kernel/`), bootloader, and low-level drivers remain strictly `#![no_std]`.

---

## 2. Tri-Agent Execution Guidelines

### ⚡ Bolt Agent Guidelines (Performance & Efficiency)
- Profile code before modifying ($O(N^2) \to O(N \log N)$ or zero-allocation buffers).
- Target bottlenecks under heavy input loads.
- Maintain critical learnings in `.jules/bolt.md`.
- Keep performance changes under 50 lines with explicit benchmark justification.

### 🛡️ Sentinel Agent Guidelines (Security & Compliance)
- Enforce strict input validation on all VFS paths, network sockets, and syscall boundaries.
- Replace dummy/mock security routines with real cryptography (Dilithium-5 / SHA3) and sandboxing (`pledge`/`unveil`).
- Maintain critical security learnings in `.jules/sentinel.md`.

### 🎨 Palette Agent Guidelines (UX & Accessibility)
- Ensure WCAG 2.1 AA keyboard navigation, high-contrast visual focus indicators, and ARIA labels across all desktop components.
- Polish interactive user flows in Zenith desktop and installer wizards.
- Maintain critical UX learnings in `.jules/palette.md` and `.Jules/palette.md`.

---

## 3. Immediate Domain Action Plan

### Domain A: Code Quality & Microkernel Refactoring
- Decompose monolithic files like `src/package/universal.rs` into modular sub-modules (`mod.rs`, `adapter.rs`, `resolver.rs`, `hooks.rs`).
- Maintain 100% pass rate across native Rust test suites and Python integration suites.

### Domain B: Kernel & Microkernel Performance
- Expand lock-free `io_uring` ring buffer pool for asynchronous disk I/O in `src/kernel/sigma_io_uring.rs`.
- Implement NUMA-aware physical memory allocation in `src/kernel/vmm_paging.rs`.

### Domain C: Security & Capability Controls
- Extend `pledge()` and `unveil()` capability sandboxing across all userland core utilities.
- Integrate TPM 2.0 PCR sealed keys into `AuthenticatedEmergencyTargetGate` (`src/init/emergency_gate.rs`).

### Domain D: Zenith Desktop & User Experience
- Improve high-contrast theme support (`Ayu`, `GruvboxMaterial`, `MaterialOcean`) and focus outlines in Zenith desktop and installer setup configurator (`src/installer/gui_wizard.rs`).
- Add keyboard shortcut cheatsheet overlay in Zenith window manager.

---

## 4. Quality Assurance & Verification Workflow
Before submitting any changes to `main`:
1. Run `./run_sigma_tests.sh` to execute all native Rust test suites.
2. Run `pytest tests/` to execute Python system integration tests.
3. Ensure zero compiler warnings and clean lint checks.
4. Verify document synchronization across all documentation and wiki mirror directories (`./`, `docs/`, `wiki/`, `WIKI/`, `wiki_content/`, `wiki_repo/`).
