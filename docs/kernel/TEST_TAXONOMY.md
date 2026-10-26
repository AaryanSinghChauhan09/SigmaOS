# SigmaOS Kernel Test Taxonomy

To ensure complete coverage across logic, integration, performance, and recoverability, kernel testing is classified into 7 distinct tiers:

| Test Type | Scope & Purpose | Required For | Target Pipeline / Tooling |
|---|---|---|---|
| **Unit** | Pure logic, memory bounds, data structures, bitmasks | All kernel modules and public functions | `cargo test --lib`, `rustc --test` |
| **Property** | Invariants under randomly generated inputs | Allocators, VFS path parsing, lock managers | `proptest` / standalone generators |
| **Integration** | Cross-subsystem interactions & lifecycle transitions | Scheduler + Memory, VFS + Block Cache | `tests/kernel/integration/` |
| **Boot** | Firmware-to-shell state machine validation | Boot-affecting PRs, Daily CI | `scripts/ci/qemu_boot_smoke.sh` |
| **Fuzz** | Resilience against malformed inputs & crashes | Syscall dispatch, ELF parser, VFS metadata | `fuzz/fuzz_targets/` |
| **Stress** | Exhaustion limits, lock contention, OOM recovery | Swap queue, scheduler run queues, PIDs | `tests/stress_and_fuzz_tests.rs` |
| **Fault Injection** | Failure recovery under disk/memory/IRQ faults | Storage block drivers, installer, update | `installer/tests/failure_injection.rs` |
