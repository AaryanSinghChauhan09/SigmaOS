# SigmaOS POSIX Shared Memory

## Overview

SigmaOS provides POSIX-compatible shared memory via `shm_open`/`shm_unlink`/`ftruncate`/`mmap`.

**Location:** `src/ipc/sigma_shm.rs`

---

## API

```rust
let mut ns = SigmaShmNamespace::new();

// Create shared memory object
let fd = ns.shm_open("/my_shm", O_CREAT | O_RDWR, 0o600, uid, now_ns).unwrap();

// Set size
ns.ftruncate(fd, 65536).unwrap();

// Write from process A
ns.write(fd, 0, b"hello from A").unwrap();

// Read from process B (using same fd or reopen)
let mut buf = [0u8; 12];
ns.read(fd, 0, &mut buf).unwrap();

// Cleanup
ns.close(fd).unwrap();
ns.shm_unlink("/my_shm").unwrap();
```
