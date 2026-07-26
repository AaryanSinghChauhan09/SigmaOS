# OSS Absorption: Rust Language & Cargo — Systems Programming Safety

> **Status**: 📋 Planned | **Source Project**: Mozilla/Rust Foundation | **Target Shard**: `SigmaOS Core Language Runtime`

---

## 1. Executive Summary

Rust is SigmaOS's primary implementation language. Its ownership and borrow-checker model eliminates entire classes of memory safety bugs at compile time — no use-after-free, no data races, no null pointer dereferences — without a garbage collector. Cargo provides a hermetic, reproducible build system.

SigmaOS leverages Rust's **ownership model**, **async runtime integration**, and **Cargo workspace** for a zero-unsafe-code policy in all userland shards.

---

## 2. Key Features Absorbed

### 2.1 Ownership-Based Memory Safety

All SigmaOS userland code is `#![forbid(unsafe_code)]` by default. Unsafe blocks in the kernel are explicitly audited and documented.

```rust
// sigma-pkg: safe package resolution — no unsafe, no GC
pub fn resolve_deps(manifest: &Manifest) -> Result<DepGraph, ResolveError> {
    let mut graph = DepGraph::new();
    for dep in &manifest.dependencies {
        let resolved = registry.fetch(dep)?;
        graph.add(resolved);
    }
    Ok(graph) // Ownership ensures graph is valid at return
}
```

### 2.2 Async Runtime (`sigma-async`)

SigmaOS implements a custom async executor (`sigma-async`) tailored to the microkernel's IPC model, with work-stealing task queues and no heap allocations in the hot path.

```rust
// sigma-async: zero-alloc IPC-aware async executor
#[sigma_async::main]
async fn main() {
    let response = sigma_ipc::call("S-FS", FsRequest::Open("/etc/sigma/config.toml")).await?;
    println!("File opened: {:?}", response);
}
```

### 2.3 Cargo Workspace for Reproducible Builds

The entire SigmaOS codebase is one Cargo workspace with pinned dependencies and `cargo-vendor` for offline builds. CI verifies that every build is byte-for-byte reproducible.

```bash
$ sigma build --check-reproducible
Σ [BUILD] Reproducibility check:
  Build A hash: blake3:a1b2c3d4...
  Build B hash: blake3:a1b2c3d4...  ✓ IDENTICAL
```

---

## 3. References & Standards

- Rust — `rust-lang.org` (MIT / Apache-2.0)
- Cargo — `doc.rust-lang.org/cargo`
