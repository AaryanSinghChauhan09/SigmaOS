# AI Agent Guidelines: Component Development in SigmaOS Inspired by Linux & BSD

## Overview & Philosophy

SigmaOS is a sovereign, zero-dependency, safe Rust (`#![no_std]`) operating system designed to absorb innovations from Linux distributions, BSD variants, microkernels, and cloud ecosystems.

As an **AI Agent developing, expanding, or maintaining components in SigmaOS**, your mission is to transform conceptual OS paradigms into high-performance, `#![no_std]` compliant, zero-external-dependency Rust implementations.

This guide provides architectural inspiration from Linux and BSD distributions, standard engineering rules, component design patterns, and a step-by-step development protocol for AI agents.

---

## 1. Distro Inspirations for Component Architecture

When adding or expanding components in SigmaOS, take inspiration from these landmark Linux and BSD innovations:

### 1.1 Linux Kernel & Ecosystem Inspirations
* **cgroups v2 & Pressure Stall Information (PSI)**: Design resource controllers with memory limits (`memory.max`, `memory.high`) and memory pressure stall tracking (`src/open_source_os_gap_closure.rs`).
* **Kernel Samepage Merging (KSM)**: Implement page frame hash scanning and deduplication algorithms for efficient copy-on-write RAM utilization.
* **OverlayFS Union Mounts**: Support multi-tier file path resolution combining lower read-only layers with upper writable layers and whiteout node generation.
* **eBPF & XDP Fast Path**: Build zero-copy packet filtering and socket message redirection engines (`EbpfSockmapRedirectEngine`).
* **io_uring Asynchronous I/O**: Implement submission (`SQ`) and completion (`CQ`) ring buffers supporting zero-syscall batch operations.
* **Arch Linux ALPM & pacman Hooks**: Support dynamic pre/post transaction triggers (`PacmanAurHookPatchEngine`).
* **Gentoo Portage USE Flags & Eclass**: Build conditional dependency solvers and slot/subslot package atom resolvers (`GentooEclassSlotEngine`).
* **Void Linux xbps & runit**: Combine atomic transaction journals with 3-stage service supervision (`VoidXbpsTransactionJournalEngine`, `RunitSupervisor`).

### 1.2 BSD Subsystem & Security Inspirations
* **FreeBSD VNET Isolation**: Provide independent, virtualized kernel network stack instances (`FreeBsdVnetEngine`) per container or jail.
* **FreeBSD Capsicum Capabilities**: Enforce file descriptor capability rights masks (`CapRights`) restricting global VFS namespace access.
* **FreeBSD GEOM Storage Framework**: Implement modular block storage transformations (`g_part`, `g_mirror`, `g_stripe`, `g_eli` encryption).
* **OpenBSD Pledge & Unveil**: Implement process capability drops (`pledge`) and VFS path restrictions (`unveil`) evaluated at syscall gates.
* **OpenBSD PF & CARP**: Combine stateful packet tracking with ALTQ QoS queues and CARP virtual router master/backup failover state sync (`OpenBsdPfCarpStateEngine`).
* **DragonFly BSD HAMMER2**: Design CoW storage engines with Multi-Master PFS subvolumes, transaction generations, and Merkle root integrity checks (`Hammer2StorageEngine`).
* **NetBSD Rump Kernels**: Provide isolated userland driver hypercall dispatchers separating drivers from kernel core memory (`NetBsdRumpKernelEngine`).

---

## 2. Core Engineering Rules for AI Agents

When modifying or implementing components in `src/`, AI agents **MUST** strictly adhere to these rules:

1. **Zero External Crate Dependencies**:
   - `Cargo.toml` must contain zero external crate dependencies.
   - All data structures, parsers, cryptographic primitives, and utility functions must use `crate::klib` or core Rust types (`alloc::vec::Vec`, `alloc::string::String`, `alloc::collections::BTreeMap`).

2. **`#![no_std]` Core Compatibility**:
   - Kernel and library code must compile cleanly in `#![no_std]` target environments.
   - Use `extern crate alloc;` for dynamic allocation primitives when operating outside standard std mode.
   - Apply conditional imports for testing (`#[cfg(test)] use std::collections::HashMap;` vs `#[cfg(not(test))] use crate::klib::HashMap;`).

3. **No Unsafe Code Without Core Exemption**:
   - Maintain safe Rust memory safety guarantees (`SIGMA_NO_UNSAFE`).
   - Any hardware MMIO or port I/O abstractions must be encapsulated inside verified HAL wrappers.

