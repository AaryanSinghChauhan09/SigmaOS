# SigmaOS Virtual Memory Management Policy

## Overview

The Virtual Memory Manager (`src/mm/virtual.rs`) governs process virtual memory areas (VMAs), mapping protection flags, user/kernel isolation, demand paging, and memory limit enforcement across SigmaOS.

---

## Core Guarantees & Semantics

1. **User/Kernel Mapping Semantics**: Kernel memory ranges (higher-half canonical addresses, e.g. `0xFFFF_8000_0000_0000`+) are strictly isolated from user virtual space (`0x0000_0000_0000_0000` - `0x0000_7FFF_FFFF_FFFF`). User processes attempting to access kernel mappings trigger protection faults.
2. **Mapping Set/Update Semantics**: VMAs support dynamic creation, mmap region expansion, split/merge operations, and protection bit modification (`PROT_READ`, `PROT_WRITE`, `PROT_EXEC`).
3. **Protection Semantics (W^X Policy)**: Strict Write XOR Execute (W^X) security policy is enforced across virtual mappings. Mappings marked executable (`PROT_EXEC`) cannot be marked writable (`PROT_WRITE`) simultaneously, preventing code injection attacks.
4. **Swap Semantics**: VMAs support transparent demand paging and backing swap file/zram pages. Pages belonging to anonymous VMAs are candidate targets for swap-out under memory pressure.
5. **Reclaim Semantics**: Background kswapd and mGLRU aging scan process VMAs to identify cold pages for swap-out or page compaction without corrupting virtual space accounting.
6. **OOM & Mapping Exhaustion Behavior**: Process VMA limits and total committed virtual memory are strictly bounded. Reaching process VMA exhaustion or system commit limits returns `VirtualMemoryError::OomLimitExceeded`, avoiding kernel panics.

---

## Implementation Specifications

- **Source Implementation**: `src/mm/virtual.rs`
- **Unit Tests**: `tests/mm/virtual.rs`

---

## Enforcement & Verification

- `cargo check --lib`
- `rustc --test --edition=2021 tests/mm/virtual.rs -o build/test_virt && ./build/test_virt`
