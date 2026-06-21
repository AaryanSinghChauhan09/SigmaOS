# SigmaOS Microkernel IPC Architecture

## Overview

SigmaOS implements a **Synchronous Message Passing** IPC model inspired by QNX and Minix 3. This isolates drivers as user-mode processes, achieving Zero-Reboot Driver Recovery.

## Core Components

### `sigma_ipc_msg_t` — The Message Unit

```c
typedef struct {
    sigma_u32 sender_pid; // Automatically filled by kernel
    sigma_u32 type;       // Message category (e.g., KBD_READ_REQ)
    sigma_u32 data[6];    // 24-byte payload (optimised for register copies)
} sigma_ipc_msg_t;        // Total: 32 bytes
```

### `sys_send` / `sys_receive` Syscalls

| Syscall | ID | Description |
|---|---|---|
| `sys_send(dest_pid, msg)` | `0x05` | Enqueues message to `dest_pid`'s mailbox; wakes them up |
| `sys_receive(out_msg)` | `0x06` | Blocks caller until a message arrives; **0% CPU** while waiting |

## Task States

```
READY ──► RUNNING ──► WAIT_IPC (0% CPU) ──► READY
                │                           ▲
                └──► BLOCKED ───────────────┘
```

A task enters `TASK_STATE_WAIT_IPC` (state `3`) when `sys_receive` finds an empty mailbox. The scheduler's `yield()` skips it entirely until `sched_wake_ipc()` is called.

## User-Mode Driver Pattern

```cpp
// Driver server loop (runs in Ring 3 User Mode)
while (1) {
    sigma_ipc_msg_t req;
    sys_receive(&req);          // Block — 0 CPU used

    // Handle request...
    sigma_ipc_msg_t reply;
    reply.data[0] = result;
    sys_send(req.sender_pid, &reply);
}
```

If this driver crashes, the kernel reaps the shard and restarts it — **no kernel panic**.

## Relevant Source Files

- `include/kernel/sigma_ipc.h` — Message structure and API
- `kernel/core/ipc/sigma_ipc.cpp` — Queue logic and blocking
- `kernel/core/process/scheduler.c` — WAIT_IPC state
- `kernel/core/system/SovereignSyscall.cpp` — Syscall dispatch
- `userland/servers/sigma_keyboard_server.cpp` — POC driver server
