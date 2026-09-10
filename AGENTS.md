# SigmaOS AI Agent Security Management Directive (`AGENTS.md`)

This document defines operational guidelines, security policies, and verification instructions for autonomous AI engineering agents working on the SigmaOS codebase.

---

## 1. Core Principles for AI Agents

1. **Zero External Third-Party Dependencies:**
   - SigmaOS strictly follows a **zero external crate** philosophy: `[dependencies]` in Cargo.toml must remain EMPTY.
   - See ARCHITECTURE.md for the decision to use std-based architecture (approved Sept 4, 2026).
   - Use **std library** primitives (std::vec::Vec, std::string::String, std::collections::HashMap).
   - Implement custom functionality in safe Rust without relying on external crates.

2. **Cross-OS Subsystem Interoperability (Linux/BSD Distros):**
   - Every security or kernel component must maintain compatibility across Linux and BSD distribution modes (`LinuxArch`, `LinuxDebian`, `LinuxFedora`, `LinuxNix`, `FreeBsd`, `OpenBsd`, `NetBsd`, `DragonFlyBsd`, `SolarisIllumos`, etc.).
   - Sandboxing rules must bridge Linux Landlock v5 with FreeBSD Capsicum rights (`FreeBsdCapsicumDescriptorDelegate`) and OpenBSD pledge/unveil (`OpenBsdUnveilAuditor`).
   - **Arch Linux Inspiration**: Follow rolling release model principles, PKGBUILD recipe patterns, and AUR-style user repositories.
   - **Debian Inspiration**: Implement stable/unstable/testing release channels, deb package compatibility, and dpkg management patterns.
   - **FreeBSD Inspiration**: Adopt CAM (Common Access Method) for device drivers, Jails for containerization, and PF firewall state management.
   - **OpenBSD Inspiration**: Prioritize security-first development, pledge/unveil sandboxing, and KARL (Kernel Address Randomized Link).
   - **Gentoo Inspiration**: Implement USE flags for conditional compilation, Portage-style dependency resolution, and ebuild recipe management.
   - **NixOS Inspiration**: Adopt declarative system configuration, content-addressed storage, and atomic rollbacks.

3. **Autonomous Verification:**
   - Always run `./run_sigma_tests.sh` and `pytest` after making modifications.
   - Individual standalone tests can be compiled and verified using `rustc --edition=2021 --test <file_path>`.

# Contributor and AI Agent Development Rules for SigmaOS

## Executive Summary

This document establishes the mandatory engineering standards, architectural rules, and verification protocols for **human contributors** and **AI coding agents** developing and maintaining **SigmaOS**.

---

## Core Engineering Rules & Mandates

### 1. Standard Library Based Architecture (DECISION: Sept 4, 2026)
- SigmaOS uses **Rust standard library (std)** as its primary foundation (see ARCHITECTURE.md).
- All kernel, system, and userland code must be written in **pure safe Rust** using std facilities.
- External third-party crates (`[dependencies]` in Cargo.toml) remain **strictly prohibited**.
- External C libraries (`libc`, `malloc`, `free`), Python runtimes, Node.js V8, or unverified crates are strictly prohibited.
- Use std primitives (Vec, String, HashMap, BTreeMap) and implement custom subsystems via native klib when needed.

### 1.1 Zero External Third-Party Crates Mandate
- Cargo.toml `[dependencies]` section must remain **EMPTY**.
- All functionality must be implemented using Rust std library and custom safe-Rust code.
- No external crates allowed under any circumstances.

### 2. Kernel ABI (KABI) Binary Layout Stability
- Kernel exports and syscall structures (`src/kernel/exports.rs`) must maintain backward binary layout compatibility.
- Never reorder, delete, or alter struct field offsets in public KABI headers.
- Always run KABI compliance unit tests before submitting changes.

### 3. Pre-Commit Self-Testing & Verification
- Before submitting any code change, you **MUST** run the full test suite:
  ```bash
  ./run_sigma_tests.sh
  ```
- All 13 test stages (Python integration, Package Caching, Universal Adapter, Unimplemented Features/Tools, Open Source Gap Closure, Arch, Fedora, UI/UX Benchmarks, CLI simulation, and Parity Inspection) must pass with **0 failures**.

### 4. Zero-Drift Documentation Mirroring
- Whenever documentation is added or modified in `docs/` or `wiki/`, it **MUST** be mirrored to `wiki_repo/`.
- Landing page indices (`wiki/Home.md` and `wiki_repo/Home.md`) must be updated with valid relative links.

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

