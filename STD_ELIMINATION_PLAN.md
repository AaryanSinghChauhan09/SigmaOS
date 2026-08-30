# std Usage Elimination Plan for SigmaOS Kernel

**Date:** August 10, 2026\
**Scope:** Core Kernel Modules\
**Status:** Analysis Complete, Implementation In Progress

***

## Current std Usage Analysis

**Total std usage instances found:** 54 across kernel modules\
**Critical modules identified:** 15 core kernel files

***

## Priority Categories

### 🔴 CRITICAL - Core Kernel Functionality

1.  `src/kernel/syscall/table.rs` - System call interface
2.  `src/kernel/mm/page_cache.rs` - Memory management
3.  `src/kernel/block_dev.rs` - Block device operations
4.  `src/kernel/numa_scheduler.rs` - NUMA scheduling

### 🟡 HIGH - Subsystem Support

5.  `src/kernel/subsystems/registry.rs` - Subsystem registry
6.  `src/kernel/linux_bsd_innovations.rs` - Linux/BSD compatibility
7.  `src/kernel/self_healing.rs` - Self-healing mechanisms
8.  `src/kernel/net/socket_layer.rs` - Network socket layer

### 🟢 MEDIUM - Legacy Drivers & Features

9.  `src/kernel/drivers/legacy/*` - Legacy driver implementations
10. `src/kernel/fs/*` - Filesystem components
11. `src/kernel/proc/*` - Process management
12. `src/kernel/power/*` - Power management

***

## Replacement Strategy

### Phase 1: Core Infrastructure (Week 1)

**Target:** syscall/table.rs, mm/page\_cache.rs, block\_dev.rs

**Replacements needed:**

*   `std::collections::HashMap` → `klib::HashMap`
*   `std::vec::Vec` → `klib::Vec`
*   `std::string::String` → `klib::SigmaString`
*   `std::sync::atomic` → `core::sync::atomic`

### Phase 2: Network & Process (Week 2)

**Target:** net/socket\_layer.rs, proc/\* modules

**Replacements needed:**

*   String operations → klib::string
*   Process vectors → klib::Vec
*   Synchronization → klib::async\_runtime

### Phase 3: Legacy & Compatibility (Week 3-4)

**Target:** drivers/legacy/*, fs/*, power/\*

**Replacements needed:**

*   Complete std removal from legacy code
*   Compatibility layer updates

***

## Implementation Example

### Before (std usage):

```rust
use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

pub struct SyscallTable {
    handlers: HashMap<u64, String>,
    arguments: Vec<String>,
}
```

### After (klib usage):

```rust
use crate::klib::HashMap;
use crate::klib::SigmaString as String;
use crate::klib::Vec;

pub struct SyscallTable {
    handlers: HashMap<u64, String>,
    arguments: Vec<String>,
}
```

***

## Challenges & Solutions

### Challenge 1: Test Code std Usage

**Solution:** Use `#[cfg(test)]` guards to keep std for tests only

```rust
#[cfg(test)]
use std::collections::HashMap;

#[cfg(not(test))]
use crate::klib::HashMap;
```

### Challenge 2: Conditional Compilation

**Solution:** Feature flags for hosted vs bare-metal builds

```rust
#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use crate::klib::HashMap;
```

### Challenge 3: Complex Type Dependencies

**Solution:** Incremental replacement with compatibility shims

***

## Success Metrics

*   ✅ Zero std usage in core kernel modules
*   ✅ All kernel code compiles with `#![no_std]`
*   ✅ No performance regression
*   ✅ Tests still pass with klib implementations

***

## Current Status

**Analysis:** ✅ Complete\
**Implementation:** 🔄 In Progress\
**Testing:** ⏳ Pending\
**Documentation:** ⏳ Pending

***

## Next Immediate Actions

1.  **Start with syscall/table.rs** - Most critical for kernel operation
2.  **Update klib if needed** - Add any missing functionality
3.  **Test compilation** - Ensure no build failures
4.  **Run tests** - Verify functionality maintained
5.  **Document changes** - Update dependency reduction guide

***

*Plan Version: 1.0*\
*Last Updated: August 10, 2026*\
*Owner: SigmaOS Development Team*
