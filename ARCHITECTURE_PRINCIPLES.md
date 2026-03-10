# SigmaOS Sovereign Architecture Principles

Strict adherence to the **Native vs. Userland split** for performance, security, and developer agility.

## 1. The Native Core (Ring 0 Simulation)

**Languages:** Assembly (x86_64), C, C++, Rust.  
**Location:** `/kernel` and `/bootloader`.

- **Bootloader:** Low-level entry points (`boot.asm`, `long_mode.asm`).
- **Memory Management:** Slab allocators and MMU logic in C (`slab_allocator.c`, `mmu_core.c`).
- **Security:** Vanguard Cryptography implemented in Rust (`vanguard_crypto.rs`).
- **Drivers:** Native NIC and PCI scanning logic.

## 2. The Standard Library

**Languages:** C.  
**Location:** `/libc`.

- Provides basic `string.h`, `stdlib.h`, and `stdio.h` implementations for the native kernel.

## 3. The Sovereign Userland (Ring 3)

**Languages:** Python (System API), HTML5/CSS3/JavaScript (GUI).  
**Location:** `/userland`.

- **System API (`/userland/system_api`):** Python orchestration layer. Handles high-level logic and interfaces with the Native Core via the `FFI_Bridge`.
- **Desktop GUI (`/userland/desktop_gui`):** High-fidelity Web interface rendered via Chromium/Blink engines.
- **AI Apps (`/userland/apps`):** High-level multidisciplinary tools (Agentic AI, Legal Pros, Data Matrix).

## 4. The Bridge Layer (FFI)

Uses `ctypes` (Python) to call native binary functions in `/kernel`.

- **Principle:** Python "requests", C/Rust "executes".

---
*This architecture ensures SigmaOS remains the fastest, most secure, and most customizable OS on the market.*
