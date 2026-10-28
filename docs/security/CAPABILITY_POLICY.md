# SigmaOS Capability Policy

## Overview
SigmaOS replaces root user privileges with granular POSIX capabilities and capability tokens.

---

## Core Capabilities

| Capability | Description | Permitted Subsystems |
|---|---|---|
| `CAP_SYS_ADMIN` | Perform mount, unmount, swap operations | System init, Storage daemon |
| `CAP_NET_RAW` | Open raw network sockets, bind privileged ports | Network stack, VPN agent |
| `CAP_SYS_PTRACE` | Trace processes, read process memory | Debugger, tracing tools |
| `CAP_DAC_OVERRIDE` | Bypass VFS permission checks | Backup agent |
| `CAP_SYS_TIME` | Set system RTC and wall clock time | NTP daemon |

---

## Capability Invariants

1. Capabilities are immutable once dropped by a process.
2. Children inherit process capabilities only if `CAP_INHERITABLE` is explicitly set.
3. Every privileged syscall checks caller process capabilities prior to execution.
