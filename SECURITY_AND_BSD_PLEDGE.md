# Security and BSD Pledge Integration

In an effort to improve the security posture of SigmaOS and reduce code-scanning alerts, we have integrated a concept inspired by OpenBSD's `pledge(2)` system call.

## The Pledge System

The `bsd_pledge` module provides a way for processes to voluntarily restrict their own access to system resources. Once a process calls `pledge`, it can only reduce its privileges further, never increase them.

## Reduced Dependencies

We have also initiated a massive effort to reduce dependencies on pre-defined standard library functions, transitioning towards `core` and `alloc` crates to allow for a pure `no_std` kernel environment where possible.
