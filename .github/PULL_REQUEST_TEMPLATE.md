## Summary
What physical memory management (PMM), IPC, or kernel component correctness or capability gap does this PR address?

## Scope
Boot map | reservation | early allocator | runtime allocator | page metadata | VMM | DMA | zones | NUMA | IPC handler | Component

## Reference study
Upstream study / What SigmaOS adopts / Reference study

## Memory-map evidence
Source (UEFI/ACPI/bootloader/QEMU), memory types, ranges, and exact target.

## Invariants and safety
Reserved-range policy, checked arithmetic, frame states, ownership/refcounts,
zeroing, physical/virtual/DMA address separation, and failure behavior.

## Risks and rollback
Safety and rollback

## Runtime evidence
Unit/model | booted QEMU | SMP QEMU | physical hardware
List memory size, architecture, boot protocol, and exact operations tested.

## Tests
Malformed maps, overlaps/holes, allocation/free, double-free, exhaustion,
coalescing, VMM map/unmap, zeroing, DMA, and anything not tested.

## Status and documentation
Proposed | Prototype | Integrated | Supported
Link updated memory docs, feature inventory, and test evidence.

## Reviewer checklist
- [ ] Unknown/reserved memory is never allocated
- [ ] Critical kernel/boot/firmware/MMIO ranges are reserved
- [ ] Arithmetic and PFN conversions are checked
- [ ] Physical, virtual, and DMA addresses are distinct
- [ ] Allocation/free accounting and ownership are tested
- [ ] User-visible pages are zeroed before exposure
- [ ] Required CI and QEMU checks pass; review conversations resolved
