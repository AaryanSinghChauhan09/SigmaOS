# 🏗️ SIGMAOS SOFTWARE ENGINEERING PARADIGMS & CORE OS FUNDAMENTALS MASTER SPECIFICATION

> **Target Repository:** [https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
> **Document Version:** 1.0.0
> **Status:** Active Engineering Guidelines & Subsystem Architectural Specification

---

## 🏛️ PART 1: OBJECT-ORIENTED PROGRAMMING (OOP) IN RUST

SigmaOS applies modern Object-Oriented Principles through safe, zero-cost Rust abstractions:

1. **Objects & Structs:** Concrete memory layouts representing OS resources (`TaskControlBlock`, `VirtualMemoryArea`, `FileDescriptor`).
2. **Instances & State:** Thread-safe state instances instantiated on heap or stack.
3. **Encapsulation:** Private fields accessed strictly through public domain methods (`pub fn`).
4. **Abstraction:** Trait interfaces hiding hardware-specific details (`SchedulerPolicy`, `BlockDevice`, `NetworkStack`).
5. **Inheritance vs Composition:** Preferred **Composition over Inheritance** using struct embedding and trait delegates (`has-a` rather than `is-a`).
6. **Polymorphism:** Dynamic (`Box<dyn SchedulerPolicy>`) and static (generics `<T: BlockDevice>`) trait dispatch.

---

## 📐 PART 2: SOLID DESIGN PRINCIPLES IN KERNEL DEVELOPMENT

1. **S - Single Responsibility Principle (SRP):** Each module handles one domain (e.g., `src/memory/` handles physical pages, `src/net/` handles sockets).
2. **O - Open/Closed Principle (OCP):** Subsystems are extensible via trait implementation without modifying core dispatchers.
3. **L - Liskov Substitution Principle (LSP):** Pluggable schedulers or filesystems can be swapped interchangeably without breaking kernel invariants.
4. **I - Interface Segregation Principle (ISP):** Fine-grained traits (`Readable`, `Writable`, `Seekable`) instead of monolithic fat interfaces.
5. **D - Dependency Inversion Principle (DIP):** High-level OS subsystems depend on abstract traits rather than low-level MMIO driver implementations.

---

## ⚡ PART 3: SOFTWARE ENGINEERING PRAGMATISM & CLEAN CODE

* **DRY (Don't Repeat Yourself):** Reusable kernel primitives and utility macros.
* **KISS (Keep It Simple, Stupid):** Straightforward lock-free algorithms over complex nested state machines.
* **YAGNI (You Aren't Gonna Need It):** Lean data structures avoiding premature abstractions.
* **Separation of Concerns:** Clear boundaries between Userland, Syscall API, VFS, Microkernel Core, and Hardware Drivers.
* **Design by Contract (DbC):** Strict enforcement of **Preconditions**, **Postconditions**, and **Invariants** on system state transitions.
* **Document Your Code & Refactor Regularly:** Thorough inline rustdoc comments for every kernel type and module.

---

## 💻 PART 4: OPERATING SYSTEM CORE DOMAINS

1. **Process Management:** Task state transitions, EEVDF/CFS/RT scheduling, POSIX signals, thread control blocks (TCBs).
2. **Memory Management:** Page table translation, buddy allocator, slab caches, MGLRU page eviction, guard pages.
3. **File Management:** Virtual Filesystem (VFS) layer, inode management, Btrfs/ZFS snapshots, buffer cache.
4. **System Security:** Capability-based access control, OpenBSD Pledge/Unveil, constant-time crypto, Dilithium-5 PQC.
5. **OS Structure:** Microkernel-inspired fault isolation, modular driver buses, eBPF CO-RE safety filters.
6. **Concurrency & Deadlocks:** Lock-free atomic data structures, Lockdep dependency graph deadlock detector, Coffman condition checkers.

---

*End of Master Specification.*
