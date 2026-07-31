# 📦 SigmaOS Declarative Package Manager Development Plan

This document details the architectural design and implementation plan for the **SigmaOS Declarative Package Manager**, taking inspiration from the modular package paradigms of **NixOS** (functional, content-addressed declarations) and **Void Linux** (fast, zero-dependency `xbps` C/C++ style execution speeds).

---

## 🗺️ Architectural Inspiration
*   **NixOS (Nixpkgs):** Fully functional, declarative environment configurations where system profiles are built atomically as immutable symlink graphs.
*   **Void Linux (XBPS):** High performance, binary packages metadata constraint resolution written strictly without heavy dynamic runtimes.

---

## 🏗️ OOP Design & Content-Addressed Stores

SigmaOS implements package management utilizing a transactional database and a DPLL SAT dependency resolution engine:

```text
  [Package Manager Request]
             |
             v
   +-------------------+       +-----------------------+
   |  DPLL SAT Solver  | <---> | ContentAddressedStore |
   | (Dependency-graph)|       |  (Deduped /store/ sha)|
   +-------------------+       +-----------------------+
             |
             v
   +-------------------+
   | Transaction Engine|
   | (Atomic symlinks) |
   +-------------------+
```

### Package State Hierarchy:
```text
  State::Available ➡️ State::Resolving ➡️ State::Downloading ➡️ State::Installing ➡️ State::Activated
```

### Polymorphic Dependency Interface:
```rust
pub trait DependencyResolver {
    fn resolve_dependencies(&self, root_package: &PackageRecipe) -> Result<Vec<PackageRecipe>, ResolveError>;
    fn detect_overlap_conflicts(&self, plan: &[PackageRecipe]) -> Result<(), ConflictError>;
}
```

---

## 🛠️ Multi-Language Architecture (Rust, Zig, Nim)

### ⚡ Rust: SHA-256 Content-Address Storage (CAS) Builder
```rust
use std::collections::HashMap;

pub struct ContentAddressedStore {
    // Map from SHA-256 hash to local storage location
    store_registry: HashMap<String, String>,
}

impl ContentAddressedStore {
    pub fn new() -> Self {
        Self { store_registry: HashMap::new() }
    }

    pub fn insert_package(&mut self, hash: String, store_path: String) -> Result<(), &'static str> {
        if self.store_registry.contains_key(&hash) {
            return Err("Package hash collision or duplicate entry!");
        }
        self.store_registry.insert(hash, store_path);
        Ok(())
    }
}
```

### ⚡ Zig: XBPS-Style Direct Metadata Reader
```zig
const std = @import("std");

pub const PackageMetadata = struct {
    name: []const u8,
    version: []const u8,
    sha256: [32]u8,
};

pub fn parseMetadata(allocator: std::mem.Allocator, bytes: []const u8) !PackageMetadata {
    var stream = std::json::TokenStream.init(bytes);
    const parsed = try std::json::parse(PackageMetadata, &stream, .{ .allocator = allocator });
    return parsed;
}
```

### ⚡ Nim: Zero-Dependency Symlink Appender
```nim
import os

proc linkPackageBin*(sourcePath, targetLink: string): bool {.exportc, cdecl.} =
  try:
    createSymlink(sourcePath, targetLink)
    result = true
  except OSError:
    result = false
```

---

## 📈 Quality Assurance & Solver Tests

1.  **Atomic Rollback Test:** Verify that interrupting package installation at any step successfully rolls back system symlink profile maps.
2.  **SAT Graph Audit:** Ensure circular or conflicting dependencies are identified immediately before transaction execution.
