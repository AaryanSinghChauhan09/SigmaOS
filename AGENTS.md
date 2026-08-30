# System Instructions & Guidelines for AI Agents Working on SigmaOS

Welcome to **SigmaOS** — a zero-dependency, post-quantum resilient, capability-based sovereign operating system microkernel built in Rust and C++.

These instructions provide actionable guidance, testing procedures, and architectural principles for AI agents (and human contributors) interacting with this codebase.

***

## 🚀 1. Core Architectural Directives

1.  **Zero-Dependency & Self-Containment (`no_std`):**
    *   The kernel core and primary subsystems are designed to target bare-metal targets (`#![no_std]`).
    *   Avoid adding runtime dependencies on standard `std` libraries inside microkernel shard components unless conditionally gated under test environments (`#[cfg(not(target_os = "none"))]`).
2.  **Capability-Based Security Model:**
    *   Never introduce generic root/admin ACL checks. System call access is authorized exclusively via hardware-enforced 64-bit `CapabilityToken` verification gates.
3.  **Windows NT & Distro Parity Standards:**
    *   Hardware drivers must follow the WDM-style `IoManager`, `DriverObject`, `DeviceObject`, and `DeviceExtension` abstractions.
    *   Kernel memory allocations must respect tagged `Paged` (swappable) and `NonPaged` (always resident) memory pool boundaries.

***

## 🧪 2. Testing & Verification Procedures

Because full workspace `cargo test` builds can experience dependency or workspace-level compilation issues on hosted test runners, **always prioritize standalone file compilation for unit tests**.

### Standalone Testing Commands

To run tests on modified or new Rust subsystem modules:

```bash
# Test driver framework and WDM lifecycle
rustc --test --edition=2021 src/driver/device.rs -o build/driver_tests && ./build/driver_tests && rm build/driver_tests

# Test CachyOS BORE scheduler
rustc --test --edition=2021 src/kernel/scheduler.rs -o build/sched_tests && ./build/sched_tests && rm build/sched_tests

# Test memory manager & performance allocator stack
rustc --test --edition=2021 src/kernel/perf_mm.rs -o build/perf_mm_tests && ./build/perf_mm_tests && rm build/perf_mm_tests
rustc --test --edition=2021 src/kernel/memory.rs -o build/mem_tests && ./build/mem_tests && rm build/mem_tests

# Test planned features & multi-OS kernel subsystems
rustc --test --edition=2021 src/unimplemented_features.rs -o build/feat_tests && ./build/feat_tests && rm build/feat_tests

# Test tools & productivity suite
rustc --test --edition=2021 src/unimplemented_tools.rs -o build/tools_tests && ./build/tools_tests && rm build/tools_tests

# Test ReactOS / Win32 compatibility extensions
rustc --test --edition=2021 src/compatibility/reactos.rs -o build/reactos_tests && ./build/reactos_tests && rm build/reactos_tests
```

***

## 🛠️ 3. Coding Conventions

1.  **Explicit Type Annotations for Collections:**
    *   When instantiating custom vector collections or HashMap types, always supply explicit type annotations (e.g., `let mut keys: std::vec::Vec<String> = std::vec::Vec::new();`) to prevent compiler type-inference ambiguities.
2.  **Bounds-Checked Memory Operations:**
    *   When copying raw byte buffers using `copy_nonoverlapping`, always clamp the length using `.min()` against the target array capacity.
3.  **No Unused Warnings:**
    *   Keep the code clean of compiler warnings. Remove unused `mut` modifiers or prefix unused parameters with underscores (`_param`).
