# SigmaOS IPC Handler Master Architecture, Specification, and Maintenance Plan

## Subsystem Overview & Vision

SigmaOS identifies inter-process communication (IPC), signals, process lifecycle, and resource limits as core operating-system subsystems. The IPC handler in SigmaOS is developed as a formally specified, testable, and capability-gated subsystem rather than a collection of isolated modules.

This plan turns IPC development into a recurring engineering maintenance program with scoped pull requests, mandatory security rules, strict lock-ordering invariants, and measurable acceptance criteria.

---

## Subsystem Scope & Ownership Boundaries

### Owned Components

The IPC handler subsystem is responsible for safe message and data transfer between processes, threads, and kernel services. It owns:

- **Pipes and anonymous pipes**: In-memory byte-stream transfer with blocking, non-blocking, and backpressure support.
- **FIFOs / named pipes**: Filesystem-bound named pipe channels with VFS integration.
- **UNIX-domain sockets**: Stream (`SOCK_STREAM`) and datagram (`SOCK_DGRAM`) IPC sockets with peer credential passing.
- **Shared-memory regions**: Bounded, reference-counted shared memory mappings across address spaces with lifetime and access-control enforcement.
- **Message queues**: Priority and FIFO ordered bounded message queues with explicit timeout and overflow policies.
- **Event notification primitives**: `eventfd`-like event wakeups and `poll`/`epoll`/`kqueue` readiness primitives.
- **Signal delivery control plane**: Asynchronous signal multiplexing as a control-plane IPC mechanism.
- **File-descriptor passing**: Controlled transfer of file descriptors over UNIX-domain sockets, gated behind security review.
- **Permission, capability, and namespace checks**: Owner, group, capability bitmask, IPC namespace, and sandbox policy verification for every channel.

### Initially Excluded Components

To prevent unsafe coupling during core IPC handler development, the following are explicitly excluded from initial subsystem ownership:

- General network sockets (TCP/UDP/IP stack)
- Display / GPU transport (Wayland / DRM zero-copy display buffer passing)
- Package-manager transport
- Container runtime communication

---

## Periodic Task Cadence

IPC handler maintenance and feature delivery follow a structured engineering cadence:

| Period | IPC Handler Development | Cross-Component Maintenance | Required PR |
|---|---|---|---|
| **Daily** | Triage IPC bugs, deadlocks, leaked descriptors, and flaky concurrency tests | Triage CI, build failures, security reports, and documentation drift | Bug report or focused fix PR |
| **Weekly** | Implement one IPC primitive or semantic improvement | Dependency/toolchain updates, lint cleanup, test repair | One scoped PR |
| **Biweekly** | Run IPC integration tests with processes, threads, signals, VFS, and resource limits | Audit `FEATURE_STATUS.toml`, docs, and component ownership | Status/evidence PR |
| **Monthly** | Fuzz IPC parsers, stress concurrent channels, audit descriptor leaks and permission checks | Build release candidate, boot smoke test, installer/update check | Release-readiness PR |
| **Quarterly** | Review IPC model against Linux and BSD behavior; remove duplicate implementations and dead code | Security review, API stability review, roadmap consolidation | Architecture RFC plus cleanup PRs |

---

## Development Phases

### Phase 1: IPC Contract

Define the formal IPC contract, limits, and status tracking:

- **Supported Mechanisms**: Anonymous pipes, FIFOs, UNIX-domain sockets, shared memory, message queues, event notification objects, signal control plane.
- **Blocking & Non-Blocking**: Explicit `O_NONBLOCK` support returning `EAGAIN` / `WOULDBLOCK`.
- **Byte-Stream vs Message Boundaries**: Clear distinction between raw byte streams (pipes/stream sockets) and discrete message boundaries (message queues/datagram sockets).
- **Descriptor Ownership**: Atomic, idempotent reference-counted handle closure (`FileDescriptorHandle`).
- **Buffer Capacity & Backpressure**: Hard cap on kernel buffers (`MessageBuffer` capacity) to prevent memory exhaustion.
- **Error Codes**: Unified `IPCError` mapping (`Success`, `NotConnected`, `BufferFull`, `BufferEmpty`, `InvalidSize`, `PermissionDenied`, `EndpointClosed`, `NotFound`, `AlreadyExists`, `Timeout`, `InternalError`, `InvalidCapability`).
- **Permission Model**: Owner UID/GID, mode permissions (`IpcPermissions`), capability flags (`IPCCapability`), and namespace isolation (`IpcNamespace`).

