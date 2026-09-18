# AI Agent Filesystem Management in SigmaOS

## Overview

SigmaOS filesystem architecture (`src/filesystem/`, `src/filesystem/vfs.rs`, `src/filesystem/sigmafs.rs`, `src/filesystem/cow_snapshot.rs`) implements a zero-copy Virtual Filesystem (VFS) abstraction layer supporting Copy-on-Write (CoW) snapshots, pseudo-filesystems (`procfs`, `sysfs`, `devfs`, `configfs`), OpenBSD `unveil()` path restrictions, and high-performance file descriptors.

AI agents (such as Jules, Herdr agentic tasks, and automated code refactoring subagents) must adhere to these filesystem guidelines when navigating, reading, and modifying files.

---

## Filesystem Architecture & VFS

```
Agent File Request → OpenBSD Unveil Sandbox Gate (`check_path`)
                             │
                             ▼
                 SigmaOS VFS Virtual Mounts
                             │
        ┌────────────────────┼────────────────────┐
        ▼                    ▼                    ▼
   SigmaFS / Ext4       ProcFS / SysFS       CoW Snapshots
(Persistent Files)   (Pseudo Telemetry)   (Btrfs/ZFS/HAMMER2)
```

---

## 1. Unveil Path Restrictions for AI Agents

Before executing userland scripts or modifying workspace directories, AI agents MUST restrict their visible filesystem hierarchy:

```rust
use sigmaos::filesystem::UnveilEngine;

let mut unveil = UnveilEngine::new();

// Expose workspace directory with read/write/create access
unveil.unveil("/userland/workspace", "rwc")?;

// Expose temporary directory
unveil.unveil("/tmp", "rwc")?;

// Lock unveil ruleset (subsequent calls will fail)
unveil.lock();
```

---

## 2. Copy-on-Write (CoW) Snapshots for Safe Modifications

When modifying core system files or executing multi-file refactoring operations, agents MUST create a point-in-time CoW snapshot:

```rust
use sigmaos::filesystem::CowSnapshotEngine;

let mut cow_engine = CowSnapshotEngine::new();

// Write new block under CoW semantics (prevents corrupting existing block)
let block_id = cow_engine.write_block(b"fn main() { println!(\"SigmaOS\"); }");

// Clone block reference with O(1) metadata copy
let clone_id = cow_engine.cow_clone_block(block_id)?;
```

---

## 3. Pseudo-Filesystems (`procfs`, `sysfs`, `devfs`, `configfs`)

AI agents can query system state, CPU statistics, and hardware devices via standard pseudo-filesystems:

| Pseudo-FS | Mount Point | Purpose for AI Agents |
|-----------|-------------|-----------------------|
| `/proc` | `/proc/` | Read PID status (`/proc/[pid]/status`), memory info (`/proc/meminfo`) |
| `/sys` | `/sys/` | Query cgroup controllers (`/sys/fs/cgroup/`) and kernel modules |
| `/dev` | `/dev/` | Access character/block devices (`/dev/null`, `/dev/zero`, `/dev/urandom`) |
| `/config` | `/config/` | Dynamic runtime kernel configuration tree (`configfs`) |

---

## 4. File Descriptor Management & Cleanups

Agents MUST close unused file descriptors (`SovereignFileDescriptor`) promptly to avoid FD exhaustion (`EMFILE`):

```rust
use sigmaos::filesystem::SovereignFileDescriptor;

// Scope file descriptor handle
{
    let fd = SovereignFileDescriptor::open("/userland/workspace/src/lib.rs", "r")?;
    let contents = fd.read_to_string()?;
    // FD automatically closed on drop
}
```
