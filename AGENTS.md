# AGENTS.md - AI Agent Operational & File Management Guidelines for SigmaOS

Welcome, AI Agent! This document outlines operational procedures, architectural conventions, and strict file management guidelines when working on the **SigmaOS** codebase.

---

## 1. Core Directives & Architecture Principles

1. **Zero External Dependencies (`#![no_std]`)**:
   - SigmaOS operates on a strict sovereign zero-dependency philosophy.
   - Do **not** add external third-party crates under `[dependencies]` in `Cargo.toml`.
   - Use `alloc::` primitives (`alloc::string::String`, `alloc::vec::Vec`, `alloc::boxed::Box`, `alloc::format!`) for heap allocations in kernel/distro space.

2. **Multi-Architecture Support**:
   - Maintain multi-arch abstractions across supported architectures: `x86_32`, `x86_64`, `aarch64`, `riscv64`, `loongarch64`, `powerpc64`, and `s390x`.
   - Architectural register contexts and trap handlers live in `src/arch/portability.rs`.

3. **Subsystem Interoperability**:
   - Core subsystem bridges (VFS, Init, Package Management, Security, Kernel, Memory) route through `src/distro/linux_bsd_inspirations.rs` (`SovereignUniversalDistroBridge`).

---

## 2. File Management & Organization Guidelines

### 2.1 Code Base Layout
- **Kernel Core**: `src/kernel/`, `src/klib/`, `src/memory/`, `src/arch/`
- **Distro Innovations & Parity**: `src/distro/`
  - `src/distro/linux_bsd_inspirations.rs` - Cross-subsystem distro bridge, Landlock v5, eBPF XDP zero-copy, OpenBSD pledge/unveil, FreeBSD jails, and Illumos zones.
  - `src/distro/sovereign_nextgen_distro_leap.rs` - `sched_ext` BPF scheduling, CAS store, HAMMER2 CoW deduplication.
- **Package Management Subsystem**: `src/package/`, `src/sigpkg/`
  - `src/sigpkg/universal_adapter.rs` - Universal package format adapter router (.deb, .rpm, PKGBUILD, ebuild, apk, snap, flatpak, hpkg).
  - `src/sigpkg/universal_oop_system.rs` - Strategy, Adapter, Factory, Decorator, and Observer pattern implementations for package management.
- **Compatibility & Standards**: `src/compatibility/`
- **Drivers & Hardware**: `src/drivers/`
- **Documentation**: `docs/` and `wiki/`

### 2.2 Rules for Modifying Existing Files
1. **Targeted Editing**:
   - Always trace imports and conditional compilation gates (`#[cfg(test)]`, `#[cfg(feature = "standalone_test")]`) before editing source files.
   - Avoid modifying generated build artifacts under `build/` or `target/`.
2. **Atomic & Reversible File Operations**:
   - Use Copy-on-Write (CoW) transaction semantics when updating critical configuration or state files.
   - Perform verification with read-only tools immediately after any file creation, modification, or deletion.

### 2.3 Rules for Adding New Files
1. Place new module source files under the appropriate domain directory (`src/distro/`, `src/package/`, `src/drivers/`, etc.).
2. Re-export new modules in `mod.rs` and `src/lib.rs` as required.
3. Add standalone unit test blocks or test binaries in `tests/` or via inline `#[cfg(test)]` / `#[cfg(feature = "standalone_test")]` blocks.

---

## 3. Testing Protocols & Verification

All changes must be validated against the native SigmaOS test suite:

```bash
# Run full test suite (atomic unit tests, subsystem inspection, Python pytest suite)
./run_sigma_tests.sh

# Run standalone module tests
rustc --edition=2021 --test --cfg 'feature="standalone_test"' src/distro/linux_bsd_inspirations.rs -o build/test_inspirations && ./build/test_inspirations

# 🤖 SigmaOS AI Agent Governance Specification (`AGENTS.md`)

**Version:** 1.3.0
**Scope:** Autonomous AI Agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️), Process, Memory, Loader, & Desktop Management

---

## EXECUTIVE SUMMARY & AGENT ARCHITECTURE

SigmaOS features an AI-native architecture where autonomous agent processes govern kernel scheduling, memory pools, dynamic module loading, and desktop environments.

```
                  +-----------------------------------+
                  |   SIGMAOS AI AGENT GOVERNANCE     |
                  +-----------------------------------+
                                    |
         +--------------------------+--------------------------+
         |                          |                          |
         v                          v                          v
  ⚡ BOLT PROCESS            🎨 PALETTE PROCESS         🛡️ SENTINEL PROCESS
  • Render Frame Profiling   • Theme & Layout Engine     • Desktop App Sandbox Audit
  • Compositor Optimization  • WCAG 2.1 AA Focus Outlines • Web2App IPC Channel Check
  • Sub-µs Memory Access     • Semantic ARIA Tags        • Post-Quantum Verification
