# SigmaOS Physical Memory Manager (PMM) Master Architecture, Support Matrix, and PR Plan

SigmaOS’s memory-management subsystem relies on physical frame allocation to identify usable physical memory pages, reserve non-allocatable memory ranges, and provide frames to the virtual-memory manager (VMM) and DMA layer. Code presence and unit tests in userland or kernel modules do not establish hardware bootability or kernel integration without explicit runtime verification in target environments (e.g. QEMU x86_64).

## Architecture Overview and Two-Stage Memory Management

SigmaOS adopts the industry-standard **two-stage physical memory management** pattern:
1. **Boot-Time Early Allocator (`memblock` equivalent):** Parses firmware/bootloader memory maps (UEFI, ACPI, Multiboot2, DTB), tracks usable RAM and boot reservations (kernel image, initrd, page tables, MMIO, framebuffer, ACPI tables), and satisfies early page allocation requests before main runtime page metadata and paging structures are online.
2. **Runtime Page-Frame Allocator:** Once physical page descriptors and page tables are initialized, remaining usable physical pages are transferred to the primary frame allocator (e.g., conservative bitmap allocator or buddy allocator with power-of-two page orders) for long-term kernel and user frame management.

---

## PMM Rules and Invariants

1. **Only explicitly usable RAM is allocatable:** The system defaults all physical address space to "reserved" and marks only validated usable regions as free. Reserved areas include kernel image, bootloader data, page tables, firmware tables, MMIO windows, framebuffers, crash logs, and unknown memory types (e.g. ACPI NVS, reserved, unusable, unaccepted).
2. **Keep physical and virtual addresses distinct:** Physical addresses (`PhysAddr`, PFN) are typed distinct entities and cannot be directly dereferenced as pointers. CPU access requires explicit VMM virtual mappings.
3. **Use checked arithmetic everywhere:** All calculations (`base + size`, PFN conversions, page alignment checks, order bounds) must use checked arithmetic (`checked_add`, `checked_mul`) to reject overflow, wraparound, out-of-bounds access, and overlapping regions.
4. **Define ownership and page states:** Every physical frame is in exactly one state (`Reserved`, `Free`, `Allocated`, `Wired`/`Pinned`, `Device`/`Unmanaged`). Every state transition must have a single clear owner and verified lifetime/reference counting rules.
5. **Keep PMM and VMM responsibilities separate:** PMM manages physical frame allocation and accounting; VMM handles address space mappings, page table walking, and permissions. Neither silently performs the other's duties.
6. **Zero memory before exposing it to less-trusted consumers:** User-space allocations and shared buffers must be scrubbed/zeroed prior to mapping to prevent sensitive kernel or cross-process data leakage.
7. **Make allocation context explicit:** Allocation requests explicitly specify flags/policies (e.g., `GFP_KERNEL`, non-blocking, interrupt context) rather than making implicit context assumptions.
8. **Keep DMA policy in a dedicated DMA API:** Physical frame addresses are not assumed to equal bus or DMA addresses. Device mappings and DMA addressing rules belong in a dedicated DMA subsystem.
9. **Do not overbuild NUMA/zones prematurely:** Establish single-node QEMU boot and allocation first. Introduce zones (`ZONE_DMA`, `ZONE_DMA32`, `ZONE_NORMAL`) only for addressability limits, and add NUMA topology when hardware/ACPI SLIT data is validated.
10. **Support observability without exposing writable allocator internals:** Provide atomic/lock-free read-only diagnostic counters for total, usable, reserved, free, allocated, and high-order frames with consistent snapshot semantics.
11. **Evidence gates capability claims:** Unit tests prove model logic; "Integrated" status requires real boot memory-map parsing and frame allocation in QEMU; "Supported" status requires repeatable CI tests across multiple RAM configurations and architectures.

---

## Proposed PR Sequence Roadmap

