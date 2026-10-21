# Sovereign Architectural & Engineering Principles

This document catalogues the foundational engineering, architectural, and design principles enforced across SigmaOS kernel, userland, and documentation.

## 1. Architecture & Design Principles
- **Abstraction**: Isolating high-level abstractions from concrete hardware details via clean, zero-cost traits.
- **Architecture**: Clean layered architecture separating kernel space, VFS, system services, and userland.
- **Blackbox**: Subsystems expose minimal, opaque, well-specified interfaces without exposing internal memory layouts.
- **Booch Method**: Object and component modeling emphasizing structural isolation and message passing.
- **Bootloader Integration**: Multi-stage boot protocol support (Multiboot2, Limine, UEFI Handover).
- **Cache Conscious Design**: Cache-line aligned data structures (64-byte alignment) avoiding false sharing.
- **Clear Abstraction & Maximise Cohesion**: Every module addresses a singular domain; internal types operate harmoniously.
- **Command-Query Separation (CQS)**: Methods that change state do not return data, and queries return data without mutating state.
- **Component Isolation**: Isolated memory contexts and process bounds with zero accidental cross-talk.
- **Composition Over Inheritance**: Rust trait composition and struct embedding over deep polymorphic hierarchies.
- **Minimise Coupling**: Subsystems interact via strictly versioned interfaces or message queues, never direct memory access.
- **Deutsch Limit**: Ensuring no more than 50 visual or conceptual primitives are simultaneously in view.
- **Direct Device Control**: Bounded MMIO, port I/O abstractions, and ring buffers for low-latency hardware control.
- **Encapsulation & Information Hiding**: Strict privacy defaults (`pub(crate)`, private structs, opaque tokens).
- **Gall's Law**: A complex system that works is found to have evolved from a simple system that worked.
- **GRASP (General Responsibility Assignment Software Patterns)**: Information expert, creator, controller, low coupling, high cohesion.
- **KISS (Keep It Simple Stupid)**: Avoid overengineering; straight-line control flow where possible.
- **Layered Design**: HAL -> Core Kernel -> VFS/Networking -> System Daemons -> User Applications.
- **Least Astonishment**: API naming, errno definitions, and calling conventions align with established systems standards.
- **Least Privilege**: Sandboxed execution rings, pledge/unveil capabilities, and drop-privilege lifecycles.
- **Locality of Reference**: Contiguous array allocations, data-oriented memory structures, and page alignment.
- **Low-Level Control & Languages**: Built with `#![no_std]` Rust, Zig, assembly, and minimal C runtime shims.
- **Manual Memory Control & Discipline**: Zero uncontrolled runtime allocations, slab/buddy physical allocators, bounded pools.
- **Memory Safety**: Borrow checker guarantees, lifetime tracking, and zero-panic runtime paths.
- **Minimal Kernel Philosophy**: Microkernel / hybrid design keeping scheduling and IPC minimal and fast.
- **Modular Kernel Design**: Hot-pluggable driver units, dynamic module loaders, and isolated subsystem domains.
- **Open-Closed Principle (OCP)**: Extensible driver and filesystem registries open for extension, closed for modification.
- **Power of Ten**: Following NASA JPL coding rules (fixed upper loop bounds, no heap allocation after init, static asserts).
- **Separation of Concerns & Mechanism from Policy**: Mechanisms live in the kernel; policies are governed by userland daemons.
- **SOLID Principles**: Single Responsibility, Open/Closed, Liskov Substitution, Interface Segregation, Dependency Inversion.
- **Virtual Memory Abstraction**: 4-level/5-level paging tables, recursive mapping, copy-on-write, and ASLR.
- **Zero-Copy**: Direct DMA transfer, kernel ring buffers, splice/vmsplice zero-copy pipelines.
- **Zero-One-Infinity Rule**: An entity may only exist 0 times, 1 time, or infinite (arbitrary N) times.

