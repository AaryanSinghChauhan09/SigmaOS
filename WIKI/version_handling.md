# SigmaOS Version Handling & Testing Architecture

## Overview

SigmaOS utilizes a unified, zero-dependency versioning model across its native package manager (`sigma-pkg`), multi-distro universal adapters, kernel subsystem modules, and zero-allocation SAT solvers. This document specifies the version representation, normalization algorithms, version constraint evaluation, and automated testing procedures for SigmaOS version handling.

---

## 1. Version Representation

The primary version structure in SigmaOS is `Version`, defined in `#![no_std]` compatible Rust modules (`src/sigpkg/universal_oop_system.rs`, `src/sigpkg/universal_adapter.rs`, `src/sigpkg/zero_alloc_resolver.rs`):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}
```

### Display & Comparison Traits
`Version` implements `Display`, `PartialEq`, `Eq`, `PartialOrd`, and `Ord`, enabling standard comparison operations (`<`, `<=`, `==`, `>=`, `>`) based on numeric tuple ordering `(major, minor, patch)`.

---

## 2. Foreign Version Normalization Algorithms

When absorbing packages from foreign Linux, BSD, and Unix distributions, SigmaOS standardizes heterogeneous version strings into canonical `Version` representations using `translate_to_native_package` in `UniversalPackageAdapter`:

### A. Strip Revision & Epoch Identifiers
- **Debian/Ubuntu (`.deb`):** Strips distribution revisions following `-` (e.g., `8.2.1-1ubuntu1` -> `8.2.1`).
- **Fedora/RHEL (`.rpm`):** Strips RPM build release suffixes and epoch numbers (e.g., `1:2.1.0-4.el9` -> `2.1.0`).
- **Void Linux (`.xbps`):** Strips XBPS revisions containing `_` (e.g., `5.2.15_1` -> `5.2.15`).

### B. Auto-Padding for Semantic Versioning
The parser counts dot delimiters (`.`) in the cleaned string:
- `0` dots (e.g., `"3"`): Appends `.0.0` -> `3.0.0`
- `1` dot (e.g., `"3.2"`): Appends `.0` -> `3.2.0`
- `2+` dots (e.g., `"3.2.1.4"`): Parses the first 3 numeric components.

### C. ASCII Digit Filtering
Non-numeric characters in major/minor/patch positions are filtered to ensure safe integer parsing:
```rust
pub fn parse(version_str: &str) -> Result<Self, &'static str> {
    let clean = version_str.split('-').next().unwrap_or(version_str);
    let mut parts = clean.split('.');
    let major = parts.next().unwrap_or("0").chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap_or(0);
    let minor = parts.next().unwrap_or("0").chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap_or(0);
    let patch = parts.next().unwrap_or("0").chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap_or(0);
    Ok(Version::new(major, minor, patch))
}
```

---

## 3. Version Constraints & SAT Solver Constraints

### Version Constraints (`VersionConstraint`)
Package dependencies declare version requirements via the `VersionConstraint` enum:
- `Exact(Version)`: Package must match version exactly.
- `GreaterThan(Version)`: Version > threshold.
- `LessThan(Version)`: Version < threshold.
- `GreaterOrEqual(Version)`: Version >= threshold.
- `LessOrEqual(Version)`: Version <= threshold.
- `Any`: Any version satisfies dependency.

### Bare-Metal SAT Solver Constraints (`PackageConstraint`)
For microkernel and low-memory environments, `src/unimplemented_features.rs` provides a zero-allocation SAT solver using range bounds:
```rust
pub struct PackageConstraint {
    pub target_id: u16,
    pub min_version: PkgVersion,
    pub max_version: PkgVersion,
}
```

---

## 4. Testing Procedures for Version Handling

Version parsing, comparison, and constraint evaluation are covered by automated unit and integration tests across the test suite:

### A. Standalone Rust Unit Tests
Run standalone test runners for version handling modules:
```bash
# Test Universal Package Adapter version parsing
rustc --test --edition 2021 tests/test_universal_adapter.rs -o build/test_universal_adapter
./build/test_universal_adapter

# Test zero-allocation package resolver
rustc --test --edition 2021 src/sigpkg/zero_alloc_resolver.rs -o build/test_zero_alloc
./build/test_zero_alloc
```

### B. Master Test Suite
Execute the comprehensive test script to run Python pytest integration suites, standalone Rust modules, and inspection test binaries:
```bash
./run_sigma_tests.sh
```

### C. Specific Test Cases Covered
1. **SemVer Parsing & Comparison (`test_apt_control_parsing_and_translation`):** Verifies version `8.2.1` parses into `Version::new(8, 2, 1)` and compares correctly against constraints.
2. **Complex Revisions (`test_universal_adapter_all_formats`):** Validates UCL (`7.0.11`), OpenBSD (`3.3a`), and Slackware (`1.0`) versions.
3. **SAT Solver Backtracking (`test_section_6_3_sat_solver`):** Tests bounds checking for package versions `1.0` through `2.1`.
