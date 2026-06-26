# Kernel Architecture

The SigmaOS kernel (`vmlinuz-sigma`) is a freestanding x86_64 binary. It does not link against any host libc and includes no hosted standard library headers. Everything it needs is implemented in `klib/` — the sovereign C library.

---

## Design Goals

- **Freestanding**: `-nostdlib -ffreestanding`. No glibc symbols in the output binary.
- **Modular shards**: Each subsystem is an isolated "shard" that communicates through well-defined interfaces, not shared global state.
- **Sovereign Singletons**: All active driver and core system shards use Meyer singletons (`SigmaOS::SovereignEngine`) to ensure safe static initialization and eliminate global state races.
- **Direct hardware**: No HAL abstraction tax in hot paths. Architecture-specific code lives in `arch/x86_64/` and is inlined where performance matters.

---

## Subsystems

### Interrupt Descriptor Table (IDT)

Initialized by `sigma_idt_init()` early in `kmain.cpp`, before interrupts are enabled. Registers ISR stubs for CPU exception vectors 0–31 and hardware IRQ vectors 32+.

Each IDT gate is a DPL=0 interrupt gate with the kernel code segment selector. When a CPU exception fires:
1. The CPU saves registers and pushes an error code (for exceptions that have one).
2. The IDT gate transfers control to the registered ISR stub.
3. The stub saves all general-purpose registers, calls the C exception handler, then restores registers and returns via `iretq`.

Without a valid IDT, any page fault, division by zero, or invalid opcode causes a triple-fault, which resets the CPU.

---

### Scheduler (MLFQ + Round-Robin)

The SigmaOS scheduler implements a **Multi-Level Feedback Queue (MLFQ)**:

- **4 priority levels** (0 = highest, 3 = lowest).
- New tasks start at level 0.
- A task that uses its full time slice is demoted to the next level.
- A task that yields early stays at its current level.
- Periodic priority boost (every 50ms) prevents starvation.

Interactive tasks (keyboard input, shell) naturally stay in high-priority queues. CPU-bound tasks (compilers, codecs) sink to low-priority queues where they don't starve interactive work.

Round-Robin is used within each queue level to ensure fairness among tasks at the same priority.

Future: **SCHED_SOVEREIGN** — a hard real-time class with deterministic execution deadlines, planned for the IoT and industrial SigmaOS profiles.

---

### Memory Manager (VMM + PMM)

**Physical Memory Manager (PMM)**:
- Bitmap allocator over the physical memory map (from BIOS/UEFI `e820`).
- Allocates 4 KB page frames.
- `QBMP` (Quick Bitmap) with 8-byte alignment and debug assertions.

**Virtual Memory Manager (VMM)**:
- 4-level paging (PML4 → PDPT → PD → PT) for full 48-bit virtual address space.
- Kernel mapped at the higher half (`0xFFFFFFFF80000000+`).
- Each user process gets its own PML4 root, fully isolating virtual address spaces.
- On context switch, `cr3` is updated to the new PML4 physical address.

**Slab Allocator** (planned):
- Fixed-size, lockless O(1) allocator for frequently allocated kernel objects (task structs, socket descriptors, inode entries).
- Eliminates heap fragmentation from `malloc`/`free`.

---

### Virtual File System (VFS)

All filesystem operations go through the VFS layer:

```c
typedef struct vfs_node {
    char name[128];
    sigma_u32 inode_id;
    sigma_size_t size;
    sigma_u32 flags;  // FILE | DIRECTORY | BLOCK_DEV | CHAR_DEV
    sigma_i32 (*read)(struct vfs_node* node, void* buf, sigma_size_t size, sigma_u64 offset);
    sigma_i32 (*write)(struct vfs_node* node, const void* buf, sigma_size_t size, sigma_u64 offset);
} vfs_node_t;
```

Filesystem drivers (Ext4, FAT32) register their `read`/`write` callbacks on mount. User processes call `sigma_read`/`sigma_write` syscalls, which the VFS routes to the correct driver without knowing its implementation.

