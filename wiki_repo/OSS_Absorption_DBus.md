# OSS Absorption: DBus

## Overview

DBus is a message bus system used across Linux for inter-process communication (IPC). It allows multiple applications to communicate, and it heavily relies on a central daemon broker (`dbus-daemon` or `dbus-broker`) to enforce security policies and route messages.

## Key Principles Absorbed

### Native Typed IPC (`sigma_ipc`)

- DBus uses an XML-based introspection format and complex serialization (GVariant).
- SigmaOS absorbs this using native, strongly-typed Rust enums (`sigma_ipc::Message`, `Payload`). Serialization is compiled into the binary, avoiding runtime XML parsing overhead.

### Broker-less Peer-to-Peer Routing

- Standard DBus routes all traffic through a central daemon, causing a performance bottleneck.
- `sigma_ipc::IpcEngine` enables direct peer-to-peer message passing (e.g., via shared memory or Unix domain sockets). Capabilities are verified at connection time by `sigma_security`, eliminating the need for an active man-in-the-middle broker.

## Displaced Technologies

| Technology | SigmaOS Replacement |
| --- | --- |
| DBus / dbus-daemon | `sigma_ipc::IpcEngine` |
| XML Introspection | Native Rust Traits & Enums |
| kdbus (deprecated) | Direct memory IPC |

## Status

**Core Absorbed** — The `sigma_ipc` crate provides the foundational types and routing engine for replacing DBus across SigmaOS.
