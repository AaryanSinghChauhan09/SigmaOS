# SigmaOS IPC Handler Master Architecture, Development Phases & Maintenance Plan

## Executive Summary

SigmaOS identifies IPC, signals, process lifecycle, and resource limits as core operating-system areas. Rather than treating IPC as a collection of isolated, ad-hoc modules, the **IPC Handler Subsystem** is defined as a formally specified, testable kernel subsystem responsible for safe, high-throughput, and policy-governed message and data transfer between processes, threads, and kernel services.

This specification establishes the architecture, boundaries, development phases, periodic maintenance program, core invariants, security threat model, pull request guidelines, and capability progression criteria for the SigmaOS IPC Handler.

---

## 1. Scope & Subsystem Boundaries

The IPC Handler owns safe message and data transfer mechanisms across user-space processes, kernel tasks, and system services.

### In-Scope Component Ownership

The IPC Handler explicitly owns:

- **Pipes & Anonymous Pipes:** Unidirectional byte-stream communication channels with buffer management and reader/writer lifecycle tracking.
- **FIFOs / Named Pipes:** Filesystem-node backed pipes with VFS open mode validation and access control.
- **UNIX-Domain Sockets:** Stream (`SOCK_STREAM`) and datagram (`SOCK_DGRAM`) IPC sockets with path binding, peer credentials, and half-close semantics.
- **Shared-Memory Regions:** Reference-counted shared page mappings crossing process address space boundaries with explicit lifetime and permission enforcement.
- **Message Queues:** Bounded POSIX and System V message queues supporting priority/FIFO message ordering, non-blocking modes, and timeout handling.
- **Event Notification Primitives:** Low-latency eventfd-like counter and semaphore wakeup objects with poll/epoll integration.
- **Signal Control Plane:** Signal delivery as an IPC control mechanism, real-time queued signals, and signal action dispatching.
- **File-Descriptor Passing:** Secure descriptor transfer over UNIX-domain sockets, gated by capability checks and descriptor table lifecycle rules.
- **Access Control & Permissions:** Owner, group, capability (`CAP_SYS_ADMIN`, `CAP_IPC_LOCK`, `CAP_NET_ADMIN`), namespace, and sandbox isolation checks on every IPC channel.

### Out-of-Scope (Deferred Subsystems)

To prevent premature coupling and unsafe abstractions, the initial IPC Handler does **not** own:

- General network sockets (TCP/IP stack)
- Display/GPU transport protocols (Wayland/Direct Rendering Manager)
- Package-manager transport
- Container runtime management communications

These higher-level transports will consume the core IPC Handler once primitive lifecycle rules and security guards are fully validated.

---

## 2. Periodic Task Cadence

IPC handler maintenance operates on a recurring schedule to prevent technical debt, race conditions, descriptor leaks, and documentation drift.

| Cadence | IPC Handler Development Focus | Cross-Component Maintenance | Required PR Deliverable |
|---|---|---|---|
| **Daily** | Triage IPC bugs, deadlocks, leaked descriptors, and flaky concurrency tests | Triage CI, build failures, security reports, and documentation drift | Bug report or focused fix PR |
| **Weekly** | Implement one IPC primitive or semantic improvement | Dependency/toolchain updates, lint cleanup, test repair | One scoped PR |
| **Biweekly** | Run IPC integration tests with processes, threads, signals, VFS, and resource limits | Audit `FEATURE_STATUS.toml`, docs, and component ownership | Status/evidence PR |
| **Monthly** | Fuzz IPC parsers, stress concurrent channels, audit descriptor leaks and permission checks | Build release candidate, boot smoke test, installer/update check | Release-readiness PR |
| **Quarterly** | Review IPC model against Linux and BSD behavior; remove duplicate implementations and dead code | Security review, API stability review, roadmap consolidation | Architecture RFC plus cleanup PRs |

---

## 3. Development Phases

### Phase 1: IPC Contract

Establish formal specifications, limits, status tracking, and interface contracts.

- Create `docs/ipc-handler.md` and mirror in `wiki/ipc-handler.md`.
- Register `[ipc.handler]` in `FEATURE_STATUS.toml`.
- Specify blocking, non-blocking, pollable, and asynchronous behavior.
- Define byte-stream versus message-boundary semantics.
- Define descriptor ownership, buffer limits, backpressure, error codes, and permission models.

### Phase 2: Core IPC Objects

