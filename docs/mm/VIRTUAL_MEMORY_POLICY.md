# Virtual Memory Policy Specification

`docs/mm/VIRTUAL_MEMORY_POLICY.md`

## Overview
The Virtual Memory Manager (`src/mm/virtual.rs`) manages Virtual Memory Areas (VMAs), process address spaces, permission enforcement, and memory isolation in SigmaOS.

## Specification

### User/Kernel Isolation
- **User Address Space:** `0x0000_0000_0000_0000` to `0x0000_7FFF_FFFF_FFFF`. User processes can only create mappings within this range.
- **Kernel Address Space:** `0xFFFF_8000_0000_0000` to `0xFFFF_FFFF_FFFF_FFFF`. User accesses to kernel virtual addresses trigger an immediate Page Fault (`#PF`) with protection violation.

### Protection Semantics (W^X)
- Enforces Write XOR Execute (W^X) policy: a VMA can be writable or executable, but never both simultaneously (`VmaFlags::WRITE` and `VmaFlags::EXECUTE` are mutually exclusive).
- Mappings created or updated with invalid permission combinations return `VirtualMemoryError::ProtectionViolation`.

### Swap & Reclaim Integration
- VMAs marked as `SWAPPABLE` can have underlying physical pages paged out to swap slots under memory pressure.
- Page fault handler catches faults on swapped-out VMAs and triggers atomic swap-in requests.

### Memory Limits & OOM Behavior
- Per-process virtual memory consumption is constrained by configured limits (`vma_limit_bytes`).
- Attempting to allocate or expand a VMA past process limits returns `VirtualMemoryError::OomLimitExceeded`.
- The system OOM killer monitors aggregate VMA allocations to select victim processes under unresolvable frame pressure.
