# SigmaOS Physical Memory Manager Master Architecture, Support Matrix, and PR Plan

SigmaOS’s public README says the memory-management source contains modules for memory management, but explicitly warns that code presence and unit tests do not establish kernel integration, bootability, or hardware support. I could not verify a detailed PMM status entry in the public feature inventory, so the first PR should inspect the actual allocator and architecture boot handoff before labeling any component “missing” or “implemented.” The plan assumes the PMM’s job is to identify usable physical pages, reserve non-allocatable ranges, and provide frames to the virtual-memory manager and DMA layer.

The key industry pattern is **two-stage memory management**: a boot-time allocator tracks firmware-provided memory and reservations; after core page metadata and paging are ready, usable pages transfer to the runtime page allocator. Linux uses memblock early and releases remaining pages to the buddy allocator; its runtime allocator also models nodes and zones. FreeBSD’s VM tracks physical pages with explicit states and queues. These are design references, not required SigmaOS APIs.

## PMM rules and invariants

- **Only explicitly usable RAM is allocatable.** Start from “reserved” and mark only validated usable ranges free. Exclude the kernel, bootloader data still in use, page tables, firmware tables until consumed, memory-mapped I/O, framebuffer, crash logs, and all reserved/unknown ranges. ACPI/UEFI memory types distinguish usable, reserved, reclaimable, NVS, unusable, and unaccepted ranges.
- **Keep physical and virtual addresses distinct.** Use typed addresses and frame numbers; a physical address is not a dereferenceable pointer. Map memory explicitly through the VMM when CPU access is required.
- **Use checked arithmetic everywhere.** Validate `base + size`, page alignment, PFN conversions, array lengths, and allocator order before modifying state. Reject overflow, wraparound, out-of-range, and overlapping invalid regions.
- **Define ownership and page states.** Each frame is in exactly one state—reserved, free, allocated, wired/pinned, or device/unmanaged as supported—and every transition has a single owner and a checked reference/lifetime rule.
- **Keep PMM and VMM responsibilities separate.** PMM allocates physical frames; VMM maps them into address spaces and controls page permissions. Neither should silently perform the other’s work.
- **Zero memory before exposing it to a less-trusted consumer.** Define whether the PMM or caller provides zeroing; user mappings and shared buffers must never expose stale data from a previous owner.
- **Make allocation context explicit.** Define whether an allocation may block, reclaim, or run in interrupt context. Linux allocation APIs use flags to express allowed zones and reclaim behavior; SigmaOS should use typed request policies rather than hidden context assumptions.
- **Keep DMA policy in a DMA API.** PMM may supply backing frames, but drivers must not assume a physical frame address is directly usable as a DMA address. DMA mapping and device addressing belong to a dedicated layer.
- **Do not overbuild NUMA/zones prematurely.** First validate a single-node QEMU path. Add zones for actual addressability constraints (such as DMA masks) and NUMA only when the platform and firmware data are validated. Linux uses nodes and zones for these constraints.
- **Support observability without exposing writable allocator internals.** Provide read-only counters for present, reserved, free, allocated, and high-order frames, with snapshot consistency defined.
- **Evidence gates capability claims.** A bitmap/buddy model test is not a booted PMM. “Integrated” requires real boot memory-map parsing and frame allocation in QEMU; “Supported” additionally requires repeatable tests across named architectures/configurations.

## Proposed PR sequence

