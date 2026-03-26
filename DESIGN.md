# SigmaOS Architecture & Design Manifesto
===================================================
> **Version:** Sovereign Ring-0 Architecture (v21.5)
> **Goal:** 100% Third-party and High-Level Library Erasure. Total Custom OS Subsystems via OOP natively written in Assembly, Machine Language, C, Rust and C++.

---

## The Low-Level Mandate
SigmaOS strictly prohibits the use of external packages (like `pip`, `npm`, `cargo`) or arbitrary language dependency wrappers (like `#include <iostream>`, `<stdlib.h>`, `glibc`, `msvcrt.dll`, etc.).
We believe an Operating System acts as its own execution layer; it shouldn't be subservient to the high-level language wrappers compiled above it. 

### Why No `<string>` or `json`?
When a high-level library evaluates rules or imports nested strings, it relies on bloated abstraction algorithms. We bypass those and speak directly to CPU pipelines:
1. **Automation & Personalisation**: Hand-rolled OOP (Object-Oriented Programming) classes running in C++ (`AutomationSubsystem`) and Rust (`#![no_std] SigmaAutomation`) act as the logic operators instead of high-level equivalents.
2. **Absorbing Linux Distros**: Our C++ `LinuxAbsorber.hpp` uses polymorphism (`AbstractDistroAbsorber`) mapping pure system memory streams to parse Alpine's APK headers or Arch Linux's Pacman binaries natively. No external `libapk`.

---

## Component Separation: The `native_core` Subsystem
Located exclusively at `sigma_core/native_core/`, the entire python-less core executes our operations. 
*   **`SigmaKernel.hpp` & `main.cpp`**: Boot system using `.asm` and pure C++ bypasses. Our internal OOP OS kernel. Includes custom `MemoryAllocator` and custom `String` allocation avoiding libc's `malloc()`.
*   **`sys_fast_ring.asm`**: Lightning-fast unbuffered Machine language instructions managing fast-context syscall integration natively.
*   **`SigmaAutomation.rs`**: Built using `#![no_std]` Rust guaranteeing memory safety without overhead. Provides natively hooked features like `DistroPackage`. 

## Our OOP Commitment
While utilizing raw byte streams, Memory Alignments (xmm 128-bit operations), and System Calls, SigmaOS maintains aggressive Object-Oriented layouts. Class encapsulation manages resources automatically (`MemoryAllocator` overrides global `new` Operator), separating memory allocation logic from feature deployment.

This ensures **SigmaOS is Automation, Customization, and Personalization readied.** Every component scales infinitely without external interference.