4. **Self-Contained Unit Testing**:
   - Every new component or struct **must** include a unit test block (`#[cfg(test)] mod tests`) demonstrating its functionality.
   - Tests must compile standalone (`rustc --test --edition=2021`) and pass when invoked by `./run_sigma_tests.sh`.

---

## 3. Step-by-Step AI Agent Component Development Protocol

Follow this 5-step workflow when building a new component in SigmaOS:

```
[ Step 1: Design & Inspiration ] ----> [ Step 2: Implementation ]
                                                |
                                                v
[ Step 5: Integration & Test ] <---- [ Step 4: Verification ] <---- [ Step 3: Unit Testing ]
```

### Step 1: Design & Inspiration Selection
- Identify the target Linux or BSD subsystem paradigm (e.g. eBPF, VNET, GEOM, io_uring, Pledge/Unveil).
- Determine whether the component belongs in `src/kernel/`, `src/filesystem/`, `src/net/`, `src/security/`, `src/distro/`, or `src/open_source_os_gap_closure.rs`.

### Step 2: Safe Rust Implementation
- Define clear `pub struct` and `pub enum` types representing system state.
- Provide a `new()` constructor and `Default` trait implementation.
- Implement methods for core operations, state transitions, and error handling using `Result<T, &'static str>` or `Option<T>`.

### Step 3: Embedded Unit Test Suite
- Write unit tests under `#[cfg(test)] mod tests` verifying constructor defaults, state updates, edge cases, and error conditions.

### Step 4: Verification & Standalone Test Runner
- Compile the component standalone via `rustc --test --edition=2021 <filepath> -o build/test_out && ./build/test_out`.
- Run `./run_sigma_tests.sh` to confirm 100% test suite compliance without regressions.

### Step 5: Module Re-Export & Documentation Synchronization
- Declare and re-export the new engine in the corresponding parent `mod.rs` or `src/lib.rs`.
- Update relevant documentation in `docs/` and `wiki/` to reflect the new component capability.

---

## 4. Canonical Code Template for AI Agents

Below is a complete, production-grade template for AI agents creating new Linux/BSD-inspired components in SigmaOS:

```rust
// SPDX-License-Identifier: MIT
// SigmaOS Component: Sovereign Linux/BSD Inspired Engine Template

#[cfg(not(test))]
use alloc::string::{String, ToString};
#[cfg(not(test))]
use alloc::vec::Vec;
#[cfg(not(test))]
use alloc::collections::BTreeMap;

#[cfg(test)]
use std::string::String;
#[cfg(test)]
use std::vec::Vec;
#[cfg(test)]
use std::collections::BTreeMap;

/// System state status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentStatus {
    Active,
    Standby,
    Disabled,
}

/// Sovereign Component Engine Struct
#[derive(Debug, Clone)]
pub struct SovereignEngineComponent {
    pub name: String,
    pub status: ComponentStatus,
    pub metrics: BTreeMap<String, u64>,
}

impl SovereignEngineComponent {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            status: ComponentStatus::Active,
            metrics: BTreeMap::new(),
        }
    }

    pub fn record_metric(&mut self, key: &str, value: u64) {
        self.metrics.insert(String::from(key), value);
    }

    pub fn get_metric(&self, key: &str) -> Option<u64> {
        self.metrics.get(key).copied()
    }
}

impl Default for SovereignEngineComponent {
    fn default() -> Self {
        Self::new("default_component")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_engine_component() {
        let mut comp = SovereignEngineComponent::new("test_engine");
        assert_eq!(comp.status, ComponentStatus::Active);
        comp.record_metric("events_processed", 42);
        assert_eq!(comp.get_metric("events_processed"), Some(42));
        assert_eq!(comp.get_metric("missing_key"), None);
    }
}
```

---

## 5. Verification Checklist for AI Agents

Before submitting any code or documentation changes, AI agents **MUST** check off every item:

- [ ] Zero external crate dependencies in `Cargo.toml`.
- [ ] Code compiles under `#![no_std]` targets without `std` leakage outside test blocks.
- [ ] Component includes a unit test block (`#[cfg(test)] mod tests`).
- [ ] `./run_sigma_tests.sh` executes with **100% pass rate** (0 failures).
- [ ] Component re-exported in module tree (`mod.rs` / `src/lib.rs`).
- [ ] Pre-commit instructions executed and verified.
