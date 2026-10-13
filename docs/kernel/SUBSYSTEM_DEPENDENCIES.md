# SigmaOS Subsystem Dependencies and Initialization Sequence

## Overview

SigmaOS enforces a strict, acyclic dependency initialization graph during system boot. Failure to satisfy dependency order leads to controlled kernel diagnostic halts rather than undefined behavior.

## Initialization Sequence Order

1. **Stage 0: Early Boot & Hardware Probe**
   - Serial UART Console (`src/drivers/uart.rs`)
   - Early CPU & GDT/IDT Table Initialization
2. **Stage 1: Physical & Virtual Memory Subsystem**
   - Frame Allocator (`src/memory/physical.rs`)
   - Paging & Page Tables (`src/mm/paging.rs`, `src/mm/page_table.rs`)
   - Virtual Memory Manager (`src/mm/virtual.rs`)
3. **Stage 2: Core Kernel Execution & Interrupts**
   - APIC / PIC Interrupt Controllers
   - System Timer & CPU Frequency Governors (`src/kernel/cpufreq.rs`)
   - POSIX 450 Syscall Dispatcher (`src/kernel/tier1_kernel_execution_suite.rs`)
4. **Stage 3: Process & Scheduler Subsystem**
   - Process Control Block Table (`src/process/table.rs`)
   - Bounded RunQueue & CFS Scheduler (`src/scheduler/run_queue.rs`, `src/scheduler/core.rs`)
5. **Stage 4: Storage, Filesystem & VFS**
   - Block Device Manager (`src/storage/block.rs`)
   - Inode & Directory Managers (`src/fs/inode.rs`, `src/fs/directory.rs`)
   - VFS Process Fd Table (`src/fs/fd_table.rs`)
6. **Stage 5: Userspace, Init & Security**
   - User/Group Database (`src/userland/user_management.rs`)
   - Service Manager & Init (`src/init/service_manager.rs`)
