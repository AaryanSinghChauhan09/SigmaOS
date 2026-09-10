# SigmaOS Architecture Decision Document

**Date**: September 4, 2026
**Decision**: Commit to **std-based architecture** (not no_std)
**Status**: APPROVED
**Impact**: Resolves 303+ build errors

---

## Executive Summary

SigmaOS will use **Rust standard library (std)** as its primary dependency model. This decision is based on:

1. **Codebase analysis**: 4,901 `use std::` imports vs 0 `use alloc::` imports
2. **Existing patterns**: All core modules already use std
3. **Practical need**: Full OS requires allocations, networking, threading - all std features
4. **Build efficiency**: Removes architectural confusion causing 303 compilation errors

---

## Architectural Decision

### Decision: **USE STD-BASED ARCHITECTURE**

**Rationale:**
- Codebase is already std-based (4,901 std imports prove this)
- No_std goal conflicts with practical OS requirements
- std provides all needed features efficiently
- Eliminates architectural confusion in build errors

**Impact:**
- ✅ Fixes 303 E0433 "alloc not found" errors
- ✅ Simplifies module organization
- ✅ Clarifies dependency model
- ✅ Reduces compilation error surface

---

## Implementation Strategy

### Phase 1: Remove all alloc Remnants (1 hour)
```
1. Remove all #![no_std] attributes from src/ modules
2. Replace alloc imports with std imports
3. Remove extern crate alloc statements
4. Update Cargo.toml if needed
```

**Files affected:**
- src/extended_distro_matrix.rs - Remove extern crate alloc
- src/tools/data_tools.rs - Remove extern crate alloc
- Any other modules with no_std attributes

**Commands to use:**
```bash
# Find all no_std in src/
grep -r "#!\[no_std\]" src/

# Find all extern crate alloc in src/
grep -r "extern crate alloc" src/

# Find all use alloc:: in src/
grep -r "use alloc::" src/
```

### Phase 2: Standardize Imports (30 min)
```
1. Ensure all imports use std::
2. Remove custom allocator configurations
3. Clean up redundant imports
```

### Phase 3: Module Reorganization (2-4 hours)
```
1. Simplify module hierarchy
2. Reduce re-export complexity
3. Add aggregation layers
4. Document module structure
```

**Pattern**: Instead of lib.rs exporting 50+ modules:
```rust
// OLD (doesn't work with generics)
pub mod foo;
pub mod bar;
// ... 50 more modules
pub use foo::*;
pub use bar::*;
// Type parameters get lost

// NEW (proper aggregation)
pub mod collections {
    pub use alloc::vec::Vec;
    pub use alloc::collections::BTreeMap;
}
pub mod core_types {
    pub use crate::collections::*;
}
pub use core_types::*;  // Explicit re-export
```

### Phase 4: Type Parameter Specification (2-3 hours)
```
1. Add explicit generic bounds at module boundaries
2. Create type aliases for common collections
3. Use concrete types where needed
4. Document generic constraints
```

**Example**:
```rust
// BEFORE (type parameter lost)
pub struct Container {
    items: Vec,  // ERROR: needs type parameter
}

// AFTER (explicit)
pub struct Container<T> {
    items: Vec<T>,  // Clear what type it holds
}

// Or with type alias
pub type ItemVec = Vec<Item>;
pub struct Container {
    items: ItemVec,  // Clear intent
}
```

### Phase 5: Verify Build (1-2 hours)
```
1. cargo check --target x86_64-unknown-linux-gnu
2. Fix remaining errors (now mostly fixable)
3. cargo build --release
4. cargo test --all
```

---

## Architectural Principles

### 1. **Rust Standard Library as Foundation**
- All core types from std (Vec, String, HashMap, etc.)
- All I/O from std (File, networking, threading)
- All memory management through std allocator

### 2. **Modular Organization**
- Clear module boundaries
- Explicit re-exports
- Type parameters at boundaries
- No implicit type inference across modules

### 3. **Custom Allocator (Future)**
- Implement custom allocator as opt-in feature
- Use #[global_allocator] attribute
- Keep std as default for development

### 4. **Feature Flags for Variants**
```toml
[features]
default = ["std"]
std = []
embedded = []  # If future embedded target needed
minimal = []   # Minimal feature set
```

---

## Module Hierarchy (Proposed)

