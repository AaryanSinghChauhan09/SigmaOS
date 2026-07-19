# 🌐 Multi-Language Microkernel Coexistence & FFI Specifications

This document defines the foreign function interfaces, allocator mappings, and alignment specifications for writing supplementary SigmaOS components in **Rust**, **Zig**, and **Nim**.

---

## 1. Multi-Language Memory Interoperability

To bypass standard-library overhead and dynamic bindings, all three languages communicate via standard C-compatible (`extern "C"`) layouts and draw physical memory frames from the central Buddy Allocator.

---

## 2. Multi-Language Implementation Code

The blocks below present complete, syntactically-valid, direct-implementation templates across Rust, Zig, and Nim.

### 2.1 Rust: Core Memory Map Exporter
```rust
// WIKI Code Block: Rust FFI Allocator Page Exporter
use core::ptr::NonNull;

#[no_mangle]
pub unsafe extern "C" fn ffi_alloc_pages(count: usize) -> *mut u8 {
    if count == 0 {
        return core::ptr::null_mut();
    }
    // Allocates pages using PAGE_SIZE alignment (4096 bytes)
    let size = count * 4096;
    let raw_ptr = core::alloc::alloc(core::alloc::Layout::from_size_align(size, 4096).unwrap());
    raw_ptr
}

#[no_mangle]
pub unsafe extern "C" fn ffi_free_pages(ptr: *mut u8, count: usize) {
    if !ptr.is_null() && count > 0 {
        let size = count * 4096;
        core::alloc::dealloc(ptr, core::alloc::Layout::from_size_align(size, 4096).unwrap());
    }
}
```

### 2.2 Zig: Zero-Dependency Hardware Driver
```zig
// WIKI Code Block: Zig Driver Implementation Importing Rust Memory
const std = @import("std");

extern fn ffi_alloc_pages(count: usize) callconv(.C) [*]u8;
extern fn ffi_free_pages(ptr: [*]u8, count: usize) callconv(.C) void;

pub const PCIeDriver = struct {
    id: u32,
    mapped_pages: usize,
    buffer: []u8,

    pub fn init(id: u32, count: usize) PCIeDriver {
        const ptr = ffi_alloc_pages(count);
        return PCIeDriver{
            .id = id,
            .mapped_pages = count,
            .buffer = ptr[0..(count * 4096)],
        };
    }

    pub fn deinit(self: *PCIeDriver) void {
        ffi_free_pages(self.buffer.ptr, self.mapped_pages);
    }
};
```

### 2.3 Nim: Sandboxed High-Level Subsystem
```nim
# WIKI Code Block: Nim Network Subsystem Importing Rust Allocators
proc ffi_alloc_pages*(count: int): pointer {.importc: "ffi_alloc_pages", cdecl.}
proc ffi_free_pages*(ptr: pointer, count: int) {.importc: "ffi_free_pages", cdecl.}

type
  BufferManager* = object
    data*: pointer
    length*: int

proc newBufferManager*(pages: int): BufferManager =
  let rawPtr = ffi_alloc_pages(pages)
  result = BufferManager(
    data: rawPtr,
    length: pages * 4096
  )

proc destroy*(manager: var BufferManager) =
  if manager.data != nil:
    ffi_free_pages(manager.data, manager.length div 4096)
    manager.data = nil
```
