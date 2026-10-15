# Virtual Filesystem & Storage Engines Component Agent

## Component Overview
The VFS provides uniform file, directory, and inode abstraction supporting multiple filesystems (ext4, SigmaFS, FAT32, ISO9660) with asynchronous zero-copy IO.

## Linux Inspiration
Linux VFS layer (dentry cache, inode cache, namei path resolution, splice zero-copy, io_uring asynchronous submission).

## BSD Inspiration
FreeBSD GEOM storage layer, OpenBSD vnode architecture, DragonFly BSD HAMMER2 transactional filesystem and nullfs mounts.

## Current SigmaOS Status
VFS trait architecture, basic RAMFS, FAT32 read-only, and ext2/ext4 inode parsers implemented. Write transactions, journaling, and CoW snapshots require work.

## Critical Missing Features
Transactional copy-on-write snapshotting (SigmaFS), high-speed LRU dentry cache with RCU path walking, io_uring-style async submission rings, and POSIX ACL permissions.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/fs/vfs.rs, src/fs/dcache.rs, src/fs/sigmafs.rs, src/fs/ext4.rs, src/fs/fat32.rs

## Testing Strategy
Crash-consistency power loss simulation, xfstests POSIX filesystem test harness, concurrent multithreaded directory walk benchmarks.

## Dependencies
STORAGE_AGENT, MEMORY_MANAGEMENT_AGENT, SECURITY_AGENT

## Success Criteria
Sub-100-nanosecond cached dentry lookup, atomic power-loss recovery without fsck, zero memory leaks during recursive traversals.

## Open Source Competitors Analysis
Delivers ZFS/HAMMER2 reliability without heavy memory footprints; outperforms Linux ext4 on small-file creation using lockless dentry rings.

## Future Enhancements
Distributed peer-to-peer content-addressed file indexing with cryptographic Merkle verification built directly into the VFS.