### Phase 2: Core IPC Objects

Canonical internal representations implemented in `src/ipc/ipc.rs`:

- `IpcChannel`: Canonical IPC channel structure holding buffers, wait queues, permissions, and stats.
- `IpcEndpoint`: Endpoint handle bound to a channel ID with read/write capability masks.
- `MessageBuffer`: Bounded ring-buffer structure with zero-allocation pointer arithmetic.
- `SharedMemoryRegion`: Reference-counted shared memory handle with memory mapping metadata.
- `WaitQueue`: Thread wait/wakeup queue supporting `wake_one` and `wake_all`.
- `FileDescriptorHandle`: File descriptor wrapper tracking channel mapping and closure status.
- `IpcPermissions`: Access control metadata (owner UID/GID, permission mode bitmask, capabilities).
- `IpcStatistics`: Telemetry counter tracking bytes read/written, messages sent/received, and dropped messages.

#### Core Invariants

1. A descriptor is closed exactly once (idempotent close).
2. A channel cannot be used after both endpoints are closed.
3. A reader blocked on an empty pipe must be woken when data arrives or when the writer closes.
4. A writer blocked on a full pipe must be woken when buffer space becomes available or when the reader closes.
5. Shared-memory mappings must enforce lifetime, permissions, and address-space isolation.
6. Every wait operation must have a wakeup path and, where appropriate, a timeout.

### Phase 3: Pipes and FIFOs

Implementation order:
1. Anonymous pipe creation and descriptor allocation.
2. Byte-stream read/write semantics.
3. Non-blocking mode (`O_NONBLOCK`).
4. Bounded pipe capacity and backpressure.
5. End-of-file (EOF) behavior when all write ends close and buffer is empty.
6. Broken-pipe behavior (`EPIPE` / `SIGPIPE`) when writing with all read ends closed.
7. FIFO creation (`mkfifo`), filesystem mode permissions, and VFS integration.
8. Poll/select readiness semantics (`POLLIN`, `POLLOUT`, `POLLHUP`).

### Phase 4: UNIX-Domain Sockets

UNIX-domain socket deliverables:
- Socket creation (`socketpair`, `socket`), `bind`, `listen`, `accept`, `connect`, `close`.
- Stream (`SOCK_STREAM`) and datagram (`SOCK_DGRAM`) modes.
- Peer credentials passing (`SO_PEERCRED` / `ucred`).
- Filesystem socket path permission checks.
- Controlled file-descriptor passing (`SCM_RIGHTS`), gated behind capability review.
- Socket shutdown (`shutdown`) and half-close semantics.
- Concurrent connect/accept/close stress verification.

### Phase 5: Shared Memory and Message Queues

Shared Memory Requirements:
- Explicit creation, mapping, resize, and unmapping APIs.
- Reference-counted region lifetime management.
- Permission checks at creation and mapping time.
- Alignment, size, and overflow validation.
- Copy-in / copy-out boundaries for unsafe user pointers.
- Cache-coherency and synchronization documentation.

Message Queue Requirements:
- Bounded queues with hard limits on message count and total byte size.
- Priority and FIFO ordering support.
- Blocking and non-blocking send/receive modes.
- Timeout support on send/receive.
- Clean queue teardown and waiter wakeup semantics.
- No unbounded kernel memory allocation from user requests.

---

## Missing Components Matrix

