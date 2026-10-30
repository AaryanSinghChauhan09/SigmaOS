# SigmaOS Development Scope

Status: **frozen for Phase 0**

SigmaOS is currently optimizing for a secure x86_64 microVM host. New work
must improve the execution path below or provide evidence for it:

1. Bootloader and x86_64 kernel entry.
2. Ring 3 transition, user page tables, and syscall entry/return.
3. Dynamic ELF loading for a bounded static/musl workload.
4. Interrupt-driven virtio-block and virtio-net on QEMU.
5. Minimal filesystem and init/userspace shell.
6. Reproducible CI artifacts, benchmarks, fuzzing, and security evidence.

## Explicitly deferred

- Additional distro-persona and “parity” modules.
- Creative-suite expansion and encyclopedia revisions.
- Physical GPU, Wi-Fi, and broad bare-metal support.
- Performance superiority claims without same-machine baselines.

Existing code outside this scope is preserved, but new feature work should not
expand it until the Phase-0 exit criterion is met. This is a deletion-friendly,
reviewable boundary: every milestone must add implementation, tests, and an
observable artifact together.