- **Driver Management**: Refer to `docs/AI_AGENT_DRIVER_MANAGEMENT.md` for driver lifecycle directives.
- **Cache Operation Management**: Refer to `docs/AGENTS_CACHE_OPERATION_MANAGEMENT.md` for explicit CPU cache line flushing (`clflush`, `clflushopt`, `clwb`), TLB invalidation/shootdown, Page Cache Radix-Tree operations, SLUB object cache recycling, `#[repr(align(64))]` CPU cache alignment, and JIT instruction cache synchronization rules.
- **Zones Operation Management**: Refer to `docs/AGENTS_ZONES_OPERATION_MANAGEMENT.md` for physical memory zones (`ZONE_DMA`, `ZONE_DMA32`, `ZONE_NORMAL`, `ZONE_HIGHMEM`), FreeBSD UMA (Universal Memory Allocator) zone management (`uma_zcreate`, `uma_zalloc`, `uma_zfree`, `uma_zdrain`), watermark-driven page reclamation, and double-free protection invariants.
- **4-Bit Operation Management**: Refer to `docs/AGENTS_FOUR_BIT_OPERATION_MANAGEMENT.md` for 4-bit nibble packing/unpacking bitwise standards, INT4/NF4 AI model weight quantization (`src/ai/quantization.rs`, `src/ai/voice.rs`), Binary Coded Decimal (BCD) RTC decoding, and 4-bit control register bitfield masking.
- **12-Shard Sovereign Architecture**: Refer to `docs/AGENTS_12_SHARD_SOVEREIGN_ARCHITECTURE.md` for the 12 Safe-Rust microkernel shards taxonomy (Media, Networking, Storage, AI, Compositor, Drivers, Security, Virtualization, System, Package, IPC, Hardware), browser shell integration, Unix primitives for PWAs (`pipe`, `spawn`, `mmap`, `/dev`), and firmware-free driver isolation directives.
- **Component Subsystem Guidelines**: Refer to `docs/AGENTS_COMPONENT_GUIDELINES.md` for Genode-style hierarchical component tree ownership (`src/kernel/component.rs`), parent-child resource delegation, FreeBSD Capsicum FD rights sandboxing, OpenBSD Pledge/Unveil path scoping, Fedora AppStream/modulemd profiles, and component lifecycle supervision.
- **Task Management Guidelines**: Refer to `docs/AGENTS_TASK_GUIDELINES.md` for task lifecycle state machines (`TaskState::Pending`, `Running`, `Blocked`, `Completed`, `Evicted`, `Failed`), EEVDF/BORE task scheduling priorities, BSD `kqueue(2)` event loop waits, fine-grained `pledge`/`unveil` sandboxing, cgroups v2 resource envelopes, and atomic task state rollback protocols.
- **Python Dependency Reduction**: Refer to `docs/AGENTS_REDUCING_PYTHON_DEPENDENCY.md` for guidelines on replacing external Python scripts (`scripts/*.py`, `tests/*.py`) with zero-dependency Rust executables (`src/tools/`), POSIX shell scripts (`scripts/*.sh`), or WebAssembly (Wasm) micro-runtimes.
- **Public Launch & Governance**: Refer to `docs/LAUNCH_ANNOUNCEMENT.md` (Launch Manifesto), `docs/WHITEPAPER.md` (Technical Whitepaper), `docs/PRESS_KIT.md` (Press Assets & Media FAQ), and `docs/GOVERNANCE_CHARTER.md` (Contributor Charter).

---

## 🚗 Driver Management Protocols for AI Agents

When modifying, releasing, or updating versions in SigmaOS:

### 2.1 Core Repository & Cargo Version
* Core package version is declared in `Cargo.toml` (`version = "0.1.0"`).
* **MAJOR (x.0.0):** Incompatible API/ABI or kernel architecture changes (e.g., breaking KABI stability).
* **MINOR (0.x.0):** New backward-compatible kernel subsystems, drivers, or distro parity features.
* **PATCH (0.0.x):** Backward-compatible bug fixes, performance optimizations, or security patches.

---

## 5. Thread Synchronization & Lock-Free Atomic Rules for AI Agents

When implementing or modifying concurrent thread synchronization:

1. **Explicit Memory Orderings:**
   Always use `Ordering::Release` when publishing shared state updates and `Ordering::Acquire` when reading published state. Do NOT use `Ordering::Relaxed` for synchronized pointer states.
