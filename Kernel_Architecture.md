# 🖥️ Kernel Architecture Deep Dive

The SigmaOS kernel is a monolithic bare-metal kernel written in **pure C11 and x86-64 Assembly** with zero standard library dependencies. This page documents all kernel subsystems.

---

## Boot Sequence

```text
BIOS/UEFI POST
    └─► boot.asm (MBR / Stage-1 Bootloader)
            └─► hal.asm (Hardware Abstraction Layer setup)
                    └─► idt.c + idt.asm (Interrupt Descriptor Table)
                            └─► pit.c (Programmable Interval Timer)
                                    └─► pmm.c (Physical Memory Manager)
                                            └─► vmm.c (Virtual Memory Manager)
                                                    └─► main.c (Kernel Entry)
                                                            └─► init.c (Subsystem Init)
                                                                    └─► omni_shell.c (Shell)
```

---

## Subsystem Map

### Memory Management

| File | Description |
| ---- | ----------- |
| `pmm.c` | Physical Memory Manager — page frame allocator using bitmap |
| `vmm.c` | Virtual Memory Manager — 4-level paging, TLB shootdown |
| `slab.c` | Slab allocator for fixed-size kernel objects |
| `quantum_rcu.c` | Read-Copy-Update for lock-free concurrent access |

### Process & Scheduling

| File | Description |
| ---- | ----------- |
| `process.c` | Process creation, forking, context switching |
| `scheduler.c` | Round-robin scheduler with priority bands |
| `scheduler_ai.c` | AI-assisted adaptive scheduler (heuristic load prediction) |
| `task_switch.asm` | Low-level register save/restore for context switch |
| `signal.c` | POSIX-compatible signal delivery |

### Filesystem (VFS)

| File | Description |
| ---- | ----------- |
| `vfs.c` | Virtual File System: inode tree, path resolution, mount points |
| `procfs.c` | `/proc`-style process info filesystem |
| `syscall.c` | System call dispatch table (200+ syscalls) |

### Hardware & Drivers

| File | Description |
| ---- | ----------- |
| `hal.asm` + `hal.c` | Hardware Abstraction Layer |
| `idt.asm` + `idt.c` | Interrupt Descriptor Table + ISR handlers |
| `pit.c` | Programmable Interval Timer (system clock) |
| `keyboard_master.c` | PS/2 keyboard scancode→ASCII driver |
| `sound_core.c` | PC speaker beep driver via I/O port 0x61 |
| `drivers/` | Extended device driver directory |

### IPC & Networking

| File | Description |
| ---- | ----------- |
| `ipc.c` | Full IPC: pipes, message queues, shared memory, semaphores |
| `net.c` | TCP/IP stack (ARP, IP, TCP, UDP) — zero external libs |
| `net_firewall.c` | Kernel-level stateful packet filter |
| `SovereignNetMesh.c` | Mesh networking + onion routing (Tails OS equivl.) |

### Security

| File | Description |
| ---- | ----------- |
| `sovereign_bpf.c` | Ring-0 eBPF sandbox for verified bytecode execution |
| `cgroup_shard.c` | CPU/Memory cgroup isolation (Linux cgroups equivalent) |
| `oom_killer.c` | Out-of-Memory heuristic sacrificer |
| `SovereignLatticePQC.c` | Post-Quantum Cryptography — LWE Lattice |
| `SovereignAmnesicShard.c` | Volatile memory forensic wiper |

### Advanced Modules

| File | Description |
| ---- | ----------- |
| `elf_loader.c` | Native ELF binary loader + relocations |
| `SovereignHypervisorZenith.c` | Type-2 Hypervisor with VM isolation |
| `hot_replace.c` | Live kernel module hot-patching engine |
| `mod_loader.c` | Dynamic kernel module loader/unloader |
| `linux_shim.c` | Thin Linux ABI compatibility shim |
| `posix_bridge.c` | POSIX syscall translation bridge |

---

## Key Data Structures

### Process Control Block (`process.c`)

```c
typedef struct SigmaProcess {
    sigma_u32     pid;
    sigma_u32     ppid;
    char          name[64];
    void*         stack_top;
    void*         page_dir;
    sigma_u32     state;       // RUNNING, SLEEPING, ZOMBIE
    sigma_i64     priority;
    sigma_u64     cpu_time_us;
} SigmaProcess_t;
```

### VFS Inode (`vfs.c`)

```c
typedef struct SigmaInode {
    sigma_u32  ino;
    sigma_u32  size;
    sigma_u32  mode;    // Permissions + type
    sigma_u64  mtime;
    void*      data;
} SigmaInode_t;
```

---

## Syscall Interface

SigmaOS uses the standard `x86-64 Linux ABI` syscall convention:

```asm
; Write "Hello" to stdout
mov rax, 1          ; SYS_WRITE
mov rdi, 1          ; fd = stdout
mov rsi, msg        ; buffer
mov rdx, 5          ; length
syscall
```

The dispatch table in `syscall.c` maps syscall numbers to internal kernel handlers.
