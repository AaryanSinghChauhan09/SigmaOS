# 🌐 Multi-Language Microkernel Coexistence & FFI Plan

This plan details the architecture and integration roadmap for SigmaOS’s multi-language microkernel ecosystem. By incorporating high-performance, modern systems languages—specifically **Rust**, **Zig**, and **Nim**—SigmaOS establishes a highly flexible, zero-dependency, capability-gated runtime environment.

---

## 1. Multi-Language FFI & Allocator Interop

To prevent performance penalties or memory segmentation faults, the microkernel enforces a strict, C-compatible binary interface (`extern "C"`) and standardizes on a unified shared-heap allocation protocol:

```
               +--------------------------------------+
               |             Rust Kernel              |
               | - Exports: alloc_page(), free_page() |
               +--------------------------------------+
                         |                      |
            (C FFI)      |                      | (C FFI)
                         v                      v
             +-----------------------+      +-----------------------+
             |      Zig Module       |      |      Nim Module       |
             | - Allocator: C-FFI    |      | - Allocator: C-FFI    |
             | - Safe bounds checks  |      | - GC: ARC (Deferred)  |
             +-----------------------+      +-----------------------+
```

### 1.1 Shared Page Frame Allocation (Zero-Dependency Allocator)
Rust exports raw page allocation symbols to the FFI gate. Zig and Nim modules map their custom allocators directly to these symbols, ensuring that all three languages allocate physical memory from the exact same central Buddy Allocator.

---

## 2. Component Implementation Blueprints

### 2.1 Rust: Core Microkernel & Capability Scheduler
Rust manages the high-privilege hardware ring (`Ring 0`), page tables, capability validation, and thread contexts.

```rust
// src/kernel/ffi_gate.rs
use core::ptr::NonNull;

/// Rust exports safe allocator functions to foreign languages
#[no_mangle]
pub extern "C" fn sigma_alloc_pages(pages: usize) -> *mut u8 {
    // In a real system, queries the global Buddy Allocator instance
    let size = pages * 4096;
    unsafe {
        let ptr = alloc(size);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn sigma_free_pages(ptr: *mut u8, pages: usize) {
    let size = pages * 4096;
    unsafe {
        free(ptr, size);
    }
}

extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8, size: usize);
}
```

### 2.2 Zig: Low-Level Storage & Device Drivers
Zig excels at explicit, highly robust low-level bitwise operations. It is utilized to write safe, zero-dependency NVMe, USB, and memory controllers, communicating directly with the Rust kernel via C FFI.

```zig
// src/zig/drivers/nvme.zig
const std = @import("std");

// Import Rust microkernel page allocation gates
extern fn sigma_alloc_pages(pages: usize) callconv(.C) [*]u8;
extern fn sigma_free_pages(ptr: [*]u8, pages: usize) callconv(.C) void;

pub const NvmeError = error{
    DeviceTimeout,
    ControllerResetFailed,
};

pub const NvmeController = struct {
    bar0_address: usize,
    page_count: usize,
    dma_buffer: []u8,

    pub fn init(bar0: usize, pages: usize) NvmeError!NvmeController {
        const raw_ptr = sigma_alloc_pages(pages);
        return NvmeController{
            .bar0_address = bar0,
            .page_count = pages,
            .dma_buffer = raw_ptr[0..(pages * 4096)],
        };
    }

    pub fn shutdown(self: *NvmeController) void {
        sigma_free_pages(self.dma_buffer.ptr, self.page_count);
    }
};
```

### 2.3 Nim: Sandboxed High-Level Services & S-NET Protocols
Nim is compiled directly to high-density, zero-dependency C files, utilizing its ultra-fast ARC (Automatic Reference Counting) compiler mode. It is ideal for implementing networking layers, compliance policy parsers, and command CLI systems.

```nim
# src/nim/network/dns.nim
{.compile: "src/nim/network/dns.c".}

# Import allocation gates from Rust kernel
proc sigma_alloc_pages*(pages: int): pointer {.importc: "sigma_alloc_pages", cdecl.}
proc sigma_free_pages*(ptr: pointer, pages: int) {.importc: "sigma_free_pages", cdecl.}

type
  DnsPacket* = object
    id*: uint16
    flags*: uint16
    questions*: int
    buffer*: pointer
    capacity*: int

proc newDnsPacket*(questions: int): DnsPacket =
  # Allocates 1 physical page frame from the central Rust Buddy Allocator
  let rawBuffer = sigma_alloc_pages(1)
  result = DnsPacket(
    id: 0xDEAD_u16,
    flags: 0x0100_u16, # Standard recursive query
    questions: questions,
    buffer: rawBuffer,
    capacity: 4096
  )

proc destroy*(packet: var DnsPacket) =
  if packet.buffer != nil:
    sigma_free_pages(packet.buffer, 1)
    packet.buffer = nil
```

---

## 3. Deployment & Compilation Roadmap

1.  **Phase 1: Configure Multi-Language Toolchains (Milestone 1)**
    *   Integrate a global unified build script or Makefile supporting `cargo`, `zig cc`, and `nim c --gc:arc`.
    *   Verify cross-compilation binaries compatibility for target platform architectures.
2.  **Phase 2: Standardize Shared memory Maps (Milestone 2)**
    *   Write FFI headers defining `sigma_alloc_pages` and capability token layout constraints.
    *   Confirm memory boundary safety during dynamic allocation calls across Rust, Zig, and Nim.
3.  **Phase 3: FFI Interface Test Suite (Milestone 3)**
    *   Create mock test drivers in `tests/integration_test.rs` compiling Zig and Nim targets.
    *   Verify that passing buffers via FFI does not trigger page faults or cache-coherency stalls.
