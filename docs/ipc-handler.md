# SigmaOS IPC Handler Master Architecture, Specification, and Maintenance Program

## Overview

In SigmaOS, Inter-Process Communication (IPC) is a foundational operating system subsystem alongside thread/process lifecycle, signal delivery, memory management, and resource limits. Rather than treating IPC as an ad-hoc collection of isolated modules, the IPC Handler is specified and managed as a formally modeled, testable, and auditable subsystem with strict safety invariants, explicit capability tracking, and a recurring engineering cadence.

## Scope and Subsystem Boundaries

The IPC Handler is responsible for safe, high-performance message, event, and data transfer between processes, threads, and kernel services.

### Owned Components and Responsibilities

- **Pipes and Anonymous Pipes**: Unidirectional byte streams with bounded buffer capacity, blocking/non-blocking semantics, and wakeup safety.
- **FIFOs / Named Pipes**: Filesystem-bound named pipes integrated with VFS permissions and open-mode semantics.
- **UNIX-Domain Sockets**: Local stream and datagram IPC supporting peer credentials, path-based access control, and file descriptor passing.
- **Shared Memory Regions**: Reference-counted memory pages shared across address spaces with creation, mapping, overflow, and teardown safety.
- **Message Queues**: Bounded, priority or FIFO ordered message channels with timeout, backpressure, and resource limit enforcement.
- **Event Notification Primitives**: `eventfd`-like lightweight event objects for interrupt/async task wakeups and poll/epoll integration.
- **Signal Control Plane**: Delivery of POSIX and real-time signals as high-priority control-plane IPC messages.
- **File-Descriptor Passing**: Controlled transfer of file descriptor ownership across process boundaries with capability validation.
- **Permissions and Access Control**: Owner, group, capability, namespace, and sandbox rules for every channel creation, lookup, and transfer.

### Out-of-Scope (Initial Non-Goals)

- General network stack sockets (TCP/UDP/IP)
- GPU and display surface transport
- Package manager daemon transport
- Container runtime remote RPC engines

These external frameworks consume the core IPC Handler as an underlying transport mechanism.

---

## Periodic Task Cadence

To maintain long-term stability and prevent technical debt, IPC development follows a structured maintenance cadence:

| Period | IPC Handler Development | Cross-Component Maintenance | Required PR |
|---|---|---|---|
| **Daily** | Triage IPC bugs, deadlocks, leaked descriptors, and flaky concurrency tests | Triage CI, build failures, security reports, and documentation drift | Bug report or focused fix PR |
| **Weekly** | Implement one IPC primitive or semantic improvement | Dependency/toolchain updates, lint cleanup, test repair | One scoped PR |
| **Biweekly** | Run IPC integration tests with processes, threads, signals, VFS, and resource limits | Audit `FEATURE_STATUS.toml`, docs, and component ownership | Status/evidence PR |
| **Monthly** | Fuzz IPC parsers, stress concurrent channels, audit descriptor leaks and permission checks | Build release candidate, boot smoke test, installer/update check | Release-readiness PR |
| **Quarterly** | Review IPC model against Linux and BSD behavior; remove duplicate implementations and dead code | Security review, API stability review, roadmap consolidation | Architecture RFC plus cleanup PRs |

---

## Development Phases

### Phase 1: IPC Contract and Formal Specification
- Establish canonical semantics, buffer limits, permissions, error codes, and capability status tracking in `FEATURE_STATUS.toml`.

### Phase 2: Core IPC Objects
- Define canonical abstractions in `src/ipc/ipc.rs`: `IpcChannel`, `IpcEndpoint`, `MessageBuffer`, `SharedMemoryRegion`, `WaitQueue`, `FileDescriptorHandle`, `IpcPermissions`, and `IpcStatistics`.

### Phase 3: Pipes and FIFOs
- Implement anonymous pipes with blocking/non-blocking modes, EOF on writer closure, `EPIPE` on reader closure, backpressure, and VFS FIFO integration.

### Phase 4: UNIX-Domain Sockets
- Implement stream/datagram sockets, peer credentials, socket path permissions, half-close/shutdown, and gated file-descriptor passing.

### Phase 5: Shared Memory and Bounded Message Queues
- Safe multi-process shared memory mapping, reference-counted lifetime, copy-in/copy-out safety, bounded message queues, and non-blocking timeout support.

---

## Core IPC Objects and Invariants

The IPC subsystem relies on 8 canonical structures defined in `src/ipc/ipc.rs`:

1. `IpcChannel`: Shared IPC communication channel with bounded capacity and wait queue integration.
2. `IpcEndpoint`: Typed handle (`Reader` or `Writer`) representing a process boundary endpoint.
3. `MessageBuffer`: Bounded byte storage with bounds checking and zero-copy slicing capabilities.
4. `SharedMemoryRegion`: Reference-counted shared memory object with permission enforcement.
5. `WaitQueue`: Thread-safe waiter registry for blocking readers and writers.
6. `FileDescriptorHandle`: Managed wrapper ensuring file descriptor cleanup and single-close guarantees.
7. `IpcPermissions`: Owner UID/GID, mode flags, capability bitmask, and sandbox restrictions.
8. `IpcStatistics`: Diagnostic counter tracking bytes read/written, message counts, dropped packets, and active endpoints.

