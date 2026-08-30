# 🧩 SigmaOS Zero-Dependency & Minimal Predefined Libraries Blueprint

This document details the architectural strategy and OOP design specifications to systematically eliminate runtime dependencies on standard/predefined libraries (`std` or external non-verified crates) inside **SigmaOS**, ensuring a self-contained, high-security microkernel environment.

***

## 🗺️ 1. Paradigm Vision: Absolute Self-Containment

Traditional microkernels and operating systems rely heavily on external standard libraries or standard container crates (such as `std::collections` or dynamic allocator primitives). This introduces several critical limitations:

*   **Bloated Binary Size**: Standard library map structures carry massive generic overhead.
*   **Unpredictable Memory Behaviors**: Dynamic `HashMap` structures can fail unpredictably or invoke heap compaction lock contention.
*   **Security Vulnerability Surface**: Incorporating third-party packages exposes the operating system to external supply-chain attacks.

**SigmaOS** resolves these problems by enforcing a **Sovereign, Zero-Dependency `#![no_std]` Architecture**:

```text
  +---------------------------------------------------------------------------------+
  |                               SigmaOS Kernel Core                               |
  |                                                                                 |
  |   +-------------------+   +--------------------+   +------------------------+   |
  |   |   Sovereign Vec   |   |   StaticHashMap    |   |    Core Alloc Shim     |   |
  |   | (Custom Drop/Push)|   | (Association Grid) |   | (Non-blocking Layout)  |   |
  |   +-------------------+   +--------------------+   +------------------------+   |
  +---------------------------------------------------------------------------------+
```

Every module must rely exclusively on `core` language primitives. Common structures are replaced with lightweight, allocation-free custom implementations.

***

## 🏗️ 2. Core Self-Contained Custom Data Structures

### 2.1 Sovereign Allocation Vector (`Vec<T>`)

*   **Mission**: Replaces `std::vec::Vec` without requiring a standard allocator on bare-metal.
*   **OOP Design Pattern**: Implements standard array behaviors through `Deref` and `DerefMut` targeting a raw slice.
*   **Hardened Safety**: Fully implements the `Drop` trait to clean up allocated memory frames and contains explicit bounds and null-pointer checking inside `push` to prevent out-of-bounds writing.

### 2.2 Micro Association Grid (`StaticHashMap<K, V>`)

*   **Mission**: Replaces `std::collections::HashMap` for lightweight configuration pairs.
*   **OOP Design Pattern**: Implements key-value pairs inside a fixed-size array segment using direct index mapping and hashing multipliers.
*   **Benefit**: Prevents heap allocation during lookups.

### 2.3 Non-Blocking Allocator Shims (`alloc` & `free`)

*   **Mission**: Encapsulates dynamic memory mapping blocks.
*   **Mechanism**: Uses target-specific attributes (`#[cfg(target_os = "none")]`) to route allocations to physical buddy page tables, while providing safe `std_alloc` layouts during hosted unit tests.