| PR | Proposed title | Scope | Acceptance criteria |
|---|---|---|---|
| 1 | `docs(memory): audit PMM, boot map, and allocator status` | Inspect actual PMM/VMM files, boot protocol, memory-map sources, kernel/initrd/page-table reservations, and test coverage. Update component docs and `FEATURE_STATUS.toml`. | Every claimed allocator maps to code, boot input, tests, and evidence; unknown status is recorded honestly; physical/virtual/DMA responsibilities are separated. |
| 2 | `docs(memory): define physical-frame states and ownership contract` | Specify frame states, allocation/free API, alignment, order, zeroing, pin/reference behavior, failure behavior, concurrency, and early-boot/runtime boundary. | State diagram, invariants, and error behavior are reviewed before allocator rewrites; public API does not expose raw mutable allocator metadata. |
| 3 | `test(memory): add memory-map parser and reservation fixtures` | Add fixtures for usable/reserved/ACPI/NVS/unusable/unaccepted/overlapping/adjacent/unaligned/zero-size regions, malformed descriptors, and overflow. | Parser rejects invalid records safely; overlapping inputs produce deterministic reservations; unknown types are never freed. UEFI/ACPI maps explicitly distinguish available RAM from regions that must not be allocated. |
| 4 | `feat(bootmem): add normalized physical-region map` | Convert bootloader/firmware map into sorted, page-aligned, non-overlapping usable and reserved ranges; preserve provenance and attributes. | Boot map handles holes and conflicting descriptors deterministically; all additions/subtractions use checked arithmetic; output matches test fixtures. |
| 5 | `feat(bootmem): reserve kernel image and boot-critical ranges` | Reserve kernel sections, bootloader data, initrd, early page tables, allocator metadata, firmware/runtime areas, framebuffer, and MMIO windows. | Allocator cannot return any reserved frame; test asserts all critical ranges are reserved before free-list construction; unknown ranges default to reserved. |
| 6 | `feat(bootmem): implement bounded early allocator` | Add simple boot-time allocation for aligned contiguous ranges before the runtime PMM is available; support explicit address limits and failure results. | Allocations are aligned, non-overlapping, and constrained to usable ranges; reservation updates are reflected; allocator fails cleanly on exhaustion. Linux memblock is a reference for early allocation before the regular allocator is online. |
| 7 | `feat(pmm): implement first runtime page-frame allocator` | Implement a conservative bitmap or equivalent frame allocator for 4 KiB pages, backed by normalized usable ranges. Choose the simplest testable design first. | QEMU boot allocates two distinct aligned frames, frees/reallocates them, and reports consistent counts; holes/reserved pages are never returned. |
| 8 | `feat(pmm): add allocator metadata placement and self-reservation` | Place bitmap/page descriptors in usable RAM and reserve their backing before making other frames free. Handle metadata sizing overflow and fragmentation. | Metadata never overlaps kernel/reservations; insufficient metadata memory fails boot with a clear diagnostic; accounting includes allocator-owned pages. |
| 9 | `feat(pmm): add checked frame free and double-free detection` | Add ownership checks, valid frame/order checks, reserved-frame protection, and debug poisoning/canaries where practical. | Double-free, freeing reserved/out-of-range frames, invalid order, and unaligned inputs are rejected in debug/test builds; production behavior is documented. |
| 10 | `feat(pmm): add buddy allocator for contiguous page orders` | Replace or layer a buddy allocator only after the first runtime PMM works; support splitting/coalescing and a defined maximum order. | Property tests verify alignment, no overlap, complete coalescing, allocation/free conservation, and fragmentation behavior over randomized sequences. Linux’s buddy allocator manages power-of-two page blocks and splits/coalesces them by order. |
| 11 | `feat(pmm): add physical-page metadata and reference ownership` | Add page descriptors with state, refcount/pin count, owner or usage tags, and mapping/device references needed by VMM, COW, and DMA. | Refcounts cannot underflow/overflow; free requires no mappings, pins, or outstanding references; COW and unmap tests return pages correctly. FreeBSD’s `vm_page_t` is a reference for explicit page state and queue ownership. |
| 12 | `feat(pmm): add zero-on-allocation and secure page reuse policy` | Add zeroed-frame API or mandatory zero-before-user-exposure path; define kernel/DMA buffer scrubbing policy. | Newly allocated user-visible pages are zero; released pages are scrubbed before reuse according to policy; no stale data leaks in cross-process tests. |
| 13 | `feat(pmm): integrate physical frames with x86_64 paging and VMM` | Connect PMM frame allocation/free to page-table creation, kernel heap growth, user mappings, unmap, and fault handling. | QEMU maps/unmaps real frames; permissions and address-space isolation are tested; unmapped/free frames are not still reachable in page tables. The repo identifies boot and paging validation as unfinished; this PR requires runtime evidence. |
| 14 | `feat(pmm): add DMA-capable frame constraints and mapping boundary` | Add allocation constraints only if needed by real drivers; route device mappings through the DMA API with address masks and mapping failures. | Drivers consume DMA mappings, not guessed physical addresses; frame constraints are validated; map/unmap and failure behavior are tested. Linux recommends DMA APIs because device bus addresses can differ from CPU physical addresses. |
| 15 | `feat(pmm): add per-zone accounting for addressability constraints` | Add only zones required by validated device/architecture limits, such as DMA/DMA32; document fallback order and reserves. | Allocations honor device mask/zone constraints and fallbacks; scarce low-address memory is not consumed unnecessarily; zone counters reconcile. Linux models zones for architectural addressability and memory constraints. |
| 16 | `feat(pmm): add NUMA node metadata and policy hooks` | Defer until firmware/ACPI topology and SMP are integrated; add node IDs and local-first/fallback allocation policy. | QEMU NUMA test validates node assignment and fallback; absent/malformed topology safely behaves as UMA. Linux allocates node structures from firmware-reported topology. |
| 17 | `debug(pmm): add allocator consistency checker and memory diagnostics` | Add debug walk to verify free structures, page states, counts, overlapping blocks, and reserved-range exclusion; expose read-only stats. | Checker runs after init and in tests; corruption yields localized diagnostics; stats are versioned and read-only. |
| 18 | `test(pmm): add randomized, exhaustion, and fault-injection tests` | Add property tests for allocation/free sequences, fragmentation, overflow, map corruption, and allocation failure at each boot phase. | After every generated sequence, conservation and disjointness invariants hold; failures are reproducible; QEMU exhaustion does not corrupt unrelated memory. Linux has page-allocation fault injection for testing failure paths. |
| 19 | `test(pmm): add QEMU memory-size and architecture matrix` | Exercise low/high RAM sizes, memory holes, reserved regions, and supported architectures; test boot-to-user allocation. | CI/documented runs capture map, reservations, frame counts, allocation/free, and VMM mapping; x86_64/AArch64/RISC-V maturity is recorded separately. |
| 20 | `docs(memory): publish PMM design, diagnostics, and support matrix` | Document memory map format, page states, APIs, zones, DMA boundary, errors, debug options, and limits. | Each supported claim links to runtime evidence; limits and unimplemented hotplug/NUMA/reclaim features are clear. |

