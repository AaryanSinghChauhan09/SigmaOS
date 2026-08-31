# SigmaOS Security: Pledge & Sandbox Subsystem

SigmaOS implements a hardened, OpenBSD-inspired capability restriction model to sandbox processes without relying on the Linux kernel LSM (SELinux/AppArmor) framework. The system is fully compatible with `#![no_std]` targets.

## OpenBSD `pledge(2)` Parity

Located in [`src/security/pledge.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/security/pledge.rs), SigmaOS implements a faithful port of OpenBSD's `pledge(2)` syscall:

### Pledge Permissions

    stdio     — basic I/O operations
    rpath     — read-only filesystem access
    wpath     — write filesystem access
    cpath     — path creation (mkdir, rename)
    exec      — process execution
    inet      — network socket creation
    dns       — DNS resolver lookups
    proc      — process management (fork/wait)

### Usage in Rust

```rust
use crate::security::pledge::{Pledge, Permission};

let mut pledge = Pledge::new();
pledge.add_permission(Permission::Stdio);
pledge.add_permission(Permission::Rpath);
pledge.commit().expect("pledge failed — process violated policy");
```

***

## Firejail-Inspired Sandboxing

Located in [`src/security/sandbox.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/security/sandbox.rs), the SigmaOS sandbox subsystem borrows from Firejail's namespace isolation model:

*   **Network namespaces**: Isolates the sandbox from host networking
*   **Filesystem overlays**: Read-only bind mounts over sensitive paths
*   **Seccomp profile generator**: Auto-generates minimal syscall allowlists
*   **Capability dropping**: Drops all Linux capabilities to minimal needed set

***

## Sandboxie-Inspired Process Isolation

SigmaOS also implements Windows Sandboxie-style token-level isolation for processes. This allows running untrusted packages in a fully isolated token context, preventing access to the host filesystem and registry equivalents.

Key design points:

*   Custom token namespace: Each sandbox has a unique security token ID
*   Object access mediation: All IPC, file, and device requests are mediated
*   Write redirected: Writes to protected paths are redirected to a COW layer

***

## BSD Jails Integration

Inspired by FreeBSD Jails, SigmaOS can create lightweight, kernel-enforced execution environments:

*   Network identity isolation (each jail has its own IP/routing context)
*   Filesystem root isolation (chroot-like without the historical weaknesses)
*   Process isolation (jail cannot signal processes outside its boundary)

See [`docs/components/VIRTUALIZATION.md`](../docs/components/VIRTUALIZATION.md) for container details.
