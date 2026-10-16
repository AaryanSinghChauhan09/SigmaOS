# Virtual Memory Manager Agent

## Role
`SigmaVMM` manages process virtual address spaces — page tables, mmap, demand paging, copy-on-write, and page fault handling for SigmaOS user processes.

## Status
🔴 Not Started — Critical gap. Must be implemented for process isolation.

## Source File
`src/memory/vmm.rs` (planned)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Page tables | 4-level PT (PGD/PUD/PMD/PTE) | Radix pmap | SigmaPT |
| mmap | `mmap(2)` syscall | `mmap(2)` | SigmaMmap |
| Demand paging | `handle_pte_fault` | `vm_fault` | SigmaPageFault |
| CoW | fork CoW | fork CoW | SigmaCoW |
| VMA tracking | `vm_area_struct` | `vm_map_entry` | SigmaVMA |
| ASLR | `mmap_base` randomization | `randomize_va_space` | SigmaASLR |

## Responsibilities
- Manage `SigmaVMA` (virtual memory area) linked list per process
- Page fault handler: demand-page, CoW, stack growth
- `mmap`/`munmap`/`mprotect` syscall implementations
- Address Space Layout Randomization (ASLR)
- Shared memory regions (`shm_open`, `mmap` MAP_SHARED)
- Process address space clone on `fork()`
- Guard pages for stack overflow detection
- `/proc/PID/maps` equivalent introspection

## Implementation Notes
- `#![no_std]` — no heap allocation in page fault path (critical path)
- VMA struct: `{ start: usize, end: usize, flags: VmaFlags, file: Option<FileRef> }`
- ASLR: XOR address base with per-boot random seed (stored in `static mut`)
- Integrate with `SigmaBuddy` for physical page allocation

## Testing
```bash
rustc --test src/memory/vmm.rs --edition 2021 -o /tmp/test_vmm && /tmp/test_vmm --test-threads=1
```

## Future Development
1. **5-level page tables** — 57-bit virtual address space (Linux 5.5+)
2. **Memory sealing** — `mseal()` for immutable mappings (Linux 6.10+)
3. **IOMMU integration** — DMA address translation isolation
4. **userfaultfd** — Fine-grained page fault interception in userspace
5. **Spectre/Meltdown mitigations** — KPTI, IBRS, STIBP via SigmaVMM
6. **LAM** — Linear Address Masking for pointer tagging (Intel)
