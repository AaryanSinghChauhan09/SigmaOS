# SigmaOS IPC Handler Master Architecture, Support Matrix, and Program Plan

SigmaOS identifies IPC, signals, process lifecycle, and resource limits as core operating-system areas. The IPC handler is developed as a formally specified, testable subsystem rather than a collection of isolated modules. This plan establishes a recurring maintenance program with scoped pull requests, security rules, and measurable acceptance criteria.

## Repository Context and Scope

SigmaOS is a Rust-based experimental OS repository with kernel, userspace, desktop, packaging, tooling, and documentation areas. Its process-management architecture includes thread management, signals, IPC, and resource limits, making IPC a high-priority integration point sequenced before or alongside advanced scheduler work.

### Scope and Boundaries

The IPC handler is responsible for safe message and data transfer between processes, threads, and kernel services.

**Owned Mechanisms:**
- Pipes and anonymous pipes
- FIFOs / named pipes
- UNIX-domain sockets
- Shared-memory regions
- Message queues
- Event notification primitives (e.g., eventfd-like wakeup objects)
- Signal delivery as a control-plane IPC mechanism
- File-descriptor passing (gated behind descriptor lifecycle stability)
- Permission, capability, and namespace checks for every IPC channel

**Initially Excluded Mechanisms:**
- General network sockets
- Display/GPU transport
- Package-manager transport
- Container runtime communication

Keeping the initial scope narrow avoids unsafe coupling while providing robust abstractions that higher-level components can consume.

---

## Periodic Task Cadence

| Period | IPC Handler Development | Cross-Component Maintenance | Required PR |
|---|---|---|---|
| Daily | Triage IPC bugs, deadlocks, leaked descriptors, and flaky concurrency tests | Triage CI, build failures, security reports, and documentation drift | Bug report or focused fix PR |
| Weekly | Implement one IPC primitive or semantic improvement | Dependency/toolchain updates, lint cleanup, test repair | One scoped PR |
| Biweekly | Run IPC integration tests with processes, threads, signals, VFS, and resource limits | Audit `FEATURE_STATUS.toml`, docs, and component ownership | Status/evidence PR |
| Monthly | Fuzz IPC parsers, stress concurrent channels, audit descriptor leaks and permission checks | Build release candidate, boot smoke test, installer/update check | Release-readiness PR |
| Quarterly | Review IPC model against Linux and BSD behavior; remove duplicate implementations and dead code | Security review, API stability review, roadmap consolidation | Architecture RFC plus cleanup PRs |

---

## Development Phases

### Phase 1: IPC Contract
Specify supported IPC mechanisms and their exact semantics in `docs/ipc-handler.md` and track progress in `FEATURE_STATUS.toml`.

**Required Specifications:**
- Supported IPC mechanisms and exact semantics
- Blocking, non-blocking, pollable, and asynchronous behavior
- Byte-stream versus message-boundary semantics
- Descriptor ownership and close semantics
- Buffer limits, backpressure, and queue capacity
- Error codes and failure behavior
- Permission model: owner, group, capability, namespace, and sandbox rules
- Capability status breakdown: implemented, modeled, integrated, hardware-tested

**Acceptance Criteria:**
- `cargo check --lib` passes
- `cargo test --lib` passes
- `./run_sigma_tests.sh` passes
- Every public API has documentation and tests
- No claim of POSIX/Linux/BSD compatibility without a passing compatibility test matrix

### Phase 2: Core IPC Objects
Implement canonical internal structures:
- `IpcChannel`
- `IpcEndpoint`
- `MessageBuffer`
- `SharedMemoryRegion`
- `WaitQueue`
- `FileDescriptorHandle`
- `IpcPermissions`
- `IpcStatistics`

**Core Invariants:**
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
3. Non-blocking mode
4. Pipe capacity and backpressure
5. End-of-file (EOF) behavior when all write ends close
6. Broken-pipe (`EPIPE`) behavior when all read ends close
7. FIFO creation, open modes, permissions, and VFS integration
8. Poll/select readiness semantics

### Phase 4: UNIX-Domain Sockets
Deliverables after pipes stabilize:
- Socket creation, bind, listen, accept, connect, close
- Stream and datagram modes
- Peer credentials verification
- Filesystem socket permissions
- Descriptor passing gated behind security review
- Socket shutdown and half-close semantics
- Concurrent connect/accept/close stress tests

### Phase 5: Shared Memory and Message Queues
**Shared Memory Controls:**
- Explicit creation, mapping, resize, and unmapping APIs
- Reference-counted region lifetime
- Permission checks at creation and mapping boundaries
- Alignment, size, and overflow validation
- Copy-in/copy-out boundaries for user pointers
- Cache-coherency and synchronization documentation
- Guard pages and bounds checks during development

**Message Queue Controls:**
- Bounded queue capacity
- Priority or FIFO ordering
- Blocking and non-blocking send/receive
- Timeout support
- Teardown semantics preventing kernel memory leaks

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
- Define behavior for blocking, timeout, interruption, and closure on every operation.
- Disallow kernel-side unbounded queues, buffers, allocations, or recursion from user input.
- Validate user-provided lengths, pointers, flags, paths, and descriptors at the API boundary.
- Ensure every resource acquisition has a defined release path (including panic/error paths).
- Test concurrent operations for lost wakeups, spurious wakeups, and use-after-close.
- Prohibit silent data truncation; return explicit errors or document truncation semantics.

### Security Rules
- Apply least privilege by default across all IPC channels.
- Enforce permissions at creation, connection, mapping, and message delivery boundaries.
- Fail closed when capability, namespace, or ownership checks are ambiguous.
- Treat file-descriptor passing as security-sensitive, requiring two approvals.
- Threat-model every mechanism against spoofing, tampering, information disclosure, DoS, privilege elevation, and resource exhaustion.
- Never expose raw kernel addresses, pointers, or internal object IDs through IPC.

### Rust-Specific Rules
- Minimize `unsafe`; every `unsafe` block must include a `// SAFETY:` comment covering aliasing, lifetime, initialization, and panic safety.
- Prefer ownership transfer and typed handles over raw integer descriptors inside the kernel.
- Use checked arithmetic for sizes, offsets, capacities, and reference counts.
- Avoid holding locks across copy-in/copy-out operations.
- Document lock ordering for every shared IPC structure.

### Testing Rules
Every IPC PR must include relevant tests from:
- Unit tests for buffer, queue, permission, and state logic
- Integration tests across process creation, threading, signals, and descriptor closure
- Concurrency stress tests for simultaneous readers, writers, connectors, and closers
- Timeout and cancellation tests
- Fuzz tests for serialized messages, socket paths, and control data
- Resource-exhaustion tests for descriptor, memory, and queue limits
- Regression test attached to every fixed bug

---

## First 10 IPC PR Sequence

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

## Governance and Merge Policy

**Repository Labels:**
- `area:ipc`, `area:pipes`, `area:sockets`, `area:shared-memory`, `area:security`
- `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`
- `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`
- `needs-second-reviewer`

**Merge Rules:**
- All changes land via PRs; direct commits to `main` are disabled.
- One maintainer approval required for standard IPC PRs.
- Two approvals required for shared memory, descriptor passing, syscall dispatch, security policy, or kernel-memory changes.
- CI must pass before merge (`cargo fmt`, `clippy`, `cargo test`, `./run_sigma_tests.sh`).
- Every PR must update tests, documentation, and `FEATURE_STATUS.toml`.
