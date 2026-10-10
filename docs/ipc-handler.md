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

## IPC Mechanisms & Exact Semantics

| Mechanism | Boundary Model | Transfer Mode | Default Capacity | Backpressure Strategy |
|---|---|---|---|---|
| **Anonymous Pipe** | Byte-stream | Unidirectional | 64 KiB | Writer blocks when full; Reader blocks when empty |
| **FIFO (Named Pipe)** | Byte-stream | Bi/Unidirectional | 64 KiB | Writer blocks when full; Reader blocks when empty |
| **UNIX Stream Socket** | Byte-stream | Bidirectional | 128 KiB | Writer blocks when transmit queue full |
| **UNIX Datagram Socket** | Message-boundary | Bidirectional | 64 KiB / msg | Drop / EAGAIN when queue full |
| **Shared Memory** | Direct Memory Access | Zero-Copy Shared | Configurable (Page-aligned) | Synchronization via Futex / Lock primitives |
| **Message Queue** | Priority Message | Bidirectional | 10 msg (8 KiB/msg) | Sender blocks on full queue; Receiver blocks on empty |
| **Eventfd** | 64-bit Counter | Bidirectional | 8 Bytes (`u64`) | Overflow caps at `u64::MAX - 1` |

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

## Error Codes & Failure Modes

| Error Code | Symbolic Name | Trigger Condition |
|---|---|---|
| `EAGAIN` / `EWOULDBLOCK` | Resource temporarily unavailable | Non-blocking operation would block |
| `EBADF` | Bad file descriptor | Invalid or closed descriptor handle |
| `EPIPE` | Broken pipe | Writing to a pipe/socket with no readers |
| `EACCES` / `EPERM` | Permission denied | Operation violated DAC/MAC/Capability check |
| `EMSGSIZE` | Message too long | Datagram or message exceeds buffer limit |
| `ENOBUFS` / `ENOMEM` | No buffer space available | Queue capacity or kernel memory limit reached |
| `EEXIST` | File / IPC object exists | Exclusive object creation (`O_CREAT \| O_EXCL`) failed |
| `EINVAL` | Invalid argument | Invalid flags, alignment, or size parameters |

---

## Security Model & Access Control

1. **Discretionary Access Control (DAC)**:
   - File/Inode permissions (`mode_t`: Owner, Group, Others) applied to FIFOs, UNIX Sockets, and POSIX Shared Memory objects.
2. **Capabilities**:
   - `CAP_IPC_LOCK`: Required to lock shared memory segments in RAM.
   - `CAP_IPC_OWNER`: Required to override permission checks on IPC objects owned by other users.
3. **Namespaces (`ipc_namespace`)**:
   - Shared memory segments, message queues, and semaphores are scoped within isolated IPC namespaces.
4. **File Descriptor Passing Security**:
   - Passing FDs via `SCM_RIGHTS` over UNIX sockets requires explicit target capability checks and audited handle translation to prevent illegal capability amplification.

---

## Threat Model & Risk Analysis

| Threat Category | Potential Risk / Attack Vector | Mitigation Strategy |
|---|---|---|
| **Spoofing** | Untrusted process impersonating privileged peer | `SO_PEERCRED` verification enforced by kernel upon connection |
| **Tampering** | Concurrent modification of in-flight message buffers | Memory isolation, strict copy-in/copy-out, atomic pointer operations |
| **Information Disclosure** | Residual data exposure in shared memory or kernel buffers | Kernel zero-initializes allocated pages before mapping or read delivery |
| **Denial of Service (DoS)** | Exhaustion of kernel memory via unbounded queue allocations | Hard capacity caps on pipe buffers, message queue depth, and socket backlogs |
| **Elevation of Privilege** | Capability amplification via unauthorized file descriptor passing | Explicit capability inspection on passed file descriptors (`SCM_RIGHTS`) |
| **Resource Exhaustion** | File descriptor leakage across process forks / execs | Support for `FD_CLOEXEC` / `O_CLOEXEC` flags by default on IPC creation |

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
