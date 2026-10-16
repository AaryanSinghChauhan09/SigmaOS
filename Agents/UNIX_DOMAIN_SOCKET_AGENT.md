# Unix Domain Socket Agent

| Field | Value |
|---|---|
| **Portal** | IPC & Networking |
| **Maintainer Agent** | UNIX_DOMAIN_SOCKET_AGENT |
| **Status** | Active |
| **Engine** | `src/kernel/sigma_unix_domain_socket_engine.rs` |
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
Implements AF_UNIX stream, datagram, and sequenced-packet sockets. Supports abstract namespace sockets (Linux), credential passing (SCM_CREDENTIALS), file descriptor passing (SCM_RIGHTS), and socket activation via systemd.

## 2. Technical Specifications
- Socket types: SOCK_STREAM, SOCK_DGRAM, SOCK_SEQPACKET
- Namespace: filesystem path and abstract (`\0`-prefixed)
- Ancillary data: SCM_RIGHTS (fd passing), SCM_CREDENTIALS
- Buffer sizes: SO_SNDBUF / SO_RCVBUF tunable

## 3. Implementation
Engine: [`sigma_unix_domain_socket_engine.rs`](../src/kernel/sigma_unix_domain_socket_engine.rs)

## 4. Standard Library Authorization
`std`, `libc`, and all predefined functions are fully authorized. Use `std::os::unix::net::UnixListener` freely.

## 5. Configuration
```toml
[unix_sockets]
socket_dir = "/run/sigma"
rcvbuf_bytes = 262144
sndbuf_bytes = 262144
abstract_namespace = true
```

## 6. Diagnostics
```bash
ss -x                            # list Unix domain sockets
socat - UNIX-CONNECT:/run/sigma.sock
sigma-diag ipc --type unix
```

## 7. See Also
- [DBUS_SYSTEMBUS_AGENT](./DBUS_SYSTEMBUS_AGENT.md)
- [POSIX_MQUEUE_SHM_AGENT](./POSIX_MQUEUE_SHM_AGENT.md)
