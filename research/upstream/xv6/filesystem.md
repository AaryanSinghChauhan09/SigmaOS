# Upstream Study: xv6 Simple Inode Filesystem

1. **Upstream Problem**: On-disk crash recovery, inode indexing, and block buffer caching.
2. **Data Structure & Control Flow**: Buffer cache (`bcache`), transaction log (`log`), inodes (`dinode`), block bitmaps.
3. **Security & Failure Assumptions**: Disk write panics on unrecoverable log header corruption.
4. **Direct Adoption**: Transaction logging concept and 4-layer filesystem structure (block, log, inode, file).
5. **Rust/SigmaOS Adaptation**: Safe Rust RAM disk VFS backend with transactional journaling.
6. **Exit Test Criteria**: VFS performs write transaction, crashes mid-flight in test, and restores coherent state.
7. **Licensing & Attribution**: MIT License (xv6 / MIT OS Group).
