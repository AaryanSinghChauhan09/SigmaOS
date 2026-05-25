# SovereignFS — Sovereign Journaling Filesystem

**SovereignFS** (SFS) is SigmaOS's own filesystem designed for deterministic
write latency, cryptographic block integrity, and native rollback support.

## Design Principles
1. **Copy-on-Write (CoW):** Every write creates a new extent; old data is
   preserved until explicitly pruned. Enables instant snapshots.
2. **Journaling:** All metadata changes are journaled before data is written
   — survives power failures cleanly.
3. **Block Integrity:** Each 4 KB block carries a BLAKE3 checksum; the kernel
   rejects tampered blocks at read time.

## On-Disk Layout
```
[Superblock 4K] [Journal 64MB] [Inode Table] [Data Extents ...]
```

## Roadmap
- [ ] Superblock & journal format specification
- [ ] `sfs_mkfs` tool
- [ ] Kernel VFS integration
- [ ] Snapshot/rollback API
