# IPC Handler Subsystem Specification and Maintenance Program

SigmaOS identifies IPC, signals, process lifecycle, and resource limits as core operating-system areas. The IPC handler is developed as a formally specified, testable subsystem rather than a collection of isolated modules. This specification establishes the IPC handler architecture, recurring maintenance program, scoped pull requests, security rules, and measurable acceptance criteria.

## Repository Context

SigmaOS is a Rust-based experimental OS repository with kernel, userspace, desktop, packaging, tooling, and documentation areas. The process-management architecture includes thread management, signals, IPC, and resource limits, making IPC a core integration point.

IPC is sequenced alongside and before advanced scheduler work, because scheduling, signals, file descriptors, and IPC all depend on reliable process/thread wakeup and synchronization semantics.

## IPC Handler Scope

The IPC handler is responsible for safe message and data transfer between processes, threads, and kernel services.

### Owned Components
- Pipes and anonymous pipes
- FIFOs / named pipes
- UNIX-domain sockets
- Shared-memory regions
- Message queues
- Event notification primitives (e.g. eventfd-like wakeup objects)
- Signal delivery as a control-plane IPC mechanism
- File-descriptor passing (gated behind descriptor lifecycle stability and explicit security review)
- Permission, capability, and namespace checks for every IPC channel

### Excluded Components (Initial Non-Goals)
- General network sockets (TCP/UDP/IP)
- Display/GPU transport
- Package-manager transport
- Container runtime communication

Keeping the core IPC scope focused prevents unsafe coupling while allowing higher-level subsystems to consume IPC primitives later.

## Periodic Task Cadence

| Period | IPC Handler Development | Cross-Component Maintenance | Required PR |
|---|---|---|---|
| **Daily** | Triage IPC bugs, deadlocks, leaked descriptors, and flaky concurrency tests | Triage CI, build failures, security reports, and documentation drift | Bug report or focused fix PR |
| **Weekly** | Implement one IPC primitive or semantic improvement | Dependency/toolchain updates, lint cleanup, test repair | One scoped PR |
| **Biweekly** | Run IPC integration tests with processes, threads, signals, VFS, and resource limits | Audit `FEATURE_STATUS.toml`, docs, and component ownership | Status/evidence PR |
| **Monthly** | Fuzz IPC parsers, stress concurrent channels, audit descriptor leaks and permission checks | Build release candidate, boot smoke test, installer/update check | Release-readiness PR |
| **Quarterly** | Review IPC model against Linux and BSD behavior; remove duplicate implementations and dead code | Security review, API stability review, roadmap consolidation | Architecture RFC plus cleanup PRs |

## Development Phases

### Phase 1: IPC Contract
- Documented in `docs/ipc-handler.md` and mirrored in `wiki/ipc-handler.md`.
- Registered in `FEATURE_STATUS.toml` under `[ipc.handler]`.
- Specifies exact semantics for blocking, non-blocking, pollable, and async behavior; message boundaries vs byte-streams; buffer limits and backpressure; error codes; permission models (owner, group, capability, namespace, sandbox); and status tracking.

### Phase 2: Core IPC Objects
Canonical internal representations implemented in `src/ipc/ipc.rs`:
- `IpcChannel`
- `IpcEndpoint`
- `MessageBuffer`
- `SharedMemoryRegion`
- `WaitQueue`
- `FileDescriptorHandle`
- `IpcPermissions`
- `IpcStatistics`

Core Invariants:
1. A descriptor is closed exactly once.
2. A channel cannot be used after both endpoints are closed.
3. A reader blocked on an empty pipe must be woken when data arrives or the writer closes.
4. A writer blocked on a full pipe must be woken when space becomes available or the reader closes.
5. Shared-memory mappings must enforce lifetime, permissions, and address-space isolation.
6. Every wait must have a wakeup path and, where appropriate, a timeout.

### Phase 3: Pipes and FIFOs
Implementation sequence:
1. Anonymous pipe creation and descriptor allocation
2. Byte-stream read/write semantics
3. Non-blocking mode
4. Pipe capacity and backpressure
5. End-of-file behavior when all write ends close
6. Broken-pipe (`EPIPE`) behavior when all read ends close
7. FIFO creation, open modes, permissions, and filesystem integration
8. Poll/select readiness semantics

### Phase 4: UNIX-Domain Sockets
- Socket creation, bind, listen, accept, connect, close
- Stream and datagram modes
- Peer credentials support
- Filesystem socket permissions
- Descriptor passing gated behind explicit security review
- Socket shutdown and half-close semantics
- Concurrent connect/accept/close stress tests

### Phase 5: Shared Memory and Message Queues
Shared Memory Controls:
- Explicit creation, mapping, resize, and unmapping APIs
- Reference-counted region lifetime
- Permission checks at creation and mapping time
- Alignment, size, and overflow validation
- Copy-in/copy-out boundaries for unsafe user pointers
- Cache-coherency and synchronization rules
- Optional guard pages or bounds checks

Message Queue Requirements:
- Bounded queues
- Explicit priority or FIFO ordering
- Blocking and non-blocking send/receive with timeout support
- Clean queue teardown semantics
- Bounded kernel memory allocations

## Missing Components Deliverable Matrix

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

Required Verification:
```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

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

## Governance and Merge Policy

### Labels
- `area:ipc`, `area:pipes`, `area:sockets`, `area:shared-memory`, `area:security`
- `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`
- `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`
- `needs-second-reviewer`

### Merge Rules
- All changes arrive through PRs; no direct commits to `main`.
- One maintainer approval for ordinary IPC changes.
- Two approvals for shared memory, descriptor passing, syscall dispatch, security policy, or kernel-memory changes.
- CI must pass before merge.
- Every behavior-changing PR must update tests, documentation, and `FEATURE_STATUS.toml`.
