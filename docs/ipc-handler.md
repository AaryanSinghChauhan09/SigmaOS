# SigmaOS Inter-Process Communication (IPC) Handler Contract & Architecture Specification

## Overview

The SigmaOS IPC Handler provides safe, secure, high-performance message and data transfer mechanisms between processes, threads, and kernel services. Rather than functioning as an ad-hoc collection of isolated communication utilities, the IPC Handler is a formally specified, testable subsystem with defined semantics, security controls, buffer limits, and capability status tracking.

---

## Scope & Boundaries

### In-Scope IPC Mechanisms
The IPC Handler owns and manages:

1. **Anonymous Pipes**: Unidirectional byte-stream channels between processes with parent-child relationships or shared file descriptors.
2. **FIFOs / Named Pipes**: Filesystem-bound unidirectional or bidirectional byte-stream channels accessible across unrelated processes.
3. **UNIX-Domain Sockets (`AF_UNIX`)**: Local IPC supporting byte-stream (`SOCK_STREAM`), datagram (`SOCK_DGRAM`), and sequential packet modes, with peer credentials (`SO_PEERCRED`).
4. **Shared-Memory Regions (`shm`)**: Memory-mapped pages shared between address spaces with reference-counted lifetime and access permission controls.
5. **Message Queues**: Bounded, priority-ordered or FIFO message passing queues (`posix_mq` / `sysv_mq`).
6. **Event Notification Primitives (`eventfd`)**: Counter/event wakeup objects for event notification and async poll integration.
7. **Signal Delivery Control Plane**: Process control-plane signal notifications (`sigqueue`, `signalfd`).
8. **File Descriptor Passing**: Secure descriptor transfer over UNIX-domain sockets with capability verification.
9. **IPC Access Control**: Permission, capability (`CAP_IPC_LOCK`, `CAP_IPC_OWNER`), and namespace (`ipc_namespace`) checks on every IPC object.

### Explicitly Excluded (Out-of-Scope Initial Targets)
To prevent unsafe coupling and preserve modular isolation, the following transports consume the IPC handler but are **not** managed directly by the core IPC subsystem:

- Network sockets (`AF_INET`, `AF_INET6`)
- Display/GPU transport (`virtio-gpu`, DRM/KMS buffer sharing)
- Package-manager transport
- Container runtime communication bridge

---

## Development Phases

### Phase 1: IPC Contract
Establish formal contract in `docs/ipc-handler.md` and `FEATURE_STATUS.toml`. Specify exact semantics, limits, security controls, failure modes, and capability tracking.

### Phase 2: Core IPC Objects
Implement canonical internal representations:
- `IpcChannel`: Abstract base channel representing an active communication link.
- `IpcEndpoint`: Read or write end of an IPC channel.
- `MessageBuffer`: Bounded ring or linear buffer for data payload.
- `SharedMemoryRegion`: Reference-counted shared memory segment mapped across address spaces.
- `WaitQueue`: Thread wait queue for blocking I/O and event notifications.
- `FileDescriptorHandle`: Reference-counted handle wrapper around kernel resources.
- `IpcPermissions`: Owner, group, mode, and capability rulesets.
- `IpcStatistics`: Operational counters for telemetry, leak detection, and diagnostics.

**Core Invariants**:
- A descriptor is closed exactly once.
- A channel cannot be used after both endpoints are closed.
- A reader blocked on an empty pipe must be woken when data arrives or the writer closes.
- A writer blocked on a full pipe must be woken when space becomes available or the reader closes.
- Shared-memory mappings must enforce lifetime, permissions, and address-space isolation.
- Every wait must have a wakeup path and, where appropriate, a timeout.

### Phase 3: Pipes and FIFOs
Implement in sequence:
1. Anonymous pipe creation and descriptor allocation
2. Byte-stream read/write semantics
3. Non-blocking mode (`O_NONBLOCK`)
4. Pipe capacity and backpressure (64 KiB default capacity)
5. End-of-file behavior when all write ends close
6. Broken-pipe behavior (`EPIPE` / `SIGPIPE`) when all read ends close
7. FIFO creation, open modes, permissions, and VFS integration
8. Poll/select readiness semantics (`POLLIN`, `POLLOUT`, `POLLHUP`)

