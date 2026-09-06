# AI Agent Process Interaction Operation Management Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                      AI Process Interaction Manager                             |
|      (ProcessInteractionManager, IpcMailboxRouter, StandardStreamController)    |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                       IPC Addressing Router & Mailbox                           |
|       (Direct PID-to-PID, Indirect Mailbox, CapabilityToken::Ipc Gate)          |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
| SovereignPipe Engine  |   | Signal Dispatch Table |   | Standard Streams      |
| (PIPE_BUF Atomic 4KB) |   | (SIGPIPE/SIGCHLD/wait)|   | (isatty, stdbuf, flush|
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                       Kernel IPC & VFS Stream Subsystem                         |
|         (Named FIFOs, Shared Memory Mappings, Lockless Ring Buffers)            |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **IPC Mailbox & Addressing Router**:
   - Supports Direct (PID-to-PID) and Indirect (topic/mailbox) communication topologies (`1:1`, `1:N`, `N:N`).
   - Capability-gated via `CapabilityToken::Ipc` to prevent unauthorized process snooping or payload spoofing.

2. **SovereignPipe Engine**:
   - Implements lock-free ring buffers with 4096-byte (`PIPE_BUF`) atomic write guarantees.
   - Endpoint reference counting tracks `reader_count` and `writer_count`, signaling `EOF` on writer close and `EPIPE`/`SIGPIPE` on broken reader writes.

3. **Signal & Stream Control Subsystem**:
   - Dispatches POSIX/BSD signals (`SIGCHLD`, `SIGPIPE`, `SIGCONT`) to agent processes.
   - `StandardStreamController` checks `isatty` to auto-select `LineBuffered` vs `BlockBuffered(4096)` stream strategies.

4. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