### Safety Invariants

- **Single Close Guarantee**: A file descriptor or IPC endpoint is closed exactly once.
- **Endpoint Teardown**: A channel cannot be written to or read from after both endpoints are dropped.
- **Wakeup Safety**: A reader blocked on an empty channel must be woken immediately when data is written or all write ends close.
- **Backpressure Safety**: A writer blocked on a full channel must be woken immediately when space is freed or all read ends close.
- **Address-Space Isolation**: Shared memory mappings validate bounds, alignment, and permissions prior to page table insertion.
- **Bounded Waiting**: Every wait operation on a kernel wait-queue supports explicit timeout and cancellation paths.

---

## Missing Components and PR Deliverable Matrix

| Component | Missing Work | PR Deliverable |
|---|---|---|
| IPC Contract | Formal semantics, limits, and status tracking | `docs(ipc): define IPC handler contract` |
| Pipes | Blocking, EOF, backpressure, poll readiness | `feat(ipc): anonymous pipe core` |
| FIFOs | Named pipe creation, permissions, VFS integration | `feat(ipc): FIFO support with VFS tests` |
| UNIX Sockets | Stream/datagram lifecycle and peer credentials | `feat(ipc): UNIX-domain socket core` |
| Shared Memory | Safe mapping, permissions, lifetime, and teardown | `feat(ipc): reference-counted shared memory` |
| Message Queues | Bounded queues, timeouts, and overflow policy | `feat(ipc): bounded message queues` |
| Event Wakeups | Eventfd-like notification and poll integration | `feat(ipc): event wakeup primitives` |
| Descriptor Passing | Secure FD transfer with capability checks | `feat(ipc): controlled file-descriptor passing` |
| IPC Permissions | Owner, group, capability, namespace enforcement | `feat(security): IPC access-control policy` |
| Fuzzing | Fuzz message parsing, socket paths, and control messages | `test(ipc): fuzz IPC input parsers` |
| Observability | Per-channel statistics, tracing, and leak detection | `feat(observability): IPC diagnostics` |
| Compatibility Tests | Linux/BSD behavioral parity matrix | `test(ipc): POSIX compatibility matrix` |

---

## Industry-Standard Rules

### Correctness Rules
- Every IPC operation must explicitly handle blocking, non-blocking, timeout, signal interruption, and closure.
- Kernel-side allocations driven by user IPC requests must be bounded by system limits (`RLIMIT_MSGQUEUE`, pipe buffer caps).
- All user input (pointers, lengths, flags, path strings, descriptor indices) must be validated before acquisition.
- Every resource acquisition path must have a corresponding drop/release path, including panic and error unwinding.

### Security Rules
- Enforce least privilege and check capabilities at creation, connection, and transfer boundaries.
- Fail closed if permissions, capability bits, or namespace ownership are ambiguous.
- File descriptor passing is treated as security-sensitive, requiring explicit validation and dual review.
- Never leak raw kernel addresses, spinlock structures, or internal memory pointers through IPC messages.

### Rust-Specific Rules
- Minimize `unsafe` blocks; every `unsafe` block must feature a `// SAFETY:` comment covering aliasing, lifetime, initialization, and panic safety.
- Prefer strong Rust types and ownership handles (`Arc`, `Mutex`, `Rc`) over raw integer file descriptors inside kernel logic.
- Use checked arithmetic (`checked_add`, `checked_mul`) for buffer offsets, channel capacity, and reference counts.
- Never hold kernel locks across user copy-in (`copy_from_user`) or copy-out (`copy_to_user`) routines.

### Testing Rules
Every IPC PR must include tests covering:
- Unit tests for queue, buffer, and state transitions.
- Concurrency stress tests with simultaneous readers, writers, and closers.
- Timeout and cancellation behavior.
- Fuzzing parsers and resource-exhaustion boundary tests.

---

## First 10 IPC Pull Requests Roadmap

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

## Governance and Merge Rules

### Mandatory Labels
`area:ipc`, `area:pipes`, `area:sockets`, `area:shared-memory`, `area:security`, `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`, `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`, `needs-second-reviewer`.

### Merge Policy
- All modifications enter via pull requests; direct pushes to `main` are strictly forbidden.
- Requires 1 maintainer approval for ordinary IPC enhancements.
- Requires 2 maintainer approvals for shared memory, file descriptor passing, syscall dispatch, or security policy changes.
- All automated CI checks (`cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`) must pass before merging.
- Updates to documentation and `FEATURE_STATUS.toml` must accompany all behavior changes.