**Recommended first milestone:** complete PRs 1–9 and 13 for a single x86_64 QEMU configuration. A correct simple frame allocator with safe reservations and real paging integration is more valuable than an unvalidated buddy allocator, NUMA policy, memory hotplug, or page-cache integration.

## Periodic maintenance PR cadence

| Cadence | Proposed recurring PR/task | Required evidence |
|---|---|---|
| Every PR | `ci(pmm): run memory-map, allocator, and VMM checks` | Formatting/build, parser fixtures, allocator invariants, and affected QEMU boot tests; skipped tests explained. |
| Weekly | `maint(pmm): triage allocator failures and memory-corruption reports` | Reproducer, memory map/configuration, affected phase, owner, severity, and fix or tracked issue. |
| Weekly | `test(pmm): review allocation/free conservation results` | Free/allocated/reserved counts and regression trends from deterministic tests. |
| Monthly | `audit(pmm): verify reservations and memory-map assumptions` | Kernel/initrd/page-table/firmware/MMIO/framebuffer ranges checked against bootloader/firmware input; unknown memory types remain reserved. |
| Monthly | `test(pmm): run randomized allocator and fragmentation suite` | Randomized allocation/free, order splits/coalescing, exhaustion, and invalid-free results; deterministic seed retained. |
| Monthly | `security(pmm): review stale-data and physical-memory exposure paths` | Zeroing, user mapping, DMA, shared memory, descriptor lifetime, and privilege boundaries reviewed. |
| Quarterly | `test(pmm): validate memory-size and hole matrix` | Repeat boot tests across memory sizes and configurations; verify accounting and no allocation from holes/reserved regions. |
| Quarterly | `perf(pmm): refresh allocator latency and fragmentation baselines` | Allocation/free latency by order, fragmentation, metadata overhead, and lock contention; exact QEMU/hardware details recorded. |
| Each release candidate | `release(pmm): certify memory-manager claims` | Tested boot target, map source, reservations, frame allocator, VMM, DMA boundaries, and known limitations. |
| After each incident | `fix(pmm): prevent recurrence of <corruption or allocation bug>` | Root cause, minimized test, security/data-integrity impact, regression test, and backport decision. |

