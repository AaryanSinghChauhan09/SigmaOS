# Comparative Analysis: Memory Management

| Metric / Feature | Linux | xv6 | Redox | SigmaOS Target |
|---|---|---|---|---|
| Physical Allocator | Buddy System | Freelist | Bitmap / Buddy | Bitmap + Buddy hybrid |
| Virtual Memory | 4/5-level Paging | 3-level Paging | 4-level Paging | 4-level Paging (x86_64) |
| Kernel Heap | Slab / Slub | `kalloc` | Ralloc | Sovereign Slab Allocator |
