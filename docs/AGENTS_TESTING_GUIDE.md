# SigmaOS AI Agent Test Management & Verification Guide

This guide defines test execution protocols, test suite categories, and standalone compilation commands for AI coding agents developing on the SigmaOS repository.

---

## 1. Master Native Test Runner Execution

Always run the native master test script as your primary verification step:

```bash
./run_sigma_tests.sh
```

**Coverage:**
* **224 Atomic Core Tests:** Primitive data structures, `#![no_std]` allocators, VMM/PMM, and kernel schedulers.
* **11 Security Validation Tests:** IPv4/IPv6 address validation, path traversal boundaries, and null byte rejection.
* **57 Subsystem Inspection Tests:** CachyOS BORE scheduler, memory paging, zero-copy IPC channels, and capability gates.

---

## 2. Standalone Subsystem Unit Test Commands

When working on specific subsystems, use standalone `rustc --test` commands for rapid isolated compilation and verification:

| Subsystem | Source File | Compilation & Test Command |
|---|---|---|
| **Security Validation** | `src/security/input_validation.rs` | `rustc --test src/security/input_validation.rs --edition=2021 -o build/input_val_test && ./build/input_val_test` |
| **Distro Innovations** | `src/distro/missing_distro_innovations.rs` | `rustc --test src/distro/missing_distro_innovations.rs --edition=2021 --cfg 'feature="standalone_test"' -o build/missing_distros_test && ./build/missing_distros_test` |
| **Distro System Gaps** | `src/distro/linux_bsd_distro_gaps.rs` | `rustc --test src/distro/linux_bsd_distro_gaps.rs --edition=2021 -o build/distro_gaps_test && ./build/distro_gaps_test` |
| **Ecosystem Bridge** | `src/compatibility/linux_bsd_ecosystem_bridge.rs` | `rustc --test src/compatibility/linux_bsd_ecosystem_bridge.rs --edition=2021 -o build/ecosystem_bridge_test && ./build/ecosystem_bridge_test` |
| **Media Engine** | `src/media/distro_media_engine.rs` | `rustc --test src/media/distro_media_engine.rs --edition=2021 -o build/distro_media_test && ./build/distro_media_test` |
| **Sovereign Commands** | `src/tools/sovereign_commands.rs` | `rustc --test src/tools/sovereign_commands.rs --edition=2021 -o build/tools_test && ./build/tools_test` |
| **Unimplemented Features**| `src/unimplemented_features.rs` | `rustc --test src/unimplemented_features.rs --edition=2021 -o build/unimplemented_test && ./build/unimplemented_test` |
| **Wiki Ideas Engine** | `src/distro/wiki_ideas_implementation.rs` | `rustc --test src/distro/wiki_ideas_implementation.rs --edition=2021 -o build/wiki_ideas_test && ./build/wiki_ideas_test` |
| **Multi-Arch HAL** | `src/hal/multi_arch.rs` | `rustc --test src/hal/multi_arch.rs --edition=2021 -o build/hal_test && ./build/hal_test` |
| **Open Source Supremacy**| `src/open_source_os_gap_closure.rs` | `rustc --test src/open_source_os_gap_closure.rs --edition=2021 -o build/gap_closure_test && ./build/gap_closure_test` |
| **klib Zero-Dependency** | `src/klib/conversion.rs` | `rustc --test src/klib/conversion.rs --edition=2021 -o build/conv_test && ./build/conv_test` |

---

## 3. Host C11 Microkernel Verification Tests

To verify host microkernel C ABI bindings via CMake:

```bash
mkdir -p build/cpp_host_build && cd build/cpp_host_build && cmake ../../tests/cpp_host && make && ./host_tests
```

---

## 4. Python Integration & Stress Test Suites

To execute Python integration, core unit, and fuzzing benchmarks:

```bash
pytest tests/test_unit_core.py tests/test_integration_system.py tests/test_stress_fuzz_bench.py
```

or via standard library:

```bash
python3 -m unittest discover -s tests -p "test_*.py"
```

---

## 5. Test Management Rules for AI Agents

1. **Test-Driven Modifications:** Any new feature, driver, or security fix MUST be accompanied by unit tests (`#[test]`) in the relevant file.
2. **Zero Failures Allowed:** Pre-commit validation requires 0 failing tests across all Rust, C11, and Python test runners.
3. **No Unused Code Warnings:** Ensure unit tests and test structures do not leave unused variables or fields (use `_var` or prefix unused fields with `_`).