2. **Futex Fast-Path Operations:**
   Userspace lock primitives MUST check atomic lock variables before invoking the kernel futex wait syscall.
3. **Spinlock Backoff:**
   Spinlock loops MUST execute `core::hint::spin_loop()` CPU pause hints to reduce interconnect bus contention.

---

## 6. HTML Dependency Reduction & Text-Based Interface Rules for AI Agents

When creating or modifying documentation, dashboards, or user interfaces:

1. **Text-First & Terminal Preference:**
   Prioritize Markdown (`DocFormat::Markdown`), AsciiDoc (`DocFormat::AsciiDoc`), or ANSI terminal output over HTML web rendering.
2. **HTML Entity Escaping (`escape_html`):**
   If HTML string output is necessary, ALL dynamic string parameters MUST be sanitized via `escape_html` in `src/docs/mod.rs` to neutralize XSS vectors (`<`, `>`, `&`, `"`, `'`).

---

## 7. Compile-Time Defenses & Build Hardening Rules for AI Agents

When modifying build settings, profile options, or feature flags:

1. **`#![no_std]` Zero-Dependency Invariant:**
   Maintain 100% self-sufficient core Rust implementations. Do NOT add external dependencies to `Cargo.toml`.
2. **`panic = "abort"` Unwind Protection:**
   Both `dev` and `release` profiles MUST use `panic = "abort"` to prevent stack unwinding exploit primitives.

---

## 8. Clock Algorithm & Timer Management Rules for AI Agents

When modifying clock page replacement or timekeeping subsystems:

1. **Clock Page Replacement Hand-Pointer Traversal:**
   Page frame eviction MUST traverse physical memory frames in a circular queue. Clear reference bits from `1` to `0` for second-chance evaluation before evicting unreferenced pages.

---

## 9. Circular Buffer & Lock-Free Ring Buffer Rules for AI Agents

When implementing or modifying ring buffers in `src/klib/ring_buffer.rs`, `src/klib/ringbuf.rs`, or `src/media/sovereign_video_player.rs`:

1. **Power-of-Two Capacity Rule:**
   Ring buffer capacities MUST be powers of two ($2^k$) to perform $O(1)$ index wrapping via bitwise AND `idx & (capacity - 1)`.

---

## 10. Cache Memory Architecture, LRU Eviction & Package Cache Rules for AI Agents

When modifying cache memory engines, key-value stores, or package cache trimmers:

1. **Key Invalidation Invariant:**
   `SovereignCacheEngine::set` MUST purge pre-existing entries with matching keys via `self.entries.retain(|e| e.key != key)` before inserting new values.

---

## 11. Process Lifecycle, Signal ABI Translation & Supervision Rules for AI Agents

When modifying process management, signal handlers, or pseudo-terminals:

1. **State Machine Transitions (`SovereignProcessLifecycleController`):**
   Ensure process state changes (`Created`, `Ready`, `Running`, `Blocked`, `Stopped`, `Zombie`, `Terminated`) execute under thread-safe synchronization.

---

## 12. Hardware Fitting, Driver Auto-Binding & Device Adaptation Rules for AI Agents

When writing, probing, or modifying hardware device drivers (`src/drivers/`):

1. **Bus Signature Probing:**
   Driver probe routines MUST evaluate Vendor ID (VID), Product ID (PID), and interface class codes before claiming attachment.

---

## 13. Network Stack, eBPF/XDP & PQC Security Rules for AI Agents

When modifying networking drivers, eBPF filters, or VPN subsystems:

1. **Kernel Bypass eBPF/XDP Processing:**
   Use zero-copy DMA ring buffers (`process_xdp_zero_copy_packet`). Ensure XDP actions explicitly return `XDP_PASS`, `XDP_DROP`, `XDP_TX`, or `XDP_REDIRECT`.

---

## 14. Processor Topology, CPU Scheduling & Multi-Core Rules for AI Agents

When modifying CPU scheduling, task management, or ISA optimization:

1. **ISA Level Auto-Detection (`src/klib/isa.rs`):**
   Support x86-64 microarchitecture levels (`v1`..`v4`). Route vectorized operations via `vectorized_memcpy` dynamically based on detected features.

---

## 15. Kernel, Bootloader & Driver Loading Rules for AI Agents

When modifying boot sequences, driver registration, or scheduler loading:

