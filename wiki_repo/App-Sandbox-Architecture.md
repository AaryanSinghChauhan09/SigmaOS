# App Sandbox Architecture

SigmaOS isolates third-party applications and userland shards using a capability-based sandboxing model.

## 1. Capability System
System operations are governed by fine-grained capability flags:
- `CAP_SYS_NET` (Networking)
- `CAP_SYS_FS_READ` (Filesystem read access)
- `CAP_SYS_FS_WRITE` (Filesystem write access)
- `CAP_SYS_PROCESS` (Spawn and manage threads)
- `CAP_SYS_HARDWARE` (Raw driver interface)

## 2. pledge/unveil
Applications use `pledge` to restrict their set of active capability tokens, preventing sandbox escapes.
