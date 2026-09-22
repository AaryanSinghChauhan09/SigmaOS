# SigmaOS AI Agent Component Development Guide

## Executive Summary & Vision
This guide defines the standardized methodology for autonomous AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) developing, maintaining, and extending system components within **SigmaOS**. Drawing deep architectural and philosophical inspiration from over 15 benchmark Linux and BSD distributions, SigmaOS synthesizes the highest-performing open-source design patterns into a zero-dependency, 100% safe-Rust operating system ecosystem.

---

## Strategic Distro Inspirations & Architectural Parity

AI agents building components for SigmaOS must adhere to the design principles derived from benchmark distributions:

| Distribution Benchmark | Core Architectural Inspiration | SigmaOS Implementation Target |
| :--- | :--- | :--- |
| **CachyOS** | Micro-architecture tuning (`x86-64-v3`/`v4`), ISA extensions, BORE CPU scheduling | `src/kernel/scheduler.rs`, `src/arch/` HAL |
| **FreeBSD** | Clean Base OS vs Ports separation, Poudriere jail build isolation | `sigma-base`, `sigma-ports`, `src/sigpkg/` |
| **DragonFly BSD** | Lockless LWKT messaging, per-CPU thread queues, Virtual Kernels (`vkernel`) | `src/kernel/ipc.rs`, `src/kernel/linux_parity.rs` |
| **OpenBSD** | Proactive security, `pledge(2)` syscall limits, `unveil(2)` path sandboxing, KARL | `src/security/pledge.rs`, `src/security/unveil.rs` |
| **Fedora** | Immutable OSTree roots, Greenboot early boot health verification & auto-rollback | `src/system/boot_health.rs`, `src/compatibility/fedora.rs` |
| **Arch Linux** | Simple rolling release `.sigpkg` format, ALPM DB integrity | `src/package/universal.rs`, `src/sigpkg/arch_compat.rs` |
| **Gentoo** | Conditional USE flags, slotting, ebuild transaction solvers | `src/sigpkg/sovereign_package_innovations.rs` |
| **Void Linux** | XBPS binary package management, `runit` supervision, `xbps-src` templates | `src/distro/linux_bsd_inspirations.rs` |
| **Qubes OS** | Compartmentalized domain isolation (`sys-net`, `vault`), Anti-Evil-Maid boot checks | `src/security/qubes_isolation.rs` |
| **Alpine Linux** | Diskless RAM rootfs overlays (`lbu`), musl lightweight userland | `src/distro/bsd_linux_innovations.rs` |
| **NixOS / Guix** | Atomic store paths (`/sigma/store`), declarative channel commit pinning | `src/distro/additional_linux_bsd_components.rs` |
| **Zorin OS** | Executable Security Guard (`zorin-exec-guard`), adaptive theme chameleon engine | `src/security/exec_guard.rs`, `src/desktop/` |
| **Linux Mint** | WebApp Manager (PWAs as desktop apps), Timeshift CoW snapshotting | `src/desktop/web_wasm_bridge.rs`, `src/system/snapshot.rs` |
| **Omarchy** | Agentic Wayland compositor (`ZenithCompositor`), hotkey studio, Quickshell plugins | `src/distro/omarchy.rs`, `src/distro/omarchy_inspiration.rs` |

---

## 5-Layer SigmaOS Architecture

