# ⚡ Zig Integration Plan for SigmaOS

This document specifies the integration roadmap for Zig FFI bridges, bare-metal memory safety, and explicit allocation boundaries within the SigmaOS microkernel.

---

## 1. High-Performance Bare-Metal Allocation
Zig's explicit memory management allows SigmaOS to perform low-level hardware structures modeling with zero-allocation overhead.

### Zig Implementation (Zero-Dependency Slab Cache)
```zig
const std = @import("std");

pub const CacheMeta = struct {
    size: usize,
    free_slot: usize,
};

pub const ZeroAllocSlab = struct {
    meta: CacheMeta,
    buffer: [*]u8,
    max_items: usize,

    pub fn init(buffer: [*]u8, size: usize, max_items: usize) ZeroAllocSlab {
        return ZeroAllocSlab{
            .meta = CacheMeta{
                .size = size,
                .free_slot = 0,
            },
            .buffer = buffer,
            .max_items = max_items,
        };
    }

    pub fn alloc(self: *ZeroAllocSlab) ?[*]u8 {
        if (self.meta.free_slot >= self.max_items) {
            return null;
        }
        const offset = self.meta.free_slot * self.meta.size;
        self.meta.free_slot += 1;
        return @ptrCast(self.buffer + offset);
    }
};
```

---

## 2. FFI Bridge to Rust Kernel & Drop Semantics
To interface with Rust's capability manager and custom no-std `Vec<T>`, Zig modules export clean `extern "C"` functions and utilize explicit deallocation bounds.

### Rust Interface with Drop Implementation
To prevent severe memory leaks in the bare-metal kernel, all custom `Vec<T>` structs implement `Drop` to recursively drop internal items and deallocate their backing raw pointers:

```rust
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

extern "C" {
    fn alloc_slab(size: usize) -> *mut u8;
    fn free_slab(ptr: *mut u8);
}
```

### Zig Deallocator Integration
```zig
pub fn deallocate_slab(allocator: *std::mem::Allocator, ptr: [*]u8, size: usize) void {
    allocator.free(ptr[0..size]);
}
```