| PR | Title | Scope & Objectives | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(memory): audit PMM, boot map, and allocator status` | Inspect actual PMM/VMM files, boot protocol, memory-map sources, kernel/initrd/page-table reservations, and test coverage. Update component docs and `FEATURE_STATUS.toml`. | Every claimed allocator maps to code, boot input, tests, and evidence; unknown status is recorded honestly; physical/virtual/DMA responsibilities are separated. |
| 2 | `docs(memory): define physical-frame states and ownership contract` | Specify frame states, allocation/free API, alignment, order, zeroing, pin/reference behavior, failure behavior, concurrency, and early-boot/runtime boundary. | State diagram, invariants, and error behavior are reviewed before allocator rewrites; public API does not expose raw mutable allocator metadata. |
| 3 | `test(memory): add memory-map parser and reservation fixtures` | Add fixtures for usable/reserved/ACPI/NVS/unusable/unaccepted/overlapping/adjacent/unaligned/zero-size regions, malformed descriptors, and overflow. | Parser rejects invalid records safely; overlapping inputs produce deterministic reservations; unknown types are never freed. UEFI/ACPI maps explicitly distinguish available RAM from regions that must not be allocated. |
| 4 | `feat(bootmem): add normalized physical-region map` | Convert bootloader/firmware map into sorted, page-aligned, non-overlapping usable and reserved ranges; preserve provenance and attributes. | Boot map handles holes and conflicting descriptors deterministically; all additions/subtractions use checked arithmetic; output matches test fixtures. |
| 5 | `feat(bootmem): reserve kernel image and boot-critical ranges` | Reserve kernel sections, bootloader data, initrd, early page tables, allocator metadata, firmware/runtime areas, framebuffer, and MMIO windows. | Allocator cannot return any reserved frame; test asserts all critical ranges are reserved before free-list construction; unknown ranges default to reserved. |
| 6 | `feat(bootmem): implement bounded early allocator` | Add simple boot-time allocation for aligned contiguous ranges before the runtime PMM is available; support explicit address limits and failure results. | Allocations are aligned, non-overlapping, and constrained to usable ranges; reservation updates are reflected; allocator fails cleanly on exhaustion. |
| 7 | `feat(pmm): implement first runtime page-frame allocator` | Implement a conservative bitmap or equivalent frame allocator for 4 KiB pages, backed by normalized usable ranges. | QEMU boot allocates two distinct aligned frames, frees/reallocates them, and reports consistent counts; holes/reserved pages are never returned. |
| 8 | `feat(pmm): add allocator metadata placement and self-reservation` | Place bitmap/page descriptors in usable RAM and reserve their backing before making other frames free. Handle metadata sizing overflow and fragmentation. | Metadata never overlaps kernel/reservations; insufficient metadata memory fails boot with a clear diagnostic; accounting includes allocator-owned pages. |
| 9 | `feat(pmm): add checked frame free and double-free detection` | Add ownership checks, valid frame/order checks, reserved-frame protection, and debug poisoning/canaries where practical. | Double-free, freeing reserved/out-of-range frames, invalid order, and unaligned inputs are rejected in debug/test builds; production behavior is documented. |
| 10 | `feat(pmm): add buddy allocator for contiguous page orders` | Replace or layer a buddy allocator only after the first runtime PMM works; support splitting/coalescing and a defined maximum order. | Property tests verify alignment, no overlap, complete coalescing, allocation/free conservation, and fragmentation behavior over randomized sequences. |
| 11 | `feat(pmm): add physical-page metadata and reference ownership` | Add page descriptors with state, refcount/pin count, owner or usage tags, and mapping/device references needed by VMM, COW, and DMA. | Refcounts cannot underflow/overflow; free requires no mappings, pins, or outstanding references; COW and unmap tests return pages correctly. |
| 12 | `feat(pmm): add zero-on-allocation and secure page reuse policy` | Add zeroed-frame API or mandatory zero-before-user-exposure path; define kernel/DMA buffer scrubbing policy. | Newly allocated user-visible pages are zero; released pages are scrubbed before reuse according to policy; no stale data leaks in cross-process tests. |
| 13 | `feat(pmm): integrate physical frames with x86_64 paging and VMM` | Connect PMM frame allocation/free to page-table creation, kernel heap growth, user mappings, unmap, and fault handling. | QEMU maps/unmaps real frames; permissions and address-space isolation are tested; unmapped/free frames are not still reachable in page tables. |
| 14 | `feat(pmm): add DMA-capable frame constraints and mapping boundary` | Add allocation constraints only if needed by real drivers; route device mappings through the DMA API with address masks and mapping failures. | Drivers consume DMA mappings, not guessed physical addresses; frame constraints are validated; map/unmap and failure behavior are tested. |
| 15 | `feat(pmm): add per-zone accounting for addressability constraints` | Add only zones required by validated device/architecture limits, such as DMA/DMA32; document fallback order and reserves. | Allocations honor device mask/zone constraints and fallbacks; scarce low-address memory is not consumed unnecessarily; zone counters reconcile. |
| 16 | `feat(pmm): add NUMA node metadata and policy hooks` | Defer until firmware/ACPI topology and SMP are integrated; add node IDs and local-first/fallback allocation policy. | QEMU NUMA test validates node assignment and fallback; absent/malformed topology safely behaves as UMA. |
| 17 | `debug(pmm): add allocator consistency checker and memory diagnostics` | Add debug walk to verify free structures, page states, counts, overlapping blocks, and reserved-range exclusion; expose read-only stats. | Checker runs after init and in tests; corruption yields localized diagnostics; stats are versioned and read-only. |
| 18 | `test(pmm): add randomized, exhaustion, and fault-injection tests` | Add property tests for allocation/free sequences, fragmentation, overflow, map corruption, and allocation failure at each boot phase. | After every generated sequence, conservation and disjointness invariants hold; failures are reproducible; QEMU exhaustion does not corrupt unrelated memory. |
| 19 | `test(pmm): add QEMU memory-size and architecture matrix` | Exercise low/high RAM sizes, memory holes, reserved regions, and supported architectures; test boot-to-user allocation. | CI/documented runs capture map, reservations, frame counts, allocation/free, and VMM mapping; x86_64/AArch64/RISC-V maturity is recorded separately. |
| 20 | `docs(memory): publish PMM design, diagnostics, and support matrix` | Document memory map format, page states, APIs, zones, DMA boundary, errors, debug options, and limits. | Each supported claim links to runtime evidence; limits and unimplemented hotplug/NUMA/reclaim features are clear. |

*Recommended Initial Focus:* Complete PRs 1–9 and 13 for single x86_64 QEMU configuration.

---

## Periodic Maintenance PR Cadence

- **Every PR:** `ci(pmm): run memory-map, allocator, and VMM checks` - Verify build, parser fixtures, allocator invariants, and QEMU boot tests.
- **Weekly:** `maint(pmm): triage allocator failures and memory-corruption reports` - Capture reproducer, map configuration, owner, and severity.
- **Weekly:** `test(pmm): review allocation/free conservation results` - Verify free/allocated/reserved frame counter integrity.
- **Monthly:** `audit(pmm): verify reservations and memory-map assumptions` - Ensure kernel/initrd/page-table/firmware/MMIO/framebuffer ranges are strictly protected.
- **Monthly:** `test(pmm): run randomized allocator and fragmentation suite` - Stress order splits, coalescing, and fault injection.
- **Monthly:** `security(pmm): review stale-data and physical-memory exposure paths` - Verify zeroing policies and page isolation.
- **Quarterly:** `test(pmm): validate memory-size and hole matrix` - Re-test low/high RAM sizes, memory holes, and reserved boundaries.
- **Quarterly:** `perf(pmm): refresh allocator latency and fragmentation baselines` - Record allocation latency and fragmentation ratios.
- **Each Release Candidate:** `release(pmm): certify memory-manager claims` - Certify target hardware/QEMU matrix and VMM/DMA interfaces.
- **After Each Incident:** `fix(pmm): prevent recurrence of <corruption or allocation bug>` - Add minimized test cases and fix root causes.

---

## Cross-Component Implementation Rules

- **Bootloader/firmware handoff:** Standardize versioned memory-map descriptors; parse UEFI/ACPI/Multiboot2 maps without assuming unlisted regions are free.
- **Boot allocator to runtime PMM:** Transfer only non-reserved regions to the runtime frame allocator; maintain debug logs for reservation owners.
- **VMM/page tables:** Allocate page-table frames through PMM; ensure freed physical frames are invalidated in page tables and TLBs.
- **Kernel heap/slab:** Keep physical page frame allocation distinct from object-granularity slab/heap management.
- **DMA/drivers:** Route DMA allocations through dedicated DMA mapping interfaces rather than raw physical pointer casting.
- **Process isolation:** Scrub/zero frames before userland exposure; enforce strict ownership tags.
- **Crash handling:** Reserve a dedicated emergency page buffer for panic diagnostics that bypasses standard allocation locks.

---

## PR Template and Merge Gates

```text
## Summary
What PMM correctness, integration, or capability gap does this PR address?

