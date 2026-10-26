# SigmaOS IPC Handler Master Architecture & Specification

## Overview & Boundary Definition

The SigmaOS IPC Handler is the formally specified, safe message and data transfer component operating between processes, threads, and kernel services. In SigmaOS, thread management, signals, IPC, and resource limits are tightly coupled core operating-system areas. Developing the IPC handler as a formally specified, testable subsystem ensures clean boundaries and predictable synchronization semantics prior to and alongside advanced scheduler work.

### Subsystem Boundaries & Scope

The IPC Handler **owns**:
- **Pipes and anonymous pipes**: Bounded byte-stream communication channels with reader/writer endpoint tracking.
- **FIFOs / named pipes**: Filesystem-backed named IPC streams with VFS mode and permission integration.
- **UNIX-domain sockets**: Stream and datagram local socket channels supporting peer credentials and half-close semantics.
- **Shared-memory regions**: Reference-counted POSIX/SysV/custom zero-copy shared memory mappings enforcing address-space isolation.
- **Message queues**: Priority and FIFO-ordered bounded kernel message queues with non-blocking and timeout support.
- **Event notification primitives**: `eventfd`-like counter/event notification objects with poll-readiness integration.
- **Signal delivery as a control-plane IPC mechanism**: Signal generation, queued real-time signals, and mask/pending bitset state.
- **File-descriptor passing**: Controlled transfer of open file handle references across UNIX sockets, gated by strict capability checks.
- **Permission, capability, and namespace checks**: Fine-grained access control policy evaluation per channel creation, connection, and transfer.

The IPC Handler **does not initially own**:
- General network sockets (TCP/UDP/IP protocol stacks).
- Display/GPU transport (Wayland/Direct Rendering Manager buffers).
- Package-manager transport (daemon RPC).
- Container runtime communication.

---

## Periodic Task Cadence

| Period | IPC Handler Development | Cross-Component Maintenance | Required PR |
|---|---|---|---|
| **Daily** | Triage IPC bugs, deadlocks, leaked descriptors, and flaky concurrency tests | Triage CI, build failures, security reports, and documentation drift | Bug report or focused fix PR |
| **Weekly** | Implement one IPC primitive or semantic improvement | Dependency/toolchain updates, lint cleanup, test repair | One scoped PR |
| **Biweekly** | Run IPC integration tests with processes, threads, signals, VFS, and resource limits | Audit `FEATURE_STATUS.toml`, docs, and component ownership | Status/evidence PR |
| **Monthly** | Fuzz IPC parsers, stress concurrent channels, audit descriptor leaks and permission checks | Build release candidate, boot smoke test, installer/update check | Release-readiness PR |
| **Quarterly** | Review IPC model against Linux and BSD behavior; remove duplicate implementations and dead code | Security review, API stability review, roadmap consolidation | Architecture RFC plus cleanup PRs |

---

## Development Phases & Deliverables

### Phase 1: IPC Contract
- Formally document IPC semantics, failure modes, buffer backpressure, descriptor lifecycles, and capabilities.
- Add entry to `FEATURE_STATUS.toml` under `[ipc.handler]`.
- Enforce strict verification gates (`cargo fmt --check`, `cargo clippy`, `cargo check`, `cargo test`, `./run_sigma_tests.sh`).

### Phase 2: Core IPC Objects
Establish canonical internal representations:
- `IpcChannel`: Core channel container with reader/writer state, buffer, and wait queues.
- `IpcEndpoint`: Typed endpoint handle representing read, write, or bidirection connection ports.
- `MessageBuffer`: Bounded FIFO ring buffer with zero-allocation byte transfers.
- `SharedMemoryRegion`: Reference-counted shared memory object with protection flags (Read/Write/Exec).
- `WaitQueue`: Thread wait-queue for blocking readers/writers with atomic wakeups.
- `FileDescriptorHandle`: Reference-counted handle representing open kernel file descriptors.
- `IpcPermissions`: Owner UID, Group GID, Mode bits, and Capability flags.
- `IpcStatistics`: Telemetry counters for bytes sent/received, messages processed, and waiter counts.

#### Core Invariants:
1. A descriptor is closed exactly once.
2. A channel cannot be used after both endpoints are closed.
3. A reader blocked on an empty pipe must be woken when data arrives or the writer closes.
4. A writer blocked on a full pipe must be woken when space becomes available or the reader closes.
5. Shared-memory mappings must enforce lifetime, permissions, and address-space isolation.
6. Every wait must have a wakeup path and, where appropriate, a timeout.

### Phase 3: Pipes and FIFOs
1. Anonymous pipe creation (`pipe`/`pipe2`) and descriptor allocation.
2. Byte-stream read/write semantics.
3. Non-blocking mode (`O_NONBLOCK`).
4. Pipe capacity and backpressure limits (default 64 KB capacity).
5. End-of-file (EOF) behavior when all write ends close (readers receive 0 bytes).
6. Broken-pipe (`EPIPE` / `SIGPIPE`) behavior when writing after all read ends close.
7. FIFO creation (`mkfifo`), open modes, permissions, and filesystem integration.
8. Poll/select readiness semantics (`POLLIN`, `POLLOUT`, `POLLHUP`, `POLLERR`).

### Phase 4: UNIX-Domain Sockets
1. Socket creation (`AF_UNIX` / `AF_LOCAL`), bind, listen, accept, connect, close.
2. `SOCK_STREAM` and `SOCK_DGRAM` modes.
3. Peer credentials (`SO_PEERCRED` / `ucred` containing UID, GID, PID).
4. Filesystem socket permissions.
5. Descriptor passing (`SCM_RIGHTS`), gated behind explicit security review.
6. Socket shutdown (`SHUT_RD`, `SHUT_WR`, `SHUT_RDWR`) and half-close semantics.
7. Concurrent connect/accept/close stress tests.

### Phase 5: Shared Memory and Message Queues
1. **Shared Memory**:
   - Creation, mapping (`mmap`), resize (`ftruncate`), and unmapping (`munmap`) APIs.
   - Reference-counted region lifetime management.
   - Permission checks at creation and mapping time.
   - Alignment, size, and overflow validation.
   - Copy-in/copy-out boundaries for unsafe user pointers.
   - Cache-coherency and synchronization documentation.
2. **Message Queues**:
   - Bounded queues with explicit maximum message count and message size.
   - Priority-ordered or FIFO message retrieval.
   - Blocking and non-blocking send/receive (`mq_send`, `mq_timedsend`, `mq_receive`, `mq_timedreceive`).
   - Timeout support via `timespec`.
   - Queue teardown semantics and resource leak protection.

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

## Industry-Standard Rules & Guidelines

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
- Treat file-descriptor passing as security-sensitive and require two maintainer approvals.
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
- Unit tests for buffer, queue, permission, and state logic.
- Integration tests across process creation, threading, signals, and descriptor closure.
- Concurrency stress tests for simultaneous readers, writers, connectors, and closers.
- Timeout and cancellation tests.
- Fuzz tests for serialized messages, socket paths, control data, and malformed requests.
- Resource-exhaustion tests for descriptor, memory, and queue limits.
- Regression test attached to every fixed bug.

---

## Initial 10-PR Roadmap

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
- **Merge Rules**:
  - All changes arrive through Pull Requests; no direct commits to `main`.
  - One maintainer approval for ordinary IPC changes.
  - Two maintainer approvals for shared memory, descriptor passing, syscall dispatch, security policy, or kernel-memory changes.
  - CI must pass cleanly before merge (`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`).
  - Every behavior-changing PR must update tests, documentation, and `FEATURE_STATUS.toml`.