Implement the canonical internal representations in `src/ipc/`:

- `IpcChannel`: High-level communication handle binding endpoints.
- `IpcEndpoint`: Reader or writer end of an IPC channel.
- `MessageBuffer`: Bounded kernel buffer for transient payload storage.
- `SharedMemoryRegion`: Reference-counted shared physical page container.
- `WaitQueue`: Thread wait-queue integration for blocking/unblocking threads.
- `FileDescriptorHandle`: Reference-counted handle wrapper in process FD tables.
- `IpcPermissions`: Owner UID/GID, mode bits, capability flags, and namespace IDs.
- `IpcStatistics`: Telemetry counter tracking message counts, bytes transferred, and drop counts.

### Phase 3: Pipes and FIFOs

Sequenced implementation order:

1. Anonymous pipe creation and descriptor allocation.
2. Byte-stream read/write semantics.
3. Non-blocking mode (`O_NONBLOCK`).
4. Pipe capacity bounds and backpressure policy.
5. End-of-file (EOF) behavior when all write ends close.
6. Broken-pipe (`EPIPE` / `SIGPIPE`) behavior when all read ends close.
7. FIFO creation (`mkfifo`), open modes, VFS node binding, and permissions.
8. Poll/select readiness semantics (`POLLIN`, `POLLOUT`, `POLLHUP`).

### Phase 4: UNIX-Domain Sockets

1. Socket creation (`socket`), bind, listen, accept, connect, and close.
2. Stream (`SOCK_STREAM`) and datagram (`SOCK_DGRAM`) modes.
3. Peer credentials passing (`SO_PEERCRED` / `SCM_CREDENTIALS`).
4. Filesystem socket permissions.
5. File descriptor passing (`SCM_RIGHTS`), gated behind explicit security review.
6. Socket shutdown (`shutdown`) and half-close semantics.
7. Concurrent connect/accept/close stress verification.

### Phase 5: Shared Memory and Message Queues

Shared memory controls:

- Explicit creation, mapping (`mmap`), resize, and unmapping (`munmap`) APIs.
- Reference-counted region lifetime management.
- Permission checks at creation and mapping time.
- Alignment, size, and overflow validation.
- Safe copy-in / copy-out boundary validations for user pointers.
- Documentation of cache-coherency and lock ordering invariants.

Message queue controls:

- Bounded queue capacities preventing kernel memory exhaustion.
- Explicit priority or FIFO message ordering.
- Blocking and non-blocking send (`mq_send`) and receive (`mq_receive`).
- Timeout support (`mq_timedsend` / `mq_timedreceive`).
- Queue teardown semantics and safe unlinking (`mq_unlink`).

---

## 4. Canonical Core IPC Objects & Core Invariants

### Core IPC Objects

```rust
pub struct IpcChannel {
    pub id: u64,
    pub permissions: IpcPermissions,
    pub statistics: IpcStatistics,
}

pub struct IpcEndpoint {
    pub channel_id: u64,
    pub is_reader: bool,
    pub is_writer: bool,
    pub non_blocking: bool,
}

pub struct MessageBuffer {
    pub capacity: usize,
    pub length: usize,
    pub read_offset: usize,
    pub write_offset: usize,
}

pub struct SharedMemoryRegion {
    pub id: u64,
    pub physical_address: usize,
    pub size: usize,
    pub ref_count: usize,
    pub permissions: IpcPermissions,
}

pub struct WaitQueue {
    pub pending_threads: usize,
}

pub struct FileDescriptorHandle {
    pub fd: i32,
    pub flags: u32,
    pub channel: Option<IpcEndpoint>,
}

pub struct IpcPermissions {
    pub uid: u32,
    pub gid: u32,
    pub mode: u16,
    pub capability_mask: u64,
    pub namespace_id: u64,
}

pub struct IpcStatistics {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub messages_count: u64,
    pub dropped_count: u64,
}
```

### Core Architectural Invariants

