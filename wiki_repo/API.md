# SigmaOS API Documentation

## System Call API

SigmaOS provides a system call interface for userland applications to interact with the kernel.

### System Call Numbers

```rust
// System call numbers
pub const SYS_EXIT: usize = 1;
pub const SYS_READ: usize = 2;
pub const SYS_WRITE: usize = 3;
pub const SYS_OPEN: usize = 4;
pub const SYS_CLOSE: usize = 5;
pub const SYS_MMAP: usize = 9;
pub const SYS_MUNMAP: usize = 11;
pub const SYS_IOCTL: usize = 16;
pub const SYS_SOCKET: usize = 41;
pub const SYS_CONNECT: usize = 42;
pub const SYS_ACCEPT: usize = 43;
pub const SYS_SENDTO: usize = 44;
pub const SYS_RECVFROM: usize = 45;
pub const SYS_CLONE: usize = 56;
pub const SYS_EXECVE: usize = 59;
pub const SYS_EXIT_GROUP: usize = 231;
```

### System Call Dispatch

System calls are dispatched through the syscall dispatch table:

```rust
// kernel/core/syscall.rs
pub fn syscall_dispatch(num: usize, args: &[usize]) -> Result<usize, SyscallError> {
    match num {
        SYS_READ => sys_read(args),
        SYS_WRITE => sys_write(args),
        SYS_OPEN => sys_open(args),
        SYS_CLOSE => sys_close(args),
        SYS_MMAP => sys_mmap(args),
        SYS_CLONE => sys_clone(args),
        SYS_EXECVE => sys_execve(args),
        _ => Err(SyscallError::InvalidSyscall),
    }
}
```

## Memory Management API

### Buddy Allocator

```rust
// kernel/mm/buddy_allocator.rs

/// Initialize buddy allocator
pub unsafe fn sigma_buddy_init(base_addr: u64, total_mem: u64) -> i32;

/// Allocate physical frames
pub unsafe fn sigma_buddy_alloc(order: u8) -> u64;

/// Free physical frames
pub unsafe fn sigma_buddy_free(addr: u64, order: u8) -> i32;

/// Get free frame count
pub unsafe fn sigma_buddy_get_free() -> u64;

/// Get total frame count
pub unsafe fn sigma_buddy_get_total() -> u64;
```

### Slab Allocator

```rust
// kernel/mm/slab_allocator.rs

/// Initialize slab allocator
pub unsafe fn sigma_slab_init() -> i32;

/// Allocate object from slab
pub unsafe fn sigma_slab_alloc(size: usize, align: usize) -> *mut u8;

/// Free object to slab
pub unsafe fn sigma_slab_free(ptr: *mut u8, size: usize);
```

## Scheduler API

### Round-Robin Scheduler

```rust
// kernel/scheduler/round_robin_scheduler.rs

/// Initialize scheduler
pub unsafe fn sigma_sched_init() -> i32;

/// Add task to scheduler
pub unsafe fn sigma_sched_add_task(tcb: *const TaskControlBlock) -> i32;

/// Remove task from scheduler
pub unsafe fn sigma_sched_remove_task(tid: u64) -> i32;

/// Scheduler tick (called by timer interrupt)
pub unsafe fn sigma_sched_tick() -> u64;

/// Yield current task
pub unsafe fn sigma_sched_yield() -> u64;

/// Get current task ID
pub unsafe fn sigma_sched_get_current() -> u64;

/// Get task count
pub unsafe fn sigma_sched_get_count() -> usize;

/// Get total context switches
pub unsafe fn sigma_sched_get_switches() -> u64;
```

## Security API

### Capability System

```rust
// kernel/security/sigma_capability.rs

/// Check if process has capability
pub unsafe fn sigma_cap_check(pid: u64, cap: u64) -> i32;

/// Grant capability to process
pub unsafe fn sigma_cap_grant(pid: u64, cap: u64) -> i32;

/// Revoke capability from process
pub unsafe fn sigma_cap_revoke(pid: u64, cap: u64) -> i32;

/// Get process capability set
pub unsafe fn sigma_cap_get(pid: u64, cap_set: *mut CapabilitySet) -> i32;
```

### Capability Table

```rust
// kernel/core/capability_table.rs

/// Insert root capability
pub unsafe fn sigma_cap_table_insert_root(cap: Capability) -> u64;

/// Derive child capability
pub unsafe fn sigma_cap_table_derive(parent: u64, rights: u64) -> u64;

/// Revoke capability and descendants
pub unsafe fn sigma_cap_table_revoke(cap: u64) -> i32;
```