These are maintenance review tasks, not automatic code changes. For any allocator performance claim, include the workload and configuration; prioritize correctness and memory safety over microbenchmarks.

## Cross-component implementation rules

- **Bootloader/firmware handoff:** Define one versioned memory-map structure; validate descriptor count/size and address arithmetic; preserve provenance. ACPI/UEFI distinguishes usable memory from reserved, NVS, unusable, and unaccepted ranges—never treat a missing descriptor as usable by default.
- **Boot allocator to runtime PMM:** Transfer only regions not reserved by the kernel, bootloader, firmware, or early allocations. Keep a debug record of each reservation and its owner. Linux’s boot-time allocator is explicitly temporary and its remaining usable memory is handed off to the buddy allocator.
- **VMM/page tables:** Allocate page-table frames through PMM, but maintain mapping permissions and TLB invalidation in VMM. A freed physical page must not remain mapped in any address space.
- **Kernel heap/slab:** The PMM should expose page-granular allocation; the heap/slab layer serves object-sized allocations. Linux likewise offers separate page, slab, and virtually contiguous allocation APIs.
- **DMA/drivers:** Keep DMA address, CPU physical address, and virtual address as different types. Use DMA mappings and ownership/synchronization APIs; never assume physical equals bus address.
- **Process isolation:** Enforce page ownership, permissions, and zeroing before user exposure; test cross-process reuse.
- **Swap and reclaim:** Do not add reclaim behavior to the PMM until VFS/page cache, backing store, and wait/synchronization paths are safe. Allocation failure should be explicit rather than hidden in unbounded retry.
- **Crash handling:** Preserve a small, documented reserve for panic diagnostics only after regular allocation works; panic paths must not depend on normal locks or allocations that may already be exhausted.
- **Architecture support:** Keep physical-address width, page size, direct-map ranges, and firmware parser behavior architecture-specific behind a common interface; validate each architecture independently.
- **Diagnostics:** Expose present, usable, reserved, managed, free, and allocated memory separately; do not conflate “installed RAM” with “allocator-managed pages.”
- **Memory hotplug and NUMA:** Treat as later features. Linux supports memory hotplug and node/zone management, but they depend on page metadata, migration, isolation, and VMM cooperation; do not add superficial APIs before those components exist.

## PR template and merge gates

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

Require one maintainer approval for isolated tests/docs and two relevant reviewers for boot memory-map changes, frame-state transitions, page-table integration, DMA constraints, or user-page exposure. Protect the default branch with pull-request reviews and required checks. GitHub branch protection supports these gates.

## Completion gates

- **Prototype:** allocator algorithms or memory-map parsing pass unit tests, but are not connected to boot-time physical memory.
- **Integrated:** a booted QEMU target parses its actual memory map, reserves critical ranges, allocates/frees physical frames, and maps them through the VMM without corrupting the kernel.
- **Supported:** repeatable tests cover multiple RAM sizes, holes/reservations, allocation failure, page ownership/zeroing, and each advertised architecture; limitations for NUMA, hotplug, DMA, or reclaim are explicit.

The first goal is not “Linux-grade memory management”; it is **never handing out a reserved or already-owned frame**, then proving the allocator works in a real SigmaOS boot. The project’s status policy explicitly says module tests do not prove integration, and Linux/FreeBSD references both demonstrate that PMM correctness depends on boot-map handling, explicit page state, and careful handoff to runtime allocators.