1. **Idempotent Close:** A file descriptor or IPC handle is closed exactly once; double-close or use-after-close returns `EBADF`.
2. **Endpoint Termination Safety:** An IPC channel cannot be written to or read from after both endpoints are closed.
3. **Reader Wakeup Contract:** A reader thread blocked on an empty pipe or queue MUST be woken when new data arrives or when all writer endpoints close.
4. **Writer Wakeup & Backpressure Contract:** A writer thread blocked on a full pipe or queue MUST be woken when buffer space becomes available or when all reader endpoints close (triggering `EPIPE` / `SIGPIPE`).
5. **Address Space Isolation:** Shared-memory mappings must strictly enforce lifetime reference counting, read/write/execute permission checks, and user-space address range isolation.
6. **No Indefinite Unblockable Wait:** Every wait operation must register a valid wakeup path and, where requested, respect a absolute or relative timeout limit.

---

## 5. Missing Components Matrix

| Component | Missing Work Description | PR Deliverable |
|---|---|---|
| **IPC Contract** | Formal semantics, limits, and status tracking | `docs(ipc): define IPC handler contract` |
| **Pipes** | Blocking, EOF, backpressure, poll readiness | `feat(ipc): anonymous pipe core` |
| **FIFOs** | Named pipe creation, permissions, VFS integration | `feat(ipc): FIFO support with VFS tests` |
| **UNIX Sockets** | Stream/datagram lifecycle and peer credentials | `feat(ipc): UNIX-domain socket core` |
| **Shared Memory** | Safe mapping, permissions, lifetime, and teardown | `feat(ipc): reference-counted shared memory` |
| **Message Queues** | Bounded queues, timeouts, and overflow policy | `feat(ipc): bounded message queues` |
| **Event Wakeups** | Eventfd-like notification and poll integration | `feat(ipc): event wakeup primitives` |
| **Descriptor Passing** | Secure FD transfer with capability checks | `feat(ipc): controlled file-descriptor passing` |
| **IPC Permissions** | Owner, group, capability, namespace enforcement | `feat(security): IPC access-control policy` |
| **Fuzzing** | Fuzz message parsing, socket paths, and control messages | `test(ipc): fuzz IPC input parsers` |
| **Observability** | Per-channel statistics, tracing, and leak detection | `feat(observability): IPC diagnostics` |
| **Compatibility Tests** | Linux/BSD behavioral parity matrix | `test(ipc): POSIX compatibility matrix` |

---

## 6. Industry-Standard Rules

### Correctness Rules

- Every IPC operation must define explicit behavior for blocking, timeout, signal interruption (`EINTR`), and endpoint closure.
- No kernel-side unbounded queue, buffer allocation, or recursion from user input.
- All user-provided lengths, pointers, flags, socket paths, and descriptors must be validated at the syscall boundary.
- Every resource acquisition (locks, physical pages, FD table entries) must have a deterministic release path, including error, cancellation, and panic unwind paths.
- Every concurrent operation must be verified against lost wakeups, spurious wakeups, and use-after-close races.
- No silent data truncation; return `EMSGSIZE` or explicitly document the byte truncation rule.

### Security Rules

- Apply least privilege by default across all IPC channels.
- Enforce permissions at channel creation, connection, mapping, and message delivery boundaries.
- Fail closed when capability, namespace, or ownership credentials are ambiguous or invalid.
- Treat file-descriptor passing as a security-critical boundary requiring explicit reviewer verification.
- Threat-model every new IPC mechanism against STRIDE (Spoofing, Tampering, Info Disclosure, DoS, Elevation of Privilege, Resource Exhaustion).
- Do not leak kernel virtual addresses, raw heap pointers, or internal object pointers to user space.

### Rust-Specific Rules

- Minimize `unsafe` code; every `unsafe` block must include a `// SAFETY:` comment detailing memory aliasing, lifetime guarantees, bounds initialization, and panic safety.
- Prefer ownership transfer (`Arc`, typed wrappers) over raw integer descriptors inside the kernel.
- Use checked arithmetic (`checked_add`, `checked_mul`) for buffer capacities, offsets, and reference counts.
- Avoid holding locks across user copy-in/copy-out operations (`copy_from_user` / `copy_to_user`).
- Explicitly document and enforce lock ordering for every nested IPC structure lock.

### Testing Rules

Every IPC pull request must include test coverage from this matrix:

- Unit tests for buffer, queue, permission, and state machine logic.
- Integration tests across process creation, threading, signals, VFS, and descriptor closing.
- Concurrency stress tests for simultaneous readers, writers, connectors, and closers.
- Timeout and signal cancellation tests.
- Fuzz tests for serialized messages, socket paths, control messages, and malformed inputs.
- Resource-exhaustion tests for descriptor limits, memory bounds, and queue capacities.
- Regression test attached to every fixed bug.