### Phase 4: UNIX-Domain Sockets
Deliverables following pipe stabilization:
- Socket creation, bind, listen, accept, connect, close
- Stream (`SOCK_STREAM`) and datagram (`SOCK_DGRAM`) modes
- Peer credentials (`SO_PEERCRED`)
- Filesystem socket permissions
- Controlled file-descriptor passing (`SCM_RIGHTS`)
- Socket shutdown and half-close semantics
- Concurrent connect/accept/close stress tests

### Phase 5: Shared Memory and Message Queues
- **Shared Memory Controls**: Explicit creation, mapping, resize, unmapping APIs; reference-counted region lifetime; permission checks at creation and mapping time; alignment, size, and overflow validation; copy-in/copy-out boundaries for unsafe user pointers; cache-coherency documentation; guard pages.
- **Message Queues**: Bounded queues; priority or FIFO ordering; blocking and non-blocking send/receive; timeout support; queue teardown semantics; bounded kernel memory allocations.

---

## Operating Semantics

### 1. Blocking vs Non-Blocking vs Pollable Behavior
- **Blocking Mode**:
  - `read` on empty channel blocks until data arrives, EOF is signaled, or timeout expires.
  - `write` on full channel blocks until buffer space becomes available or all readers close.
  - `recv` / `msgrcv` blocks until a matching message is enqueued.
- **Non-Blocking Mode (`O_NONBLOCK`)**:
  - Returns `EWOULDBLOCK` / `EAGAIN` immediately if the operation cannot proceed without blocking.
- **Poll / Select Integration (`POLLIN`, `POLLOUT`, `POLLERR`, `POLLHUP`)**:
  - All IPC endpoints register wait nodes with `WaitQueue`.
  - `POLLIN` raised when readable bytes / messages > 0 or EOF reached.
  - `POLLOUT` raised when write buffer has space exceeding low-watermark threshold.
  - `POLLHUP` raised when peer endpoint disconnects or writes end closes.

### 2. Byte-Stream vs Message-Boundary Semantics
- **Byte-Stream (Pipes, FIFOs, `SOCK_STREAM`)**: No message boundary preservation. Partial reads and writes are valid and standard.
- **Message-Boundary (`SOCK_DGRAM`, `posix_mq`)**: Explicit message boundary preservation. If read buffer is smaller than message size, excess bytes are truncated or error `EMSGSIZE` is returned.

### 3. Descriptor Ownership & Lifecycle Invariants
- **Single-Close Invariant**: A file descriptor handle is closed exactly once. Duplicate close calls return `EBADF`.
- **Peer Closure / Disconnection**:
  - When all write ends of a pipe/FIFO close, readers observe `EOF` (0 bytes read).
  - When all read ends close, subsequent writes fail with `EPIPE` and send `SIGPIPE` to the writing thread.
  - Reference-counted descriptors ensure underlying channels remain allocated until the last handle drops (`Arc` cleanup).

### 4. Memory Bounds & Kernel Protection
- **No Unbounded Memory Allocation**: All queues, pipes, and buffers enforce static or runtime-capped upper limits.
- **Copy Boundary**: All transfer operations across user-kernel boundaries use explicit copy-in/copy-out routines (`copy_from_user`, `copy_to_user`) with page fault safety guards.

---

## Missing Components Matrix

| Component | Missing Work | PR Deliverable |
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

## Industry-Standard Rules

### Correctness Rules
- Every IPC operation must define behavior for blocking, timeout, interruption, and closure.
- No kernel-side unbounded queue, buffer, allocation, or recursion from user input.
- All user-provided lengths, pointers, flags, paths, and descriptors must be validated at the boundary.
- Every resource acquisition must have a defined release path, including error and panic paths.
- Every concurrent operation must be tested for lost wakeups, spurious wakeups, and use-after-close.
- No silent data truncation; return an explicit error or document the exact truncation rule.

