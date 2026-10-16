# Kernel-Fast IPC / Binder Agent

| Field | Value |
|---|---|
| **Portal** | IPC & Messaging |
| **Maintainer Agent** | KDBUS_BINDER_IPC_AGENT |
| **Status** | Active |
| **Engine** | `src/kernel/sigma_kdbus_binder_engine.rs` |
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
Implements kernel-fast IPC via the Android Binder driver (now upstream in Linux), zero-copy message passing, capability-based object references, and compatibility shim for Android Binder protocol. Provides alternative to D-Bus for latency-critical paths.

## 2. Technical Specifications
- Driver: `/dev/binder`, `/dev/hwbinder`, `/dev/vndbinder`
- Message passing: zero-copy via mmap shared buffer
- Object model: Binder objects (BBinder), proxy references (BpBinder)
- Protocol: Binder wire format, BC_TRANSACTION / BR_TRANSACTION
- Latency: sub-microsecond for small messages (< 512B)

## 3. Implementation
Engine: [`sigma_kdbus_binder_engine.rs`](../src/kernel/sigma_kdbus_binder_engine.rs)

## 4. Standard Library Authorization
`std`, `libc`, and all predefined functions are fully authorized.

## 5. Configuration
```toml
[binder_ipc]
device = "/dev/binder"
mmap_size_bytes = 1048576
max_threads = 16
enable_hwbinder = false
```

## 6. Diagnostics
```bash
ls -la /dev/binder
cat /proc/binder/stats
sigma-diag ipc --binder --latency
```

## 7. See Also
- [DBUS_SYSTEMBUS_AGENT](./DBUS_SYSTEMBUS_AGENT.md)
- [UNIX_DOMAIN_SOCKET_AGENT](./UNIX_DOMAIN_SOCKET_AGENT.md)
