# IPC Mechanisms Agent

## Role
`SigmaIPC` provides the full suite of Inter-Process Communication primitives for SigmaOS — pipes, FIFOs, message queues, shared memory, semaphores, signals, and sockets — inspired by POSIX and Plan 9.

## Status
🟡 In Progress — Basic pipes/signals in place; POSIX MQ, SHM, and semaphores needed.

## Source File
`src/ipc/` (planned: `pipe.rs`, `mqueue.rs`, `shm.rs`, `semaphore.rs`, `socket.rs`)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Pipes | `pipe(2)`, `splice` | `pipe(2)` | SigmaPipe |
| POSIX MQ | `mq_open(3)` | `mq_open(3)` | SigmaMQ |
| Shared mem | `shm_open(3)`, SysV | `shm_open(3)` | SigmaSHM |
| Semaphores | POSIX + SysV | POSIX | SigmaSem |
| Unix sockets | `AF_UNIX` | `AF_UNIX` | SigmaUnixSocket |
| D-Bus | dbus-daemon / kdbus | D-Bus | SigmaBus |
| Signals | POSIX signals | POSIX signals | SigmaSignal |

## Responsibilities
- Anonymous and named pipe creation
- POSIX message queue: `mq_open`, `mq_send`, `mq_receive`, `mq_notify`
- Shared memory segments with permission control
- Named/unnamed semaphores (POSIX)
- Unix domain sockets for local fast IPC
- Signal delivery, masking, and real-time signals (SIGRTMIN-SIGRTMAX)
- Eventfd, signalfd, timerfd descriptors
- DBus/Varlink for service communication

## Implementation Notes
- `#![no_std]` — ring buffer for pipe without heap allocation
- POSIX MQ: priority queue (max-heap) of messages
- Shared memory: backed by anonymous memory mapping
- `static mut SigmaMQTable`: array of MQ descriptors

## Testing
```bash
rustc --test src/ipc/pipe.rs --edition 2021 -o /tmp/test_ipc && /tmp/test_ipc --test-threads=1
```

## Future Development
1. **io_uring for IPC** — Zero-copy IPC through shared ring buffers
2. **D-Bus in Rust** — Replace dbus-daemon with native SigmaBus
3. **Landlock for IPC** — Restrict pipe/socket access by policy
4. **Memfd sealing** — Immutable shared memory for security
5. **Cross-namespace IPC** — Container-aware IPC isolation
