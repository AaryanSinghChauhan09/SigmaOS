# Virtual Memory Policy

## Core Semantics
1. **User/Kernel Isolation**: Virtual address space is divided into User Space (`< 0x0000_8000_0000_0000`) and Kernel Space (`>= 0xFFFF_8000_0000_0000`). User processes cannot map, read, or write kernel virtual memory.
2. **W^X (Write XOR Execute) Protection**: Virtual memory areas (VMAs) cannot be simultaneously writable and executable. Any attempt to map or protect a VMA with both `PROT_WRITE` and `PROT_EXEC` returns `VmaError::InvalidPermissions`.
3. **Mapping Set/Update Semantics**: VMA creations, modifications (`mprotect`), and destructions (`munmap`) maintain non-overlapping ordered VMA lists within the process address space descriptor.
4. **Swap & Reclaim Interaction**: Anonymous memory VMAs can be swapped out under high memory pressure, marking underlying page table entries with swap signatures.
5. **OOM & Limit Enforcement**: Creating or extending VMAs checks process-specific address space limits (`RLIMIT_AS`). Exceeding limits returns `VmaError::OomLimitExceeded`.
6. **Mapping Exhaustion**: Total VMAs per process are bounded (`MAX_VMAS_PER_PROCESS = 65536`) to prevent kernel structure exhaustion.