### Security Rules
- Apply least privilege by default.
- Enforce permissions at channel creation, connection, mapping, and message-delivery boundaries.
- Fail closed when capability, namespace, or ownership checks are ambiguous.
- Treat file-descriptor passing as security-sensitive and require two approvals.
- Threat-model every new IPC mechanism: spoofing, tampering, information disclosure, denial of service, elevation of privilege, and resource exhaustion.
- Do not expose kernel addresses, raw pointers, or internal object identities through IPC.

### Rust-Specific Rules
- Minimize `unsafe`; every `unsafe` block requires a `// SAFETY:` comment covering aliasing, lifetime, initialization, and panic behavior.
- Prefer ownership transfer and typed handles over raw integer descriptors inside the kernel.
- Use checked arithmetic for sizes, offsets, capacities, and reference counts.
- Avoid holding locks across copy-in/copy-out operations where possible.
- Document lock ordering for every shared IPC structure.

### Testing Rules
Every IPC PR must include relevant tests from this matrix:
- Unit tests for buffer, queue, permission, and state logic
- Integration tests across process creation, threading, signals, and descriptor closure
- Concurrency stress tests for simultaneous readers, writers, connectors, and closers
- Timeout and cancellation tests
- Fuzz tests for serialized messages, socket paths, control data, and malformed requests
- Resource-exhaustion tests for descriptor, memory, and queue limits
- Regression test attached to every fixed bug

---

## Periodic Maintenance Program & Cadence

| Cadence | IPC Development | Cross-Component Maintenance | Required PR |
|---|---|---|---|
| **Daily** | Triage IPC bugs, deadlocks, descriptor leaks, flaky concurrency tests | Triage CI, build failures, security reports, doc drift | Bug fix PR |
| **Weekly** | Implement one IPC primitive or semantic improvement | Toolchain updates, lint cleanup, test repairs | Scoped Feature PR |
| **Biweekly** | Run IPC integration tests with processes, threads, signals, VFS | Audit `FEATURE_STATUS.toml`, docs, component ownership | Status/Evidence PR |
| **Monthly** | Fuzz IPC parsers, stress concurrent channels, audit descriptor leaks | Build release candidate, boot smoke test | Release-readiness PR |
| **Quarterly** | Review IPC model against Linux/BSD behavior; clean dead code | Security review, API stability review, roadmap consolidation | Architecture RFC + Cleanup |

---

## First 10 IPC PRs Sequence Roadmap

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

## Governance & Merge Policy

- **Labels**: `area:ipc`, `area:pipes`, `area:sockets`, `area:shared-memory`, `area:security`, `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`, `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`, `needs-second-reviewer`.
- **Merge Policy**:
  - All changes arrive through PRs; no direct commits to `main`.
  - One maintainer approval for ordinary IPC changes.
  - Two approvals for shared memory, descriptor passing, syscall dispatch, security policy, or kernel-memory changes.
  - CI must pass before merge (`cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`).
  - Every behavior-changing PR must update tests, documentation, and `FEATURE_STATUS.toml`.

---

## Capability Status Tracking

| Mechanism | Primitive Status | System Integration | Verification Level |
|---|---|---|---|
| **Anonymous Pipes** | Modeled & Specified | VFS / Process Planned | Architecture Specified |
| **FIFOs / Named Pipes** | Modeled & Specified | VFS Planned | Architecture Specified |
| **UNIX Sockets (`AF_UNIX`)** | Modeled & Specified | Socket VFS Subsystem Planned | Architecture Specified |
| **Shared Memory** | Modeled & Specified | Memory Manager (`mm`) Planned | Architecture Specified |
| **Message Queues** | Modeled & Specified | IPC Subsystem Planned | Architecture Specified |
| **Futex / Eventfd** | Implemented & Unit-Tested | Kernel Scheduler & WaitQueue | Unit-Tested |
| **Signal Delivery Control** | Implemented & Unit-Tested | POSIX Compat Layer | Unit-Tested |