## Hardware Abstraction API

### APIC/PIC

```rust
// kernel/core/hal/sigma_pic.rs

/// Initialize PIC
pub unsafe fn sigma_pic_init() -> i32;

/// Initialize APIC
pub unsafe fn sigma_apic_init() -> i32;

/// Enable IRQ
pub unsafe fn sigma_irq_enable(irq: u8);

/// Disable IRQ
pub unsafe fn sigma_irq_disable(irq: u8);
```

### Timer

```rust
// kernel/core/hal/sigma_timer.rs

/// Initialize HPET
pub unsafe fn sigma_hpet_init() -> i32;

/// Initialize APIC timer
pub unsafe fn sigma_apic_timer_init(frequency: u64) -> i32;

/// Get current time
pub unsafe fn sigma_get_time() -> u64;
```

### Framebuffer

```rust
// kernel/gfx/sigma_framebuffer.rs

/// Initialize framebuffer
pub unsafe fn sigma_fb_init(info: *const FramebufferInfo) -> i32;

/// Put pixel
pub unsafe fn sigma_fb_putpixel(x: u32, y: u32, color: u32);

/// Get pixel
pub unsafe fn sigma_fb_getpixel(x: u32, y: u32) -> u32;

/// Fill rectangle
pub unsafe fn sigma_fb_fill_rect(x: u32, y: u32, w: u32, h: u32, color: u32);

/// Clear screen
pub unsafe fn sigma_fb_clear(color: u32);

/// Print character
pub unsafe fn sigma_fb_putchar(c: u8, x: u32, y: u32, color: u32);

/// Print string
pub unsafe fn sigma_fb_puts(s: *const u8, x: u32, y: u32, color: u32);
```

## Standard Library API

### Sigma Libc

```rust
// lib/sigma_libc/sigma_libc.rs

/// Memory allocation
pub unsafe fn sigma_malloc(size: usize) -> *mut u8;
pub unsafe fn sigma_free(ptr: *mut u8);

/// String operations
pub unsafe fn sigma_strlen(s: *const u8) -> usize;
pub unsafe fn sigma_strcpy(dst: *mut u8, src: *const u8) -> *mut u8;
pub unsafe fn sigma_strcmp(s1: *const u8, s2: *const u8) -> i32;

/// File I/O
pub unsafe fn sigma_open(path: *const u8, flags: i32) -> i32;
pub unsafe fn sigma_read(fd: i32, buf: *mut u8, count: usize) -> isize;
pub unsafe fn sigma_write(fd: i32, buf: *const u8, count: usize) -> isize;
pub unsafe fn sigma_close(fd: i32) -> i32;

/// Process management
pub unsafe fn sigma_exit(status: i32) -> !;
pub unsafe fn sigma_fork() -> i32;
```

### Zero-Allocation API

```rust
// kernel/core/sigma_zero_alloc.rs

/// Fixed-size string
pub struct ZeroAllocString<const N: usize>;

/// String comparison
pub unsafe fn sigma_strcmp(s1: *const u8, s2: *const u8) -> i32;

/// String length
pub unsafe fn sigma_strlen(s: *const u8) -> usize;

/// Memory copy
pub unsafe fn sigma_memcpy(dst: *mut u8, src: *const u8, n: usize);

/// Formatted output (no allocation)
pub unsafe fn sigma_sprintf(buf: *mut u8, fmt: *const u8, args: VaList) -> i32;
```

## Feature Flags API

```rust
// tools/feature_flags/sigma_features.rs

/// Initialize feature flags
pub unsafe fn sigma_features_init() -> i32;

/// Add feature
pub unsafe fn sigma_feature_add(name: *const u8, flags: u64) -> i32;

/// Enable feature
pub unsafe fn sigma_feature_enable(name: *const u8) -> i32;

/// Disable feature
pub unsafe fn sigma_feature_disable(name: *const u8) -> i32;

/// Check if feature enabled
pub unsafe fn sigma_feature_is_enabled(name: *const u8) -> i32;
```

## Init System API

### Sigma Init

```rust
// init/sigma_init.rs

/// Initialize init system
pub unsafe fn sigma_init_init() -> i32;

/// Start service
pub unsafe fn sigma_init_start_service(name: *const u8) -> i32;

/// Stop service
pub unsafe fn sigma_init_stop_service(name: *const u8) -> i32;

/// Restart service
pub unsafe fn sigma_init_restart_service(name: *const u8) -> i32;

/// Get service status
pub unsafe fn sigma_init_service_status(name: *const u8) -> ServiceState;
```

