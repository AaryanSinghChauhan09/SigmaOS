# BSD Inspirations and Parity in SigmaOS

## Overview

SigmaOS absorbs foundational, high-reliability concepts from the BSD family (**FreeBSD**, **OpenBSD**, and **NetBSD**) to achieve superior system stability, auditability, and network throughput.

---

## Key Technologies Adopted

```
┌─────────────────────────────────────────────────────────────┐
│                     SigmaOS BSD Parity                      │
├──────────────────────────────┬──────────────────────────────┤
│ OpenBSD                      │ FreeBSD                      │
│ - pledge() / unveil() sandbox│ - kqueue / kevent            │
│ - W^X memory permissions     │ - Capsicum capabilities      │
│ - Arc4random CSPRNG          │ - ZFS / B-Tree snapshotting  │
├──────────────────────────────┼──────────────────────────────┤
│ NetBSD                       │ DragonFly BSD                │
│ - Rump kernel architecture   │ - HAMMER2 CoW semantics      │
│ - Highly portable drivers    │ - Lockless token scheduling  │
└──────────────────────────────┴──────────────────────────────┘
```

---

## Implementations in SigmaOS

### 1. OpenBSD `pledge` and `unveil` Equivalents
Integrated into [`src/distro/linux_bsd_inspirations.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/distro/linux_bsd_inspirations.rs) and [`src/system/sandbox.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/system/sandbox.rs):
- Restricted system call subsets declared per process.
- Hierarchical filesystem view restricted via `unveil` before entering event loops.

### 2. FreeBSD `kqueue` Event Notification
Integrated alongside `io_uring` to provide scalable event dispatching for file descriptors, signals, timers, and userspace notifications.

### 3. Capsicum-Style Capability Rights
Integrated into [`src/security/capability.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/security/capability.rs) and [`src/security/capability_enforcer.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/security/capability_enforcer.rs), assigning explicit rights matrices directly to descriptors.
