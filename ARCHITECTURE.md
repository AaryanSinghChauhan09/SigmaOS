# SigmaOS Technical Architecture

This document outlines the high-level technical architecture of SigmaOS. For more detailed and up-to-date information, please see our [Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) (specifically the Architecture section).

## Overview

SigmaOS is built on a **microkernel architecture** written in Rust. It emphasizes:

1.  **Memory Safety**: Leveraging Rust's ownership model to prevent memory leaks and data races.
2.  **Capability-based Security**: Sentinel, our security subsystem, ensures that processes only access what they explicitly have tokens for.
3.  **Modularity**: Device drivers, network stacks, and filesystems run as unprivileged user-space servers.

## Core Components

*   **The Microkernel**: Handles IPC, thread scheduling, and interrupts.
*   **VFS (Virtual Filesystem)**: User-space server coordinating mount points and inode access.
*   **sigpkg**: Universal multiformat packaging system capable of handling source compilation and OCI images.
*   **Palette**: The display server and graphical user interface.
*   **Bolt**: A low-latency audio subsystem.

Please refer to the source tree (`src/kernel`, `src/servers`, etc.) for implementation details.