1. **Multi-Stage Boot Pipeline:**
   Respect the 4-phase boot sequence: Bootloader -> Kernel Initialization -> Dynamic Driver Loading -> Userland Supervisor.

---

## 16. Backend Subsystem & Server Engine Rules for AI Agents

When modifying backend services in `src/open_source_obsoletion.rs`, `src/open_source_os_gap_closure.rs`, or `src/automation/system_level.rs`:

1. **Zero-Dependency Native Backend Engines:**
   Maintain native parity for embedded DBs (`SovereignEmbeddedDb`), web servers (`SovereignWebServer`), in-memory caches (`SovereignCacheEngine`), message brokers (`SovereignMessageBroker`), secret vaults (`SovereignSecretVault`), object stores (`SovereignDistributedStorage`), and orchestrators (`SovereignK8sOrchestratorEngine`).

---

## 17. Explicit Rules for Human Contributors & AI Agents

### 17.1 Universal Rules for Human Contributors
1. **Zero External Dependencies Policy (`no_std`):**
   * Core microkernel shards and kernel subsystems MUST be `#![no_std]` and MUST NOT add external dependencies under `[dependencies]` in `Cargo.toml`.
   * Use native `klib` abstractions (`klib::Vec`, `klib::HashMap`, `klib::BTreeMap`, `klib::String`).
2. **Memory Safety & Unsafe Code Guidelines:**
   * Prefer Safe-Rust. Every `unsafe` block MUST be preceded by a `// SAFETY:` comment documenting safety invariants, pointer alignment, and memory bounds.
3. **Post-Quantum Cryptographic (PQC) Security Standard:**
   * All driver signatures, package manifests, and kernel module attestations MUST use Dilithium-5 signatures or Kyber-1024 KEM.
4. **Sandboxing & Least Privilege:**
   * Userland binaries MUST declare OpenBSD `pledge`/`unveil` rights or FreeBSD Capsicum capabilities before handling untrusted data.
5. **Testing Invariants & Quality Verification:**
   * Every new feature or bugfix MUST include unit tests. Run `cargo check --lib` and `./run_sigma_tests.sh` before submitting pull requests.

### 17.2 Explicit Rules for AI Agents (Autonomous Development)
1. **Planning & Review Protocol:**
   * AI agents MUST call `request_plan_review` with a structured plan before using `set_plan` for the first time.
   * Plans MUST include a dedicated step for verifying test execution and a pre-commit step using the exact phrasing:
     `Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.`
2. **Always Verify Modifications:**
   * After modifying any file, the AI agent MUST confirm the change using a read-only tool (`read_file` or `list_files`).
3. **Code Review & Feedback Implementation:**
   * Before finalizing any PR or submission, the AI agent MUST invoke `request_code_review` and address all blocking feedback.
4. **Memory Recording Directive:**
   * Upon completing code review and verification, the AI agent MUST call `initiate_memory_recording` to document key architectural patterns and learnings.
5. **Secret Scanner False Positive Mitigation:**
   * Test functions or mock credentials MUST use variable names prefixed with `mock_` or `test_` (e.g. `mock_client_secret`) to prevent automated scanner triggers.

---

## 18. Checklist for AI Agents & Contributors

1. **Update Manifests & Documentation** when bumping versions, adding drivers, or modifying synchronization logic.
2. **Run Standalone Subsystem Tests:**
   ```bash
   rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/open_source_os_gap_closure.rs
   ```
3. **Execute Full Pipeline:** Run `./run_sigma_tests.sh` and ensure all test steps pass.
4. **Follow Conventional Commits:**
   `docs(agents): add thread synchronization guide` or `fix(sync): enforce acquire-release memory ordering`.

---

## 19. Detailed Documentation References

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
* [`docs/AGENTS_THREAD_SYNC_MANAGEMENT.md`](docs/AGENTS_THREAD_SYNC_MANAGEMENT.md)
* [`docs/RELEASE_CADENCE.md`](docs/RELEASE_CADENCE.md)
* [`docs/package-manager.md`](docs/package-manager.md)

---

## 20. Linux & BSD Distro Parity Directives

