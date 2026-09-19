# SigmaOS io_uring — Asynchronous I/O Ring

## Overview

SigmaOS implements an io_uring-inspired async I/O interface. Two ring buffers (submission + completion) provide zero-copy, batch-capable, kernel-polled asynchronous I/O.

**Location:** `src/kernel/sigma_io_uring.rs`

---

## Architecture

```
Userspace                    Kernel
─────────                    ──────
submit SQEs ──→ [SQ ring] ──→ process()
                             │
                             ▼ execute I/O
                         [CQ ring]
collect CQEs ←── [CQ ring] ←──
```

Both rings are power-of-2 sized lock-free circular buffers.

---

## Supported Operations

| Op | Description |
|----|-------------|
| `Nop` | No-op (probe/test) |
| `Read` / `Readv` / `ReadFixed` | Read from fd |
| `Write` / `Writev` / `WriteFixed` | Write to fd |
| `Fsync` | Sync file data |
| `PollAdd` / `PollRemove` | Poll fd events |
| `Send` / `Recv` | Socket I/O |
| `Openat` / `Close` | File open/close |
| `Timeout` | Timer-based wait |
| `AsyncCancel` | Cancel pending op |
| `Splice` / `Tee` | Zero-copy data movement |

---

## API Reference

```rust
let mut ring = IoUring::new(1, 256, 0); // id=1, depth=256

// Register file for I/O
ring.register_file(3, file_data);

// Build SQEs
let sqes = vec![
    IoUringSqe::read(3, buf_addr, 4096, 0, user_tag_1),
    IoUringSqe::write(4, buf_addr, 4096, 0, user_tag_2),
    IoUringSqe::nop(user_tag_3),
];

// Submit and wait (synchronous path)
let cqes = ring.submit_and_wait(&sqes);
for cqe in &cqes {
    if cqe.is_ok() { println!("op {} completed: {} bytes", cqe.user_data, cqe.res); }
}

// System-wide manager
let mut mgr = IoUringManager::new();
let fd = mgr.setup(256, IORING_SETUP_SQPOLL); // kernel-polled
mgr.get_mut(fd).unwrap().submit_one(IoUringSqe::nop(0));
```

---

## Setup Flags

| Flag | Value | Description |
|------|-------|-------------|
| `IORING_SETUP_IOPOLL` | 0x1 | Poll for completions (no IRQ) |
| `IORING_SETUP_SQPOLL` | 0x2 | Kernel sq poll thread |
| `IORING_SETUP_SINGLE_ISSUER` | 0x1000 | Single submitter thread |

---

## Comparison

| Feature | Linux io_uring | POSIX AIO | SigmaOS io_uring |
|---------|---------------|-----------|-----------------|
| Zero-copy | Yes | No | Yes |
| Batch submit | Yes | No | Yes |
| Kernel poll | Yes | No | Yes |
| Ring buffer | Yes | No | Yes |
| no_std | No | No | **Yes** |
| Registered buffers | Yes | No | Yes |
