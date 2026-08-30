# Std Dependency Reduction Plan

**Date**: August 10, 2026\
**Status**: In Progress\
**Repository**: SigmaOS

***

## Current Status Analysis

### Std Usage Statistics

*   **Total std imports**: 345 instances
*   **std::collections usage**: 165 instances
*   **std::sync usage**: 28 instances
*   **std::path usage**: 15 instances
*   **std::string usage**: 37 instances
*   **std::vec usage**: 22 instances
*   **Other std modules**: 78 instances

### High Priority Files for Std Reduction

#### Critical Compatibility Modules

*   `src/compatibility/linux_adapter.rs` - std::collections::HashMap
*   `src/compatibility/fedora.rs` - std::collections::HashMap
*   `src/compatibility/superiority.rs` - std::collections::{HashMap, VecDeque}
*   `src/compatibility/localsend.rs` - std::collections::{HashMap, HashSet}
*   `src/compatibility/cross_platform.rs` - std::collections::HashMap
*   `src/compatibility/lattice.rs` - std::collections::HashMap
*   `src/compatibility/endeavour.rs` - std::collections::HashMap
*   `src/compatibility/india_professional_tools.rs` - std::collections::HashMap
*   `src/compatibility/gap_closure.rs` - std::collections::{HashMap, HashSet}
*   `src/compatibility/prism.rs` - std::collections::HashMap
*   `src/compatibility/cachy_os.rs` - std::sync::atomic::{AtomicBool, AtomicUsize, Ordering}

#### Critical ML/AI Modules

*   `src/ml/sigma_aid.rs` - std::string::{String, ToString}, std::vec::Vec

#### Other Critical Areas

*   Multiple drivers, network, and filesystem modules using std types

***

## Replacement Strategy

### Phase 1: Collections Replacement

**Target**: Replace std::collections with klib equivalents

**Replacements**:

*   `std::collections::HashMap` → `klib::collections::HashMap`
*   `std::collections::HashSet` → `klib::collections::HashSet`
*   `std::collections::VecDeque` → `klib::collections::VecDeque`
*   `std::collections::BTreeMap` → `klib::collections::BTreeMap`
*   `std::collections::BTreeSet` → `klib::collections::BTreeSet`

### Phase 2: String Replacement

**Target**: Replace std::string with klib equivalents

**Replacements**:

*   `std::string::String` → `klib::string::String`
*   `std::string::ToString` → Use klib string methods

### Phase 3: Atomic Types Replacement

**Target**: Replace std::sync::atomic with klib equivalents

**Replacements**:

*   `std::sync::atomic::AtomicBool` → `klib::sync::atomic::AtomicBool`
*   `std::sync::atomic::AtomicUsize` → `klib::sync::atomic::AtomicUsize`
*   `std::sync::atomic::AtomicU64` → `klib::sync::atomic::AtomicU64`
*   `std::sync::atomic::Ordering` → `klib::sync::atomic::Ordering`

### Phase 4: Path Handling Replacement

**Target**: Replace std::path with klib equivalents

**Replacements**:

*   `std::path::Path` → `klib::path::Path`
*   `std::path::PathBuf` → `klib::path::PathBuf`

### Phase 5: Memory Allocation Replacement

**Target**: Replace std::alloc with klib equivalents

**Replacements**:

*   `std::alloc::alloc` → `klib::alloc::alloc`
*   `std::alloc::Layout` → `klib::alloc::Layout`

***

## Implementation Plan

### Step 1: Klib Enhancement

1.  Ensure klib has complete implementations of all required types
2.  Add missing collection types if needed
3.  Implement atomic types if not present
4.  Add path handling functionality

### Step 2: Systematic Replacement

1.  Start with high-usage modules (compatibility layer)
2.  Replace imports one module at a time
3.  Test compilation after each replacement
4.  Fix any type mismatches or API differences

### Step 3: Validation

1.  Ensure all tests pass after replacements
2.  Verify performance is not degraded
3.  Check for any hidden std dependencies
4.  Run comprehensive integration tests

***

## Priority Order

### High Priority (Immediate)

1.  Compatibility modules (165 std::collections instances)
2.  ML/AI modules (performance critical)
3.  Core kernel modules (security critical)

### Medium Priority (Week 2)

1.  Network modules
2.  Filesystem modules
3.  Device drivers

### Low Priority (Week 3)

1.  Application-level modules
2.  Utility modules
3.  Legacy compatibility shims

***

## Success Metrics

**Target**: 95% reduction in std usage
**Current**: 345 std imports
**Target**: < 20 std imports (only for testing/FFI)

**Milestones**:

*   Week 1: Reduce to 200 std imports
*   Week 2: Reduce to 100 std imports
*   Week 3: Reduce to < 20 std imports

***

## Challenges & Solutions

### Challenge 1: API Compatibility

**Issue**: klib may have different API than std
**Solution**: Create compatibility shims for transition period

### Challenge 2: Performance

**Issue**: klib implementations may be slower
**Solution**: Optimize critical paths after migration

### Challenge 3: Test Coverage

**Issue**: Tests may rely on std behavior
**Solution**: Update tests to use klib equivalents

***

**Status**: Ready to begin systematic std dependency elimination
