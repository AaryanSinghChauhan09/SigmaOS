## Summary

<!-- What memory-management or component behavior changed and why? -->

## Memory / Subsystem component

- [ ] Memory policy (`docs/memory/MEMORY_POLICY.md`)
- [ ] Physical memory manager (`src/memory/physical.rs`, `src/memory/frame.rs`)
- [ ] Buddy allocator (`src/memory/buddy_allocator.rs`, `src/memory/sigma_buddy.rs`)
- [ ] Slab allocator (`src/memory/slab_allocator.rs`)
- [ ] Paging / Page tables (`src/mm/paging.rs`, `src/mm/page_table.rs`)
- [ ] Virtual memory (`src/mm/virtual.rs`, `src/mm/virtual_memory.rs`)
- [ ] Memory protection / W^X (`src/mm/virtual.rs`)
- [ ] Swap / zram / zswap (`src/swap/core.rs`, `src/swap/enqueue.rs`)
- [ ] Reclaim / compaction / kswapd (`src/memory/kswapd.rs`, `src/memory/compaction.rs`)
- [ ] OOM / cgroups (`src/memory/oom_killer.rs`, `src/memory/cgroups.rs`)
- [ ] procfs / diagnostics / observability
- [ ] Security / capabilities / W^X
- [ ] Documentation / ADRs
- [ ] CI / tooling

## Recurring task

- [ ] Every PR build & test validation
- [ ] Daily CI triage
- [ ] Weekly subsystem lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly allocator audit
- [ ] Biweekly paging & protection audit
- [ ] Monthly boot-to-shell validation
- [ ] Monthly memory stress/fuzz triage
- [ ] Monthly swap rehearsal
- [ ] Quarterly architecture / security / release review

## Reference behavior

<!-- Required for lifecycle, ABI, or memory changes -->

- POSIX section:
- Linux/BSD/Redox reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Changes

-
-

## Memory safety & concurrency review

- [ ] Lifecycle transitions follow authoritative 11-stage state machine
- [ ] Race-free allocation and atomic ref counts verified
- [ ] Lock ordering (Virtual -> Paging -> Physical -> Swap) documented and respected
- [ ] Double-free prevention enforced (`DoubleFree` detection)
- [ ] Reserved memory regions strictly protected
- [ ] Non-canonical virtual addresses rejected
- [ ] User/kernel space isolation enforced (`USER_SPACE_START` vs `KERNEL_SPACE_START`)
- [ ] W^X (Write XOR Execute) security policy enforced
- [ ] Lost-wakeup / lost-update prevention flags checked (`reclaim_wakeup_notified`, `tlb_flush_required`)
- [ ] OOM is treated as a designed state rather than an accidental panic
- [ ] No unbounded allocation or recursion

## Evidence

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] Standalone unit test suite (`tests/memory/physical.rs`, `tests/mm/paging.rs`, `tests/mm/virtual.rs`, `tests/swap/core.rs`)
- [ ] Integration tests
- [ ] Boot-to-shell evidence, if boot-affecting
- [ ] Memory stress/benchmark report, if performance-affecting
- [ ] Security review requested, if applicable

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] `MEMORY_RULES.toml`
- [ ] Memory documentation (`docs/memory/`, `docs/mm/`, `docs/swap/`)

## Risks and rollback

- Risk:
- Rollback:
