# SigmaOS IPC Handler Master Architecture and Maintenance Plan

SigmaOS identifies IPC, signals, process lifecycle, and resource limits as core operating-system areas. The IPC handler is developed as a formally specified, testable subsystem rather than a collection of isolated modules. This document turns IPC work into a recurring maintenance program with scoped pull requests, security rules, and measurable acceptance criteria.

## Repository Context

SigmaOS is a Rust-based experimental OS repository with kernel, userspace, desktop, packaging, tooling, and documentation areas; its README explicitly distinguishes code presence and unit tests from a proven bootable, installable OS. Its process-management architecture includes thread management, signals, IPC, and resource limits, making IPC a high-priority integration point.

IPC work is sequenced **before or alongside** advanced scheduler work because scheduling, signals, file descriptors, and IPC all depend on reliable process/thread wakeup and synchronization semantics.

## IPC Handler Scope

The IPC handler is the component responsible for safe message and data transfer between processes, threads, and kernel services.

### Owned Components
- Pipes and anonymous pipes
- FIFOs / named pipes
- UNIX-domain sockets
- Shared-memory regions
- Message queues
- Event notification primitives, such as eventfd-like wakeup objects
- Signal delivery as a control-plane IPC mechanism
- File-descriptor passing, only after descriptor lifecycle rules are stable
- Permission, capability, and namespace checks for every IPC channel

### Excluded Components (Initial Non-Goals)
- General network sockets
- Display/GPU transport
- Package-manager transport
- Container runtime communication

These components can consume the IPC handler later, but keeping the first implementation narrow avoids unsafe coupling.

## Periodic Task Cadence

| Period | IPC Handler Development | Cross-Component Maintenance | Required PR |
|---|---|---|---|
| Daily | Triage IPC bugs, deadlocks, leaked descriptors, and flaky concurrency tests | Triage CI, build failures, security reports, and documentation drift | Bug report or focused fix PR |
| Weekly | Implement one IPC primitive or semantic improvement | Dependency/toolchain updates, lint cleanup, test repair | One scoped PR |
| Biweekly | Run IPC integration tests with processes, threads, signals, VFS, and resource limits | Audit `FEATURE_STATUS.toml`, docs, and component ownership | Status/evidence PR |
| Monthly | Fuzz IPC parsers, stress concurrent channels, audit descriptor leaks and permission checks | Build release candidate, boot smoke test, installer/update check | Release-readiness PR |
| Quarterly | Review IPC model against Linux and BSD behavior; remove duplicate implementations and dead code | Security review, API stability review, roadmap consolidation | Architecture RFC plus cleanup PRs |

## Development Phases

### Phase 1: IPC Contract
Create `docs/ipc-handler.md` and add an IPC entry to `FEATURE_STATUS.toml`.

Specify:
- Supported IPC mechanisms and their exact semantics
- Blocking, non-blocking, pollable, and asynchronous behavior
- Byte-stream versus message-boundary semantics
- Descriptor ownership and close semantics
- Buffer limits, backpressure, and queue capacity
- Error codes and failure behavior
- Permission model: owner, group, capability, namespace, and sandbox rules
- What is implemented, modeled, integrated, and hardware-tested

Acceptance criteria:
- `cargo check --lib` passes
- `cargo test --lib` passes
- `./run_sigma_tests.sh` passes
- Every public API has documentation and tests
- No claim of POSIX/Linux/BSD compatibility without a passing compatibility test matrix

### Phase 2: Core IPC Objects
Implement a canonical internal representation:
- `IpcChannel`
- `IpcEndpoint`
- `MessageBuffer`
- `SharedMemoryRegion`
- `WaitQueue`
- `FileDescriptorHandle`
- `IpcPermissions`
- `IpcStatistics`

Core invariants:
- A descriptor is closed exactly once.
- A channel cannot be used after both endpoints are closed.
- A reader blocked on an empty pipe must be woken when data arrives or the writer closes.
- A writer blocked on a full pipe must be woken when space becomes available or the reader closes.
- Shared-memory mappings must enforce lifetime, permissions, and address-space isolation.
- Every wait must have a wakeup path and, where appropriate, a timeout.

### Phase 3: Pipes and FIFOs
Implement in this order:
1. Anonymous pipe creation and descriptor allocation
2. Byte-stream read/write semantics
3. Non-blocking mode
4. Pipe capacity and backpressure
5. End-of-file behavior when all write ends close
6. Broken-pipe behavior when all read ends close
7. FIFO creation, open modes, permissions, and filesystem integration
8. Poll/select readiness semantics

Example PR: `feat(ipc): implement anonymous pipes with blocking, EOF, and backpressure tests`

### Phase 4: UNIX-Domain Sockets
Add UNIX-domain sockets only after pipes are stable.

Deliverables:
- Socket creation, bind, listen, accept, connect, close
- Stream and datagram modes
- Peer credentials, where supported
- Filesystem socket permissions
- Descriptor passing, gated behind explicit security review
- Socket shutdown and half-close semantics
- Concurrent connect/accept/close stress tests

Example PR: `feat(ipc): add UNIX-domain stream sockets with permission and lifecycle tests`

### Phase 5: Shared Memory and Message Queues
Shared memory is the highest-risk IPC mechanism because it crosses address-space boundaries.

Required controls:
- Explicit creation, mapping, resize, and unmapping APIs
- Reference-counted region lifetime
- Permission checks at creation and mapping time
- Alignment, size, and overflow validation
- Copy-in/copy-out boundaries for unsafe user pointers
- Cache-coherency and synchronization documentation
- Optional guard pages or bounds checks during development

Message queues should provide:
- Bounded queues
- Priority or FIFO ordering, explicitly chosen
- Blocking and non-blocking send/receive
- Timeout support
- Queue teardown semantics
- No unbounded kernel memory growth from user requests

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

Required verification commands:
```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

## Pull Request Template

The canonical PR template for IPC and core subsystem changes is located at `.github/PULL_REQUEST_TEMPLATE.md`.

## First 10 IPC PRs

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

## Governance

### Labels
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

### Merge Policy
- All changes arrive through PRs; no direct commits to `main`.
- One maintainer approval for ordinary IPC changes.
- Two approvals for shared memory, descriptor passing, syscall dispatch, security policy, or kernel-memory changes.
- CI must pass before merge.
- Every behavior-changing PR must update tests, documentation, and `FEATURE_STATUS.toml`.
