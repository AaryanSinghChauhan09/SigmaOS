# SigmaOS Kernel Internals

The SigmaOS kernel is a freestanding microkernel — compiled with `-nostdlib -ffreestanding`, zero glibc symbols, runs the same binary on x86_64, ARM64, and RISC-V.

---

## Kernel Source Layout

```
kernel/
├── core/           # Scheduler, MM, syscall dispatch, IRQ
├── arch/           # CPU-specific: x86_64, arm64, riscv64
├── boot/           # Early boot, multiboot2, UEFI stub
├── bpf/            # eBPF VM + verifier
├── crypto/         # In-kernel PQC accelerator hooks
├── diagnostics/    # Crash reporter, panic handler
├── drivers/        # Core built-in drivers (e1000, VirtIO)
├── fs/             # VFS core, UBC (Unified Buffer Cache)
├── hal/            # SovereignHAL bridge
├── hypervisor/     # KVM-like hypervisor stub
├── ipc/            # sigma-bus IPC, shared memory
├── kpatch/         # Live kernel patching
├── memory/         # Buddy + Slab allocators, VMM
├── mm/             # Page table walker, ASLR engine
├── net/            # Network stack core
├── orchestration/  # Cgroup + namespace enforcement
├── power/          # ACPI, P-state governor
├── recovery/       # Self-healing, rollback
├── resilience/     # Watchdog, fault tolerance
├── rust/           # Rust kernel modules (safe code paths)
├── sched/          # Scheduler implementations
├── security/       # AVC, pledge/unveil, zero-trust
├── self_healing/   # Autonomous fault recovery
├── shards/         # Kernel-resident shard loader
├── shell/          # sigma-ksh emergency kernel shell
├── storage/        # Block layer, I/O scheduler
├── syscalls/       # Syscall table + dispatch
├── telemetry/      # Kernel telemetry hooks
└── virt/           # Virtualisation support
```

---

## Boot Sequence

```
UEFI firmware
  └── sigma-boot.efi (Phase G)  ← loads kernel ELF
        └── multiboot2 header   ← sets up identity-mapped page tables
              └── sigma_kernel_main()
                    ├── HAL init (PCI, ACPI, APIC)
                    ├── Physical MM init (buddy allocator)
                    ├── Virtual MM init (4-level paging, ASLR)
                    ├── IDT + APIC/HPET timer
                    ├── Scheduler init (MLFQ)
                    ├── Syscall gate (LSTAR MSR)
                    ├── VFS mount (tmpfs root)
                    ├── Init process (PID 1 — sigma_init)
                    └── Idle loop
```

---

## Scheduler

| Mode | Algorithm | Use Case |
|------|-----------|---------|
| Normal | MLFQ (4 queues, aging) | Interactive + background |
| Fair | CFS clone (vruntime, RB tree) | CPU-bound batch work |
| RT | EDF (earliest-deadline-first) | `release/rtos` tasks |
| AI | TinyLlama pre-warming | Phase H — predictive |

### Phase G Implementation Order
1. Round-robin (64 tasks) — unblocks boot
2. MLFQ with 4 queues
3. CFS vruntime + red-black tree
4. NUMA-aware placement (ACPI SRAT)
5. EDF + priority inheritance
6. sigma-ai predictive scheduler

---

## Memory Management

### Physical (`kernel/memory/`, `kernel/core/sigma_mm.cpp`)
- **Buddy allocator**: 2^n contiguous page frames, O(log n) alloc/free
- **Slab allocator**: per-type object caches, `kmalloc()`/`kfree()`
- Physical page bitmap for 4 GB addressable RAM (Phase G baseline)

### Virtual (`kernel/mm/sigma_vmm.cpp`)
- **4-level paging** (PML4 → PDPT → PD → PT on x86_64)
- Per-process page tables, kernel mapped in upper half
- **ASLR**: 42-bit entropy per VMA region (`/proc/sys/kernel/randomize_va_space` equivalent)
- **W^X**: no page is `PROT_WRITE | PROT_EXEC` simultaneously

---

## Syscall Dispatch (`kernel/syscalls/`, `kernel/core/sigma_syscall_dispatch.cpp`)

Phase G target: 30 essential syscalls via `syscall` instruction (LSTAR MSR).

| ID | Name | Description |
|----|------|-------------|
| 1 | `sys_write` | Write to fd |
| 2 | `sys_read` | Read from fd |
| 3 | `sys_open` | Open file |
| 4 | `sys_close` | Close fd |
| 5 | `sys_exit` | Terminate process |
| 6 | `sys_fork` | Fork process |
| 7 | `sys_execve` | Execute program |
| 8 | `sys_mmap` | Map memory |
| 9 | `sys_munmap` | Unmap memory |
| 10 | `sys_socket` | Create socket |
| 11 | `sys_pledge` | Restrict capabilities |
| 12 | `sys_unveil` | Restrict filesystem |
| 13 | `sys_sigaction` | Signal handler |
| 14 | `sys_kill` | Send signal |
| 15 | `sys_waitpid` | Wait for child |
| ... | (30 total) | ... |

---

## IPC (`kernel/ipc/`)

- **sigma-bus**: capability-gated message-passing, zero-copy where possible
- **Shared memory**: `sys_mmap` with `MAP_SHARED` + VMA sharing
- **Signals**: POSIX-compatible signal delivery

---

## Security Enforcement (`kernel/security/`)

- `sigma_pledge`: process declares capability set at exec time
- `sigma_unveil`: process declares allowed filesystem paths
- **AVC**: O(1) access vector cache for MAC policy decisions
- **Namespace isolation**: PID, mount, network, UTS, IPC, user namespaces
- **Cgroup v2**: CPU, memory, I/O resource limits per container

---

## Current Status (v15.0 baseline)

| Subsystem | Status |
|-----------|--------|
| Scheduler (stub) | 🔄 Headers complete, bodies stubbed |
| Buddy allocator | 🔄 Partial |
| Slab allocator | 🔄 Partial |
| Page table walker | 🔄 Partial |
| APIC + timer | ⬜ Phase G |
| Syscall dispatch | ⬜ Phase G |
| sigma-boot.efi | ⬜ Phase G |
| IPC (sigma-bus) | 🔄 Framework done |
| AVC / pledge | ✅ Implemented |
| Cgroup enforcement | ✅ Implemented |
| PQC crypto | ✅ Implemented |
| eBPF VM | 🔄 Stub |

---

*See also: [Architecture-Overview](Architecture-Overview) · [HAL](HAL) · [Phase G Roadmap](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/PHASE_G_ROADMAP.md)*
