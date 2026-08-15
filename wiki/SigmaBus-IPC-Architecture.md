# SigmaBus IPC Architecture

## Overview

SigmaBus is SigmaOS's inter-process communication (IPC) subsystem, inspired by D-Bus but implemented with **zero external library dependencies**. It provides type-safe, efficient message passing between kernel subsystems and userspace processes.

## Design Philosophy

| Traditional D-Bus | SigmaBus |
|------------------|---------|
| C library (`libdbus`) | Pure Rust, no external deps |
| XML introspection | Hash-based interface lookup |
| Socket-based | In-kernel message routing |
| Separate daemon | Integrated into kernel |
| Full daemon | `SigmaBus` struct |

## Message Types

```
Method Call (→ Service)       Reply (← Service)
  ┌──────────────────────┐     ┌──────────────────────┐
  │ Type: MethodCall     │ ──► │ Type: MethodReturn   │
  │ Serial: 42           │     │ ReplySerial: 42       │
  │ Dest: FileManager    │     │ Payload: result       │
  │ Interface: OpenFile  │     └──────────────────────┘
  │ Payload: path        │
  └──────────────────────┘

Signal (Broadcast)
  ┌──────────────────────┐ ──► All subscribers
  │ Type: Signal         │
  │ Interface: SysEvents │
  │ Member: Shutdown     │
  └──────────────────────┘
```

## Message Header

The 64-byte cache-aligned message header:

```rust
#[repr(C, align(64))]
pub struct MessageHeader {
    pub msg_type: MessageType,   // 1 byte
    pub serial: u64,             // Unique message ID
    pub reply_serial: u64,       // For method returns
    pub sender: u64,             // FNV-1a hash of sender name
    pub destination: u64,        // FNV-1a hash of dest name
    pub interface_hash: u64,     // FNV-1a hash of interface
    pub member_hash: u64,        // FNV-1a hash of method/signal
    pub payload_len: u32,        // Payload length (max 4KB)
    pub flags: u16,              // Message flags
    pub version: u8,             // Protocol version
}
```

## FNV-1a String Hashing

Instead of string comparison, SigmaBus uses FNV-1a hashing for interface and member lookup - this is `const fn` computable at compile time:

```rust
pub const fn fnv1a_hash(s: &str) -> u64 {
    const FNV_PRIME: u64 = 0x00000100000001B3;
    const FNV_OFFSET: u64 = 0xCBF29CE484222325;
    // ...
}

// Usage: pre-compute hashes at compile time
const FILE_MANAGER_HASH: u64 = fnv1a_hash("org.sigma.FileManager");
const OPEN_FILE_HASH: u64 = fnv1a_hash("OpenFile");
```

This means interface lookups are O(1) integer comparisons, not O(n) string comparisons.

## Signal Subscription System

```
Process A subscribes to:
  interface = "org.sigma.SystemEvents"
  member = "Shutdown"

Signal sent by Init:
  ┌──────────────────────────────────────────┐
  │ SigmaBus.send(Shutdown signal)           │
  │   for each subscription:                │
  │     if filter.matches(signal):          │
  │       queue message to subscriber       │
  └──────────────────────────────────────────┘
```

## Well-Known Service Names

Following D-Bus naming conventions:

| Service Name | Description |
|-------------|-------------|
| `org.sigma.Init` | System init (PID 1 equivalent) |
| `org.sigma.FileManager` | Virtual filesystem manager |
| `org.sigma.PackageManager` | sigpkg package manager |
| `org.sigma.SecurityManager` | pledge/unveil/capsicum manager |
| `org.sigma.NetworkManager` | Network stack coordinator |
| `org.sigma.PowerManager` | ACPI and power management |
| `org.sigma.DisplayManager` | KMS/DRM display manager |
| `org.sigma.AudioManager` | Audio subsystem |

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Register service | O(log n) | BTreeMap insertion |
| Send method call | O(log n) | Destination lookup + enqueue |
| Broadcast signal | O(s) | s = number of subscribers |
| Receive next message | O(1) amortized | Queue pop |
| Interface/member lookup | O(1) | Pre-hashed with FNV-1a |

## Comparison with Other IPC Mechanisms

| Mechanism | SigmaOS Feature | Inspiration |
|-----------|----------------|------------|
| Message passing | SigmaBus | D-Bus, Mach ports |
| Capabilities | Capsicum tokens | FreeBSD |
| Shared memory | Sigma IPC segments | POSIX shm |
| Pipes | sigma-sh pipes | POSIX pipes |
| Async IPC | `atomic_ipc_deliver.cpp` | HeliOS, HeLin |

## References

- `src/ipc/sigma_bus.rs` - Main implementation
- `sigmaos/core/src/atomic_ipc_deliver.cpp` - C++ atomic IPC layer
- [D-Bus specification](https://dbus.freedesktop.org/doc/dbus-specification.html)
- [Security Architecture](Security-Architecture.md)
