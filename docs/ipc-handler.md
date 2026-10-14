# IPC Handler Master Architecture & Development Plan

## Executive Summary
SigmaOS identifies IPC, signals, process lifecycle, and resource limits as core operating-system areas. Rather than treating IPC as a collection of isolated modules, the IPC Handler is defined as a formally specified, testable subsystem responsible for safe message and data transfer between processes, threads, and kernel services.

This document establishes the architecture, scope, periodic task cadence, development phases, safety and security invariants, test matrix, pull request template, and PR roadmap for the SigmaOS IPC Handler.

## Repository Context & Sequence
SigmaOS is a Rust-based experimental OS repository distinguishing code presence and unit tests from a proven bootable OS. Its process management includes thread management, signals, IPC, and resource limits. IPC is sequenced before or alongside advanced scheduler work because scheduling, signals, file descriptors, and IPC all depend on reliable process/thread wakeup and synchronization semantics.

## IPC Handler Scope
The IPC Handler is responsible for safe message and data transfer across process, thread, and kernel boundaries.

### In Scope
- Pipes and anonymous pipes
- FIFOs / named pipes
- UNIX-domain sockets (stream and datagram)
- Shared-memory regions
- Message queues
- Event notification primitives (eventfd-like wakeup objects)
- Signal delivery as a control-plane IPC mechanism
- File-descriptor passing (gated behind descriptor lifecycle stability)
- Permission, capability, and namespace checks for every IPC channel

### Out of Scope (Initial Phase)
- General network sockets (TCP/UDP/IP)
- Display/GPU transport
- Package-manager transport
- Container runtime communication

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
- Created `docs/ipc-handler.md` and `wiki/ipc-handler.md`.
- Added `[ipc.handler]` entry to `FEATURE_STATUS.toml`.
- Specified semantics: blocking/non-blocking, message vs byte-stream, descriptor ownership, capacity limits, error codes, and permission model.

### Phase 2: Core IPC Objects
Canonical internal representations:
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
3. A reader blocked on an empty pipe must be woken when data arrives or writer closes.
4. A writer blocked on a full pipe must be woken when space becomes available or reader closes.
5. Shared-memory mappings must enforce lifetime, permissions, and address-space isolation.
6. Every wait path must have a defined wakeup path and timeout option.

### Phase 3: Pipes and FIFOs
1. Anonymous pipe creation and descriptor allocation
2. Byte-stream read/write semantics
3. Non-blocking mode
4. Pipe capacity and backpressure
5. End-of-file (EOF) behavior when all write ends close
6. Broken-pipe (`EPIPE`) behavior when all read ends close
7. FIFO creation, open modes, permissions, and VFS integration
8. Poll/select readiness semantics

### Phase 4: UNIX-Domain Sockets
1. Socket creation, bind, listen, accept, connect, close
2. Stream and datagram modes
3. Peer credentials where supported (`SO_PEERCRED` / `ucred`)
4. Filesystem socket permissions
5. Descriptor passing (gated behind explicit security review)
6. Socket shutdown and half-close semantics
7. Concurrent connect/accept/close stress tests

### Phase 5: Shared Memory and Message Queues
Shared Memory Controls:
- Explicit creation, mapping, resize, and unmapping APIs
- Reference-counted region lifetime
- Permission checks at creation and mapping time
- Alignment, size, and overflow validation
- Copy-in/copy-out boundaries for unsafe user pointers
- Cache-coherency and synchronization documentation
- Guard pages / bounds checks during development

Message Queues:
- Bounded queues
- Priority or FIFO ordering
- Blocking and non-blocking send/receive
- Timeout support
- Queue teardown semantics
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
- All user-provided lengths, pointers, flags, paths, and descriptors must be validated at boundary.
- Every resource acquisition must have a defined release path (error and panic safe).
- Every concurrent operation must be tested for lost wakeups, spurious wakeups, and use-after-close.
- No silent data truncation; return explicit error or document exact truncation rule.

### Security Rules
- Apply least privilege by default.
- Enforce permissions at creation, connection, mapping, and message-delivery boundaries.
- Fail closed when capability, namespace, or ownership checks are ambiguous.
- Treat file-descriptor passing as security-sensitive requiring 2 approvals.
- Threat-model every mechanism: spoofing, tampering, info disclosure, DoS, privilege elevation, resource exhaustion.
- Do not expose kernel addresses, raw pointers, or internal object identities through IPC.

### Rust-Specific Rules
- Minimize `unsafe`; every `unsafe` block requires a `// SAFETY:` comment covering aliasing, lifetime, initialization, and panic behavior.
- Prefer ownership transfer and typed handles over raw integer descriptors inside kernel.
- Use checked arithmetic for sizes, offsets, capacities, and reference counts.
- Avoid holding locks across copy-in/copy-out operations.
- Document lock ordering for every shared IPC structure.

### Testing Rules
Every IPC PR must include relevant tests:
- Unit tests for buffer, queue, permission, and state logic
- Integration tests across process creation, threading, signals, and descriptor closure
- Concurrency stress tests for simultaneous readers, writers, connectors, closers
- Timeout and cancellation tests
- Fuzz tests for serialized messages, socket paths, control data, malformed requests
- Resource-exhaustion tests for descriptor, memory, queue limits
- Regression test for every fixed bug

Required verification suite:
```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

## Pull Request Template

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

## Example IPC PR

**Title:**
```text
feat(ipc): implement anonymous pipes with EOF, backpressure, and wakeup safety
```

**Branch:**
```text
feat/ipc-anonymous-pipes
```

**Commits:**
```text
feat(ipc): add IpcChannel and pipe endpoint abstractions

Introduce a canonical IPC channel model with reader/writer endpoints,
buffer capacity, blocking modes, and wait-queue integration.

Refs: #<issue-id>
```

```text
fix(ipc): prevent lost wakeup when pipe reader closes during blocking write

Register the writer on the pipe wait queue under the same lock used for
reader-close detection. Add concurrent close/write stress tests.

Refs: #<issue-id>
```

**PR Body:**
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

## First 10 IPC PR Roadmap

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
