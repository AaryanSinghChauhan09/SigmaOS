## Summary

<<<<<<< HEAD
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
=======
<<<<<<< HEAD
<!-- What shell behavior changed and why? -->

## Shell component

- [ ] Lexer
- [ ] Parser
- [ ] Expansion
- [ ] Executor
- [ ] Pipelines
- [ ] Redirection
- [ ] Job control
- [ ] Signals
- [ ] Built-ins
- [ ] Interactive line editor
- [ ] Script runner
- [ ] Diagnostics
- [ ] Security / sandboxing
=======
<!-- What I/O-management behavior changed and why? -->

## I/O component

<<<<<<< HEAD
- [ ] IPC handler
- [ ] Pipes / FIFOs
- [ ] UNIX-domain sockets
- [ ] Shared memory
- [ ] Message queues
- [ ] Event wakeups
- [ ] Signals
- [ ] Process/thread management
- [ ] VFS
- [ ] Security
- [ ] Documentation
- [ ] CI / tooling

## Upstream study / Motivation

- Project / Reference:
- Research note / ADR:

## IPC & Subsystem Semantics

- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:
- Error behavior:
- Permission model:
- Resource limits:
- Backpressure policy:

## Security review

- [ ] User input validated
- [ ] Descriptor lifecycle checked
- [ ] Capability/namespace checks enforced
- [ ] No unbounded kernel allocation
- [ ] `unsafe` justified with SAFETY comments
- [ ] Threat model updated
=======
- [ ] I/O policy
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Interrupt manager
- [ ] DMA manager
- [ ] Buffer manager
- [ ] I/O scheduler
- [ ] Submit / complete
- [ ] Callback
- [ ] Interrupt
- [ ] DMA
- [ ] Buffer
- [ ] Recovery
- [ ] procfs / diagnostics
- [ ] Security / capabilities
>>>>>>> origin/main
- [ ] Documentation
- [ ] CI / tooling

## Recurring task

<<<<<<< HEAD
- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly feature/bug triage
- [ ] Weekly POSIX/sh study
- [ ] Biweekly parser/expansion audit
- [ ] Biweekly built-in review
- [ ] Monthly boot-to-shell validation
- [ ] Monthly compatibility suite
- [ ] Monthly fuzz triage
- [ ] Monthly security review
- [ ] Quarterly release/UX/performance review

## Reference behavior

<!-- Required for compatibility or grammar changes -->

- POSIX section:
- Bash/dash/Zsh/BusyBox reference:
>>>>>>> origin/main
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Changes

-
-

<<<<<<< HEAD
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
=======
## Shell safety review
>>>>>>> origin/main

- [ ] Expansion order is correct
- [ ] Quoting behavior is preserved
- [ ] Redirection failures are handled
- [ ] Exit codes are explicit
- [ ] Signal behavior is defined
- [ ] No unbounded recursion or allocation
- [ ] Interactive terminal state is restored
- [ ] Script mode does not execute invalid partial input

## Evidence

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated shell tests
- [ ] POSIX/conformance test evidence
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-shell evidence, if applicable
- [ ] Security review requested, if applicable

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Shell documentation
- [ ] Boot matrix, if applicable

## Risks and rollback

- Risk:
- Rollback:
=======
<<<<<<< HEAD
- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly I/O audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Quarterly architecture/security/release review

## Reference behavior

<!-- Required for lifecycle, ABI, or I/O changes -->

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Changes

-
-

## I/O safety review

- [ ] Lifecycle transitions are valid
- [ ] Zombies are eventually reaped
- [ ] Orphans are adopted
- [ ] PID reuse is safe
- [ ] Lock ordering is documented
- [ ] Interrupt-context safety considered
- [ ] Signal races considered
- [ ] Double-completion prevented
- [ ] Resource limits enforced
>>>>>>> origin/main
- [ ] No unbounded allocation or recursion

## Evidence

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
<<<<<<< HEAD
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
=======
- [ ] New/updated unit tests
- [ ] Integration tests
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-boot evidence, if boot-affecting
- [ ] I/O benchmark, if performance-affecting
- [ ] Security review requested, if applicable
=======
- [ ] Thread manager
- [ ] Scheduler
- [ ] Process management
- [ ] Memory management
- [ ] VFS / filesystems
- [ ] Drivers
- [ ] IPC
- [ ] Security
- [ ] Installer / packaging
- [ ] Documentation
- [ ] CI / tooling

## Tests

<<<<<<< HEAD
- [ ] Unit tests
- [ ] Integration tests
- [ ] Concurrency/stress tests
- [ ] Timeout tests
- [ ] Fuzz tests
- [ ] Resource-exhaustion tests
- [ ] Regression test attached for fixed bug
=======
- [ ] Source implementation
- [ ] Unit tests added / updated
- [ ] Integration tests added / updated
- [ ] Concurrency / stress tests added / updated
- [ ] Fuzz tests added / updated
- [ ] Boot smoke test performed, if kernel-facing
- [ ] Documentation updated
- [ ] `FEATURE_STATUS.toml` capability updated
>>>>>>> origin/main
>>>>>>> origin/main

## Status updates

<<<<<<< HEAD
- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] I/O documentation
- [ ] Boot matrix, if applicable

## Risks and rollback

- Risk:
- Rollback:
=======
```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

<<<<<<< HEAD
Paste output below.

## Capability status

- Previous status:
- New status:
- Evidence:

## Integration impact

- [ ] Unit-tested only
- [ ] Connected to runtime path
- [ ] Boot path validated
- [ ] Installer/update path validated
- [ ] Hardware tested
=======
Paste relevant output below:

```text
```

## Capability Status

- Previous status:
- New status:
- Test evidence:
>>>>>>> origin/main

## Integration Impact

- [ ] No runtime integration yet (unit-tested only)
- [ ] Connected to runtime path
- [ ] Boot path validated
- [ ] Installer / update path validated
- [ ] Hardware tested

## Reviewer Checklist

- [ ] Scope is narrow (one logical subsystem per PR)
- [ ] Public API fully documented
- [ ] `unsafe` code justified with safety invariant comments
- [ ] Lock ordering documented and deadlock-safe
- [ ] Error paths and timeout bounds tested
- [ ] `FEATURE_STATUS.toml` feature status updated
- [ ] No unsupported Linux/BSD compatibility claims
>>>>>>> origin/main
>>>>>>> origin/main
>>>>>>> origin/main