## 2. Quality, Testing & Continuous Improvement
- **Arrange-Act-Assert (AAA)**: Standardized test formatting for deterministic verification.
- **Boy Scout Rule**: Always leave the codebase cleaner than you found it.
- **Deadlock Avoidance**: Lock ranking protocols, try-lock with timeouts, and hierarchical spinlock ordering.
- **Defensive & Offensive Programming**: Strict parameter assertion at trust boundaries; fail-fast internally on invariant violation.
- **Deterministic Testing**: Mockable clock, hardware emulators, and hermetic unit test runners.
- **Fail-Fast System**: Early validation of syscall arguments, invalid pointers, and capability bits.
- **FIRST**: Fast, Isolated, Repeatable, Self-validating, Timely unit tests.
- **Ninety-Ninety Rule**: Plan for edge cases, performance margins, and residual testing early.
- **Race-Free Design**: Synchronization primitives (`AtomicBool`, lock-free queues, RCU) guaranteeing race-freedom.
- **Refactoring & Rule of Three**: Defer premature abstractions until code patterns emerge in 3 distinct call sites.
- **Safe Concurrency**: Send/Sync guarantees, lock-free ring buffers, and barrier synchronization.
- **Secure Defaults**: Deny-by-default firewall, pledge promises, and restricted userland caps.
- **YAGNI (You Aren't Gonna Need It)**: Do not implement anticipatory abstractions without concrete requirements.
- **Zen of Python / Philosophy**: Explicit is better than implicit. Simple is better than complex. Readability counts.

---

## 3. Complete Principles Reference Table

The following table enumerates **every** engineering principle enforced across SigmaOS, with a brief description of its implementation.

| # | Principle | SigmaOS Implementation |
|---|---|---|
| 1 | **ABSTRACTION** | Hardware-specific code is hidden behind zero-cost traits and HAL boundaries; upper layers never touch raw MMIO addresses directly. |
| 2 | **ARCHITECTURE** | Clean `HAL → Core Kernel → VFS/Net → System Daemons → Userland` layering; no layer bypasses its neighbour's interface. |
| 3 | **ARRANGE-ACT-ASSERT** | All kernel unit tests and integration harnesses follow strict AAA layout with named setup, action, and assertion blocks. |
| 4 | **AVOID FRAGMENTATION** | Physical memory uses a buddy allocator with power-of-two block sizes; slab allocator prevents heap fragmentation for fixed-size objects. |
| 5 | **BLACKBOX** | Subsystems expose opaque handle types (`pub struct Fd(u32)`) so callers cannot inspect or manipulate internal state. |
| 6 | **BOOCH METHOD** | Object and component decomposition drives module boundaries; each component has a documented interface, behaviour, and set of collaborators. |
| 7 | **BOOTLOADER INTEGRATION** | Multi-stage boot via Limine (UEFI + BIOS), Multiboot2, and custom sigma-boot; handover protocol documented in `boot/limine/limine.conf`. |
| 8 | **BOY SCOUT RULE** | CI lint gates reject any PR that leaves dead code, unused imports, or unresolved `TODO`s older than one release cycle. |
| 9 | **CACHE CONSCIOUS DESIGN** | Hot data structures are 64-byte aligned; cold metadata lives in separate cache lines to avoid evicting frequently accessed fields. |
| 10 | **CLEAR ABSTRACTION** | Module names, type names, and function names express intent unambiguously — no abbreviations without a project-wide glossary entry. |
| 11 | **CODE REUSABILITY** | Shared logic lives in `src/klib` and `CraftLibs`; duplication is prohibited by the Rule of Three review gate. |
| 12 | **MAXIMISE COHESION** | Every module owns a single domain (scheduler, VFS, NIC driver); cross-domain calls happen through defined IPC channels only. |
| 13 | **COLLABORATION & SHARED RESPONSIBILITY** | RFC-driven development, open code review, and rotating maintainer ownership ensure no single point of failure in the team. |
| 14 | **COMMAND-QUERY SEPARATION** | Kernel APIs that mutate state return `Result<(), Error>`; APIs that query state return data and never produce side-effects. |
| 15 | **COMMUNITY-ORIENTED DEVELOPMENT** | Contributor handbook, do-ocracy model, and public RFC registry encourage external participation without gatekeeping. |
| 16 | **COMPONENT ISOLATION** | Each subsystem runs with its own memory bounds; a panic in a driver domain does not corrupt the scheduler or VFS state. |
| 17 | **COMPOSITION OVER INHERITANCE** | Rust trait composition and struct embedding replace deep class hierarchies; behaviour is mixed in via `impl Trait for Type`. |
| 18 | **CONTINUOUS IMPROVEMENT & LONG TERM SUSTAINABILITY** | Refactoring sprints, automated regression suites, and documented architectural decision records (ADRs) sustain code quality over time. |
| 19 | **MINIMISE COUPLING** | Subsystems communicate via strictly versioned message queues or syscall interfaces; direct struct-field access across crate boundaries is forbidden. |
| 20 | **CROSS COMPILATION** | Cargo targets `x86_64-unknown-none`, `aarch64-unknown-none`, and RISC-V bare-metal; a single `Makefile` builds all targets from any host. |
| 21 | **DEADLOCK AVOIDANCE** | Global lock-rank table enforces consistent acquisition order; try-lock with deadlines is mandatory in interrupt-context code. |
| 22 | **DEFENSIVE PROGRAMMING** | All public kernel entry points validate pointer alignment, length bounds, and capability tokens before touching user memory. |
| 23 | **DEMETER LAW** | A module calls only its own methods, methods of objects it creates, and methods of objects passed to it — no deep chaining. |
| 24 | **DESIGN PATTERN** | GoF and POSA patterns (Command, Observer, Strategy, Ring-Buffer Producer-Consumer) are applied contextually and documented in ADRs. |
| 25 | **DETERMINISTIC TESTING** | Tests inject a fake clock, a deterministic allocator, and mocked hardware so results are identical across every CI run. |
| 26 | **DEUTSCH LIMIT** | Display subsystem and compositor APIs expose no more than 50 distinct visual primitives per layer to avoid cognitive overload. |
| 27 | **DIRECT DEVICE CONTROL** | Drivers use bounded MMIO wrappers, port I/O macros, and DMA ring buffers for sub-microsecond hardware access paths. |
| 28 | **DISCOVERABILITY** | `sigsh help`, `man sigma-*`, and in-kernel `sysfs` nodes expose every tunable and command without needing external documentation. |
| 29 | **DON'T REPEAT YOURSELF** | Shared logic is extracted to crate-level helpers on the third occurrence; copy-paste is a CI-blocking lint violation. |
| 30 | **EFFICIENT SCHEDULING** | The hybrid BORE/EEVDF scheduler minimises context-switch overhead; real-time tasks get dedicated CPU sets via cgroups v2. |
| 31 | **ENCAPSULATION** | Internal state is `pub(crate)` or private by default; public APIs surface only stable, versioned types. |
| 32 | **FAIL-FAST SYSTEM** | Invalid syscall arguments, misaligned pointers, and out-of-range indices return `EINVAL` immediately without partial execution. |
| 33 | **FAST-ISOLATED-REPEATABLE-SELF VALIDATING-TIMELY** | All unit tests run in under 100 ms in hermetic sandboxes and are gated by CI before any merge. |
| 34 | **FREE AND OPEN SOURCE SOFTWARE** | SigmaOS is released under GPL-compatible and permissive SPDX licences; all tooling used to build it is FOSS. |
| 35 | **GALL'S LAW** | Complex subsystems evolved from minimal working prototypes; no "big bang" rewrites — growth is always incremental. |
| 36 | **GENERAL RESPONSIBILITY ASSIGNMENT SOFTWARE PATTERNS** | GRASP roles (Information Expert, Creator, Controller, Pure Fabrication) guide which module owns which responsibility. |
| 37 | **GOVERNANCE AND DO-CRACY** | Those who do the work make the decisions; formal RFC voting applies only to architectural changes affecting stability. |
| 38 | **IF IT AIN'T BROKE DON'T FIX IT** | Stable, battle-tested subsystems (e.g., buddy allocator) are not refactored without a concrete measured defect or performance regression. |
| 39 | **INFORMATION HIDING** | Implementation details (internal allocator pools, interrupt vector tables) are never exposed in public crate APIs. |
| 40 | **INHERITANCE** | Used sparingly via Rust's trait object dispatch; preferred only where runtime polymorphism is required (e.g., VFS `dyn Filesystem`). |
| 41 | **INSTRUMENTATION** | Every critical path emits structured tracing spans (`tracing::span!`) and hardware performance counter hooks for low-overhead profiling. |
| 42 | **INTERFACE SEGREGATION** | Large monolithic traits are split into focused, minimal trait definitions so callers depend only on the methods they actually use. |
| 43 | **INTERRUPT HANDLING DISCIPLINE** | ISRs do minimal work (enqueue event, clear status register, EOI); all heavy processing is deferred to workqueue threads. |
| 44 | **INVERSION OF CONTROL** | The kernel's driver framework calls into registered driver callbacks rather than drivers calling into the kernel directly. |
| 45 | **KEEP IT SIMPLE STUPID** | Straight-line control flow is preferred; complex state machines are replaced with explicit `enum` state types and `match`. |
| 46 | **LAYERED DESIGN** | HAL → Core Kernel → VFS/Networking → System Daemons → User Applications — each layer depends only on the layer below. |
| 47 | **LEAST ASTONISHMENT** | Syscall names, errno values, and flag bits mirror established POSIX conventions so developers face no surprises. |
| 48 | **LEAST PRIVILEGE** | Processes start with a minimal capability set and must explicitly request additional rights via pledge/unveil syscalls. |
| 49 | **LISKOV SUBSTITUTION** | Any concrete type implementing a kernel trait (e.g., `Block Device`) can replace any other without altering correctness. |
| 50 | **LOCALITY OF REFERENCE** | Frequently accessed data is packed into contiguous arrays; pointer chasing through heap-allocated linked lists is avoided. |
| 51 | **LOOSE COUPLING** | Inter-subsystem communication uses typed message channels; no subsystem holds a raw pointer to another subsystem's internals. |
| 52 | **LOW LEVEL CONTROL** | Assembly stubs, raw MMIO, and inline `asm!` blocks are used where necessary; abstraction cost is always zero or measured. |
| 53 | **LOW LEVEL LANGUAGES** | Core kernel written in `#![no_std]` Rust with assembly stubs; Zig and minimal C are used only for specific legacy ABI shims. |
| 54 | **MANUAL MEMORY CONTROL** | No implicit heap allocations after kernel init; all allocations go through the slab or buddy pool with explicit lifetimes. |
| 55 | **MEMORY DISCIPLINE** | Allocation budgets are enforced per-subsystem; exceeding the budget triggers a compile-time error or a runtime audit log entry. |
| 56 | **MEMORY SAFETY** | Rust borrow checker eliminates use-after-free and data races at compile time; unsafe blocks require a safety comment justification. |
| 57 | **MERITOCRACY AND INCLUSIVITY** | Contributor influence is proportional to quality of contribution; the CoC prohibits discrimination and ensures a welcoming environment. |
| 58 | **MINIMAL KERNEL PHILOSOPHY** | Scheduling, IPC, and memory management live in the kernel; everything else (filesystems, drivers, daemons) can run in isolated domains. |
| 59 | **MODULAR KERNEL DESIGN** | Drivers and filesystems are hot-pluggable kernel modules; a faulty module can be unloaded without rebooting. |
| 60 | **MODULARITY** | SigmaOS is composed of independently versioned crates; any subsystem can be swapped, tested, or benchmarked in isolation. |
| 61 | **NINETY-NINETY RULE** | Schedules account for the last 10% of edge cases explicitly; testing matrices cover boundary conditions and hardware quirks. |
| 62 | **OBJECT-ORIENTED PROGRAMMING** | OOP concepts (encapsulation, polymorphism, abstraction) are applied via Rust traits and enums rather than class hierarchies. |
| 63 | **OFFENSIVE PROGRAMMING** | Inside trust boundaries, assertions crash immediately on invariant violations to surface bugs early during development. |
| 64 | **OPEN-CLOSED** | Extension registries (filesystems, drivers, schedulers) are open for extension via trait impl; core registration code is closed to modification. |
| 65 | **OPEN STANDARD AND INTEROPERABILITY** | SigmaOS targets POSIX, ELF, DWARF, USB, PCI, and NVMe open standards to maximise hardware and software compatibility. |
| 66 | **OPTIMISE FOR DELETION** | Code is written to be easy to remove; feature flags, isolated crates, and minimal coupling make deprecated paths trivially deletable. |
| 67 | **PLUG-IN DRIVER MODEL** | Drivers register via a trait-based plugin API; the kernel discovers and loads them without needing source-level knowledge of the driver. |
| 68 | **POLYMORPHISM** | Dynamic dispatch (`dyn Trait`) is used for runtime-variable behaviour (filesystems, device drivers); static dispatch used everywhere else. |
| 69 | **PORTABILITY** | Architecture-specific code is gated by `#[cfg(target_arch = ...)]`; all HAL abstractions are implemented for x86-64, AArch64, and RISC-V. |
| 70 | **POWER OF TEN** | Follows NASA JPL rules: fixed upper bounds on loops, no heap after init, no function pointers except driver dispatch tables. |
| 71 | **PRACTICAL LICENSING AWARENESS** | All third-party dependencies are vetted for SPDX compatibility; `cargo-deny` enforces the allowed licence list in CI. |
| 72 | **PRISTINE SOURCES** | Vendored third-party sources are never modified; patches are maintained as separate diff files applied at build time. |
| 73 | **PROCESS ISOLATION** | Each user process runs in its own virtual address space with no shared writable mappings unless explicitly established via shared memory. |
| 74 | **RACE-FREE DESIGN** | Lock-free algorithms, `AtomicUsize` primitives, and RCU read-copy-update patterns eliminate data races in hot paths. |
| 75 | **RESPECT & ETHICAL PARTICIPATION** | Code of Conduct mandates respectful discourse; maintainers are empowered to enforce it swiftly and consistently. |
| 76 | **REFACTORING** | Structured refactoring sessions are scheduled each release cycle; changes are validated against the full test suite before merge. |
| 77 | **RULE OF THREE** | Abstractions are only introduced after identical logic appears in three independent sites; earlier extraction is considered premature. |
| 78 | **SAFE CONCURRENCY** | `Send` and `Sync` trait bounds enforce thread-safety at compile time; shared state requires an explicit synchronisation primitive. |
| 79 | **SECURE DEFAULTS** | All network ports are closed, all capabilities are dropped, and all file paths are unveil-restricted by default at process start. |
| 80 | **SEPARATION OF CONCERNS** | Display management, audio, networking, and storage are owned by separate modules with no cross-concern internal access. |
| 81 | **SEPARATION OF MECHANISM & POLICY** | The kernel provides mechanisms (IPC channels, scheduling slots, memory pages); policy (priority, quota, access rights) is set by userland. |
| 82 | **SINGLE RESPONSIBILITY** | Every struct, module, and crate has one reason to change; multi-responsibility code is split during the next refactoring sprint. |
| 83 | **TRANSPARENCY & OPEN EXCHANGE** | Architecture decisions, RFC outcomes, benchmark results, and security advisories are published openly in the wiki and mailing list. |
| 84 | **OPEN/CLOSED (SOLID-O)** | Reaffirmed: extension points in the driver framework and scheduler policy layer are open via trait impls; internals are sealed. |
| 85 | **LISKOV SUBSTITUTION (SOLID-L)** | All `dyn BlockDevice` and `dyn Filesystem` implementors are substitutable in every context without altering system correctness. |
| 86 | **INTERFACE SEGREGATION (SOLID-I)** | No module is forced to depend on traits it does not use; large traits are decomposed into focused, composable interfaces. |
| 87 | **DEPENDENCY INVERSION PRINCIPLE** | High-level scheduler policy depends on `dyn CpuAllocator` abstractions, not concrete hardware topology structs. |
| 88 | **STABLE APIs** | Public kernel ABI and user-facing crate APIs follow semantic versioning; breaking changes require a major-version bump and migration guide. |
| 89 | **THEIRS** | When merging upstream ecosystem changes (Arch, BSD, Omarchy), upstream content is preserved verbatim in parity modules with minimal local patches. |
| 90 | **UNIFORM ACCESS** | Fields and computed properties are accessed through the same getter API surface, hiding whether the value is stored or computed. |
| 91 | **USER FREEDOM AND CONTROL** | Users can replace the compositor, shell, package manager, and init system without kernel recompilation or vendor lock-in. |
| 92 | **VERSION CONTROL PRACTICES** | Conventional Commits, signed tags, branch protection rules, and linear history on `main` are enforced across all repositories. |
| 93 | **VIRTUAL MEMORY ABSTRACTION** | 4-level/5-level paging, recursive page-table mapping, copy-on-write forking, ASLR, and guard pages are provided transparently. |
| 94 | **WORSE IS BETTER** | A working, simple implementation ships first; complexity is added only when benchmarks prove the simpler version is insufficient. |
| 95 | **XTHEIRS** | In merge conflicts, the upstream (`--strategy-option=theirs`) is favoured for ecosystem parity modules to maintain fidelity with the source distro. |
| 96 | **YOU AREN'T GONNA NEED IT** | Features are not implemented speculatively; every addition requires a concrete use case in the current milestone. |
| 97 | **ZEN OF PYTHON** | Readability, explicitness, and simplicity guide API design even in low-level Rust; cryptic one-liners are rejected in code review. |
| 98 | **ZERO-COPY** | DMA scatter-gather lists, splice/vmsplice pipelines, and kernel ring buffers eliminate memory copies on the critical I/O path. |
| 99 | **ZERO-ONE-INFINITY** | Architectural limits allow exactly 0, 1, or an arbitrary number of instances — never a magic constant like 2 or 7. |
