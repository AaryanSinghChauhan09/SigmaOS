# SigmaOS AI Agent Component Development Framework

## Executive Summary & Vision

The **SigmaOS AI Agent Component Development Framework** establishes a rigorous, cleanroom-engineered architecture for autonomous AI agents (specifically **Bolt ⚡**, **Palette 🎨**, and **Sentinel 🛡️**) to design, implement, audit, and evolve operating system components within SigmaOS. Drawing deep architectural inspiration from top-tier Linux and BSD distributions—such as Arch Linux's KISS/rolling packaging, Debian's strict policy control, Fedora's bleeding-edge kernel integration, FreeBSD's subsystem modularity, OpenBSD's security-first pledge/unveil sandboxing, Alpine's zero-dependency minimalism, and NixOS/Guix's declarative reproducibility—this framework defines how AI agents autonomously develop OS subsystems without introducing technical debt, memory unsafety, or performance regressions.

---

## 1. Architectural Foundations Inspired by Linux & BSD

### 1.1 Multi-Distro Engineering Paradigms
SigmaOS merges proven operating system design principles across major Linux distributions and BSD operating systems:
* **Linux Kernel Subsystem Isolation (Linux v6.8+ LTS)**: Decoupled driver sharding, lockless SPSC DMA ring queues, eBPF/XDP networking filters, and `SCHED_DEADLINE` / BORE scheduling.
* **OpenBSD Security Hardening (OpenBSD 7.4)**: Strict Default-Deny capability policies (`ZorinExecGuardPolicyEngine`), path unveiling (`unveil`), system call restriction (`pledge`), and W^X memory execution enforcement.
* **FreeBSD Subsystem Modularity (FreeBSD 14)**: Plugable MAC framework (`mac_biba`, `mac_lomac`), Capsicum capability mode sandboxing, ZFS Boot Environments (`bectl`), and UMA zone memory management (`BsdVmZoneAllocator`).
* **NixOS / Guix Declarative Reproducibility**: Content-addressable store (CAS) verification, zero-dependency hermetic builds, and transactional zero-copy rollback (`DualRootAbUpdateEngine`).
* **Arch Linux / Omarchy Userland Elegance**: Rolling package definitions (`sigpkg`), Omakase workspace layouts, Hyprland dynamic tiling, and target-side installation/setup script orchestration (`OmarchyInstallerEngine`).

---

## 2. Tri-Agent Governance Matrix (Bolt ⚡, Palette 🎨, Sentinel 🛡️)

SigmaOS component engineering is governed by three specialized, complementary AI agent personas:

| Agent Persona | Specialty Domain | Focus Areas & Directives | Mandatory Verification Threshold |
| :--- | :--- | :--- | :--- |
| **Bolt ⚡** | **Performance & Efficiency** | Lock-free data structures, zero-copy I/O (`io_uring`/XDP), SIMD acceleration, memory footprint reduction, cache locality. | < 5% CPU overhead, zero allocation loops in hot paths, O(1) or O(log N) complexity. |
| **Palette 🎨** | **UX, A11y & Visual Polish** | Wayland desktop shell, high-DPI display scaling, AT-SPI2 accessibility, contrast ratios, keyboard navigation, tooltips. | WCAG 2.2 AAA compliance, sub-16ms frame render time (60+ FPS), full keyboard accessibility. |
| **Sentinel 🛡️** | **Security & Sandboxing** | Exec Guard default-deny enforcement, PQC signature validation, Landlock/Capsicum sandboxing, ASLR/SMEP/SMAP memory protection. | Zero unsafe code leaks, 100% PQC signature coverage, zero unauthenticated RPCs/syscalls. |

---

## 3. Autonomous AI Component Lifecycle & State Machine

