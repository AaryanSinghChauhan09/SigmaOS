# SigmaOS Sovereign D-Bus IPC Wire Protocol & Message Bus

## Overview

SigmaOS implements a **pure-Rust, zero-dependency D-Bus message bus and wire protocol** (`src/ipc/dbus_sovereign.rs`).

D-Bus is the universal desktop and system IPC standard on Linux (freedesktop.org), utilized by `systemd`, `NetworkManager`, `logind`, `bluez`, `udisks2`, and graphical desktop environments.

## Features

- **Type System & Signatures**: Native representation of basic and container types (`y`, `b`, `n`, `q`, `i`, `u`, `x`, `t`, `d`, `s`, `o`, `g`, `a`, `v`, `e`, `r`, `h`).
- **Message Types**:
  - `MethodCall` (1): Remote procedure call invocation with arguments and interface.
  - `MethodReturn` (2): Response matching invocation serial.
  - `Error` (3): Exception or failure reply with error name and diagnostic string.
  - `Signal` (4): Publish/subscribe 1-to-many broadcast notifications.
- **Service Name Management**: Unique names (`:1.X`) and well-known reverse-domain names (`org.freedesktop.Systemd1`, `org.sigma.NetworkManager`).
- **Signal Match Rules**: Granular filtering on message type, sender, interface, member name, and object path.
- **Message Routing Engine**: Sub-microsecond direct point-to-point and broadcast dispatch with dropped message accounting.

## Architectural Diagram

```
+--------------------------------------------------------------------+
|                       SovereignDbusBus                             |
|  +---------------------------+       +---------------------------+ |
|  | Service: org.sigma.Network |       | Service: org.sigma.Audio  | |
|  | Unique Name: :1.1          |       | Unique Name: :1.2         | |
|  | Match Rules: [Signals...] |       | Match Rules: [Signals...] | |
|  +---------------------------+       +---------------------------+ |
|                                ^                                   |
|                                | Dispatch                          |
|  Method Calls / Returns / Errors / Broadcast Signals               |
+--------------------------------------------------------------------+
```

## Linux Parity

| Linux D-Bus Feature | SigmaOS Equivalent |
|---------------------|-------------------|
| `dbus-daemon --system` | `SovereignDbusBus::new()` |
| `RequestName()` | `bus.register_service("org.sigma.Service", pid)` |
| Method Invocation | `DbusMessage::method_call("/path", "interface", "method")` |
| Signal Emission | `DbusMessage::signal("/path", "interface", "signal")` |
| `AddMatch()` | `svc.add_match(DbusMatchRule { ... })` |

## Test Verification

6 standalone unit tests verified in test runner suite `[12]`:
- `test_dbus_service_registration`
- `test_dbus_method_call_routing`
- `test_dbus_signal_broadcast`
- `test_dbus_value_signatures`
- `test_dbus_message_error`
- `test_dbus_match_rule_filtering`