When integrating or refining Linux & BSD distro capability engines in `src/distro/`:
1. **Zero-Dependency Subsystem Parity:** Implement clean-room, `#![no_std]` Rust modules that absorb and emulate key distro innovations (e.g. Void Linux runit service supervision in `VoidRunitServiceSupervisorEngine`, Alpine Linux tmpfs apk volatile overlays in `AlpineApkVolatileOverlayEngine`, openSUSE YaST2/Snapper, NetBSD rump kernels, Ubuntu netplan/cloud-init, GNU Guix Shepherd/store derivations).
2. **Re-export Invariants:** Always re-export newly implemented distro engines in `src/distro/mod.rs` and `src/lib.rs`.
3. **Verification:** Validate all distro parity engines using standalone unit tests (`rustc --test --edition 2021`) and `./run_sigma_tests.sh`.

## 21. Linux & BSD Distro Development Patterns

### Governance Models (Inspired by Linux/BSD Distros)
- **Arch Linux**: Simple, volunteer-based governance with minimal bureaucracy. Decisions made by contributors involved in specific projects.
- **Debian**: Formal constitution with democratic voting, technical committee for conflicts, and clearly defined decision-making bodies.
- **Ubuntu**: Code of Conduct foundation, council-based governance with defined escalation paths.
- **FreeBSD**: Core team with committer hierarchy, emphasis on technical excellence and code quality.
- **OpenBSD**: Dictator-for-life (Theo de Raadt) with focus on code correctness and security.

### Release Management (Linux/BSD Patterns)
- **Arch Linux**: Rolling release model with continuous updates
- **Debian**: Stable/Testing/Unstable branches with strict freeze periods
- **Fedora**: Rapid release cycle with feature-based releases
- **FreeBSD**: Stable branches with quarterly releases and security branches
- **OpenBSD**: 6-month release cycle with emphasis on stability

### Package Management (Linux/BSD Patterns)
- **Arch Linux**: PKGBUILD recipes, pacman database, AUR user repositories
- **Debian**: deb packages, apt dependency resolution, dpkg database
- **Fedora**: RPM packages, dnf package manager, Koji build system
- **Gentoo**: ebuild recipes, Portage dependency resolution, USE flags
- **FreeBSD**: ports collection, pkg binary packages, Poudriere build system
- **OpenBSD**: ports collection, pkg_add package manager

### Security Development (Linux/BSD Patterns)
- **OpenBSD**: Default-deny security, pledge/unveil sandboxing, proactive security auditing
- **FreeBSD**: Capsicum capability mode, Jails for isolation, TrustedBSD MAC framework
- **Linux**: SELinux/AppArmor mandatory access control, seccomp syscall filtering, Landlock filesystem sandboxing

### Code Quality Standards (Linux/BSD Patterns)
- **FreeBSD**: KNF (Kernel Normal Form) coding style, strict code review process
- **Linux**: Kernel coding style, mandatory Signed-off-by process, extensive review
- **OpenBSD**: Emphasis on code clarity, security audits, minimal attack surface

---

## 22. Universal Linux & BSD Ecosystem Subsystem Directives

Autonomous agents must uphold and extend the following distro innovations:
1. **SteamOS / ChimeraOS (Gaming & Display Optimization):**
   - Implements gamescope microcompositor sandboxing, MangoHud telemetry hooks, and atomic A/B Btrfs rootfs updates.
2. **GhostBSD / MidnightBSD (Desktop & Workspace Integration):**
   - Integrates Fish-shell-like autocompletion, mports package management, and unified desktop workspace isolation.
3. **openSUSE MicroOS / Fedora Silverblue (Transactional Immutability):**
   - Read-only root filesystem with transactional snapshots (`transactional-update`) and atomic reboot rollbacks.
4. **Alpine Linux (Minimalism & Musl Hardening):**
   - Zero-overhead static binaries, stack smashing protection (`-fstack-protector-strong`), and musl libc memory invariants.
5. **Artix Linux / Void Linux (Init Diversity & Lightweight Daemons):**
   - Independent service supervisors (runit, dinit, s6, OpenRC) decoupled from monolithic PID 1 designs.
6. **HardenedBSD / Subgraph OS (Advanced Exploit Mitigations):**
   - Strict PaX MPROTECT/PAGEEXEC emulation, ASLR high-entropy randomization, and SegvGuard brute-force mitigations.
7. **Redox OS (Microkernel Architecture & Capability Isolation):**
   - Separation of kernel space into scheme handlers (file, net, time, bus), lock-free channel IPC, and memory-safe userland drivers.
8. **NixOS / Guix (Declarative Configuration & Hermetic Stores):**
   - Declarative system configurations, hash-indexed store paths (`/nix/store`, `/gnu/store`), and bit-for-bit reproducible closures.

