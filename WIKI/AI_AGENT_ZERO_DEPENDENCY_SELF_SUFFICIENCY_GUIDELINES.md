# SigmaOS AI Agent Zero-Dependency Self-Sufficiency Guidelines

## 1. Executive Summary & Overview

A core pillar of the SigmaOS philosophy is **Absolute Sovereign Self-Sufficiency**: eliminating reliance on pre-defined third-party libraries, external packages, foreign crate dependencies, and non-sovereign runtime functions. Every critical operating system subsystem—from kernel memory allocation and data structures (`klib`) to cryptographic primitives, JSON/TOML parsers, network stacks, and package adapters—is built natively in pure, safe Rust without external C or Rust crate dependencies (`[dependencies]` in `Cargo.toml` contains 0 external crates).

This document establishes the official guidelines and architectural standards for AI agents maintaining, extending, and enforcing Zero-Dependency Self-Sufficiency in SigmaOS.

---

## 2. Zero-Dependency Subsystem Replacement Architecture (`klib`)

Rather than importing external packages, AI agents utilize or extend native `klib` primitives in `src/klib/`:

| Subsystem Domain | Replaced External Package / Crate | Native `klib` / SigmaOS Implementation Module | Core Features & Primitives |
| :--- | :--- | :--- | :--- |
| **Dynamic Vectors & Strings** | `alloc::vec::Vec`, `std::string::String` | `src/klib/vec.rs`, `src/klib/string.rs` | Zero-alloc growable vectors, non-alloc stack string formatters |
| **Hash Maps & B-Trees** | `std::collections::HashMap`, `BTreeMap` | `src/klib/hashmap.rs`, `src/klib/btreemap.rs` | Custom open-addressing FNV-1a HashMaps, CoW B-Trees |
| **JSON & TOML Parsers** | `serde_json`, `toml` | `src/klib/json.rs`, `src/klib/toml.rs` | `SovereignJsonParser`, zero-copy string slice config parsers |
| **UUID Generation** | `uuid` crate | `src/klib/uuid.rs` | Native UUID v4 generator using CSPRNG entropy (`src/security/rng.rs`) |
| **Random Number Generators** | `rand` crate | `src/klib/rng.rs`, `src/security/rng.rs` | Cryptographically Secure Pseudo-Random Number Generator (CSPRNG) |
| **Base64 Encoding** | `base64` crate | `src/klib/base64.rs` | Native SIMD-accelerated Base64 encoder and decoder |
| **Merkle Tree Hashes** | `merkletree` crate | `src/klib/merkle.rs` | Cryptographic SHA256 Merkle accumulator for audit trail ledgers |
| **System Memory Allocator** | `dlmalloc`, `jemalloc` | `src/klib/custom_allocator.rs`, `src/klib/slab.rs` | Custom buddy allocator, intrusive slab allocator, CMA reservations |

---

## 3. Rules for AI Agents Enforcing Zero-Dependency Self-Sufficiency

AI agents writing or refactoring system code must enforce the five rules of self-sufficiency:

### Rule 1: Zero External Crates Policy (`Cargo.toml`)
- Never add external dependencies under `[dependencies]` in `Cargo.toml`.
- All required data structures, algorithms, or utility routines must be implemented natively within `src/klib/` or `src/`.

### Rule 2: Stack-Allocated Formatting over Heap Allocations
- In performance-critical and Ring-0 kernel routines, avoid heap allocations (`String::from`) for simple numerical or status formatting.
- Use `ZeroDependencyPrimitiveHub::format_u64_stack` or static byte buffers (`[u8; N]`).

### Rule 3: Zero-Copy String Parsing (`&str` Slices)
- Configuration parsers (`SovereignJsonParser`, `toml.rs`, `config_parser.rs`) must parse input buffer slices directly (`&str` / `&[u8]`) without allocating intermediate string objects.

### Rule 4: Custom Non-Cryptographic Hashing (`FNV-1a`)
- In-memory hashtables (`klib::HashMap`, `klib::HashSet`) utilize native 64-bit FNV-1a non-cryptographic hashing (`ZeroDependencyPrimitiveHub::fnv1a_hash_64`) with dynamic seed multipliers to mitigate hash-collision DOS attacks.

### Rule 5: Pure-Rust Device Drivers & Hardware Shims
- Replace external C/C++ hardware vendor SDKs with pure Rust drivers (`src/drivers/modern_nvme.rs`, `src/drivers/intel_e1000.rs`, `src/drivers/modern_usb.rs`, `src/drivers/modern_wifi.rs`).

---

## 4. Verification & Self-Sufficiency Audit Protocol

AI agents modifying core libraries or adding new subsystem components must execute verification:

1. **Zero-Dependency Check**:
   - Verify `Cargo.toml` remains empty under `[dependencies]`.
   - Run `is_zero_dependency_build()` checks in `src/klib/mod.rs`.
2. **Native Test Suite**:
   - Execute `./run_sigma_tests.sh` to confirm all 13 standalone test runners and inspection matrices pass without external package linkages.

---

*Approved by the SigmaOS Architecture & Zero-Dependency Steering Committee.*
