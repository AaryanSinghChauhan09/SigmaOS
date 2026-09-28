# SigmaOS Phase 1: Boot and Execution Foundation Specification

## Executive Summary

This specification outlines the technical design, architectural requirements, acceptance criteria, and implementation plan for **Phase 1: Boot and Execution Foundation** in SigmaOS. Drawing design inspirations from Linux, Limine, Redox OS, xv6, seL4, and Fuchsia, Phase 1 establishes the core Ring 3 user mode boundary, process & executable lifecycle, and SMP / CPU-local state primitives required for a production-ready sovereign operating system kernel.

---

## 1. Architectural Inspirations

| System / Protocol | Architectural Inspiration & Concepts Absorbed |
| :--- | :--- |
| **Linux Kernel** | GDT/TSS 64-bit layout, `entry_64.S` fast `syscall`/`sysret` path, `copy_from_user`/`copy_to_user` fault validation, KPTI shadow page tables, per-CPU `gs_base` data structures. |
| **Limine Boot Protocol** | High-half kernel mapping, Limine SMP bringup protocol, framebuffer & ACPI/RSDP memory tags. |
| **Redox OS** | Userland driver isolation, microkernel Scheme interface, clean event-driven IPC channels. |
| **xv6** | Simple trapframe context saving/restoring, clear PCB process state transitions (`RUNNABLE`, `RUNNING`, `SLEEPING`, `ZOMBIE`). |
| **seL4 & Fuchsia** | Capability-based object handles, strict kernel/user page permission enforcement, formal fault containment. |

---

## 2. Subsystem Components & Specifications

### 2.1 Ring 3 User Mode (`src/kernel/tss_ring3_user_mode.rs` & `src/arch/x86_64/`)

#### 2.1.1 Memory Segments & TSS Setup
- **Global Descriptor Table (GDT)**:
  - Kernel Code (`0x08`, Ring 0, `CS`)
  - Kernel Data (`0x10`, Ring 0, `SS`/`DS`)
  - User Code 64-bit (`0x2B`, Ring 3, `CS` with RPL=3)
  - User Data (`0x33`, Ring 3, `SS` with RPL=3)
  - 64-bit Task State Segment (`TSS`, 16-byte GDT descriptor)
- **Task State Segment (TSS)**:
  - Configures `rsp0` for kernel stack switching on privilege transitions (Ring 3 $\to$ Ring 0 interrupts/exceptions).
  - Configures Interrupt Stack Table (`IST1`..`IST7`) for double faults and NMI handlers.

#### 2.1.2 Fast Syscall Boundary & Exception Handling
- `MSR_STAR` (`0xC0000081`): Sets kernel CS (`0x08`) and user CS (`0x23` for sysret).
- `MSR_LSTAR` (`0xC0000082`): Entry point for `syscall` instruction.
- `MSR_SFMASK` (`0xC0000084`): RFLAGS bitmask cleared on syscall entry (IF=0).
- **Copy-In / Copy-Out Pointer Validation**:
  - Validates user pointers lie strictly below canonical user boundary (`0x0000_7FFF_FFFF_FFFF`).
  - Uses `stac`/`clac` (SMAP) and kernel page table fault handling to return `EFAULT` on invalid user memory accesses.

#### Acceptance Criteria
```text
✔ A user process executes outside Ring 0.
✔ Invalid user pointers return an error (EFAULT).
✔ A user process cannot modify kernel pages.
✔ A user fault terminates only that process.
```

---

### 2.2 Process & Executable Lifecycle (`src/process/`)

#### 2.2.1 Lifecycle Primitives
- `fork` / `clone`: Duplicates address space using Copy-On-Write (COW) page tables and assigns unique PID.
- `execve`: Parses 64-bit ELF headers (`src/process/elf_loader.rs`), maps PT_LOAD segments, sets up user stack with `argc`/`argv`/`envp`, and transfers control to `entry_point`.
- **File Descriptor Table**: Inherits open file descriptors across `fork`, supports `O_CLOEXEC` flags.
- **Process Termination & Waiting**: `exit(status)` transitions process state to `ZOMBIE`, re-parents orphan processes to `init` (PID 1), and notifies parent via `SIGCHLD` and `waitpid`.
- **Initial User Space (`init`)**: PID 1 `init` binary spawned from initial ramdisk to manage boot target runlevels.

---

### 2.3 SMP and CPU-Local State (`src/kernel/processor_management.rs`)

#### 2.3.1 Multiprocessing Architecture
- **Application Processor (AP) Bringup**:
  - Bootstraps secondary cores using ACPI MADT / Local APIC startup IPIs (INIT-SIPI-SIPI sequence).
- **Per-CPU Data (`gs_base`)**:
  - Stores CPU ID, current task PCB pointer, kernel stack top, and CPU-local scheduler runqueue pointers.
- **Inter-Processor Interrupts (IPIs)**:
  - Supports `IpiMessageType`: `TlbShootdown`, `SchedulerReschedule`, `FunctionCall`, `PanicHalt`.
- **TLB Shootdown Engine**:
  - Synchronizes page table invalidation across multi-core SMP topology on page unmapping or COW splits.

---

## 3. Milestone & Verification Matrix

| Subsystem | Target File | Primary Verification Tool |
| :--- | :--- | :--- |
| **Ring 3 User Mode & TSS** | `src/kernel/tss_ring3_user_mode.rs` | `cargo test --lib` |
| **ELF Loader & Process Lifecycle** | `src/process/elf_loader.rs` | `cargo test --lib` |
| **SMP & Processor Management** | `src/kernel/processor_management.rs` | `cargo test --lib` |
| **Zenith Desktop Desktop Integration** | `zenith_desktop/index.js` | `node tests/test_command_palette.js` |

---