---

### TCP/IP Networking Stack

Custom implementation, no lwIP or Linux kernel code:

- **Loopback NIC** (`lo`, `127.0.0.1`): Virtual network interface for same-host communication.
- **TCP state machine**: Full 3-way handshake (SYN → SYN-ACK → ACK), FIN/RST handling, retransmission timer.
- **UDP**: Connectionless datagram socket binding and sending.
- **DNS resolver**: Local resolver mapping domain names to IPv4 addresses.

Socket API:

```c
// Allocate a socket
sigma_i32 net_socket(sigma_i32 domain, sigma_i32 type, sigma_i32 protocol);

// Connect (initiates TCP 3-way handshake)
sigma_i32 net_connect(sigma_i32 fd, sigma_u32 remote_ip, sigma_u16 remote_port);

// Send data
sigma_i32 net_send(sigma_i32 fd, const void* data, sigma_size_t size);

// Receive data
sigma_i32 net_recv(sigma_i32 fd, void* buf, sigma_size_t len);
```

---

### Init System (PID 1)

`sigma_init.cpp` is PID 1. It runs the boot sequence:

1. Read `/etc/sigma-services.conf` and register all declared services.
2. Start each service in priority order (Runlevels 1–5).
3. Enter an **infinite wait loop** — PID 1 must never exit. If it does, the kernel panics and halts.
4. Watch for child process exits. If a registered service exits with a non-zero code, log the failure and restart it (up to 3 times before giving up).

```cpp
// Correct PID 1 loop — never exits
for (;;) {
    sigma_init_watchdog();  // reap zombies, restart failed services
    __asm__("hlt");         // yield CPU until next interrupt
}
```

The service array is bounded at `MAX_SERVICES` (64). Attempts to register more services log an error and are rejected — no array overflow.

---

### Syscall Dispatcher

All userland→kernel communication goes through the Sovereign Syscall Dispatcher via `int 0x80` or the `syscall` instruction:

| Syscall ID | Name | Description |
|---|---|---|
| `0x01` | `sys_write` | Write buffer to debug serial / VGA |
| `0x02` | `sys_read` | Read from keyboard input queue |
| `0x05` | `sys_socket` | Allocate a network socket |
| `0x06` | `sys_pkg_install` | Install an Alpine package into user namespace |

Dispatch is O(1) via a direct function pointer table indexed by syscall ID. Invalid IDs return `-ENOSYS` immediately.

---

### Hardware Abstraction Layer (HAL)

The HAL isolates architecture-specific assembly from the kernel core:

```
hal.h / sigma_hal.h  →  generic operations:
    cpu_halt()           // execute HLT
    timer_init(hz)       // program PIT/APIC timer
    interrupt_init()     // configure APIC/PIC
    mmu_map(virt, phys, flags)  // map a page
    read_io(port)        // inb
    write_io(port, val)  // outb
```

Architecture stubs:
- `arch/x86_64/` — NASM assembly for paging, context switch, VMM fast path
- `arch/SovereignStandardHAL.asm` — standard HAL entry points

---

## Build System

The kernel is built with CMake + Ninja. Critical flags:

```cmake
target_compile_options(vmlinuz-sigma PRIVATE
    -ffreestanding
    -nostdinc
    -fno-stack-protector
    -mno-red-zone
    -Wall -Wextra -Werror
)

target_link_libraries(vmlinuz-sigma sigma_klib)

set_target_properties(vmlinuz-sigma PROPERTIES
    LINK_FLAGS "-nostdlib -z max-page-size=0x1000 -T ${CMAKE_SOURCE_DIR}/linker.ld"
)
```

These flags must **not** be commented out. Without `-nostdlib`, the kernel links against host glibc and will not boot on bare metal.

---

*See also: [Architecture Overview](Architecture-Overview) · [Building from Source](Building-from-Source) · [HAL](HAL)*
