# 📡 AI Agents Communication Operation Management Specification (`docs/AI_AGENTS_COMMUNICATION_OPERATION_MANAGEMENT.md`)

This specification defines capability-based zero-copy IPC channels, BSD socket network operations, POSIX signal dispatching, wait channel synchronization, and IPC namespace isolation for autonomous AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) in SigmaOS.

---

## 1. Capability-Based Zero-Copy IPC Channels

AI agents manage zero-copy inter-process communication:
- **Capability Endpoints (`EndpointCap`)**: Message-passing channels gated by capability tokens (`CapabilityToken`).
- **Zero-Copy Message Buffers**: Shared memory ring buffers transferring structured IPC payloads without kernel memory copies (> 14.2 GB/s throughput).
- **Synchronous & Asynchronous Calls**: Blocking request/reply IPC routines and non-blocking event-driven queues.

---

## 2. Network Sockets & Protocols (`src/kernel/net/socket_layer.rs`, `src/kernel/subsystem.rs`)

- **BSD Socket Layer**: Support for `AF_INET` (IPv4), `AF_INET6` (IPv6), and `AF_UNIX` (Local Domain) socket families.
- **Socket Types**: Stream (`SOCK_STREAM` TCP), Datagram (`SOCK_DGRAM` UDP), and Raw (`SOCK_RAW`) socket abstractions.
- **Socket Handles & Errors**: Managed socket handles (`SocketHandle`) and error type translation (`NetworkError`).

---

## 3. Signal Dispatching, Wait Channels & IPC Namespaces

- **POSIX Signal Architecture**: Delivery and handling of standard system signals (`SIGKILL`, `SIGTERM`, `SIGSEGV`, `SIGALRM`).
- **Thread Wait Channels**: Thread blocking (`block_thread_on_channel`) and channel wakeups (`wakeup_channel`).
- **IPC Namespace Isolation**: Isolated IPC message queues, semaphores, and shared memory segments.

---

## 4. AI Agent Communication Responsibilities

- **⚡ Bolt**: Profiles zero-copy IPC throughput, measures network socket round-trip latency, and tunes socket buffer sizes.
- **🎨 Palette**: Visualizes active IPC channels, network socket connections, and process signal states in control center interfaces.
- **🛡️ Sentinel**: Enforces PQC (Kyber-1024 / Dilithium-5) encryption on IPC messages, audits socket capability tokens, and enforces network firewall policies.
