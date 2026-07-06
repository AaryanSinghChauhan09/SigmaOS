# SigmaOS Kernel Developer Handbook

> Complete reference for kernel development, from build setup to submitting patches.

---

## 1. Prerequisites

```bash

# Rust nightly (pinned in rust-toolchain.toml)

rustup show   # confirms correct toolchain

# QEMU for testing

# Windows: winget install QEMU.QEMU

# Linux:   apt install qemu-system-x86

# NASM (for boot assembly)

# Windows: winget install NASM.NASM

# Zig (for HAL + drivers)

# https://ziglang.org/download/

```

---

## 2. Build

```bash

# Full kernel build

cargo build --release --target kernel/x86_64-sigmaos.json \
    -p kernel_core

# Boot in QEMU

./qemu-boot.sh standalone

# Or with make

make iso PROFILE=standalone
make qemu PROFILE=standalone
```

---

## 3. Repository Layout

| Directory | Language | Purpose |
|-----------|----------|---------|
| `kernel/core/` | Rust `#![no_std]` | Scheduler, MM, IPC, syscalls |
| `kernel/security/` | Rust | pledge/unveil, zero-trust |
| `kernel/net/` | Rust | TCP/UDP/TLS network stack |
| `kernel/fs/` | Rust | VFS, SigmaFS, Ext4 |
| `arch/` | NASM + Zig | Boot, GDT, IDT, paging |
| `drivers/` | Rust + Zig | SDF hardware drivers |
| `hal/` | Zig + Rust | Port I/O, MMIO, PCI, ACPI |
| `kabi/` | Rust | Stable ABI boundary |
| `security/` | Ada + Rust | PQC crypto, formal verification |

---

## 4. Kernel Subsystems

### 4.1 Scheduler (`kernel/core/sigma_sched.rs`)

Three policies in one scheduler:

```rust
// Add a task
sched_add_task(pid, SCHED_MLFQ, 0, mlfq_level);
sched_add_task(pid, SCHED_EDF, deadline_ns, 0);
sched_add_task(pid, SCHED_CFS, 0, 0);

// Get next task to run (called from timer IRQ)
let next_pid = sched_tick(now_ns);
```

- **MLFQ**: 4 levels, aging every 200 ticks, interactive-friendly

- **CFS**: vruntime + min-heap, fair CPU sharing

- **EDF**: earliest deadline first, hard real-time guarantee

- **Priority**: EDF > MLFQ > CFS > idle

### 4.2 Memory Manager (`kernel/core/sigma_mm.rs`)

```rust
// Physical allocation
sigma_slab_alloc(64);   // alloc 64 bytes from slab
sigma_slab_free(ptr);   // return to slab

// Virtual memory
sigma_mm_free_pages();  // check available physical pages
```

- **Buddy allocator**: 2^n block sizes, O(log n) alloc/free

- **Slab allocator**: 8 size classes (8–1024 bytes), O(1) alloc

- **ASLR**: 42-bit entropy, randomized on every `mmap()`

- **W^X**: write+exec simultaneously denied at `mmap()` level

### 4.3 IPC (`kernel/core/ipc/SovereignIPC.rs`)

```rust
// Send to channel 0x20 (NIC RX)
send_message_zero_copy(IPC_CH_NET_RX, 0, 0, buf_ptr, len);

// Receive from channel
recv_message(IPC_CH_NET_RX, out_buf);
```

32 channels × 256-slot lock-free SPSC rings.

### 4.4 Syscall Dispatch (`kernel/core/syscall_dispatch.rs`)

```rust
// All syscalls routed through one function:
sigma_syscall_dispatch(nr, a1, a2, a3, a4, a5, a6) -> i64

// Custom SigmaOS syscalls:
SYS_SIGMA_PLEDGE    = 400  // restrict capabilities
SYS_SIGMA_UNVEIL    = 401  // restrict filesystem access
SYS_SIGMA_ATTEST    = 402  // PQC attestation
SYS_SIGMA_BUS_SEND  = 403  // sigma-bus IPC
```

---

## 5. Security Model

Every process must call `sigma_pledge()` before doing sensitive operations:

```c
// Rust (no_std)
extern "C" { fn sigma_pledge(p: *const u8, l: usize) -> i32; }
sigma_pledge(b"stdio rpath inet\0".as_ptr(), 16);
```

After pledge, the kernel enforces:

- Only declared syscalls allowed

- `sigma_unveil()` limits filesystem access to declared paths

- Violations: SIGKILL + audit log entry

---

## 6. Driver Development

See [Driver Framework](../wiki_repo/Driver-Framework.md) for full guide.

Quick skeleton:

```rust
// 1. Implement lifecycle functions
#[no_mangle] pub extern "C" fn mydrv_probe(bar: u64, irq: u8) -> i32 { 0 }
#[no_mangle] pub extern "C" fn mydrv_init() -> i32 { 0 }
#[no_mangle] pub extern "C" fn mydrv_shutdown() {}
#[no_mangle] pub extern "C" fn mydrv_irq() -> bool { false }

// 2. Register with SDF
sigma_register_driver!(SigmaDriverDescriptor {
    magic:       SIGMA_DDK_MAGIC,
    abi_version: DDK_ABI_VERSION,
    vendor_id:   0x8086,
    device_id:   0x100E,
    ring:        3,  // ring-3 isolated
    fn_probe:    Some(mydrv_probe),
    fn_init:     Some(mydrv_init),
    fn_shutdown: Some(mydrv_shutdown),
    fn_irq:      Some(mydrv_irq),
    ..Default::default()
});
```

---

## 7. Testing

```bash

# Unit tests (no hardware needed)

cargo test -p kernel_core

# QEMU boot smoke

./qemu-boot.sh standalone

# Run specific test

cargo test -p kernel_core sched_mlfq_aging

# Add a QEMU integration test

# 1. Add script to tests/kernel/

# 2. Wire to .github/workflows/ci.yml

```

---

## 8. Submitting a Patch

1. Branch from `main`: `git checkout -b feat/my-improvement`

2. Write the code + tests

3. Run: `cargo build --release && ./qemu-boot.sh standalone`

4. Update wiki page in `wiki_repo/` if adding a new subsystem

5. Open a PR — CI must be green

6. PR description must include: what changed, QEMU smoke result, any new syscalls added

---

## 9. Coding Standards

- All kernel code: `#![no_std]`, no third-party crates, no `alloc` (use slab)

- No `unsafe` without a `// SAFETY:` comment explaining why it's sound

- Every `extern "C"` export must have a corresponding C header declaration

- No `{s_name}` template placeholders left in code (check before PR)

- W^X must be respected: never `mmap` with both WRITE and EXEC

- `sigma_pledge()` must be called at the start of every new process

---

*See also: [Architecture.md](../Architecture.md) · [Driver Framework](../wiki_repo/Driver-Framework.md) · [Kernel ABI Stability](../wiki_repo/Kernel-ABI-Stability.md)*