AI agents developing SigmaOS components must strictly adhere to the following 5-phase execution state machine:

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│ 1. PROFILE &    │ ──> │ 2. BLUEPRINT &   │ ──> │ 3. CLEANROOM    │
│    GAP ANALYSIS │     │    SPECIFICATION │     │    DEVELOPMENT  │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                                                          │
                                                          ▼
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│ 5. SYNC & WIKI  │ <── │ 4. DUAL-TEST     │ <── │   VERIFICATION  │
│    TRANSFER     │     │    REGRESSION    │     │   HARNESS       │
└─────────────────┘     └──────────────────┘     └─────────────────┘
```

### Phase 1: Profile & Gap Analysis
* **Goal**: Identify missing OS capabilities or performance/security bottlenecks using system benchmarks, gap analyses, or user feature specifications.
* **Action**: Audit existing Rust modules (`src/`) against BSD and Linux reference implementations.

### Phase 2: Blueprint & Specification
* **Goal**: Draft a comprehensive, single-source architectural blueprint in `docs/`.
* **Action**: Define API signatures, data structures, memory layouts, error handling policies, and multi-distro comparative analyses.

### Phase 3: Cleanroom Development
* **Goal**: Implement zero-dependency Rust code (`src/`).
* **Guidelines**:
  - Keep changes modular and self-contained (< 50 lines for targeted micro-optimizations, full modular files for major subsystems).
  - Use `core::ptr::write_volatile` or zeroing primitives for sensitive cryptographic memory.
  - Return RAII guards (`PriorityInheritanceMutexGuard`, etc.) instead of raw unsafety.

### Phase 4: Dual-Test Regression Verification
* **Goal**: Execute both native Rust unit tests and system integration test suites.
* **Commands**:
  - `./run_sigma_tests.sh` (120+ native Rust test suites)
  - `pytest tests/` (15 Python integration test suites)
  - Standalone module compiler: `rustc --test <filepath> --edition=2021`

### Phase 5: Wiki Synchronization & Knowledge Transfer
* **Goal**: Transfer verified component documentation and implementation details to GitHub Wiki mirrors (`wiki/`, `WIKI/`, `wiki_content/`, `wiki_repo/`).
* **Action**: Update `_Sidebar.md` and master documentation indices to ensure developer discoverability.

---

## 4. Agentic Safety Boundaries & Development Directives

### ✅ Always Do
1. **Verify Before Progressing**: Always inspect edited files using read-only tools or test execution commands before marking steps complete.
2. **Zero Dependency Bloat**: Prioritize standard `core`/`alloc` Rust constructs over third-party crate dependencies.
3. **Comprehensive Tests**: Include embedded unit tests (`#[cfg(test)]`) for every new function or struct added.
4. **Multi-Mirror Parity**: Keep `docs/` blueprints and `wiki/` documentation synchronized word-for-word.

### ⚠️ Ask / Review First
1. Major architectural refactoring affecting core memory allocation (`src/memory/`) or process scheduler (`src/kernel/sched/`).
2. Adding new third-party crate dependencies to `Cargo.toml`.
3. Modifying global default-deny security policies in `ZorinExecGuardPolicyEngine`.

### 🚫 Never Do
1. Never commit plaintext passwords, static private keys, or unhashed authentication tokens.
2. Never bypass pre-commit review procedures or submit unverified changes.
3. Never edit compiled build artifacts (`target/`, `dist/`) directly—always modify source files in `src/`.

---

## 5. Verification Metrics & Quality Indicators

A component built by an AI agent is considered complete **only when**:
* **Compilation**: `rustc` compiles the module without warnings or errors.
* **Unit Test Pass Rate**: 100% pass rate across embedded `#[cfg(test)]` modules.
* **Integration Pass Rate**: 100% pass rate across Python and Rust integration test harnesses.
* **Security Inspection**: `Sentinel 🛡️` confirms zero memory leaks or unhandled capability exceptions.
* **Documentation Parity**: Complete markdown blueprints exist in `docs/` and `wiki/` with updated navigation sidebars.

---

*Document Specification maintained under MIT License. Authoritative for all AI Agents operating on SigmaOS.*
