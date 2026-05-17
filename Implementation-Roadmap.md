# SigmaOS Implementation Roadmap

This roadmap tracks the development of core features required for the "Zenith" and "Horizon" microkernel releases. 

## 1. Bootloader (Bare-Metal Start)

***Status**: Mentioned, not fully implemented.* **Implementation Plan**: Write assembly/C code to initialize CPU, set up stack, establish the initial GDT/IDT, and load the microkernel into memory.

- **Documentation**: `Bootloader-Design.md` (Wiki)

## 2. Kernel Modules Architecture

***Status**: Mentioned in repo goals.* **Implementation Plan**: Create a modular kernel lattice allowing hot-swapping of loadable modules (e.g., scheduler, MMU, cryptographic attestation) without rebooting.

- **Documentation**: `Kernel-Architecture.md` (Wiki)

## 3. Memory Management (S-MM)

***Status**: Initial definitions exist, paging incomplete.* **Implementation Plan**: Implement industrial-grade paging, secure identity mapping, segmentation, and an O(1) memory slab allocator.

- **Documentation**: `Sovereign-Memory-Management.md` (Wiki)

## 4. Process Scheduling (S-SCHED)

***Status**: Mentioned, missing logic.* **Implementation Plan**: Add multi-priority preemptive scheduling with round-robin fallback. Implement thread isolation for shard workers.

- **Documentation**: `Scheduling-Algorithms.md` (Wiki)

## 5. Device Drivers

***Status**: Referenced, stubs exist.* **Implementation Plan**: Write hardware-direct drivers for basic I/O including keyboard/HID, VESA display, and NVMe/SATA storage.

- **Documentation**: `Driver-Development.md` (Wiki)

## 6. Security Model (S-ARMOR)

***Status**: Mentioned, not implemented.* **Implementation Plan**: Define hardware privilege levels (Ring 0 vs Ring 3), namespace isolation, and secure post-quantum cryptographic syscalls.

- **Documentation**: `Security-Model.md` (Wiki)

## 7. Networking Stack (S-NET)

***Status**: Missing.* **Implementation Plan**: Implement a minimal, secure TCP/IP interconnect layer and local message-passing interface (IPC).

- **Documentation**: `Networking.md` (Wiki)

## 8. User-Space Tooling

***Status**: Not present.* **Implementation Plan**: Build a sovereign shell environment, core CLI utilities, and a non-derivative package manager.

- **Documentation**: `User-Tools.md` (Wiki)
