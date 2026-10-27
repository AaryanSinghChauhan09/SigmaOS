# PR Specification: Periodic "Study → Improve Component → Prove Improvement" Program

## Program Architecture Specification

### 1. Program Lifecycle & Cadence Model

```text
+-----------------------------------------------------------------------------+
|               PERIODIC IMPROVEMENT PROGRAM ENGINE LIFECYCLE                  |
+-----------------------------------------------------------------------------+
|  DAILY     : Build / Test / Boot Health Check & Failure Triage Log           |
|  WEEKLY    : Upstream Study -> ADR -> Small PR -> Runtime Evidence Gate     |
|  BIWEEKLY  : Re-test Component Contract & Update Capability Matrix          |
|  MONTHLY   : Complete 1 Component Milestone & Runtime Recovery Path        |
|  QUARTERLY : Reassess Architecture, Gaps, Security & Roadmap Backlog        |
+-----------------------------------------------------------------------------+
```

### 2. 10-Step Component Improvement Loop Specification

```text
1. SELECT COMPONENT               : Identify narrow gap from P0-P2 backlog
2. MEASURE CURRENT STATE          : Audit tests, benchmarks, and log traces
3. SELECT UPSTREAM REFERENCE     : Pick reference (Linux, xv6, FreeBSD, etc.)
4. STUDY THE MECHANISM            : Analyze structures, flow, assumptions
5. WRITE IMPROVEMENT PROPOSAL     : Draft ADR in research/decisions/
6. IMPLEMENT SMALLEST SAFE CHANGE : Single-component targeted Rust PR
7. RUN TESTS                      : Execute unit, integration & runtime gates
8. UPDATE DOCS & CAPABILITY STATUS: Sync matrix state and evidence logs
9. ADD REGRESSION PROTECTION      : Commit mandatory regression test suite
10. REVIEW, MERGE, AND MONITOR    : Maintainer review and daily health tracking
```

### 3. Component Domain Ownership Matrix

```text
+--------------------+---------------------------+-----------------------------------+-----------------------------------+
| COMPONENT DOMAIN   | PRIMARY UPSTREAM REF      | PERIODIC IMPROVEMENT FOCUS        | MANDATORY RUNTIME PROOF           |
+--------------------+---------------------------+-----------------------------------+-----------------------------------+
| Kernel Core        | Linux, xv6, Redox         | Boot, interrupts, syscalls, panic | QEMU boot trace log               |
| Memory             | Linux, xv6, Redox         | Frame allocator, paging, leaks    | Allocation tests & traces         |
| VFS/filesystems    | Linux, xv6, FreeBSD       | Path resolution, mounts, crashes  | Filesystem conformance tests      |
| IPC                | Linux, FreeBSD            | Pipes, signals, shm, eventfd      | Cross-process interop tests       |
| Syscalls           | Linux, FreeBSD            | ABI stability, negative checks    | Syscall table & negative tests    |
| Security           | OpenBSD, Linux, FreeBSD   | Capabilities, sandbox, unveil     | Denial tests & threat model       |
| Drivers            | Linux, FreeBSD            | Device probe, init, virtio        | QEMU device logs                  |
| Networking         | Linux, FreeBSD            | Sockets, TCP/IP, PF, WireGuard    | Network integration tests         |
| Packaging          | Arch, Debian, Fedora      | Metadata, resolver, rollback      | Package test corpus               |
| Desktop/Zenith     | Linux Mint, Omarchy, Redox| Input, windows, launcher, crash   | Graphical smoke test              |
| Installer/update   | Debian, Fedora, Mint      | Clean install, A/B swap, rollback | Install and rollback tests        |
| Distro compat      | Arch, Debian, Fedora, BSD | Metadata ABI, sandboxing          | Compatibility matrix              |
+--------------------+---------------------------+-----------------------------------+-----------------------------------+
```

### 4. 12 Monthly Component Milestones

1. **Month 1**: Kernel/boot — Reproducible QEMU x86_64 boot-to-shell (`reports/boot/qemu-x86_64.log`).
2. **Month 2**: Memory — Frame allocator, paging, and leak detection (`reports/memory/allocation_traces.log`).
3. **Month 3**: Scheduler — EEVDF fair scheduling and context preemption (`reports/scheduler/process_switch.log`).
4. **Month 4**: VFS — Path resolution, file descriptors, RAM-disk operations (`reports/vfs/vfs_conformance.json`).
5. **Month 5**: Processes/IPC — Pipes, signals, process lifecycle (`reports/ipc/cross_process_interop.log`).
6. **Month 6**: Installer/update — Clean install, atomic update, rollback (`reports/installer/rollback_test.log`).
7. **Month 7**: Security — Capability checks, pledge, and unveil sandbox policy (`reports/security/capability_denial.json`).
8. **Month 8**: Drivers — virtio-blk, virtio-net, VGA (`reports/drivers/qemu_virtio.log`).
9. **Month 9**: Packaging — Metadata parsing and dependency resolution (`reports/package/resolver_corpus.json`).
10. **Month 10**: Desktop — Minimal graphical session and input handling (`reports/desktop/graphical_smoke.png`).
11. **Month 11**: Networking — Basic socket API and TCP/IP stack path (`reports/network/socket_integration.log`).
12. **Month 12**: Release engineering — Nightly artifact, known issues, rollback docs (`reports/steward/release_checklist.json`).

### 5. Capability Maturity Progression Rules

```text
Proposed   : ADR and gap analysis merged.
Prototype  : Unit tests passing in cargo test --lib.
Integrated : Connected to boot path + QEMU/runtime log evidence provided.
Hardened   : Negative testing, fuzzing coverage, and security review complete.
Supported  : Reproducible boot/install/update evidence & verified A/B rollback.
```

### 6. Verification Commands

```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```
