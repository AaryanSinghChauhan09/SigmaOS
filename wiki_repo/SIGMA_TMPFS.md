# SigmaOS tmpfs — In-Memory Filesystem

## Overview

SigmaOS tmpfs is a sovereign in-memory virtual filesystem. Files and directories exist only in memory and are lost on unmount. Used for `/tmp`, `/run`, shared memory, and ramfs applications.

**Location:** `src/fs/sigma_tmpfs.rs`

---

## Features

- Regular files (Vec<u8>-backed, random-access read/write)
- Directories (BTreeMap<name, inode>)
- Symbolic links with loop detection (max 8 levels)
- Hard links via nlink reference counting
- File truncation and append
- Unix permission mode (UnixMode)
- Full atime/mtime/ctime tracking
- Configurable size limits
- Inode accounting

---

## API Reference

```rust
let mut fs = TmpfsMount::new("tmpfs", 128 * 1024 * 1024); // 128MB limit
fs.set_time(now_ns);

// Create directory tree
fs.mkdir("/proc").unwrap();
fs.mkdir("/tmp").unwrap();

// Create and write a file
let ino = fs.create("/tmp/hello.txt").unwrap();
fs.write_ino(ino, 0, b"Hello SigmaOS").unwrap();

// Read file
let mut buf = vec![0u8; 13];
fs.read_ino(ino, 0, &mut buf).unwrap();

// Symbolic link
fs.symlink("/tmp/link", "/tmp/hello.txt").unwrap();

// Directory listing
let entries = fs.readdir("/tmp").unwrap();

// File stats
let inode = fs.stat("/tmp/hello.txt").unwrap();
println!("size={} mtime={}", inode.size(), inode.mtime_ns);

// Unlink
fs.unlink("/tmp/hello.txt").unwrap();

// Usage statistics
let (used, avail) = fs.usage();
```

---

## Comparison

| Feature | Linux tmpfs | BSD md(4) | SigmaOS tmpfs |
|---------|------------|----------|--------------|
| Memory-backed | Yes (page cache) | Yes (swap) | Yes (Vec<u8>) |
| Size limit | Yes (size= mount opt) | Yes | Yes |
| Symlinks | Yes | No | Yes |
| POSIX permissions | Yes | No | Yes |
| no_std | No | No | **Yes** |
| Persistence | No | Optional | No |

---

## Use Cases

- `/tmp` — temporary files
- `/run` — runtime state (PID files, sockets)  
- `/dev/shm` — POSIX shared memory
- Build system scratch space
- Container overlay layers
