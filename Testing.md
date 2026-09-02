# Testing Guide

SigmaOS has a comprehensive test suite covering kernel algorithms, distro compatibility, security, and system integration.

---

## Running Tests

```bash
# Run all tests
cargo test

# Run with output (see println! output)
cargo test -- --nocapture

# Run specific test suite
cargo test --test algorithm_and_components_inspection_tests
cargo test --test distro_inspection_and_security_tests
cargo test --test linux_bsd_inspection_tests

# Run tests matching a pattern
cargo test "memory"
cargo test "security"
```

---

## Test Suites

| Test File | What It Tests |
|-----------|--------------|
| `algorithm_and_components_inspection_tests.rs` | Core algorithms: sorting, hashing, graph algorithms |
| `algorithm_inspection_tests.rs` | Algorithm correctness and edge cases |
| `distro_inspection_and_security_tests.rs` | Distro parity + security model |
| `linux_bsd_inspection_tests.rs` | Linux/BSD compatibility layer |
| `sovereign_inspection_suite.rs` | Full system verification |
| `sovereign_subsystems_inspection_tests.rs` | Individual subsystem tests |
| `sigpkg_ecosystem_tests.rs` | Package manager tests |
| `sigpkg_client_tests.rs` | Package manager client API |
| `posix_ltp_tests.rs` | POSIX compliance (LTP subset) |
| `stress_and_fuzz_tests.rs` | Stress testing and fuzzing |
| `arch_kernel_inspirations_tests.rs` | Arch Linux parity tests |
| `distro_inspirations_tests.rs` | All distro inspiration modules |
| `linuxmint_inspirations_tests.rs` | Linux Mint parity tests |
| `extended_features_tests.rs` | Extended OS feature tests |
| `integration_test.rs` | End-to-end integration tests |
| `os_algorithm_inspection_tests.rs` | OS scheduling/memory algorithms |
| `virtualization_qemu_kvm_inspection_tests.rs` | QEMU/KVM compatibility |
| `ecosystem_and_compliance_inspection_tests.rs` | Ecosystem compliance |

---

## Writing Tests

### Unit Test (inline)

```rust
// src/klib/hashmap.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut map = HashMap::new();
        map.insert("key", 42);
        assert_eq!(map.get("key"), Some(&42));
    }

    #[test]
    fn test_missing_key() {
        let map: HashMap<&str, i32> = HashMap::new();
        assert_eq!(map.get("missing"), None);
    }
}
```

### Integration Test

```rust
// tests/integration_test.rs
use sigmaos::klib::hashmap::HashMap;
use sigmaos::security::pledge::PledgeClass;

#[test]
fn test_process_pledge_restriction() {
    // Test that pledge works correctly
    let mut process = MockProcess::new();
    process.pledge(&[PledgeClass::Stdio]).unwrap();
    
    // Stdio operations should work
    assert!(process.write_stdout("hello").is_ok());
    
    // Network operations should be blocked
    assert!(process.connect_tcp("1.2.3.4", 80).is_err());
}
```

### Fuzz Test

```rust
// tests/stress_and_fuzz_tests.rs
#[test]
fn fuzz_string_parse() {
    use sigma_string_utils::parse_int_from_bytes;
    
    // Test with all byte patterns
    for b in 0u8..=255 {
        let input = &[b, b'0', b'1'];
        // Should never panic
        let _ = parse_int_from_bytes(input);
    }
}
```

---

## Test Coverage

Generate coverage report:

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Generate HTML coverage report
cargo llvm-cov --html
open target/llvm-cov/html/index.html

# Generate lcov report (for CI)
cargo llvm-cov --lcov --output-path lcov.info
```

---

## Performance Benchmarks

```bash
# Run benchmarks
cargo bench

# Benchmark specific module
cargo bench --bench memory_alloc_bench
```

---

## QEMU Integration Tests

```bash
# Run full system test in QEMU
python3 scripts/qemu_smoke_test.py

# Test with specific kernel option
python3 scripts/qemu_smoke_test.py --kernel-args "nokaslr"

# Run and connect serial console
python3 scripts/qemu_smoke_test.py --interactive
```

---

## CI Test Results

Each PR runs the full test matrix:
- `cargo check` — compilation verification
- `cargo test` — unit and integration tests  
- `cargo clippy` — lint checks
- `cargo fmt --check` — formatting check
- Architecture-specific builds (x86_64, aarch64)
- POSIX LTP subset
- Distro compatibility matrix
