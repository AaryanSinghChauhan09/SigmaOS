# Future Development: IPC and Userspace Interfaces

**Status:** Proposal. IPC types and protocol models do not prove that processes can communicate through an integrated, permission-checked runtime path.

## Scope

Plan local process communication, shared memory, event delivery, and stable userspace interfaces. Current source areas include [`src/ipc`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/ipc), [`src/syscall`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/syscall), and [`src/process`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/process).

## Linux and BSD ideas to evaluate

| Project | Design idea | SigmaOS question |
|---|---|---|
| Linux | Unix sockets, pipes, eventfd, and pidfd | Which primitives can share consistent lifetime and cancellation rules? |
| FreeBSD | kqueue event notification and Capsicum capability passing | Can event subscriptions and transferred handles be explicitly scoped? |
| OpenBSD | Small interfaces with pledge/unveil restrictions | Can each IPC endpoint expose only the operations a client needs? |
| Plan 9 | File-oriented services and 9P | Would a uniform resource namespace simplify service composition without obscuring permissions? |
| Fuchsia | Channel-based message passing and handle transfer | Can messages be bounded and handles validated before delivery? |

## Proposed work sequence

1. Define process and handle identity, ownership, inheritance, close semantics, and behavior when either peer exits.
2. Choose a small set of core primitives and document message framing, maximum sizes, blocking, cancellation, and error codes.
3. Enforce access checks when endpoints and shared memory are created, opened, transferred, and mapped.
4. Bound queues and buffers; specify backpressure and denial-of-service behavior for every asynchronous interface.
5. Specify syscall or library ABI versioning and compatibility before exposing interfaces to applications.

## Completion criteria

- Two independent processes communicate using the documented primitive through integrated kernel or userspace runtime paths.
- Peer exit, cancellation, full queues, invalid handles, and malformed messages produce defined errors without resource leaks.
- Shared-memory mappings enforce ownership, size, and permission rules at creation and map time.
- IPC permission checks are tested for allowed and denied cases at the enforcement boundary.
- Public ABI changes have compatibility notes and do not silently change message layouts.

## Maintenance

Link the implementation and enforcement call sites. Describe a mechanism as available only when a process can use it end-to-end; distinguish data structures and API models from integrated IPC.
