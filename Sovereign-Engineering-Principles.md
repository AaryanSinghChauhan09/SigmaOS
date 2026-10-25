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