AI agents must place new components into the appropriate architectural layer:

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: Agentic Desktop & WebApp Bridge (Wayland, Zenith, PWAs, WASM) │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: Universal Packaging & Storage (sigpkg, A/B Roots, Delta Sync) │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: Security & Sandboxing (Pledge, Unveil, Capabilities, Qubes)   │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: Driver Shims & Hardware HAL (PCIe ECAM, DRM/KMS, Audio, HID)  │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: Safe-Rust Kernel Shards (12 Isolation Shards, Lockless LWKT)  │
└────────────────────────────────────────────────────────────────────────┘
```

1. **Layer 1: Safe-Rust Microkernel Shards** (`src/kernel/`): 12 decoupled memory-safe shards communicating via zero-copy lockless ring buffers (`klib::ring_buffer`).
2. **Layer 2: Driver Shims & Hardware HAL** (`src/driver/`, `src/drivers/`): PCI ECAM MMIO enumeration, NVMe/SATA storage, DRM/KMS display output, and USB/HID drivers.
3. **Layer 3: Security & Isolation Subsystems** (`src/security/`): OpenBSD-style `pledge`/`unveil`, Linux Landlock v5, FreeBSD Capsicum, eBPF LSM, and user namespace isolation.
4. **Layer 4: Universal Package Manager & Storage** (`src/package/`, `src/sigpkg/`, `src/system/`): Multi-distro package metadata adapters (`.deb`, `.rpm`, `.apk`, `.xbps`, `.sigpkg`), transactional A/B root updates, and Btrfs/ZFS CoW snapshots.
5. **Layer 5: Agentic Desktop & Userland Experience** (`src/desktop/`, `src/distro/`): Zenith Wayland compositor, Omarchy Quickshell plugin system, and PWA WebApp container runtime.

---

## Tri-Agent Autonomous Development Protocols

SigmaOS development is driven by three specialized AI Agent personas:

### ⚡ Bolt (Performance Agent)
- **Focus**: Algorithmic complexity reduction, lockfree concurrency, zero-copy memory transfers, sub-millisecond latency.
- **Rule**: "Measure first, optimize second." Never sacrifice readability for unmeasurable micro-optimizations.
- **Verification**: Profile execution paths and measure memory footprint impact.

### 🎨 Palette (UX & Accessibility Agent)
- **Focus**: User interface delight, ARIA accessibility, screen reader support, keyboard navigation, responsive Wayland layouts.
- **Rule**: "Accessibility is not optional." Every user interaction must feel fluid and responsive.
- **Verification**: Validate focus states, keyboard shortcuts, and UI event responsiveness.

### 🛡️ Sentinel (Security Agent)
- **Focus**: Defensive engineering, input sanitization, path traversal mitigation, memory safety, privilege escalation bounds.
- **Rule**: "Trust nothing, verify everything." Eliminate CWE patterns (path traversal, option injection, boundary confusion).
- **Verification**: Run standalone unit tests with adversarial attack vectors.

---

## AI Agent Component Implementation Workflow

When an AI agent is tasked with creating or modifying a SigmaOS component, it must execute the following 5-step process:

### Step 1: Research & Distro Inspiration Mapping
Identify the relevant Linux or BSD distribution inspiration for the target component. Study existing architectural patterns in `src/` and `docs/`.

### Step 2: Implementation & Code Standards
- Write **100% native Rust code** (`#![no_std]` compatible for kernel/drivers where applicable).
- Adhere strictly to the zero-dependency strategy (no external crates beyond standard library / custom `klib` primitives).
- Implement robust error handling without unwrap/panic in production code paths.
- Enforce strict input validation (use `src/security/input_validation.rs` for path, filename, hostname, and IP validation).

### Step 3: Standalone & Integrated Unit Testing
Every new component or security enhancement **MUST** include standalone unit test blocks guarded by `#[cfg(test)]`.

AI agents can verify components directly using standalone `rustc` test compilation:
```bash
# Example: Standalone unit test compilation and execution
rustc --test --edition=2021 src/security/bsd_hardening.rs -o build/bsd_hardening_test && ./build/bsd_hardening_test

# Example: Running the full integration test suite
./run_sigma_tests.sh
```

### Step 4: Documentation & Synchronization
Documentation is an active part of codebase development. When adding or modifying a component:
1. Document architectural decisions in `docs/`.
2. Update or create corresponding GitHub Wiki guides.
3. Run `scripts/sync_wiki.sh` to synchronize docs across mirror directories (`docs/`, `WIKI/`, `wiki/`, `wiki_repo/`, `wiki_content/`).

### Step 5: Pre-Commit & Submission
Before submitting changes, AI agents must:
- Run all relevant unit and integration test suites.
- Request code review and implement feedback.
- Submit clean, well-documented commits with descriptive commit messages following standard conventions.

---

## Verification & Compliance Checklist

AI agents must confirm compliance with the following criteria before finalizing any component:

- [ ] **Zero External Dependencies**: Uses only native Rust or `klib` primitives.
- [ ] **Path & Input Validation**: Validates all input data against directory boundaries, NUL bytes, and control characters.
- [ ] **Memory Safety**: Uses safe Rust constructs with zero unhandled pointer dereferences.
- [ ] **Unit Test Coverage**: Includes standalone `#[cfg(test)]` blocks runnable via `rustc --test`.
- [ ] **Integration Test Parity**: Integrates cleanly into `./run_sigma_tests.sh`.
- [ ] **Documentation Mirroring**: Synchronized across `docs/`, `WIKI/`, `wiki/`, `wiki_repo/`, and `wiki_content/`.