## Scope
Boot map | reservation | early allocator | runtime allocator |
page metadata | VMM | DMA | zones | NUMA

## Memory-map evidence
Source (UEFI/ACPI/bootloader/QEMU), memory types, ranges, and exact target.

## Invariants and safety
Reserved-range policy, checked arithmetic, frame states, ownership/refcounts,
zeroing, physical/virtual/DMA address separation, and failure behavior.

## Runtime evidence
Unit/model | booted QEMU | SMP QEMU | physical hardware
List memory size, architecture, boot protocol, and exact operations tested.

## Tests
Malformed maps, overlaps/holes, allocation/free, double-free, exhaustion,
coalescing, VMM map/unmap, zeroing, DMA, and anything not tested.

## Status and documentation
Proposed | Prototype | Integrated | Supported
Link updated memory docs, feature inventory, and test evidence.

## Checklist
- [ ] Unknown/reserved memory is never allocated
- [ ] Critical kernel/boot/firmware/MMIO ranges are reserved
- [ ] Arithmetic and PFN conversions are checked
- [ ] Physical, virtual, and DMA addresses are distinct
- [ ] Allocation/free accounting and ownership are tested
- [ ] User-visible pages are zeroed before exposure
- [ ] Required CI and QEMU checks pass; review conversations resolved
```

---

## Completion Gates

- **Prototype:** Allocator algorithms or memory-map parsing pass unit tests, but are not connected to boot-time physical memory.
- **Integrated:** A booted QEMU target parses its actual memory map, reserves critical ranges, allocates/frees physical frames, and maps them through the VMM without corrupting kernel execution.
- **Supported:** Repeatable CI tests cover multiple RAM sizes, holes/reservations, allocation failures, page ownership/zeroing, and each advertised architecture with documented limitations.
