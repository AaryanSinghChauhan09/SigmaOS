# SigmaOS Sovereign fanotify Filesystem Access Notification

## Overview

SigmaOS implements a **100% Safe Rust sovereign fanotify (Filesystem Access Notification) subsystem** (`src/fs/fanotify_sovereign.rs`), absorbing the kernel-level file monitoring and access control engine introduced in Linux 2.6.37 and extended through Linux 5.x.

Unlike `inotify` which only reports past events asynchronously, `fanotify` provides **synchronous permission events** allowing security agents and anti-malware monitors to inspect and block file operations (`open`, `read`, `exec`) before they are executed by the kernel.

## Event Types & Capabilities

- **Notification Events**:
  - `FAN_ACCESS`: File data was read.
  - `FAN_MODIFY`: File data was altered.
  - `FAN_OPEN`: File descriptor was opened.
  - `FAN_CLOSE_WRITE` / `FAN_CLOSE_NOWRITE`: File closed after modification or read-only access.
- **Permission Hooks (Synchronous Blocking)**:
  - `FAN_OPEN_PERM`: Blocks file open until userspace yields `FAN_ALLOW` or `FAN_DENY`.
  - `FAN_ACCESS_PERM`: Blocks file read until verified.
- **Hierarchical Marks**:
  - Path marks, directory hierarchy marks, and entire filesystem/mount marks (`FAN_MARK_MOUNT`).

## Test Verification

6 standalone unit tests verified in test runner suite `[21]`:
- `test_fanotify_mark_and_watch`: Specific path and event mask registration.
- `test_fanotify_notification_event_queue`: Event generation and FIFO dequeue.
- `test_fanotify_permission_allow`: Validated file open permission granted.
- `test_fanotify_permission_deny`: Malicious file access blocked with `-EPERM`.
- `test_fanotify_wildcard_mount_mark`: Filesystem-wide wildcard mark monitoring.
- `test_fanotify_read_empty_queue`: Graceful empty queue handling.