### Init Abstraction

```rust
// kernel/init/init_abstraction.rs

/// Set init system
pub unsafe fn sigma_init_set_system(system: InitSystemType) -> i32;

/// Get current init system
pub unsafe fn sigma_init_get_system() -> InitSystemType;
```

## Office Tools API

### Word Processor

```rust
// applications/wordprocessor/sigma_wordprocessor.rs

/// Create new document
pub fn sigma_wordprocessor_new() -> Document;

/// Add paragraph
pub fn sigma_wordprocessor_add_paragraph(doc: &mut Document);

/// Add text
pub fn sigma_wordprocessor_add_text(doc: &mut Document, text: &str);

/// Set formatting
pub fn sigma_wordprocessor_set_format(doc: &mut Document, format: TextFormat);

/// Export document
pub fn sigma_wordprocessor_export(doc: &Document, format: ExportFormat) -> Result<Vec<u8>, Error>;
```

### Spreadsheet

```rust
// applications/spreadsheet/sigma_spreadsheet.rs

/// Create new spreadsheet
pub fn sigma_spreadsheet_new() -> Spreadsheet;

/// Add worksheet
pub fn sigma_spreadsheet_add_sheet(ss: &mut Spreadsheet, name: &str);

/// Set cell value
pub fn sigma_spreadsheet_set_cell(ss: &mut Spreadsheet, sheet: usize, row: usize, col: usize, value: CellValue);

/// Get cell value
pub fn sigma_spreadsheet_get_cell(ss: &Spreadsheet, sheet: usize, row: usize, col: usize) -> CellValue;

/// Evaluate formula
pub fn sigma_spreadsheet_eval_formula(ss: &Spreadsheet, formula: &str) -> CellValue;

/// Export spreadsheet
pub fn sigma_spreadsheet_export(ss: &Spreadsheet, format: ExportFormat) -> Result<Vec<u8>, Error>;
```

### Presentation

```rust
// applications/presentation/sigma_presentation.rs

/// Create new presentation
pub fn sigma_presentation_new() -> Presentation;

/// Add slide
pub fn sigma_presentation_add_slide(pres: &mut Presentation);

/// Add element
pub fn sigma_presentation_add_element(slide: &mut Slide, element: Element);

/// Set transition
pub fn sigma_presentation_set_transition(slide: &mut Slide, transition: TransitionType);

/// Export presentation
pub fn sigma_presentation_export(pres: &Presentation, format: ExportFormat) -> Result<Vec<u8>, Error>;
```

## Error Codes

```rust
pub enum SigmaError {
    Success = 0,
    InvalidArgument = -1,
    OutOfMemory = -2,
    PermissionDenied = -3,
    NotFound = -4,
    AlreadyExists = -5,
    NotSupported = -6,
    Busy = -7,
    Timeout = -8,
    Interrupted = -9,
    IOError = -10,
}
```

## Constants

```rust
// Page size
pub const PAGE_SIZE: usize = 4096;

// Maximum orders
pub const MAX_ORDER: usize = 10;

// Maximum tasks
pub const MAX_TASKS: usize = 256;

// Maximum frames
pub const MAX_FRAMES: usize = 262144;

// Default time slice
pub const DEFAULT_TIME_SLICE: u64 = 10;
```

## Usage Examples

### Hello World

```rust
#![no_std]
#![no_main]

use sigma_libc::{sigma_write, sigma_exit};

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let msg = b"Hello, SigmaOS!\n";
    unsafe {
        sigma_write(1, msg.as_ptr(), msg.len());
        sigma_exit(0);
    }
    0
}
```

### Memory Allocation

```rust
use sigma_libc::{sigma_malloc, sigma_free};

unsafe {
    let ptr = sigma_malloc(1024);
    if !ptr.is_null() {
        // Use memory
        sigma_free(ptr);
    }
}
```

### Task Creation

```rust
use sigma_scheduler::{TaskControlBlock, sigma_sched_add_task};

unsafe {
    let tcb = TaskControlBlock {
        tid: 1,
        // ... other fields
    };
    sigma_sched_add_task(&tcb);
}
```

## See Also

- [Architecture Documentation](./ARCHITECTURE.md)
- [Build Guide](./BUILD.md)
- [Security Documentation](./SECURITY.md)
