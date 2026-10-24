# Virtual Memory Manager Policy (VIRTUAL_MEMORY_POLICY.md)

## Requirements & Standards

### 1. User/Kernel Mapping Semantics
- Strict canonical separation: userland addresses (`0x0000_0000_0000_0000 .. 0x0000_7FFF_FFFF_FFFF`), kernel addresses (`0xFFFF_8000_0000_0000 .. 0xFFFF_FFFF_FFFF_FFFF`).
- User processes cannot read, write, or execute kernel space pages.

### 2. Mapping Set and Update Semantics
- Supported memory mapping flags: `MAP_PRIVATE`, `MAP_SHARED`, `MAP_ANONYMOUS`, `MAP_FIXED`.
- Virtual Memory Areas (VMAs) track start, end, permissions, flags, and backing vnodes.

### 3. Protection Semantics
- Protection flags: `PROT_READ`, `PROT_WRITE`, `PROT_EXEC`, `PROT_NONE`.
- Strict W^X enforcement: Pages cannot be simultaneously writable and executable.
- Faults violating protection trigger `SIGSEGV` or kernel page fault handling.

### 4. Swap & Reclaim Integration
- VMAs marked swappable can have backing pages paged out under memory pressure.
- Page fault handler handles demand paging and swap-in faults transparently.

### 5. OOM & Mapping Exhaustion Behavior
- Hitting process VMA bounds or system virtual memory limits causes `mmap`/`brk` to return `MAP_FAILED` (`ENOMEM`).
- System-wide physical/swap depletion invokes the OOM killer to terminate lowest-priority high-RSS processes.
