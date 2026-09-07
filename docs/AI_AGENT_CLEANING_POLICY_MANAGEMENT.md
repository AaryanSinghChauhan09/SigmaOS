# AI Agent Guidelines for SigmaOS System Cleaning & Garbage Collection Policy Management

This document provides guidelines, architectural specifications, and verification protocols for AI agents developing, inspecting, or extending **SigmaOS Cleaning Policies, Store Garbage Collection, and Package Cache Pruning Engines**.

---

## 1. System Architecture & Cleaning Engines Layout

SigmaOS manages system cleaning, store garbage collection, and cache pruning across three primary subsystems:

1. **Nix/Guix Content-Addressed Store GC & Profile Engine (`src/package/bsd_linux_package_innovations.rs`)**
   - **`NixGuixCasGcProfileEngine`:** Manages immutable store path garbage collection, GC roots scanning, generation profiles cleanup, and Content-Addressed Storage (CAS) bloat reclamation.

2. **Nix Pure Store GC Engine (`src/sigpkg/universal_oop_system.rs`)**
   - **`NixStoreGcEngine`:** Performs graph reachability traversals starting from active user profile GC roots to identify and safely delete dead store paths without breaking active dependencies.

3. **Pacman Cache Cleaner (`src/sigpkg/arch_pacman_engine.rs`)**
   - **`PacmanCacheCleaner`:** Prunes cached package tarballs (`.pkg.tar.zst`, `.deb`, `.rpm`) based on retention policies (`prune_cache(keep_count)`), retaining specified candidate versions per package (e.g. `paccache -r` parity).

---

## 2. Cleaning Policy Mechanics & Code Patterns

AI agents modifying garbage collection or cleaning policies must follow these patterns:

### Content-Addressed Store Garbage Collection (`NixStoreGcEngine`)
Dead store paths are collected by scanning GC root symlinks (`/nix/var/nix/gcroots/`):
- Any path not reachable from active GC roots is marked as dead.
- Reclaims disk space atomically while enforcing store immutability invariants.

```rust
use sigma::sigpkg::universal_oop_system::NixStoreGcEngine;

let mut gc = NixStoreGcEngine::new();
gc.add_gc_root("/sovereign/store/app-v1".to_string());

// Perform garbage collection pass
let freed_bytes = gc.collect_garbage();
```

### Package Cache Retention & Pruning (`PacmanCacheCleaner`)
Manages local package tarball retention rules:
- **Candidate Retain Count:** `prune_cache(keep_count)` keeps `keep_count` recent package builds (default = 3) and purges older build artifacts.

```rust
use sigma::sigpkg::arch_pacman_engine::PacmanCacheCleaner;

let mut cleaner = PacmanCacheCleaner::new(vec![
    "nginx-1.25.1.pkg.tar.zst".to_string(),
    "nginx-1.25.0.pkg.tar.zst".to_string(),
    "nginx-1.24.0.pkg.tar.zst".to_string(),
]);

// Keep 2 most recent candidate builds and purge older versions
let purged = cleaner.prune_cache(2);
assert_eq!(purged, vec!["nginx-1.24.0.pkg.tar.zst"]);
```

---

## 3. Testing & Verification Protocol for AI Agents

When making changes to garbage collection or cache cleaning policies, AI agents must execute the following validation steps:

### 1. Standalone Module Test Execution
Run standalone rustc test suites for package innovations, universal OOP system, and pacman cache cleaner:

```bash
rustc --test --edition=2021 src/package/bsd_linux_package_innovations.rs -o build/test_innovations && ./build/test_innovations
rustc --test --edition=2021 src/sigpkg/universal_oop_system.rs -o build/test_oop && ./build/test_oop
rustc --test --edition=2021 src/sigpkg/arch_pacman_engine.rs -o build/test_pacman && ./build/test_pacman
```

### 2. Full System Integration & Inspection Suite
Run the master test script to validate all C++ test runners, inspection test binaries, Python test suites, and core cleanup subsystems:

```bash
./run_sigma_tests.sh
```

---

## 4. Coding Standards & Cleaning Directives

- **GC Root Integrity:** Never delete a store path or package cache entry without verifying that it is unreferenced by active profile symlinks.
- **Atomic Space Reclamation:** Always report reclaimed disk space metrics accurately to system policy monitors.
- **Verification Rule:** Always confirm file creation/edits with `read_file` before completing steps.
