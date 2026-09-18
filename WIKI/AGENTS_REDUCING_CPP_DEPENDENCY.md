# AI Agent Guidelines: Reducing C++ Dependency & Rust-First Migration in SigmaOS

## 📌 1. Executive Strategy & Motivation

In **SigmaOS**, the long-term architectural goal is the **complete elimination of C++ runtime dependencies (`.cpp` / `.hpp`) in favor of a 100% pure, memory-safe, zero-dependency `#![no_std]` Rust microkernel**.

While C++ provided legacy HAL and driver scaffolding during initial bootstrap phases, it introduces significant microkernel risks:
* **Non-Deterministic Runtime Overhead:** C++ exception handling (`libgcc_s` unwinding) and RTTI (Run-Time Type Information) add hidden binary bloat and unpredictable latency spikes.
* **Lack of Compiler-Enforced Memory Safety:** Memory management in C++ relies on manual destructors or RAII wrappers that lack the spatial/temporal borrow checker guarantees of Rust.
* **Fragmented Abstractions:** Duplicating kernel logic between C++ headers and Rust crates creates maintainability overhead and symbol synchronization issues.

As an AI agent, your directive is to **implement all new features, drivers, and userland tools exclusively in Rust**, and systematically replace legacy C++ components with pure Rust modules.

---

## 🔁 2. C++ to Rust Module Migration Taxonomy

```
+-----------------------------------------------------------------------------------+
|                        C++ TO RUST MIGRATION TAXONOMY                             |
+-----------------------------------------------------------------------------------+
|  📦 Legacy C++ Component                      | 🦀 Pure Rust Replacement          |
+-----------------------------------------------+-----------------------------------+
|  `kernel/drivers/sigma_driver_manager.cpp`   | `src/driver/framework.rs`         |
|  `kernel/core/orchestrator/sigma_cgroup.cpp` | `src/kernel/linux_bsd_innovations.rs` |
|  `sigmaos/core/src/atomic_ipc_deliver.cpp`    | `src/kernel/ipc.rs`               |
|  `sigmaos/core/src/atomic_vfs_resolve.cpp`    | `src/filesystem/support.rs`       |
|  `sigmaos/core/src/atomic_pqc_verify.cpp`     | `src/security/pki.rs`             |
|  `userland/init/sigma_init.cpp`               | `src/userland/init.rs`            |
|  `userland/gui/zenith_compositor.cpp`         | `src/graphics/compositor.rs`      |
+-----------------------------------------------------------------------------------+
```

---

## 🌉 3. C-ABI FFI Interoperability Rules (`extern "C"`)

During the transitional phase where legacy C/C++ bootloaders or tests invoke Rust kernel functions:

1. **Expose C-Compatible Functions:**
   Use `#[no_mangle]` and `pub extern "C"` for exported entry points:
   ```rust
   #[no_mangle]
   pub extern "C" fn sigma_kernel_ipc_deliver(
       channel_id: u64,
       payload_ptr: *const u8,
       payload_len: usize,
   ) -> i32 {
       if payload_ptr.is_null() || payload_len == 0 {
           return -1; // EINVAL
       }
       let slice = unsafe { core::slice::from_raw_parts(payload_ptr, payload_len) };
       // Pure Rust zero-copy delivery
       0
   }
   ```
2. **Explicit C Types:**
   Use raw pointers (`*const T`, `*mut T`) and C-primitive types (`u32`, `u64`, `usize`, `i32`) in FFI signatures rather than Rust-specific non-repr-C structs.

---

## 🛡️ 4. AI Agent Mandatory Migration Rules

1. **Zero New C++ Code:**
   * **Never create new `.cpp`, `.hpp`, `.cc`, or `.cxx` files.** All new kernel modules, driver shards, userland daemons, and system utilities MUST be written in Rust under `src/`.
2. **Rust-First Drivers:**
   * Replace legacy C++ hardware drivers (`sigma_hda.cpp`, `sigma_80211.cpp`, `sigma_kms.cpp`) with memory-safe Rust drivers (`src/drivers/modern_nvme.rs`, `src/drivers/linux_bsd_drivers.rs`).
3. **CMake Integration Strategy:**
   * As C++ modules are migrated to Rust, update `CMakeLists.txt` to remove retired `.cpp` sources, keeping `CMakeLists.txt` lean until full transition to `cargo` / Ninja build targets is achieved.

---

## 🧪 5. Verification & Testing Procedures

AI agents must verify Rust replacement modules using standalone test execution:

```bash
# Test pure Rust replacement for driver management
rustc --test --edition=2021 src/driver/framework.rs -o build/drv_test && ./build/drv_test && rm build/drv_test

# Test pure Rust IPC & delivery
rustc --test --edition=2021 src/kernel/ipc.rs -o build/ipc_test && ./build/ipc_test && rm build/ipc_test

# Test pure Rust Zenith compositor logic
rustc --test --edition=2021 src/graphics/compositor.rs -o build/comp_test && ./build/drv_test && rm build/comp_test
```