```
src/
├── lib.rs                           # Single aggregation point
├── core/                            # Core types & traits
│   ├── types.rs                     # Basic types
│   ├── collections.rs               # Collection wrappers
│   └── mod.rs                       # Aggregates
├── kernel/                          # Kernel subsystems
│   ├── vfs.rs                       # VirtualFileSystem
│   ├── process.rs                   # ProcessManager
│   ├── network.rs                   # ZenithNet
│   └── mod.rs                       # Aggregates
├── syscalls/                        # Syscall interface
│   ├── file.rs                      # File syscalls
│   ├── network.rs                   # Network syscalls
│   ├── signal.rs                    # Signal syscalls
│   └── mod.rs                       # Aggregates
├── compatibility/                   # Distro compatibility
└── ...
```

**Aggregation pattern**:
```rust
// src/lib.rs - Single entry point
pub mod core;
pub mod kernel;
pub mod syscalls;

// Re-export commonly used types
pub use core::types::*;
pub use kernel::process::ProcessManager;
pub use kernel::network::ZenithNet;
```

---

## Build Error Resolution

### Error Category 1: Type Inference (E0282)
**Root**: Generic types lose parameters across module boundaries

**Before**:
```rust
// In 10 different modules
pub use Vec;  // ERROR: what's the type parameter?
struct MyType { items: Vec }  // ERROR: Vec<what?>
```

**After**:
```rust
// One module boundary
pub use alloc::vec::Vec;
// In other modules
pub struct MyType<T> { items: Vec<T> }  // Explicit
// Or use type aliases
pub type StringVec = Vec<String>;
pub struct MyType { items: StringVec }  // Clear intent
```

### Error Category 2: Import Confusion (E0433)
**Root**: Mixed alloc and std usage

**Solution**: Commit to std everywhere in src/

```bash
# Replace all occurrences
sed -i 's/use alloc::/use std::/g' src/**/*.rs
sed -i 's/extern crate alloc/extern crate std/g' src/**/*.rs
```

### Error Category 3: Duplicate Definitions (E0119)
**Root**: Traits/types defined multiple times

**Solution**: Single source of truth
```rust
// canonical location: src/types/hash_types.rs
pub struct MyHash { ... }

// Other modules: re-export
pub use crate::types::hash_types::MyHash;
```

---

## Feature Flags

### Default Features
```toml
[features]
default = ["std", "full"]
std = []                    # Use standard library
full = []                   # Full feature set
minimal = []                # Minimal build
```

### Build Variants
```bash
# Full build (default)
cargo build --release

# Minimal build (only core components)
cargo build --release --no-default-features --features "std"

# Embedded future target
cargo build --target riscv64-unknown-none --no-default-features
```

---

## Timeline

| Phase | Task | Duration | Start | End |
|-------|------|----------|-------|-----|
| 1 | Remove alloc remnants | 1h | T+0 | T+1 |
| 2 | Standardize imports | 0.5h | T+1 | T+1.5 |
| 3 | Reorganize modules | 2-4h | T+1.5 | T+5.5 |
| 4 | Type parameters | 2-3h | T+5.5 | T+8.5 |
| 5 | Verify build | 1-2h | T+8.5 | T+10.5 |

**Total**: 8-12 hours

---

## Risks & Mitigations

### Risk 1: Breaking Existing no_std Tools
**Mitigation**: Tools in /tools/ have own no_std attributes - not affected
- Each tool file has its own #![no_std]
- Can maintain no_std for tools while src/ uses std

### Risk 2: Future Embedded Target
**Mitigation**: Use feature flags
- Create `embedded` feature for future
- Keep std default for current development

### Risk 3: Performance Impact
**Mitigation**: std doesn't add overhead for what we use
- We already use std allocator implicitly
- std just makes it explicit and cleaner

---

## Success Criteria

- [ ] No more E0433 "alloc not found" errors
- [ ] Type inference errors (E0282) reduced 80%+
- [ ] `cargo check` passes with <500 errors
- [ ] `cargo build --release` succeeds
- [ ] `cargo test --all` passes
- [ ] All 37 unit tests pass

---

## Related Documents

- **BUILD_ANALYSIS.md** - Error analysis leading to this decision
- **CURRENT_SESSION_STATUS.md** - Session context
- **IMPLEMENTATION_ROADMAP.md** - Overall project roadmap

---

## Decision Approval

**Decision**: Commit to std-based architecture
**Approved**: YES
**Implementation**: Start immediately
**Timeline**: 8-12 hours
**Next**: Execute Phase 1 (remove alloc remnants)

---

**Status**: READY TO IMPLEMENT

This decision resolves the architectural confusion causing 303+ build errors and enables the project to move forward with full feature implementation.

