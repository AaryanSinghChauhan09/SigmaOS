# Comparative Analysis: VFS & Filesystems

| Metric / Feature | Linux | xv6 | FreeBSD | SigmaOS Target |
|---|---|---|---|---|
| Path Resolver | `namei.c` dcache walk | Inode path lookup | VFS namei | Centralized path resolver |
| FD Table | Array with `files_struct` | Array `struct file` | `filedesc` | Bounded process FD table |
| Crash Consistency | Journaling / CoW | Logging | Soft updates / ZFS | RAM-disk / CoW transaction log |
