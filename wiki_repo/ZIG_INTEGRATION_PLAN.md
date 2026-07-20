# ⚡ Zig Integration Plan for SigmaOS

This document specifies the integration roadmap for Zig FFI bridges, bare-metal memory safety, and explicit allocation boundaries within the SigmaOS microkernel.

---

## 1. High-Performance Bare-Metal Allocation
Zig's explicit memory management allows SigmaOS to perform low-level hardware structures modeling with zero-allocation overhead. We use Zig to construct complex DMA descriptor buffers for hardware abstraction.

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

## 2. FFI Bridge to Rust Kernel
To interface with Rust's capability manager, Zig modules export clean `extern "C"` functions. 

### Rust Interface
```rust
extern "C" {
    fn alloc_slab(size: usize) -> *mut u8;
    fn free_slab(ptr: *mut u8);
}
```

The C-ABI guarantees structural compatibility between the microkernel's Rust capability verifier and the lightweight Zig sub-modules.
