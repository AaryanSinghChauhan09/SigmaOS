# SigmaOS AI Agent Class Operation Management Guide

This guide defines standard protocols and architectural rules for AI agents implementing and operating kernel device, driver, filesystem, scheduler, and subsystem class operation vtables (operations structures) within SigmaOS.

---

## 1. Class Operation Abstraction Principles

SigmaOS uses zero-cost, type-safe Rust traits and vtable structures to model kernel object operation classes (e.g., `FileOperations`, `VnodeOps`, `SchedClass`, `NetDeviceOps`, `BlockDeviceOps`, `CharDeviceOps`).

AI agents modifying or adding class operations MUST strictly observe the following principles:

1. **Zero Heap Allocation in Core Vtables:** Operation function signatures MUST NOT allocate heap memory (`alloc::vec::Vec`, `alloc::string::String`) on critical dispatch paths. Use fixed stack buffers or raw slices (`&[u8]`, `&mut [u8]`).
2. **Explicit Error Handling:** All operation trait methods MUST return `Result<T, KlibError>` or `Result<T, KernelErrorCode>` rather than panicking or returning integer error codes directly.
3. **Interrupt Safety:** Class methods invoked from interrupt contexts (e.g., `NetDeviceOps::poll`, `BlockDeviceOps::request`) MUST NOT acquire blocking locks or perform sleeping operations.
4. **Const & Static Vtable Instantiation:** Class vtable structs MUST be instantiable as compile-time `const` or `static` reference tables to enable zero-overhead dispatch in kernel space.

---

## 2. Core Subsystem Class Operation Categories

### A. File & VFS Operation Class (`FileOperations` & `VnodeOps`)
Handles Virtual File System (VFS) operations inspired by Linux VFS and BSD vnode operations:
* `read(&self, file: &File, buf: &mut [u8], offset: u64) -> Result<usize, KlibError>`
* `write(&self, file: &File, buf: &[u8], offset: u64) -> Result<usize, KlibError>`
* `ioctl(&self, file: &File, cmd: u32, arg: usize) -> Result<i32, KlibError>`
* `mmap(&self, file: &File, vma: &mut VmaArea) -> Result<(), KlibError>`

### B. Scheduler Operation Class (`SchedClass`)
Defines process scheduling algorithms (e.g., EEVDF, BORE, MLFQ, Batch/Idle):
* `enqueue_task(&self, rq: &mut RunQueue, p: &TaskControlBlock)`
* `dequeue_task(&self, rq: &mut RunQueue, p: &TaskControlBlock)`
* `pick_next_task(&self, rq: &mut RunQueue) -> Option<TaskControlBlock>`
* `task_tick(&self, rq: &mut RunQueue, p: &TaskControlBlock)`

### C. Network Device Operation Class (`NetDeviceOps`)
Defines network interface operation handlers (e.g., VirtIO-Net, Intel e1000, FreeBSD VNET):
* `ndo_open(&self, dev: &NetDevice) -> Result<(), KlibError>`
* `ndo_stop(&self, dev: &NetDevice) -> Result<(), KlibError>`
* `ndo_start_xmit(&self, skb: &SkBuff, dev: &NetDevice) -> Result<TxResult, KlibError>`
* `ndo_get_stats(&self, dev: &NetDevice) -> NetDevStats`

### D. Block Device Operation Class (`BlockDeviceOps`)
Handles block storage drivers (e.g., NVMe, AHCI, virtio-blk, RAMdisk):
* `submit_request(&self, req: &BlockRequest) -> Result<(), KlibError>`
* `flush(&self) -> Result<(), KlibError>`
* `get_geometry(&self) -> BlockGeometry`

---

## 3. Class Operation Registration & Dynamic Switching

AI agents managing class instance registration MUST follow safe atomic registration patterns:

```rust
pub struct DeviceClassRegistration<T: 'static> {
    class_name: &'static str,
    vtable: &'static T,
    ref_count: AtomicU32,
}

impl<T: 'static> DeviceClassRegistration<T> {
    pub const fn new(class_name: &'static str, vtable: &'static T) -> Self {
        Self {
            class_name,
            vtable,
            ref_count: AtomicU32::new(0),
        }
    }

    pub fn acquire(&self) -> &'static T {
        self.ref_count.fetch_add(1, Ordering::Acquire);
        self.vtable
    }

    pub fn release(&self) {
        self.ref_count.fetch_sub(1, Ordering::Release);
    }
}
```

---

## 4. C11 FFI Class Operation Compatibility

When interfacing Rust class operation vtables with host C11 drivers (`tests/cpp_host/test_host.c`):

1. **`#[repr(C)]` Placement:** All vtable structures exported to C MUST be marked `#[repr(C)]`.
2. **`extern "C"` Fn Pointers:** Function pointers within FFI vtables MUST use `Option<unsafe extern "C" fn(...) -> i32>`.
3. **Null Check Validation:** AI agents MUST check function pointer validity (`Option::is_some()`) prior to dereferencing inside FFI wrappers.
