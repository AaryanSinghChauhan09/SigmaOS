# 🛠️ SIGMAOS CRITICAL COMPILATION BLOCKERS RESOLUTION SPECIFICATION

> **Target Repository:** [https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
> **Document Version:** 1.0.0
> **Status:** Active Engineering Resolution Specification

---

## 📌 OVERVIEW

This specification outlines the resolution strategies for the three Critical Compilation Blockers identified in **SigmaOS**:

1. **Issue #1: Duplicate Structure Definitions (E0428)**
2. **Issue #2: Macro Expansion Collisions (Syscall / ioctl enums)**
3. **Issue #3: `#![no_std]` Kernel vs `std` Userland Violations**

---

## 🛠️ RESOLUTION STRATEGIES & ALGORITHMS

### 1. Issue #1: Duplicate Structure Definitions (E0428)
* **Problem:** Multiple modules define identical types (e.g. `FiftyPercentRuleEngine` in `src/access/mod.rs` and `src/security/access_control.rs`).
* **Resolution Algorithm:**
  1. Identify collisions via `cargo check 2>&1 | grep "error[E0428]"`.
  2. Consolidate into canonical primary module (`src/access/mod.rs`).
  3. Re-export in consumer modules (`pub use crate::access::FiftyPercentRuleEngine`).

### 2. Issue #2: Macro Expansion Collisions
* **Problem:** Syscall and ioctl macro generators create conflicting enum definitions when Linux/BSD modes are enabled.
* **Resolution Algorithm:**
  1. Single unified `SyscallNumber` enum in `src/syscall/mod.rs` with `#[cfg(feature = "linux-abi")]` and `#[cfg(feature = "bsd-abi")]` conditional compilation attributes.
  2. Unified dispatch function matching numeric syscall IDs to handler functions.

### 3. Issue #3: `#![no_std]` Kernel Boundary Enforcer
* **Problem:** Kernel modules importing `std::*` instead of `alloc::*` and `core::*`.
* **Resolution Algorithm:**
  1. Replace `std::vec::Vec` -> `alloc::vec::Vec`.
  2. Replace `std::string::String` -> `alloc::string::String`.
  3. Replace `std::collections::HashMap` -> `alloc::collections::BTreeMap`.
  4. Enforce `#![no_std]` guard on all core kernel crates.

---

*End of Specification.*
