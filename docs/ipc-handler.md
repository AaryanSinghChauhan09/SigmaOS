# SigmaOS IPC Handler Master Architecture, Support Matrix, and Program Plan

## Repository Context and Scope

The IPC handler owns message transport, endpoint lifecycle, bounded buffers,
shared-memory permissions, descriptor ownership, wait queues, and statistics.
It does not define application policy, device drivers, or unrelated scheduler
behavior.

## Scope and Boundaries

IPC semantics are explicit: blocking calls may wait on a bounded queue, while
non-blocking calls return a deterministic retry or capacity error. EOF and
closure are terminal, observable states. Descriptors close exactly once, shared
memory checks ownership and permissions, and namespaces/capabilities gate
cross-process access.

## Periodic Task Cadence

Every PR runs focused unit tests and checks the contract. Weekly reviews inspect
backpressure and teardown. Monthly reviews refresh the support matrix. Quarterly
reviews exercise fault injection and recovery. Each release candidate requires
repeatable integration evidence. After each incident, record the message path,
permissions, recovery, and regression test.

## Development Phases

1. Document endpoint, buffer, descriptor, and permission contracts.
2. Validate local channel and wait-queue invariants.
3. Add namespace and capability checks.
4. Integrate shared memory and zero-copy paths.
5. Add cross-process and cross-namespace tests.
6. Measure bounded latency and backpressure behavior.
7. Publish the driver and application integration guide.

## Missing Components Matrix

| Component | Contract | Evidence tier |
|---|---|---|
| Message buffer | Capacity and backpressure | Unit-tested |
| IPC endpoint | Ownership and closure | Unit-tested |
| Shared memory | UID, mode, and capability checks | Unit-tested |
| Wait queue | Blocking and wake behavior | Unit-tested |
| Descriptor handle | Single-close invariant | Unit-tested |
| Cross-namespace transport | Isolation and authorization | Planned integration |

## Industry-Standard Rules

Use explicit framing, bounded allocation, deterministic errors, cancellation,
timeouts, observability, and compatibility tests. Never silently drop messages
or claim delivery without an acknowledgement contract.

## Correctness Rules

Preserve ordering where promised, reject invalid state transitions, make EOF
idempotent, and ensure every waiter has a wake or timeout path. Tests must cover
full buffers, empty reads, closure, cancellation, and duplicate close.

## Security Rules

Enforce capability and namespace checks before access, avoid confused-deputy
forwarding, validate lengths before copying, and preserve least privilege for
shared-memory and descriptor operations.

## Rust-Specific Rules

Use ownership and borrowing to represent lifetimes, avoid unchecked indexing,
return typed errors, keep unsafe blocks isolated and justified, and test
concurrency-sensitive state with deterministic fixtures.

## Testing Rules

Every implementation PR includes focused unit tests, failure-path tests,
integration evidence where applicable, and a rollback or downgrade note. A
missing platform result remains partial rather than supported.

## First 10 IPC PR Sequence

1. Audit IPC paths and publish the support matrix.
2. Define endpoint and message-frame contracts.
3. Add bounded MessageBuffer backpressure tests.
4. Add endpoint ownership and closure tests.
5. Add FileDescriptorHandle single-close enforcement.
6. Add WaitQueue blocking and timeout tests.
7. Add shared-memory permission and capability checks.
8. Add namespace isolation and cross-process tests.
9. Add fault injection, cancellation, and recovery tests.
10. Publish the driver guide and promote only evidence-backed statuses.

## Governance and Merge Policy

IPC changes require an explicit semantics section, blocking/non-blocking
behavior, EOF/closure behavior, security review, capability/namespace evidence,
focused verification commands, integration impact, and a capability-status update.

