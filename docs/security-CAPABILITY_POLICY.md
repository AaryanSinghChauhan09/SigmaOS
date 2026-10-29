# SigmaOS Capability Security Policy

## Core Capability Tokens

| Capability Name | Bit | Description | Syscalls Controlled |
|---|---|---|---|
| `CAP_PROCESS_SPAWN` | 0 | Permission to fork/exec new process | `sys_fork`, `sys_execve` |
| `CAP_NET_BIND` | 1 | Bind socket to privileged ports (< 1024) | `sys_bind` |
| `CAP_FS_MOUNT` | 2 | Mount/unmount filesystems | `sys_mount`, `sys_unmount` |
| `CAP_STORAGE_RAW` | 3 | Direct raw block device write | `sys_ioctl_raw_storage` |
| `CAP_HARDWARE_REBOOT` | 4 | Initiate system reboot/shutdown | `sys_reboot` |
| `CAP_MEMORY_MAP` | 5 | Map kernel/physical device memory | `sys_mmap_device` |

---

## Token Verification Invariant

On every system call entrypoint:
1. Kernel inspects caller's task structure for valid capability token set.
2. If token is missing, syscall immediately aborts and returns `EPERM` (-1).
3. Privilege escalation attempts generate kernel security warning logs.
