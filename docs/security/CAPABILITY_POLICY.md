# SigmaOS Kernel Capability Policy

## Overview

SigmaOS utilizes capability tokens (`CapabilityToken`) to enforce fine-grained access control across kernel syscalls and process resources.

---

## Capability Bitmask Specification

| Capability Bit | Name | Description |
|---|---|---|
| `CAP_SYS_ADMIN` | Admin Control | Perform root administrative tasks (mount, debug, module load) |
| `CAP_NET_ADMIN` | Network Config | Bind privileged ports (<1024), modify routing tables |
| `CAP_DAC_OVERRIDE` | Bypass VFS Perms | Bypass filesystem read/write/execute permission checks |
| `CAP_SYS_RAWIO` | Raw I/O Access | Access raw disk block devices and I/O ports |
| `CAP_KILL` | Process Signal | Send signals to processes owned by other users |
| `CAP_SYS_PTRACE` | Process Trace | Trace or inspect arbitrary process execution state |

---

## Verification Policy

1. Every privileged syscall MUST verify the process capability mask before executing the request:
   ```rust
   if !current_task().capabilities().has(CapabilityToken::CAP_SYS_ADMIN) {
       return Err(SyscallError::PermissionDenied);
   }
   ```
2. Unprivileged processes default to an empty capability set.
