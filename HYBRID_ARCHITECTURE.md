# SigmaOS Hybrid Architecture Specification

## Overview & Philosophy
To ensure SigmaOS remains maintainable, extensible, and high-performance, the system is engineered around a **Hybrid Model**:
- **Object-Oriented Programming (OOP)** principles govern how source code is designed, stored, and organized.
- **Procedural Programming** principles drive how the OS kernel executes at runtime for maximum CPU execution speed and zero-abstraction overhead.

---

## 🗂️ 1. OOP for Source Code Storage (Design & Organization)
1. **Encapsulation:** Kernel subsystems (memory manager, process scheduler, I/O bus, security policies) are encapsulated within distinct module and trait boundaries with clear, protected internal states and public interfaces.
2. **Inheritance & Traits:** Common base driver traits (`Driver`, `StorageDriver`, `NetworkDriver`, `GraphicsDriver`) define shared lifecycle and device behaviors, which concrete device adapters extend.
3. **Polymorphism:** Polymorphic traits allow dissimilar drivers, filesystems (Ext4, Btrfs, ZFS), and package formats (Deb, RPM, Arch, Flatpak) to present unified APIs to high-level subsystems.
4. **SOLID Principles:**
   - **Single Responsibility Principle (SRP):** Each struct/trait handles a single domain responsibility.
   - **Open/Closed Principle (OCP):** Core system modules are open for extension via traits and closed for direct code modification.
   - **Liskov Substitution Principle (LSP):** Concrete driver implementations can seamlessly replace abstract traits without breaking kernel execution.
   - **Interface Segregation Principle (ISP):** Userland and kernel APIs are cleanly segregated into dedicated interface contracts.
   - **Dependency Inversion Principle (DIP):** High-level kernel orchestration modules depend on trait abstractions rather than concrete driver implementations.
5. **Modular Repository Organization:** Source code is hierarchically structured across clean root directories (`src/kernel/`, `src/driver/`, `src/filesystem/`, `src/net/`, `src/security/`, `src/graphics/`).

---

## ⚙️ 2. Procedural Execution at Runtime
1. **Procedural Boot Sequence:** Execution flows strictly procedurally from BIOS/UEFI bootloader -> kernel initialization -> process scheduler setup -> userland init (`SigmaInit`).
2. **Procedural Scheduling & Interrupts:** Core scheduler context switching, APIC interrupt service routines (ISRs), and memory allocation page walks execute via direct, low-latency procedural routines.
3. **Procedural Driver Calls:** While drivers are authored using OOP abstractions, driver execution in hotpaths utilizes flat procedural function pointer dispatch tables (`ProceduralDriverDispatchTable`) to bypass dynamic vtable lookups and runtime overhead.
4. **Procedural System Calls:** Syscall dispatchers map directly to fast array lookup tables indexed by syscall vector numbers.
5. **Procedural Error Handling:** Uses lightweight integer/enum status codes (`DriverError`, `SyscallResult`) rather than stack-unwinding exception hierarchies.

---

## 🔄 3. Layered Integration Model
```
+-------------------------------------------------------------+
| 1. Maintenance & Design Layer (OOP Abstractions & Traits)    |
+-------------------------------------------------------------+
                              | (Compiled to direct function pointers)
                              v
+-------------------------------------------------------------+
| 2. Execution & Runtime Layer (Procedural Syscall & Interrupt) |
+-------------------------------------------------------------+
                              |
                              v
+-------------------------------------------------------------+
| 3. Mixed Driver Layer (OOP Definition -> Procedural Hooks)  |
+-------------------------------------------------------------+
```

---

## 📅 4. 5-Phase Implementation Roadmap
- **Phase 1 (0–3 Months):** Refactor kernel subsystems into OOP classes and trait contracts.
- **Phase 2 (3–6 Months):** Build procedural fast syscall dispatcher tables and CPU scheduler loops.
- **Phase 3 (6–9 Months):** Convert device drivers into OOP wrappers exporting procedural execution function tables.
- **Phase 4 (9–12 Months):** Optimize runtime with zero-allocation procedural memory allocators and interrupt handlers.
- **Phase 5 (12–18 Months):** Compile comprehensive documentation and contributor guidelines for the hybrid architecture model.
