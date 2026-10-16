# D-Bus System Bus Agent

| Field | Value |
|---|---|
| **Portal** | IPC & Messaging |
| **Maintainer Agent** | DBUS_SYSTEMBUS_AGENT |
| **Status** | Active |
| **Engine** | `src/kernel/sigma_dbus_broker_engine.rs` |
| **Language** | Rust (primary), Nim (controller) |

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
Implements the D-Bus message broker for both system bus (`/run/dbus/system_bus_socket`) and session bus. Manages service activation (`.service` files), method call routing, signal subscription, policy enforcement, and the modern dbus-broker replacement architecture.

## 2. Technical Specifications
- Transport: Unix domain socket, TCP (optional)
- Message format: D-Bus wire protocol (little-endian / big-endian)
- Service activation: D-Bus auto-activation, systemd socket activation
- Security: policy files in `/usr/share/dbus-1/`, AppArmor D-Bus mediation
- Modern broker: dbus-broker (Rust-compatible via libdbus)

## 3. Implementation
Engine: [`sigma_dbus_broker_engine.rs`](../src/kernel/sigma_dbus_broker_engine.rs)

## 4. Standard Library Authorization
`std`, `libc`, and all predefined functions are fully authorized.

## 5. Configuration
```toml
[dbus]
system_bus_socket = "/run/dbus/system_bus_socket"
session_bus_env = "DBUS_SESSION_BUS_ADDRESS"
use_dbus_broker = true
activation_timeout_ms = 5000
```

## 6. Diagnostics
```bash
dbus-monitor --system
gdbus introspect --system --dest org.freedesktop.systemd1 --object-path /
busctl tree org.freedesktop.NetworkManager
sigma-diag dbus --list-services
```

## 7. See Also
- [UNIX_DOMAIN_SOCKET_AGENT](./UNIX_DOMAIN_SOCKET_AGENT.md)
- [POSIX_MQUEUE_SHM_AGENT](./POSIX_MQUEUE_SHM_AGENT.md)