Verification commands:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

---

## 7. Pull Request Template & Example IPC PR

### PR Template

```markdown
## Summary

<!-- What changed and why? -->

## Component

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

## Changes

-

## Motivation

<!-- Link issue, roadmap item, or gap-analysis entry. -->

## IPC semantics

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

## Tests

- [ ] Unit tests
- [ ] Integration tests
- [ ] Concurrency/stress tests
- [ ] Timeout tests
- [ ] Fuzz tests
- [ ] Resource-exhaustion tests
- [ ] Regression test for fixed bug

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

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
```

### Example IPC PR

**Title:**
`feat(ipc): implement anonymous pipes with EOF, backpressure, and wakeup safety`

**Branch:**
`feat/ipc-anonymous-pipes`

**Commits:**
1. `feat(ipc): add IpcChannel and pipe endpoint abstractions`
2. `fix(ipc): prevent lost wakeup when pipe reader closes during blocking write`

**Body:**

```markdown
## Summary

Adds the first anonymous-pipe implementation for SigmaOS and fixes a
potential lost-wakeup race between a blocking writer and concurrent
reader closure.

## IPC semantics

- Blocking reads wait until data, EOF, or timeout.
- Blocking writes wait until buffer space, reader closure, or timeout.
- EOF is returned after all write ends are closed and buffered data is consumed.
- `EPIPE`-equivalent behavior is returned when all read ends are closed.
- Pipe capacity is bounded to prevent kernel memory exhaustion.

## Security review

- Buffer lengths are validated before allocation.
- Descriptor close is idempotent and reference-counted.
- No raw kernel pointers are exposed to user space.
- `unsafe` is limited to context-specific buffer handling and is documented.

## Tests

- Pipe create/read/write/close tests
- EOF and broken-pipe tests
- Non-blocking mode tests
- Concurrent reader/writer stress tests
- Reader-close-during-write regression test
- Capacity and backpressure tests

## Capability status

Thread lifecycle: Unit-tested
Anonymous pipes: Unit-tested
IPC runtime integration: Pending
```

---

## 8. First 10 IPC PRs Roadmap

1. `docs(ipc): define IPC handler contract, threat model, and acceptance criteria`
2. `feat(ipc): add canonical IpcChannel, endpoint, and descriptor-handle model`
3. `test(ipc): add descriptor lifecycle and use-after-close tests`
4. `feat(ipc): implement anonymous pipes with blocking and non-blocking modes`
5. `fix(ipc): add wait-queue wakeup safety for pipe reader/writer closure`
6. `feat(vfs): integrate FIFOs with permissions and open-mode validation`
7. `feat(ipc): add bounded message queues with timeout and overflow policy`
8. `feat(ipc): add event wakeup primitive and poll-readiness integration`
9. `feat(security): enforce IPC ownership, capability, and namespace checks`
10. `test(ipc): add concurrency, fuzz, and resource-exhaustion suites`

---

## 9. Governance & Capability Progression

### GitHub Issue & PR Labels

- `area:ipc`
- `area:pipes`
- `area:sockets`
- `area:shared-memory`
- `area:security`
- `kind:feature`
- `kind:bug`
- `kind:test`
- `kind:docs`
- `status:modeled`
- `status:unit-tested`
- `status:integrated`
- `status:hardware-tested`
- `needs-second-reviewer`

### Merge Approval Policy

- All code and spec changes arrive through Pull Requests; direct commits to `main` are strictly prohibited.
- Ordinary IPC changes require at least **one** maintainer review approval.
- High-risk changes (shared memory mappings, file-descriptor passing, syscall dispatching, access control, or physical memory allocation) require **two** maintainer approvals.
- All CI automated tests must pass before merging.
- Every behavior-changing PR must update unit/integration tests, documentation, and `FEATURE_STATUS.toml`.

### Capability Status Progression Criteria

| Status | Required Evidence & Criteria |
|---|---|
| `modeled` | Architectural plan published in `docs/` and `wiki/`, `FEATURE_STATUS.toml` entry registered. |
| `unit-tested` | Data structures, permissions, and state machine unit tests pass via `rustc --test`. |
| `integrated` | Connected to VFS, task scheduler, process lifecycle, and syscall dispatch path; passes integration test suite. |
| `hardware-tested` | Validated in end-to-end QEMU / real hardware boot-to-shell environment with live process communications. |
