# SigmaOS AI Agents Procedure Call Management & Inter-Process Communication Guide

Welcome to the **SigmaOS AI Agents Procedure Call Management Guide**. This document details procedure call conventions, System Call dispatchers, Foreign Function Interfaces (FFI), zero-copy Inter-Process Communication (IPC) ring buffers, and Remote Procedure Calls (RPC) for autonomous AI agents and core developers in SigmaOS.

---

## 1. Procedure Call Architecture & Dispatching

SigmaOS provides a unified procedure call architecture bridging userspace applications, microkernel syscalls, and autonomous AI agents:

### Core Procedure Call Mechanisms
1. **System Call Dispatcher (`SyscallContext`)**: Microkernel system call table handling 17+ core POSIX/Linux syscalls (`open`, `read`, `write`, `fork`, `exec`, `socket`, `kill`, `rt_sigaction`) with unified context isolation.
2. **C-ABI FFI Interoperability (`src/klib/ffi.rs`)**: Safe, zero-dependency C-string and raw pointer marshaling for host C libraries and POSIX ABI compatibility.
3. **Zero-Copy IPC Ring Channels (`src/kernel/ipc.rs`)**: Shared memory lock-free ring pipes achieving up to 14.2 GB/s throughput for inter-agent procedure calls.
4. **eBPF Helper Procedure Interception (`src/kernel/ebpf.rs`)**: eBPF sockmap and xdp helper procedure redirects for kernel-bypass packet routing.

---

## 2. Foreign Function Interface (FFI) Best Practices

When marshalling procedure arguments across C and Rust ABI boundaries, AI agents MUST utilize `klib::ffi`:

```rust
use sigmaos::klib::ffi::{rust_string_to_cstr, cstr_to_rust_string, cstrlen};

// Safe C-String conversion
let cstr_ptr = rust_string_to_cstr("sigmaos_service");
let len = unsafe { cstrlen(cstr_ptr) };
let rust_str = unsafe { cstr_to_rust_string(cstr_ptr) };
```

- **Rule 1**: Always check null pointers before dereferencing raw pointers (`*const u8` or `*mut u8`).
- **Rule 2**: Ensure C-strings are null-terminated (`\0`).

---

## 3. Zero-Copy IPC & Cross-Shard RPC

Procedure calls between different Sovereign System Shards (`S-SHARDS`) occur over zero-copy ring pipes:

```rust
use sigmaos::kernel::ipc::IpcChannel;

let mut channel = IpcChannel::new(4096);
let payload = b"PROC_CALL:trigger_agent_inference";

channel.send_message(payload).expect("IPC procedure call failed");
let msg = channel.receive_message().expect("Failed to receive IPC message");
```

---

## 4. Checklist for AI Agents Managing Procedure Calls

- [ ] Validated memory alignment for FFI struct pointers.
- [ ] Confirmed system call context checks capability tokens (`Pledge`/`Seccomp`).
- [ ] Checked for null-pointer safety in FFI conversions.
- [ ] Verified zero-copy IPC channels release lockless ring slots after message read.
- [ ] Executed `./run_sigma_tests.sh` to confirm procedure call and IPC test suites pass cleanly.
