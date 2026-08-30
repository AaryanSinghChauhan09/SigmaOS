# SigmaOS Security Hardening Guide

## Overview

SigmaOS applies a **defense-in-depth** approach to security, drawing from Linux kernel security subsystems (SELinux, AppArmor, seccomp) and BSD security models (pledge, unveil, Capsicum).

## 1. Memory Safety

### Unsafe Code Policy

All `unsafe` blocks **must** include a `// SAFETY:` comment explaining invariants:

```rust
// SAFETY: idx < self.len is verified above, so self.data.add(idx) is within bounds
unsafe { &*self.data.add(idx) }
```

### Integer Overflow Prevention

In kernel/klib code, use checked arithmetic:

```rust
// ❌ Vulnerable
let new_size = self.len * 2;

// ✅ Safe  
let new_size = self.len.checked_mul(2).expect("size overflow");
```

### Buffer Bounds Validation

Before any pointer arithmetic or unsafe block:

```rust
pub fn get(&self, index: usize) -> Option<&T> {
    if index >= self.len {
        return None;  // Always check bounds first
    }
    // SAFETY: index < self.len verified above
    unsafe { Some(&*self.data.add(index)) }
}
```

## 2. Security Scanning Fixes Applied

### CVE-class Issues Addressed

| Issue Type | File | Fix Applied |
|-----------|------|-------------|
| Unchecked array index | `src/klib/vec.rs` | Added bounds check with panic message |
| Raw pointer cast without validation | `src/klib/ring_buffer.rs` | Added SAFETY comments + invariant checks |
| Integer overflow in capacity calc | `src/klib/buddy_allocator.rs` | Used `checked_mul`/`next_power_of_two` |
| Use-after-free risk | `src/klib/linked_list.rs` | Box ownership model prevents UAF |
| Uninitialized memory read | `src/klib/slab.rs` | `MaybeUninit` with explicit initialization |

### Security Patterns Used

```rust
// Pattern 1: MaybeUninit for uninitialized memory (prevents reading garbage)
let data: MaybeUninit<T> = MaybeUninit::uninit();
// ... write before reading
let value = unsafe { data.assume_init_read() };

// Pattern 2: NonNull instead of raw *mut for non-null guarantees
let ptr: NonNull<T> = unsafe { NonNull::new_unchecked(raw_ptr) };

// Pattern 3: CAS (Compare-And-Swap) for lock-free correctness
match self.head.compare_exchange_weak(old, new, Ordering::AcqRel, Ordering::Relaxed) {
    Ok(_) => return,
    Err(_) => continue, // Retry
}
```

## 3. W^X Memory Protection

Inspired by OpenBSD's W^X enforcement, memory pages are never simultaneously writable and executable:

```rust
// Page permissions (src/klib/paging.rs)
pub struct PageFlags {
    readable: bool,
    writable: bool,    // W
    executable: bool,  // X
}

impl PageFlags {
    pub fn wx_check(&self) -> bool {
        // Enforce W^X: not both writable AND executable
        !(self.writable && self.executable)
    }
}
```

## 4. Capability-Based Security (Capsicum-inspired)

Inspired by FreeBSD Capsicum and OpenBSD pledge():

```rust
// src/kernel/policy_mechanism.rs
pub struct Capability {
    read: bool,
    write: bool,
    exec: bool,
    net: bool,
    fs_access: bool,
}
```

Processes operate with minimal capabilities required (principle of least privilege).

## 5. Secure Memory Zeroing

Inspired by OpenBSD explicit\_bzero and Linux's memzero\_explicit:

```rust
// src/kernel/secure_free.rs
pub fn secure_zero(ptr: *mut u8, len: usize) {
    // Use volatile write to prevent compiler from optimizing out
    for i in 0..len {
        unsafe { core::ptr::write_volatile(ptr.add(i), 0u8) };
    }
    // Memory barrier to ensure zeroing completes before deallocation
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}
```

## 6. Address Space Layout Randomization (ASLR)

ASLR randomizes the base addresses of:

*   Kernel modules
*   Stack
*   Heap

```rust
// src/kernel/memory.rs - ASLR implementation
pub fn randomize_load_address(base: usize, entropy_bits: u8) -> usize {
    let mask = ((1usize << entropy_bits) - 1) << 12; // Page-aligned
    let random = crate::kernel::crypto::get_random_usize();
    (base + (random & mask)) & !0xFFF // Page-align result
}
```

## 7. Stack Canaries

Inspired by GCC's -fstack-protector:

```rust
// src/kernel/security/ - stack protection
const STACK_CANARY: u64 = 0xDEAD_BEEF_CAFE_F00D;

pub fn check_stack_canary(canary: u64) -> bool {
    canary == STACK_CANARY
}
```

## 8. Seccomp-like System Call Filtering

Inspired by Linux seccomp and OpenBSD pledge():

```rust
// src/kernel/policy_mechanism.rs
pub struct SyscallPolicy {
    allowed: &'static [u32],  // Whitelist of allowed syscall numbers
}

impl SyscallPolicy {
    pub fn is_allowed(&self, syscall_nr: u32) -> bool {
        self.allowed.contains(&syscall_nr)
    }
}
```

## 9. Randomized Heap Allocator

Inspired by OpenBSD malloc's randomization:

```rust
// src/klib/buddy_allocator.rs
// Randomize which free block is returned from same-size freelist
// to prevent heap layout prediction attacks
```

## 10. Continuous Security Scanning

The CI/CD pipeline (`.github/workflows/`) includes:

*   `cargo audit` for dependency vulnerability scanning
*   Custom static analysis for `unsafe` block patterns
*   Memory sanitizer tests
*   Fuzzing with custom input generation

## Security Contact

Report vulnerabilities via: [SECURITY.md](../SECURITY.md)

> \[!CAUTION]
> Never commit credentials, keys, or sensitive data to the repository.
> All cryptographic keys must use the `src/kernel/crypto/` key storage APIs.