```

---

## 4. Pull Request & Commit Guidelines
- Repository git branches must follow the naming convention starting with `jules-`.
- Maintain descriptive commit messages following standard git conventions.

## 1. AGENT PERSONAS & GOVERNANCE

### ⚡ Bolt (Performance Agent)
- **Scope**: CPU scheduling, `cgroups v2`, boot speed profiling (`src/tools/bootloader.rs`), Zenith compositor render frame-rate profiling (`zenith_desktop/`), zero-allocation hot paths.
- **Rules**:
  - Maintain 60+ FPS compositor rendering and eliminate window layout recalculation bottlenecks.
  - Record learnings in `.jules/bolt.md`.

### 🎨 Palette (UX & Accessibility Agent)
- **Scope**: Desktop compositor layout, Control Center themes (`TokyoNight`, `Catppuccin`, `Nord`), boot splash graphics, WCAG 2.1 AA focus visible outlines, ARIA annotations.
- **Rules**:
  - Enforce WCAG 2.1 AA compliance across all desktop controls and web console interfaces.
  - Record learnings in `.jules/palette.md`.

### 🛡️ Sentinel (Security & Integrity Agent)
- **Scope**: LSM auditing, OpenBSD `pledge`/`unveil`, Post-Quantum Dilithium-5 module signatures, desktop process sandbox isolation (`DistrictSandbox`).
- **Rules**:
  - Enforce process isolation for desktop applets and web2app launchers.
  - Record learnings in `.jules/sentinel.md`.

---

## 2. DESKTOP ENVIRONMENT & COMPOSITOR POLICIES (`docs/AI_AGENTS_DESKTOP_ENVIRONMENTS_MANAGEMENT.md`)

- **Wayland Ozone Launchers**: Third-party web applications must be launched with Wayland Ozone isolation flags (`--ozone-platform=wayland`).
- **Accessibility Invariants**: All interactive UI elements must render high-contrast focus rings on keyboard TAB focus.

---

## 3. CANARY VALUE MANAGEMENT & SECURITY HARDENING (`docs/AGENTS_CANARY_VALUE_MANAGEMENT.md`)

- **Thread-Local SSP Canaries**: All thread guard values generated by `BinaryProtectionManager` in `src/security/binary_protection.rs` must enforce LSB NUL-byte formatting (`canary & 0xFF == 0x00`) to terminate string buffer overflow attacks.
- **OpenBSD Context Switch Guards**: CPU context switches in `src/kernel/roundrobin.rs` must validate context canary values (`stack_canary`) before restoring execution frames, triggering controlled `__stack_chk_fail` fault handling on mismatch.

---

## 4. CLOUD COMPUTING OPERATIONS MANAGEMENT (`docs/AGENTS_CLOUD_COMPUTING_OPERATIONS_MANAGEMENT.md`)

- **Headless Cloud Targets**: Booting under `SystemTarget::Cloud` (`cloud.target`) in `src/init/sigmainit.rs` must bypass GUI compositors and optimize zero-copy E1000/xHCI network queues (< 16MB RAM footprint).
- **Capability-Gated Cloud-Init**: User-data `#cloud-config` scripts executed by `CloudInitBootstrapEngine` (`src/distro/linux_bsd_parity_extended.rs`) must run inside Ring 3 sandboxes governed by `PledgeManager`.

---

## 5. STATE MANAGEMENT ARCHITECTURE (`docs/AGENTS_STATE_MANAGEMENT.md`)

- **Declarative System State Graph**: State mutations in `src/system/state.rs` must generate immutable generation snapshots supporting $O(1)$ atomic rollback (`rollback()`).
- **Process Lifecycle Machine**: Kernel process state transitions (`src/kernel/process.rs`, `src/kernel/sched/task.rs`) must adhere strictly to valid lifecycle paths (`New` $\to$ `Ready` $\to$ `Running` $\to$ `BlockedWaiting`/`BlockedSuspended` $\to$ `Zombie` $\to$ `Terminated`).

---

## 6. TOP-LEVEL COMPONENT MANAGEMENT (`docs/AGENTS_TOP_LEVEL_COMPONENT_MANAGEMENT.md`)

- **Subsystem Isolation**: Top-level components (Microkernel Core, HAL/Drivers, VFS Storage, Network, Security, Package System, Zenith Compositor, Universal Distro Bridge) must not share mutable raw global state across boundaries.
- **Cross-Subsystem Distro Bridge**: Cross-component interactions route through `SovereignUniversalDistroBridge` (`src/distro/linux_bsd_inspirations.rs`) using capability-gated IPC ring buffers and explicit trait interfaces.

---

## 7. MUTUAL EXCLUSION, MONITORS & PETERSON ALGORITHM (`docs/AGENTS_MUTUAL_EXCLUSION_MONITORS_PETERSON_MANAGEMENT.md`)

- **Peterson's Algorithm Memory Fences**: Software 2-process mutual exclusion (`flag[i] = true; turn = j;`) must issue `core::sync::atomic::fence(Ordering::SeqCst)` to guarantee memory visibility before evaluating `turn`.
- **Monitor Encapsulation**: Monitors (`BoundedBufferMonitor` in `src/kernel/linux_bsd_innovations.rs`) must fully encapsulate shared state, locks, and condition variables, preventing un-monitored direct buffer access.

---

## 8. STANDALONE TESTING & VERIFICATION PROTOCOL

Every agent module must support standalone unit testing via:
```bash
rustc --test <module_path> --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_agent && /tmp/test_agent
```
