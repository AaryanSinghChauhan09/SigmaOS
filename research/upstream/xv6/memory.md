# Upstream Study: xv6 Memory Management

1. **Upstream Problem**: Physical page allocation and 2-level paging for isolated process address spaces.
2. **Data Structure & Control Flow**: Linked list of free pages (`kmem`), page tables (`pde_t`, `pte_t`), `kalloc()` / `kfree()`.
3. **Security & Failure Assumptions**: Page fault triggers panic or process termination; user/supervisor bit protection.
4. **Direct Adoption**: Free page frame allocator and basic page table mapping rules.
5. **Rust/SigmaOS Adaptation**: Type-safe 4-level PML4 frame allocator using safe Rust `BitMap` and page guards.
6. **Exit Test Criteria**: Kernel allocates page frames, configures PML4 tables, and handles page faults without panic.
7. **Licensing & Attribution**: MIT License (xv6 / MIT OS Group).
