# SigmaOS AI Agent Process Interaction Operation Management Guidelines

## 1. Overview
SigmaOS implements high-efficiency inter-process communication (IPC) and process control frameworks managed by AI system agents (such as `ProcessInteractionManager`, `SovereignPipeController`, `IpcMailboxRouter`, and `StandardStreamController`). These guidelines define IPC addressing modes (Direct vs Indirect Mailbox, 1:1, 1:N, N:N topologies), zero-copy lock-free `SovereignPipe` queues, POSIX/BSD signal delivery (`SIGPIPE`, `SIGCHLD`, `SIGCONT`), TTY stream redirection (`isatty`), and shared memory mappings for AI agents in SigmaOS.

## 2. Core Process Interaction Management Principles

### 2.1 IPC Addressing Modes & Topologies
- **Direct vs. Indirect Mailbox**: AI processes interact using direct PID-to-PID channels or indirect capability-gated mailboxes (`IpcMailbox` in `src/ipc/ipc.rs`).
- **Communication Topologies**: Supports 1-to-1 point-to-point channels, 1-to-N fanout publication channels, and N-to-N concurrent IPC message topologies.
- **IPC Addressing Permissions**: Access to indirect mailboxes requires explicit `CapabilityToken::Ipc` authorization.

### 2.2 SovereignPipe & Zero-Copy Streaming
- **POSIX PIPE_BUF Atomic Writes**: Atomic writes under 4096 bytes (`PIPE_BUF`) guarantee atomic delivery without interleaved byte streams.
- **Reference Counted Stream Endpoints**: `SovereignPipe` tracks active `reader_count` and `writer_count`. Closing all writers issues an end-of-file (`EOF`) condition to readers; closing all readers when a writer writes triggers `EPIPE` / `SIGPIPE`.
- **Named FIFO Binding**: Supports POSIX/BSD named FIFO filesystem path bindings (`fifo_path`).

### 2.3 POSIX & BSD Signal Delivery Semantics
- **Asynchronous Signal Dispatch**: AI agents intercept and deliver signals (`SIGINT`, `SIGTERM`, `SIGKILL`, `SIGPIPE`, `SIGCHLD`, `SIGWINCH`) using kernel signal masks and `sigaction` dispatch tables.
- **Parent-Child Process Lifecycle**: Parent AI processes receive `SIGCHLD` upon subagent termination, reading subagent exit codes via `waitpid` / `wait4` without creating zombie process leaks.

### 2.4 Standard Stream Control & Buffering
- **TTY Capability Query (`isatty`)**: AI agents inspect TTY capability flags via `StandardStreamController` (`src/ipc/std_streams.rs`).
- **Automatic Buffering Strategy**: Streams attached to interactive TTYs default to Line-Buffered mode (`LineBuffered`), while file/pipe redirections default to Block-Buffered mode (`BlockBuffered(4096)`).
- **Global Stream Synchronization (`ffflush(NULL)`)**: Agents trigger multi-stream buffer synchronization (`flush_all`) prior to process fork or termination.

---
*Maintained by the SigmaOS IPC & Process Management Steering Committee.*
