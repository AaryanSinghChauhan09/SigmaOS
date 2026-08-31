# FreeBSD Capsicum Capability Mode Integration

## Overview

Capsicum is FreeBSD's capability-based security framework that SigmaOS adopts for fine-grained, object-capability access control. Unlike POSIX permissions or ACLs, Capsicum capabilities are unforgeable tokens that travel with file descriptors.

## Core Concepts

### Capability Rights

Every open file descriptor can have a subset of rights attached:

```rust
// src/security/capsicum.rs
bitflags::bitflags! {
    pub struct CapRights: u64 {
        const READ       = 1 << 0;  // CAP_READ
        const WRITE      = 1 << 1;  // CAP_WRITE
        const SEEK       = 1 << 2;  // CAP_SEEK
        const LOOKUP     = 1 << 3;  // CAP_LOOKUP (directory traversal)
        const FSTAT      = 1 << 4;  // CAP_FSTAT
        const FCNTL      = 1 << 5;  // CAP_FCNTL
        const MMAP       = 1 << 6;  // CAP_MMAP
        const MMAP_RX    = 1 << 7;  // CAP_MMAP_RX (exec mapping)
        const CONNECT    = 1 << 8;  // CAP_CONNECT (sockets)
        const ACCEPT     = 1 << 9;  // CAP_ACCEPT
        const BIND       = 1 << 10; // CAP_BIND
        const LISTEN     = 1 << 11; // CAP_LISTEN
        const RECV       = 1 << 12; // CAP_RECV
        const SEND       = 1 << 13; // CAP_SEND
        const SETSOCKOPT = 1 << 14; // CAP_SETSOCKOPT
        const GETSOCKOPT = 1 << 15; // CAP_GETSOCKOPT
        const IOCTL      = 1 << 16; // CAP_IOCTL
        const KQUEUE     = 1 << 17; // CAP_KQUEUE
        const PDWAIT     = 1 << 18; // CAP_PDWAIT (process descriptor)
        const PDKILL     = 1 << 19; // CAP_PDKILL
    }
}
```

### Capability Mode

Once a process enters capability mode (`cap_enter()`), it can no longer use global namespaces:
- No `open()` with absolute paths (must use `openat()` relative to a capability)
- No `socket()` system call (must receive socket fd from parent)
- No `getpid()` of other processes

```
Normal Mode                    Capability Mode
┌──────────────────┐          ┌──────────────────┐
│ open("/etc/conf") │          │ open("/etc/conf") │
│       ✓           │          │       ✗ (ECAPMODE)│
│                  │          │                  │
│ openat(dir_fd,   │          │ openat(dir_fd,   │
│   "conf")        │          │   "conf")        │
│       ✓           │          │       ✓           │
└──────────────────┘          └──────────────────┘
```

## SigmaOS Implementation

### Capability Token

```rust
/// An unforgeable capability token
#[derive(Debug)]
pub struct CapabilityToken {
    id: u64,
    rights: CapRights,
    target_fd: i32,
    is_revoked: AtomicBool,
}

impl CapabilityToken {
    /// Restrict rights (can only reduce, never expand)
    pub fn restrict(&self, new_rights: CapRights) -> Result<Self, CapError> {
        // Bitwise AND ensures we never grant more rights than we have
        let restricted = self.rights & new_rights;
        if restricted != new_rights {
            return Err(CapError::InsufficientRights);
        }
        Ok(CapabilityToken {
            id: next_cap_id(),
            rights: restricted,
            target_fd: self.target_fd,
            is_revoked: AtomicBool::new(false),
        })
    }

    /// Check if an operation is permitted
    pub fn check(&self, required: CapRights) -> Result<(), CapError> {
        if self.is_revoked.load(Ordering::SeqCst) {
            return Err(CapError::Revoked);
        }
        if self.rights.contains(required) {
            Ok(())
        } else {
            Err(CapError::InsufficientRights)
        }
    }
}
```

### Process Capability Table

```rust
pub struct ProcessCapTable {
    caps: BTreeMap<u64, CapabilityToken>,  // cap_id → token
    fd_to_cap: BTreeMap<i32, u64>,         // fd → cap_id
    cap_mode: bool,                         // Is process in capability mode?
}

impl ProcessCapTable {
    /// Enter capability mode (irreversible)
    pub fn cap_enter(&mut self) {
        self.cap_mode = true;
    }

    /// Open with capability (openat semantics)
    pub fn openat_cap(&self, dir_cap: u64, path: &str) -> Result<i32, CapError> {
        let cap = self.caps.get(&dir_cap).ok_or(CapError::InvalidToken)?;
        // path must not contain ".." components
        if path.contains("..") {
            return Err(CapError::PathNotAllowed);
        }
        cap.check(CapRights::LOOKUP)?;
        // ... kernel VFS traversal from cap's directory ...
        Ok(0) // new fd
    }
}
```

## Integration with pledge/unveil

SigmaOS uses a layered approach:

```
syscall → pledge check → unveil check → capsicum cap check → execute
```

This is more restrictive than any single mechanism:
- **pledge**: what syscalls can be called
- **unveil**: what paths can be accessed
- **capsicum**: what rights each fd has

## Comparison with Other Mechanisms

| Feature | Capsicum | SELinux | AppArmor | pledge/unveil |
|---------|----------|---------|----------|---------------|
| Granularity | Per-fd capability | Label-based | Path-based | Promise-based |
| Complexity | Medium | Very High | Medium | Low |
| Performance | Low overhead | Context lookups | Path lookups | Fast |
| Composability | Excellent | Good | Limited | Good |
| BSD Origins | FreeBSD | No | No | OpenBSD |

## References

- FreeBSD `cap_enter(2)`, `cap_rights_limit(2)` manpages
- [Capsicum: practical capabilities for UNIX](https://www.usenix.org/legacy/event/sec10/tech/full_papers/Watson.pdf)
- `src/security/capsicum.rs`
- `src/security/capability.rs`
- `src/security/capability_token.rs`
