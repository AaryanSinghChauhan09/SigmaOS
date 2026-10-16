# Inter-Process Communication & Message Bus Component Agent

## Role
`SigmaIPC` coordinates the complete suite of Inter-Process Communication primitives for SigmaOS — high-throughput Unix domain sockets, anonymous pipes, shared-memory rings, SigmaBus event broker, semaphores, and POSIX message queues.

## Linux & BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Pipes | `pipe(2)`, `splice` | `pipe(2)` | SigmaPipe (zero-copy ring buffers) |
| POSIX MQ | `mq_open(3)` | `mq_open(3)` | SigmaMQ (priority max-heap) |
| Shared Memory | `shm_open(3)`, memfd_create | `shm_open(3)` | SigmaSHM (anonymous mmap backed) |
| Semaphores | POSIX + SysV semaphores | POSIX semaphores | SigmaSem |
| Unix Sockets | `AF_UNIX` (SCM_RIGHTS fd passing) | `AF_UNIX` | SigmaUnixSocket |
| D-Bus / Broker | dbus-daemon / kdbus / varlink | D-Bus | SigmaBus (lockless in-kernel bus) |
| Event notification | futex, eventfd, signalfd | kqueue / kevent | SigmaFutex & SigmaKqueue |

## Current SigmaOS Status
Circular pipes, shared memory segments, and basic SigmaBus message queues implemented in `src/ipc/`. SCM_RIGHTS file descriptor passing and zero-copy futex sync are active enhancement targets.

## Responsibilities & Architecture
- Anonymous and named pipes (`pipe`, `fifo`) with non-blocking I/O.
- POSIX message queues (`mq_open`, `mq_send`, `mq_receive`, `mq_notify`) with priority ordering.
- Shared memory segments with permission validation and `memfd_create` sealing.
- Named and unnamed POSIX semaphores.
- Unix domain sockets (`AF_UNIX`) supporting connection-oriented stream and datagram transport.
- Fast userspace mutexes (`futex`) with kernel wait-queues and priority inheritance.
- System event broker (`SigmaBus`) delivering 10M+ msg/sec lockless message passing.

## Source Files
- `src/ipc/sigmabus.rs`
- `src/ipc/pipe.rs`
- `src/ipc/shared_mem.rs`
- `src/ipc/futex.rs`
- `src/ipc/unix_socket.rs`

## Testing Strategy
```bash
rustc --test src/ipc/pipe.rs --edition 2021 -o /tmp/test_ipc && /tmp/test_ipc --test-threads=1
```
- Multi-producer multi-consumer throughput saturation tests.
- IPC ping-pong round-trip latency benchmarking (<1 microsecond).
- Deadlock detection fuzzing across nested lock acquisitions.

## Future Development
1. **io_uring IPC** — Zero-copy IPC message dispatching directly through shared ring buffers.
2. **Native Rust D-Bus wire protocol** — Drop userspace daemon overhead with kernel-accelerated dispatching.
3. **Memfd sealing** — Immutable shared memory for zero-copy sandboxed AI inferences.