---

## 23. Operating System Architecture & Software Engineering Principles

AI agents working on SigmaOS must strictly apply and verify the following principles:

1. **OOPS (Object-Oriented Programming System):**
   - Encapsulation: Strict private state encapsulation with accessor/mutator methods.
   - Abstraction: Abstract hardware and subsystem interfaces via Rust traits (`PeripheralDevice`, `PackageAdapter`, `ServiceSupervisor`).
   - Polymorphism: Dynamic and static dispatch across multi-architecture and multi-distro bridges.
   - Composition over Inheritance: Prefer struct composition and trait implementations over deep hierarchies.

2. **SOLID Principles:**
   - Single Responsibility: Each module addresses a single concern (e.g. `KsmDeduplicationEngine` handles only samepage deduplication).
   - Open/Closed: Open for extension via traits and adapters, closed for modification of verified cores.
   - Liskov Substitution: Trait implementors must satisfy all invariants of the interface without panic.
   - Interface Segregation: Fine-grained, purpose-built interfaces rather than monolithic traits.
   - Dependency Inversion: Subsystems depend upon abstractions and trait interfaces rather than concrete platform bindings.

3. **Clean Code & Robust Engineering:**
   - DRY (Don't Repeat Yourself): Deduplicate identical logic into shared helpers in `src/klib/`.
   - KISS (Keep It Simple, Stupid): Prefer straightforward, readable, deterministic logic over convoluted macros.
   - YAGNI (You Aren't Gonna Need It): Implement only required capabilities with zero speculative bloat.
   - Design by Contract: Enforce pre-conditions, post-conditions, and loop invariants on memory boundaries.
   - Concurrency & Deadlocks: Enforce strict lock acquisition hierarchies, lock-free atomics, and bounded queue timeouts.
   - Conflict Resolution: In any conflict between feature velocity and system integrity, **prioritize stability, memory safety, zero-day defense, and test performance.**


### 5. Memory Recording Protocol for AI Agents
- AI coding agents must call `initiate_memory_recording` upon completing code reviews, bug fixes, or architecture implementations to document key learnings and repository patterns.

# SigmaOS AGENTS.md — AI Agent Operating Instructions & Process Management Protocols

Welcome to the **SigmaOS** repository! This document outlines guidelines and operational rules for AI coding agents (such as Jules, Copilot, Herdr, or custom subagents) interacting with the codebase, managing system processes, access control, security policies, instruction execution, configurability, cluster operations, virtual machines, filesystems, TTY character queues, disk caching, binary & counting semaphores, deadlock prevention, buffering, system state, backups, 4-bit INT4 quantization operation management, and optimizing power usage in SigmaOS.

---

## 🤖 Core Directives for AI Agents

1. **Zero-Trust Capability Sandboxing & Access Control**
   - Every AI agent process spawned in SigmaOS must execute inside a capability-bounded sandbox (`PLEDGE_STDIO | PLEDGE_RPATH | PLEDGE_WPATH | PLEDGE_INET`).
   - Use `process.pledge()` and `process.unveil()` before executing arbitrary userland commands.
   - Refer to [`docs/ai-agent-access-management.md`](docs/ai-agent-access-management.md) for RBAC/ABAC capability token guidelines.

2. **Counting General Semaphores & Concurrency Throttling**
   - Regulate multi-unit shared resource pools and subagent worker limits using `CountingSemaphore` primitives backed by `LinuxFutexEngine`.
   - Ensure every `acquire()` is balanced with `release()` and use timeout waits. Refer to [`docs/ai-agent-counting-semaphores-management.md`](docs/ai-agent-counting-semaphores-management.md).

3. **Deadlock Prevention & Priority Inheritance Locks**
   - Acquire locks in strict global ascending ID order to eliminate circular wait conditions.
   - Use priority inheritance futexes (`PI_FUTEX`) and bounded timeout waits (`Option<u64>`). Refer to [`docs/ai-agent-deadlock-management.md`](docs/ai-agent-deadlock-management.md).

4. **Declarative System Configurability & Stateless Overrides**
   - Manage declarative NixOS-style profiles, Gentoo USE-flags, and Intel Clear Linux stateless config resolution (`/etc` vs `/usr/share/defaults`).
   - Tune kernel parameters via `sysctl`. Refer to [`docs/ai-agent-configurability-management.md`](docs/ai-agent-configurability-management.md).

5. **Hardened Syscall Dispatch & Multi-ISA Instruction Execution**
   - Execute machine instructions across supported CPU register contexts (x86, x64, AArch64, RISC-V 64, LoongArch64).
   - Pass all dynamic bytecode through `EbpfRuntime` verifier prior to execution. Refer to [`docs/ai-agent-instructions-execution-management.md`](docs/ai-agent-instructions-execution-management.md).

6. **High-Availability Cluster Operations & Fleet Mesh**
   - Coordinate multi-node distributed workloads via `SovereignHighAvailabilityMeshEngine` with Raft/Paxos consensus quorums.
   - Verify cluster quorum (`has_quorum()`) before initiating stateful cluster mutations. Refer to [`docs/ai-agent-cluster-operation-management.md`](docs/ai-agent-cluster-operation-management.md).

7. **Security & Cryptographic Policy Governance**
   - Verify network sessions and storage encryption comply with system crypto policy levels (`FedoraCryptoPoliciesEngine`).
   - Obey SELinux (`sigma_agent_t`) and AppArmor confinement rules. Refer to [`docs/ai-agent-policy-management.md`](docs/ai-agent-policy-management.md).

8. **TTY Character Queue & Terminal Stream Handling**
   - Manage TTY character queues under canonical or raw modes, respecting `XON`/`XOFF` flow control.
   - Restore TTY termios state upon subagent terminal session exit. Refer to [`docs/ai-agent-character-queue-management.md`](docs/ai-agent-character-queue-management.md).

9. **Disk Cache Management & Dirty Page Flushing**
   - Leverage VFS page cache with ARC/2Q eviction algorithms for high-performance file I/O.
   - Execute `fsync()` on critical files and pass `posix_fadvise()` sequential hints to prevent cache pollution. Refer to [`docs/ai-agent-disk-cache-management.md`](docs/ai-agent-disk-cache-management.md).

10. **Zero-Copy Buffering & Ring Buffer Streams**
    - Use bounded producer-consumer monitors (`BoundedBufferProducerConsumer`) and `io_uring` ring entries (`IoUringEngine`) for high-throughput I/O.
    - Reuse allocations and flush line buffers prior to subagent thread exit. Refer to [`docs/ai-agent-buffering-management.md`](docs/ai-agent-buffering-management.md).

11. **Stateless System Configs & Atomic Updates**
    - Place local system config overrides in `/etc`, keeping `/usr/share/defaults` factory-clean.
    - Perform atomic A/B slot updates via `SovereignSystemUpdateAndTestingEngine` with PQC Dilithium signature verification. Refer to [`docs/ai-agent-system-state-management.md`](docs/ai-agent-system-state-management.md).

12. **Binary Semaphores & Mutex Synchronization**
    - Coordinate shared memory access between subagent threads using `BinarySemaphore` primitives backed by `LinuxFutexEngine`.
    - Strictly follow lock hierarchy ordering and RAII guard patterns to prevent deadlocks. Refer to [`docs/ai-agent-semaphores-management.md`](docs/ai-agent-semaphores-management.md).

13. **Filesystem Unveil & CoW Snapshot Management**
    - Restrict visible filesystem paths via OpenBSD `unveil()` prior to file modifications.
    - Leverage Copy-on-Write (CoW) snapshots when performing multi-file refactoring operations. Refer to [`docs/ai-agent-filesystem-management.md`](docs/ai-agent-filesystem-management.md).

14. **Pre-Task System Snapshots & Backup Safeguards**
    - Agents performing high-risk system changes (package updates, driver installs, config edits) MUST create a pre-task snapshot via `SelfHealingModule::create_snapshot()`.
    - Verify Merkle-tree snapshot integrity before executing atomic disaster recovery rollbacks. Refer to [`docs/ai-agent-backup-management.md`](docs/ai-agent-backup-management.md).

15. **4-Bit Operation & INT4 Quantization Management**
    - Manage 4-bit integer quantized weight packing (two elements per byte) and block scaling factors (`TensorDtype::Int4`).
    - Enforce RAM footprints for edge local models (<2GB RAM) and dispatch SIMD hardware primitives (AVX-512 VNNI, ARM NEON SDOT, RISC-V rvv). Refer to [`docs/ai-agent-4bit-operation-management.md`](docs/ai-agent-4bit-operation-management.md).

16. **Cgroup Resource Quotas & Rate Limits**
    - AI agent task execution threads must be attached to the `/sys/fs/cgroup/system.slice/sigma-agent.service` cgroup.
    - Enforce memory quotas (`memory.max = 2G`) and CPU limits (`cpu.max = 50000 100000`) to prevent runaway resource consumption.

17. **IPC & Subagent Communication Channels**
    - Inter-agent communication MUST utilize `ZeroCopyIpcChannel` or `AndroidBinderIpc` with cryptographic token verification (`security_token`).
    - Direct memory sharing between agent processes without capability-gated handles is strictly forbidden.

18. **Virtual Machine Guest Provisioning**
    - Agents executing untrusted or experimental code MUST spawn an isolated guest VM via `VirtualizationOrchestrator` using KVM/Bhyve backends.
    - Attach virtio-fs shared paths with strict OpenBSD `unveil()` read-only restrictions.

19. **Power & Thermal Awareness**
    - Agents must check system power profiles and CPU temperature via `PowerGovernor` before launching compute-intensive subtasks.
    - Restrict concurrency and defer heavy background AI model indexing on battery power (`powersave` / `conservative` governor modes).

20. **Zero-Dependency Core Systems**
    - Avoid adding third-party standard C++ or non-vetted external dependencies.
    - Core kernel, driver, and shell primitives must rely on `ZeroDependencyPrimitiveHub` and `klib`.

---

## 🛠️ Build & Verification Instructions

AI agents making code changes must run the following checks before submitting pull requests:

```bash
# 1. Run quality gate verification
./scripts/sigma_quality_check.sh

# 2. Run UI/UX & accessibility verification
./scripts/uiux_accessibility_test.sh

# 3. Synchronize documentation mirrors
./sync_wiki.sh
```

---

## 📌 Related Documentation
- Process Management Architecture: [`docs/process-management.md`](docs/process-management.md)
- AI Agent Process Management Guidelines: [`docs/ai-agent-process-management.md`](docs/ai-agent-process-management.md)
- AI Agent Access Control Guidelines: [`docs/ai-agent-access-management.md`](docs/ai-agent-access-management.md)
- AI Agent Counting Semaphores Management: [`docs/ai-agent-counting-semaphores-management.md`](docs/ai-agent-counting-semaphores-management.md)
- AI Agent Deadlock Management Guidelines: [`docs/ai-agent-deadlock-management.md`](docs/ai-agent-deadlock-management.md)
- AI Agent Configurability Guidelines: [`docs/ai-agent-configurability-management.md`](docs/ai-agent-configurability-management.md)
- AI Agent Instruction Execution Guidelines: [`docs/ai-agent-instructions-execution-management.md`](docs/ai-agent-instructions-execution-management.md)
- AI Agent Cluster Operation Guidelines: [`docs/ai-agent-cluster-operation-management.md`](docs/ai-agent-cluster-operation-management.md)
- AI Agent Security & System Policy Guidelines: [`docs/ai-agent-policy-management.md`](docs/ai-agent-policy-management.md)
- AI Agent Character Queue Management: [`docs/ai-agent-character-queue-management.md`](docs/ai-agent-character-queue-management.md)
- AI Agent Disk Cache Management Guidelines: [`docs/ai-agent-disk-cache-management.md`](docs/ai-agent-disk-cache-management.md)
- AI Agent Buffering Management Guidelines: [`docs/ai-agent-buffering-management.md`](docs/ai-agent-buffering-management.md)
- AI Agent System State & Update Guidelines: [`docs/ai-agent-system-state-management.md`](docs/ai-agent-system-state-management.md)
- AI Agent Binary Semaphores Management: [`docs/ai-agent-semaphores-management.md`](docs/ai-agent-semaphores-management.md)
- AI Agent Filesystem Management Guidelines: [`docs/ai-agent-filesystem-management.md`](docs/ai-agent-filesystem-management.md)
- AI Agent Backup & Recovery Guidelines: [`docs/ai-agent-backup-management.md`](docs/ai-agent-backup-management.md)
- AI Agent 4-Bit Operation Management Guidelines: [`docs/ai-agent-4bit-operation-management.md`](docs/ai-agent-4bit-operation-management.md)
- AI Agent Virtual Machine Management: [`docs/ai-agent-vm-management.md`](docs/ai-agent-vm-management.md)
- AI Agent Power & Thermal Management: [`docs/ai-agent-power-management.md`](docs/ai-agent-power-management.md)
- Sovereign Developer Guide: [`DEVELOPER_RULES.md`](DEVELOPER_RULES.md)
