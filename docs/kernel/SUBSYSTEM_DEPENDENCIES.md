# Kernel Subsystem Dependency Graph

This document details the exact initialization order, dependency graph, and cycle prevention rules for the SigmaOS kernel boot sequence.

## Initialization Sequence Flow

```
[Stage 0: Firmware Entry]
        │
        ▼
[Physical Allocator] ─── (Allocates raw page frames)
        │
        ▼
     [Paging]        ─── (Maps 4-level x86_64 page tables & W^X protection)
        │
        ├───► [Process Manager] ─── (PID allocator & PCB table)
        │            │
        │            ▼
        └───► [Syscall Dispatcher]
                     │
        ┌────────────┴──────────┐
        ▼                       ▼
 [Storage Block] ───► [Virtual File System (VFS)]
```

---

## Detailed Subsystem Dependency Map

| Initialization Order | Subsystem | Direct Dependencies | Initialized In Module | Test Verification File |
|---|---|---|---|---|
| **1** | Physical Allocator | None | `src/memory/physical.rs` | `tests/memory/physical.rs` |
| **2** | Paging | Physical Allocator | `src/mm/paging.rs` | `tests/mm/paging.rs` |
| **3** | Process Manager | Paging | `src/process/table.rs` | `tests/process/pid_allocator.rs` |
| **4** | Syscall Dispatcher | Paging, Process Manager | `src/kernel/syscalls.rs` | `tests/kernel/integration/subsystem_contracts_test.rs` |
| **5** | Storage Block | Paging | `src/storage/block.rs` | `tests/storage/block.rs` |
| **6** | VFS | Storage Block, Syscall Dispatcher | `src/filesystem/core.rs` | `tests/filesystem/core.rs` |

---

## Topological Sort Guarantee

Topological sorting is verified at test time via `tests/kernel/integration/subsystem_contracts_test.rs`. If a circular dependency is introduced into the kernel initialization sequence, the contract engine detects the loop and halts compilation with an explicit error message.
