# AI Agent Future Development Roadmap: Linux & BSD Kernel, Storage, & Security Parity

## Architectural Directive
This document specifies the AI agent development directives for OS kernel scheduling, CoW storage systems, and capability sandboxing across Linux & BSD distributions in SigmaOS:

### 1. Kernel CPU Scheduler Parity (BORE / SchedExt / ULE / EEVDF)
- **CachyOS BORE Scheduler**: Burst-Oriented Response Enhancer calculating dynamic timeslices (`bore_sched_timeslice_ns`) based on task CPU burst history.
- **Linux 6.12+ `sched_ext`**: User-space and eBPF-programmable CPU scheduler policy dispatching.
- **FreeBSD ULE Scheduler**: Dual-queue interactive boost scoring for desktop responsiveness.

### 2. Copy-on-Write (CoW) Storage Parity (ZFS, Btrfs, HAMMER2, Bcachefs)
- **DragonFly BSD HAMMER2**: PFS (Pseudo Filesystem) multi-master transaction replication and instantaneous zero-space snapshots.
- **FreeBSD ZFS Boot Environments (`bectl`)**: Atomic dataset snapshotting, boot menu integration, and rollbacks.
- **openSUSE Snapper Btrfs / MicroOS**: Read-only root filesystem deployments with transactional rollback staging.

### 3. Capability Sandboxing & Mandatory Access Control
- **OpenBSD `pledge(2)` & `unveil(2)`**: Promise restricting (`rpath`, `wpath`, `cpath`, `inet`, `stdio`) and path unveil locking (`unveil(NULL, NULL)`).
- **FreeBSD Capsicum & Casper**: Fine-grained file descriptor rights restriction (`cap_rights_limit`) and Casper IPC daemon service delegation.
- **Linux Landlock V5 & Seccomp-BPF**: Path-based filesystem access rules and syscall filtering.
