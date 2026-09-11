# Sovereign C++ Elimination Policy & Rust Migration Directive

This document establishes the official engineering policy for eliminating C++ dependencies, C++ runtime libraries (`libstdc++`, `libc++`), and C++ source code across all subsystems of **SigmaOS**.

---

## 1. Architectural Objective & Justification

Legacy Linux and BSD distributions depend heavily on C++ runtimes (`libstdc++`, `libc++`, `vLLM`, `Ananicy-cpp`, `llama.cpp`), introducing significant vulnerability surfaces (memory safety bugs, unhandled exceptions, RTI overhead, symbol mangling, and dynamic linking dependencies).

SigmaOS achieves complete self-sufficiency and zero-dependency memory safety by enforcing a **100% Safe Rust `#![no_std]` Architecture**.

---

## 2. Technical Directives for C++ Elimination

### 2.1 C++ Source Code Migration Plan
Legacy C++ source shards identified in `sigmaos/core/src/`, `drivers/`, `orchestrator/`, and `suites/` MUST be systematically replaced with pure Rust implementations under `src/`:

| Legacy C++ Module Path | Replacement Pure Rust Engine | Target Subsystem |
| :--- | :--- | :--- |
| `sigmaos/core/src/atomic_pqc_verify.cpp` | `KyberDilithiumPqcGuard` | `src/pillars/distro_crushing_benchmark.rs` |
| `sigmaos/core/src/atomic_vfs_resolve.cpp` | `SovereignUniversalDistroBridge` | `src/distro/linux_bsd_inspirations.rs` |
| `sigmaos/core/src/atomic_scheduler_cfs.cpp` | `BoreSchedulerGovernor` | `src/compatibility/cachy_os.rs` |
| `drivers/graphics/sigma_kms.cpp` | `SovereignDeviceManager` | `src/drivers/linux_bsd_drivers.rs` |
| `drivers/usb/sigma_usb_hcd.cpp` | `SovereignDeviceManager` | `src/drivers/linux_bsd_drivers.rs` |
| `orchestrator/main.cpp` | `SovereignUniversalDistroBridge` | `src/distro/linux_bsd_inspirations.rs` |

### 2.2 External C++ Library Replacement Strategy
External C++ runtimes and CLI utilities MUST be replaced with zero-dependency `#![no_std]` Rust engines:

* **Ananicy-cpp Replacement:** Replaced by `AnanicyCppTuningManager` in `src/compatibility/cachy_os.rs` and `src/performance/cachy_opt.rs`.
* **llama.cpp / vLLM Replacement:** Replaced by native Rust GGUF tensor memory managers (`DataPipelineEtlEngine` / `AiTensorMemoryManager`) in `src/ai/`.
* **C++ CMake Build Toolchain Replacement:** Replaced by cargo and static `rustc` compilation scripts with zero C++ compiler requirements (`g++`, `clang++`).

---

## 3. Toolchain & CI Enforcement Rules

1. **Zero C++ Runtimes in Production Binaries:** Production images compiled via `cargo build` or `./run_sigma_tests.sh` MUST NOT link against `libstdc++.so`, `libc++.so`, or `libgcc_s.so`.
2. **Static Linking Guarantee:** All userland tools and kernel modules MUST compile into self-contained static binaries.
3. **Automated Verification:** CI/CD pipelines MUST execute `no_std_check.sh` to confirm zero C++ symbols in ELF header exports.

---

## 4. Compliance Verdict

By enforcing this policy, SigmaOS completely eliminates C++ memory safety risks, runtime exception overhead, and dynamic library dependencies—guaranteeing 100% memory safety, deterministic execution, and sovereign self-sufficiency.
