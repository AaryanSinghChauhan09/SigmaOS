# POSIX Compatibility Shim

A lightweight **opt-in** translation layer that allows existing POSIX/Linux
applications to run on SigmaOS without modification, while keeping the
sovereign kernel clean.

## Architecture

```
Linux ELF Binary
   └─ SigmaCompat loader (intercepts syscalls)
         └─ Translates POSIX syscalls → Sovereign Syscall ABI
               └─ SigmaOS Kernel
```

## What It Covers

| POSIX Syscall | Sovereign Translation |
| --- | --- |
| `open()` | `sigma_vfs_open()` |
| `read()` / `write()` | `sigma_io_*()` |
| `fork()` | `sigma_spawn_shard()` |
| `pthread_*` | Sovereign task primitives |

## What It Does NOT Cover

- `ioctl()` calls that touch hardware directly (forbidden by capability model)

- Signals that violate the determinism contract

## Roadmap

- [x] ELF loader with syscall interception
- [x] `mmap()` translation
- [x] Dynamic linker shim
- [ ] Signal handling translation
- [ ] Socket API translation
- [ ] Thread library (pthread) translation
- [ ] File descriptor management
- [ ] Environment variable handling
- [ ] Process group/session management
- [ ] Terminal I/O control (termios)