| Component | Missing Work | PR Deliverable |
|---|---|---|
| IPC contract | Formal semantics, limits, and status tracking | `docs(ipc): define IPC handler contract` |
| Pipes | Blocking, EOF, backpressure, poll readiness | `feat(ipc): anonymous pipe core` |
| FIFOs | Named pipe creation, permissions, VFS integration | `feat(ipc): FIFO support with VFS tests` |
| UNIX sockets | Stream/datagram lifecycle and peer credentials | `feat(ipc): UNIX-domain socket core` |
| Shared memory | Safe mapping, permissions, lifetime, and teardown | `feat(ipc): reference-counted shared memory` |
| Message queues | Bounded queues, timeouts, and overflow policy | `feat(ipc): bounded message queues` |
| Event wakeups | Eventfd-like notification and poll integration | `feat(ipc): event wakeup primitives` |
| Descriptor passing | Secure FD transfer with capability checks | `feat(ipc): controlled file-descriptor passing` |
| IPC permissions | Owner, group, capability, namespace enforcement | `feat(security): IPC access-control policy` |
| Fuzzing | Fuzz message parsing, socket paths, and control messages | `test(ipc): fuzz IPC input parsers` |
| Observability | Per-channel statistics, tracing, and leak detection | `feat(observability): IPC diagnostics` |
| Compatibility tests | Linux/BSD behavioral parity matrix | `test(ipc): POSIX compatibility matrix` |

---

## Industry-Standard Rules

### Correctness Rules

- Every IPC operation must define explicit behavior for blocking, timeout, interruption, and closure.
- No kernel-side unbounded queue, buffer, allocation, or recursion from user input.
- All user-provided lengths, pointers, flags, paths, and descriptors must be validated at boundary entry.
- Every resource acquisition must have a defined release path, including error and panic unwind paths.
- Every concurrent operation must be tested for lost wakeups, spurious wakeups, and use-after-close races.
- No silent data truncation; return an explicit error or document exact truncation rules.

### Security Rules

- Apply least privilege by default.
- Enforce permissions at channel creation, connection, mapping, and message-delivery boundaries.
- Fail closed when capability, namespace, or ownership checks are ambiguous or missing.
- Treat file-descriptor passing as security-sensitive and require two maintainer approvals.
- Threat-model every new IPC mechanism: spoofing, tampering, information disclosure, denial of service, elevation of privilege, and resource exhaustion.
- Do not expose kernel addresses, raw pointers, or internal object identities through IPC interfaces.

### Rust-Specific Rules

- Minimize `unsafe`; every `unsafe` block requires an explicit `// SAFETY:` comment covering aliasing, lifetime, initialization, and panic behavior.
- Prefer ownership transfer and typed handles over raw integer descriptors inside kernel structures.
- Use checked arithmetic (`checked_add`, `checked_mul`) for sizes, offsets, capacities, and reference counts.
- Avoid holding locks across copy-in/copy-out operations where possible.
- Explicitly document lock ordering for every shared IPC structure.

### Testing Rules

Every IPC PR must include relevant tests:
- Unit tests for buffer, queue, permission, and state logic.
- Integration tests across process creation, threading, signals, and descriptor closure.
- Concurrency stress tests for simultaneous readers, writers, connectors, and closers.
- Timeout and cancellation tests.
- Fuzz tests for serialized messages, socket paths, control data, and malformed requests.
- Resource-exhaustion tests for descriptor, memory, and queue limits.
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

## First 10 IPC PRs Roadmap

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

## Governance & Capability Status

### PR Labels
- `area:ipc`, `area:pipes`, `area:sockets`, `area:shared-memory`, `area:security`
- `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`
- `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`
- `needs-second-reviewer`

### Merge Policy
- All changes arrive through PRs; no direct commits to `main`.
- One maintainer approval for ordinary IPC changes.
- Two approvals for shared memory, descriptor passing, syscall dispatch, security policy, or kernel memory changes.
- CI must pass before merge.
- Every behavior-changing PR must update tests, documentation, and `FEATURE_STATUS.toml`.
