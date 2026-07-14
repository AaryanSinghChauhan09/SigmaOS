# sigma-agent — Sovereign IPC Message Bus

> **Status**: ✅ Implemented — prototype in `userland/agent/src/ipc_agent.rs`  
> **Language**: Rust (`#![no_std]`, no external crates)  
> **Branch**: `feature/multi-lang-impl-batch1`

---

## Overview

`sigma-agent` is SigmaOS's IPC message-passing bus. It provides a typed, lock-free ring-buffer channel for inter-process and inter-module communication. All agents register on the `AgentBus` and communicate via fixed-size `IpcMessage` packets with Fletcher-16 checksums.

## Architecture

```
AgentBus
├── ConcreteAgent[0..15]   ← implements Agent trait
└── IpcMessage routing     ← src → dst via agent_id()
```

## Message Format

```
Magic(2) | Kind(1) | SrcId(1) | DstId(1) | Seq(2) | PayloadLen(1)
Payload[128] | Checksum(2)
```

Total message size: **137 bytes** (fixed, no dynamic allocation).

## Message Kinds

| Kind | Value | Description |
| :--- | :--- | :--- |
| `Ping` | `0x00` | Heartbeat check |
| `Pong` | `0x01` | Heartbeat reply |
| `Register` | `0x10` | Agent registration |
| `Unregister` | `0x11` | Agent deregistration |
| `Command` | `0x20` | Execute a command |
| `Response` | `0x21` | Command result |
| `Event` | `0x30` | Broadcast event |
| `Error` | `0xFF` | Error notification |

## OOP Design (Rust Traits)

```rust
pub trait Agent {
    fn agent_id(&self) -> u8;
    fn name(&self) -> &[u8];
    fn on_message(&mut self, msg: &IpcMessage) -> Option<IpcMessage>;
    fn on_register(&mut self) {}
    fn on_unregister(&mut self) {}
}
```

`ConcreteAgent` implements `Agent` and handles `Ping → Pong` and `Command → Response`.

## Checksum

Fletcher-16 over all bytes except the checksum field itself:

```
S1 = Σ bytes[i]  (mod 256)
S2 = Σ S1[i]    (mod 256)
checksum = (S2 << 8) | (S1 & 0xFF)
```

## Implementation Files

| File | Language | Description |
| :--- | :--- | :--- |
| `userland/agent/src/ipc_agent.rs` | Rust | Full IPC bus + Agent trait |

## Test Coverage

```rust
#[test] fn test_ring_buffer()           // Push/pop, empty detection
#[test] fn test_ipc_message_checksum()  // Checksum round-trip
#[test] fn test_agent_bus_ping_pong()   // Agent registration + Ping→Pong routing
#[test] fn test_concrete_agent_counts() // Message receive counter
```

## Future Work

- [ ] Async event loop (poll-based, no OS scheduler dependency)
- [ ] Multi-threaded ring buffer (CAS-based lock-free)
- [ ] Named channels (hash-indexed agent lookup)
- [ ] Capability-based access control per message kind
