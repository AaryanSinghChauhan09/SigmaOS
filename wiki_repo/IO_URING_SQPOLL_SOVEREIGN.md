# SigmaOS Sovereign io_uring SQPOLL Engine

## Overview

SigmaOS implements **io_uring SQPOLL (Submission Queue Polling)** in 100% Safe Rust (`src/kernel/io_uring_sqpoll_sovereign.rs`), providing **zero-syscall asynchronous I/O execution**.

In standard Linux asynchronous I/O, application processes must issue the `io_uring_enter(2)` system call to notify the kernel of new submission entries. With SQPOLL, a dedicated kernel polling worker thread continuously sweeps the submission queue ring buffer, eliminating system call overhead entirely for hot I/O loops.

## Core Architecture

```
User Space (App)                           Kernel Space (SigmaOS)
┌───────────────────────────┐              ┌───────────────────────────┐
│ Writes SQEs to shared ring│              │ Kernel worker thread      │
│ buffer without syscall    │              │ polls SQ ring continuously│
└─────────────┬─────────────┘              └─────────────┬─────────────┘
              │                                          │
              ▼                                          ▼
┌───────────────────────────┐              ┌───────────────────────────┐
│ SQ Ring Buffer (Lockless) │ ───────────► │ Executes I/O operations   │
└───────────────────────────┘              │ (Readv, Writev, Send, etc)│
                                           └─────────────┬─────────────┘
                                                         │
                                                         ▼
┌───────────────────────────┐              ┌───────────────────────────┐
│ Reaps CQEs from shared    │ ◄─────────── │ Posts CQEs with results   │
│ completion ring buffer    │              │ to CQ ring buffer         │
└───────────────────────────┘              └───────────────────────────┘
```

## Idle Power Optimization

To conserve CPU cycles during quiescent intervals:
1. If the submission queue remains empty for `idle_timeout_ticks`, the kernel SQ thread transitions to low-power sleep.
2. The user application detects the sleep state via `needs_wakeup()` (`IORING_SQ_NEED_WAKEUP`) and issues a lightweight wakeup event.

## Test Verification

6 standalone unit tests verified in test runner suite `[17]`:
- `test_sqpoll_submission_and_completion`: Zero-syscall submit and completion reap.
- `test_sqpoll_idle_sleep_transition`: Idle threshold detection and thread sleep/wake.
- `test_sqpoll_ring_capacity_limit`: Fixed capacity enforcement.
- `test_sqpoll_batch_processing`: Multi-SQE batch polling and posting.
- `test_sqpoll_reap_empty`: Empty completion queue handling.
- `test_sqpoll_nop_operation`: NOP opcode processing.
