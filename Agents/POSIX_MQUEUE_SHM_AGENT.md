# POSIX Message Queues & Shared Memory Agent

| Field | Value |
|---|---|
| **Portal** | IPC & Messaging |
| **Maintainer Agent** | POSIX_MQUEUE_SHM_AGENT |
| **Status** | Active |
| **Engine** | `src/kernel/sigma_posix_ipc_engine.rs` |
| **Language** | Rust (primary), Zig (accelerator) |

---

## Table of Contents
1. [Overview](#overview)
2. [Technical Specifications](#technical-specifications)
3. [Implementation](#implementation)
4. [Standard Library Authorization](#standard-library-authorization)
5. [Configuration](#configuration)
6. [Diagnostics](#diagnostics)
7. [See Also](#see-also)

---

## 1. Overview
Provides POSIX IPC primitives: message queues (`mq_open`/`mq_send`/`mq_receive`), shared memory (`shm_open`/`mmap`), and semaphores (`sem_open`/`sem_wait`). Manages `/dev/mqueue` filesystem and `/dev/shm` tmpfs.

## 2. Technical Specifications
- Message queues: POSIX mqueue (kernel-backed), priority-ordered
- Shared memory: POSIX shm (backed by tmpfs at `/dev/shm`)
- Semaphores: POSIX named (`/sem_*`) and unnamed
- Limits: `/proc/sys/fs/mqueue/` tunables (msg_max, msgsize_max, queues_max)

## 3. Implementation
Engine: [`sigma_posix_ipc_engine.rs`](../src/kernel/sigma_posix_ipc_engine.rs)

## 4. Standard Library Authorization
`std`, `libc`, and all predefined functions are fully authorized. Use `libc::mq_open`, `libc::shm_open` freely.

## 5. Configuration
```toml
[posix_ipc]
mqueue_msg_max = 256
mqueue_msgsize_max = 65536
shm_tmpfs_size_mb = 2048
sem_max = 32768
```

## 6. Diagnostics
```bash
ls /dev/mqueue/                  # list message queues
ls /dev/shm/                     # list shared memory objects
ipcs -a                          # System V IPC (legacy)
sigma-diag ipc --posix --stats
```

## 7. See Also
- [UNIX_DOMAIN_SOCKET_AGENT](./UNIX_DOMAIN_SOCKET_AGENT.md)
- [DBUS_SYSTEMBUS_AGENT](./DBUS_SYSTEMBUS_AGENT.md)
