# Kernel Architecture

The SigmaOS kernel (`vmlinuz-sigma`) is a freestanding x86_64 binary — no glibc, no hosted stdlib headers. All runtime support comes from `klib/`.

---

## Design Goals

- **Freestanding**: `-nostdlib -ffreestanding`. No glibc symbols in the output.
- **Modular shards**: Each subsystem is isolated and communicates through well-defined interfaces.
- **Zero static global state**: All active shards use Meyer singletons (`SigmaOS::SovereignEngine`).
- **Direct hardware**: Architecture-specific code lives in `arch/x86_64/`; no HAL tax in hot paths.
- **Honest stub tracking**: The Makefile emits build-time warnings for unimplemented subsystems (Buildroot BR2_BROKEN pattern). Release builds fail if stubs are enabled.

---

## Subsystems

### Interrupt Descriptor Table (IDT)

Initialized by `sigma_idt_init()` before interrupts are enabled. Registers DPL=0 interrupt gate stubs for CPU exception vectors 0–31 and hardware IRQ vectors 32+. Without a valid IDT, any exception causes a triple-fault CPU reset.

### Scheduler

**MLFQ + Round-Robin** (default): 4 priority levels; new tasks start at level 0; CPU-bound tasks sink; interactive tasks stay high; periodic boost every 50ms prevents starvation.

**SCHED_SOVEREIGN** (real-time, `release/rtos`): Tasks with priority ≥ 80 are promoted to the hard real-time class:
- Earliest Deadline First (EDF) scheduling within the RT queue
- Priority inheritance via `SovereignMutex` — no unbounded priority inversion
- Deadline miss detection with audit log entries
- Runtime tunables: `sigma-sysctl kernel.sched.rt_threshold` / `kernel.sched.rt_timeslice_us`

Source: `kernel/core/sched/sigma_sched_sovereign.cpp`

### Memory Manager (VMM + PMM)

**PMM**: QBMP bitmap allocator over the `e820` physical memory map. 4 KB frames, 8-byte aligned, O(1) amortised with `TZCNT`.

**VMM**: 4-level paging (PML4→PDPT→PD→PT). Kernel at higher half (`0xFFFFFFFF80000000+`). Each process gets its own PML4 root.

**ASLR** (`kernel/mm/sigma_aslr.cpp`): 42-bit per-region entropy on x86_64. Every `exec()` randomises stack, heap, mmap, and vDSO bases independently. W^X enforcement: `PROT_WRITE|PROT_EXEC` is denied with `-EPERM`.

**CoW pages**: Fork is O(1) — child shares parent's page tables with W bits cleared. Physical copy on first write.

### DTrace-style Kernel Tracing

**Inspired by: illumos DTrace SDT**

`SIGMA_PROBE(provider, name, ...)` is a zero-cost NOP when `SIGMA_TRACING_ENABLED` is not defined. When enabled, probes fire through a runtime gate — only active probes have any cost.

```c
// In kernel/net/sigma_tcp_stack.cpp
SIGMA_PROBE(tcp, connect__start, dst_ip, dst_port);
// ...
SIGMA_PROBE(tcp, connect__done, dst_ip, dst_port, rc);

// In kernel/security/sigma_zerotrust.cpp
SIGMA_PROBE(zerotrust, flow__decision, src_pid, dst_pid, decision);
```

CLI:
```bash
sigma-traced 'tcp:connect__start { printf("%s:%d\n", ip(arg0), arg1); }'
sigma-traced 'zerotrust:flow__decision { @[arg2] = count(); }'
```

Source: `klib/sigma_trace.cpp` / `klib/include/sigma_trace.h`

### Virtual File System (VFS)

All filesystem operations go through the VFS layer. Drivers (Ext4, FAT32) register `read`/`write` callbacks on mount. User processes call `sigma_read`/`sigma_write` syscalls; the VFS routes without knowing the driver.

```c
typedef struct vfs_node {
    char name[128];
    sigma_u32 inode_id;
    sigma_size_t size;
    sigma_u32 flags;
    sigma_i32 (*read)(struct vfs_node*, void*, sigma_size_t, sigma_u64);
    sigma_i32 (*write)(struct vfs_node*, const void*, sigma_size_t, sigma_u64);
} vfs_node_t;
```

### TCP/IP Stack

Custom implementation — no lwIP. Loopback NIC (`127.0.0.1`), full 3-way handshake, retransmission timer, UDP, local DNS resolver. Firewall (`sigma_shield`) evaluates rules against actual packet 5-tuples — no mocked data.

### Init System (PID 1)

Dependency-aware service manager with topological sort. **PID 1 must never exit** — the infinite `signalfd` event loop in `sigma_init_loop.c` reaps zombie children (SIGCHLD), restarts failed services (up to 3 retries), and handles SIGTERM for clean shutdown.

Optional: **`SIGMA_READONLY_ROOT=1`** (Bottlerocket-inspired) remounts `/` read-only before starting any service — attacker code execution cannot persist across reboots.

### Syscall Dispatcher

O(1) dispatch via direct function pointer table indexed by syscall ID. Invalid IDs return `-ENOSYS` immediately. Every entry point calls `sigma_pledge_check()` to enforce the calling process's promise set.

---

## Build Flags

```cmake
-ffreestanding -nostdinc -nostdlib
-fno-stack-protector   # kernel manages its own stack
-mno-red-zone
-mcmodel=kernel
-Wall -Wextra -Werror
```

The `SIGMA_BROKEN_SUBSYSTEMS` list in the Makefile emits warnings for every known stub. Release builds (`SIGMA_RELEASE_BUILD=1`) fail if any stubs are enabled.

---

## Known Stubs (tracked)

| Subsystem | Status | Issue |
|---|---|---|
| `sigma-jail` | Stub → **fixed** (Round 3) | Real namespace isolation via `sigma_namespace.cpp` |
| `sigma-mac` | Stub → **partially fixed** | AVC cache added; policy evaluation still basic |
| `sigma-cryptfs` | **STUB** | `derive_key()` writes zero bytes — filesystem NOT encrypted |
| `kernel/core/*.cpp` | **Missing** | Scheduler/MM/syscall source files not committed |

The `sigmad/healthd` daemon surfaces all stubs at runtime — run `sigmactl health` to see current status.

---

*See also: [HAL](HAL) · [Security Model](Security-Model) · [Building from Source](Building-from-Source) · [Performance Architecture](Performance-Architecture)*
