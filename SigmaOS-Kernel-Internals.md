# 🧠 SigmaOS Kernel Internals

> **The beating heart of Sovereign Silicon.**

This page documents the core kernel subsystems that make SigmaOS tick — all implemented from scratch with zero external dependencies.

---

## Process Scheduler (`sigma_scheduler.cpp`)

**Absorbs**: Linux CFS (Completely Fair Scheduler), L4Re RTOS EDF, Minix process tables.

SigmaOS uses a **hybrid scheduler** with two scheduling classes:

| Class | Algorithm | Use Case |
|-------|-----------|----------|
| `SCHED_NORMAL` | CFS (lowest `vruntime` wins) | General userland processes |
| `SCHED_RT` | EDF (Earliest Deadline First) | RTOS branch, real-time tasks |

- **Process table**: Up to 256 concurrent processes
- **States**: `READY`, `RUNNING`, `BLOCKED`, `ZOMBIE`
- RT tasks always preempt NORMAL tasks

---

## Inter-Process Communication (`sigma_ipc.cpp`)

**Absorbs**: L4 microkernel message passing, Plan 9 channels, LMAX Disruptor ring buffers.

| Feature | Detail |
|---------|--------|
| Queue depth | 64 messages per queue |
| Max queues | 32 |
| Message size | 256 bytes |
| Mechanism | Lock-free ring buffer |
| API | `sigma_ipc_send()`, `sigma_ipc_recv()` |

---

## Interrupt Descriptor Table (`sigma_idt.cpp`)

**Absorbs**: Intel SDM Vol 3 Ch 6, Linux `arch/x86/kernel/idt.c`.

- 256 IDT entries for x86_64
- Named exception handlers (Division Error, Page Fault, GPF, etc.)
- Page fault handler reads CR2 for faulting address
- IRQ routing for PIT Timer (IRQ0), Keyboard (IRQ1), Mouse (IRQ12)
- PIC EOI sent after each IRQ

---

## Virtual Filesystem (`sigma_vfs.cpp`)

**Absorbs**: Linux VFS superblock/inode/dentry model, Plan 9 namespace binding.

- **Mount table**: 16 simultaneous mount points
- **FD table**: 256 open file descriptors
- **Path resolution**: Longest-prefix matching against mount points
- **Operations**: `open`, `read`, `write`, `close`, `opendir`, `readdir`, `set_owner`
- FS-agnostic: FAT32 and ext2 plug in via `SigmaFSOps` function pointer tables

---

## Memory Allocator (`sigma_allocator.cpp`)

**Absorbs**: Linux buddy allocator, Doug Lea's dlmalloc concepts.

- Buddy system with orders 0–11 (4KB to 8MB contiguous)
- Linked-list free lists per order
- `sigma_malloc()` / `sigma_free()` as sovereign replacements for libc

---

## Syscall Layer (`sigma_syscalls.cpp`)

Distinct from POSIX. Syscall numbers are designed to prevent accidental ABI pollution:

| Syscall | Number | Purpose |
|---------|--------|---------|
| `SIGMA_SYS_DEBUG_PRINT` | 0x01 | Route to VGA/serial |
| `SIGMA_SYS_ALLOC_MEM` | 0x02 | Allocate memory |
| `SIGMA_SYS_FREE_MEM` | 0x03 | Free memory |
| `SIGMA_SYS_SEND_MSG` | 0x04 | IPC send |
| `SIGMA_SYS_RECV_MSG` | 0x05 | IPC receive |
| `SIGMA_SYS_HW_IO` | 0x06 | Privileged hardware I/O |
