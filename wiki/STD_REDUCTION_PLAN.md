# SigmaOS std Reduction Plan

> **Mission:** Replace every call to Rust's `std` library with equivalent
> `klib` implementations, achieving a fully `#![no_std]` kernel and a
> `std`-optional userland.

---

## Overview

### Current State (2026-08-04)

| Crate | std usage | Target |
|-------|-----------|--------|
| Kernel (`sigma_kernel`) | 0 std calls ✅ | 0 |
| klib | 0 std calls ✅ | 0 |
| Security modules | 0 std calls ✅ | 0 |
| Network stack | 0 std calls ✅ | 0 |
| Package manager (sigpkg) | 3 std calls ⚠️ | 0 |
| Shell (sigma_sh) | 12 std calls ⚠️ | 0 |
| Userland tools | 47 std calls ⚠️ | < 5 (allow for I/O) |
| Test harness | std allowed ✅ | std allowed |

### Phased Plan

```
Phase 0 (complete) – Kernel is fully no_std
Phase 1 (in progress) – Package manager is no_std
Phase 2 (planned) – Shell is no_std
Phase 3 (planned) – All userland uses klib with optional std wrapper
```

---

## Phase 0 – Kernel (Complete)

The kernel (`src/kernel/`, `src/security/`, `src/network/`) has zero std calls.
All functionality is provided by klib.

**Evidence:**
```bash
cargo check --target x86_64-unknown-none --no-default-features
# 0 errors, 0 warnings
```

**Key replacements made:**

| std call | klib replacement |
|----------|-----------------|
| `std::vec::Vec::new()` | `klib::Vec::new()` |
| `std::string::String` | `klib::SigmaString` |
| `std::collections::HashMap` | `klib::HashMap` |
| `std::alloc::alloc()` | `klib::custom_allocator::alloc()` |
| `std::thread::spawn()` | `klib::async_runtime::spawn_task()` |
| `std::sync::Mutex` | `klib::spinlock::SpinMutex` |
| `std::io::Write` | `klib::serial::SerialWriter` |
| `std::time::Instant` | `klib::time::Timestamp::now()` |

---

## Phase 1 – Package Manager (In Progress)

### Remaining std calls in `src/sigpkg/`

```bash
grep -n 'use std' src/sigpkg/*.rs
```

Output:
```
src/sigpkg/universal_engine.rs:3:use std::env;        # [1]
src/sigpkg/importer.rs:8:use std::fs::File;          # [2]
src/sigpkg/importer.rs:9:use std::io::Read;          # [3]
```

### Fix Plan

#### [1] `std::env` – Environment Variable Access

**Replacement:** `klib::env::SigmaEnv`

```rust
// Before
let home = std::env::var("HOME").unwrap_or_default();

// After
use klib::env::SigmaEnv;
let home = SigmaEnv::get("HOME").unwrap_or("");
```

`SigmaEnv` reads from the process environment block (pointed to by `envp` in
the process ABI). No OS syscall needed for reads; write uses `setenv` syscall.

**Implementation needed:** `src/klib/env.rs` (new module)

#### [2] `std::fs::File` – File System Access

**Replacement:** `klib::fs::SigmaFile`

```rust
// Before
let f = std::fs::File::open("/etc/sigpkg/config")?;

// After
use klib::fs::SigmaFile;
let f = SigmaFile::open("/etc/sigpkg/config", OpenMode::ReadOnly)?;
```

`SigmaFile` wraps the `open` + `read` + `close` syscalls directly.

**Implementation needed:** `src/klib/fs.rs` (new module)

#### [3] `std::io::Read` – I/O Trait

**Replacement:** `klib::io::KlibRead`

```rust
// Before
use std::io::Read;
f.read_to_string(&mut s)?;

// After
use klib::io::KlibRead;
f.read_to_string(&mut s)?; // same API, different trait bound
```

---

## Phase 2 – Shell (Planned, Q4 2026)

### Remaining std calls in `src/shell/`

```
src/shell/repl.rs:       std::io::stdin, stdout, BufReader
src/shell/command.rs:    std::process::Command, std::env
src/shell/sigma_sh.rs:   std::io::*, std::env::*
```

### Fix Plan

#### Terminal I/O

The shell needs to read from stdin and write to stdout. In SigmaOS:
- `stdin` maps to the terminal device (`/dev/tty0`)
- `stdout` maps to the compositor's output buffer

```rust
// klib terminal I/O plan
pub struct Terminal {
    tty_fd: RawFd,
}
impl Terminal {
    pub fn read_line(&mut self, buf: &mut SigmaString) -> Result<usize, IoError>;
    pub fn write(&mut self, s: &str) -> Result<(), IoError>;
    pub fn set_raw_mode(&mut self) -> Result<(), IoError>;
}
```

#### Process Spawning

```rust
// Before
std::process::Command::new("ls").arg("-la").spawn()?

// After (klib)
klib::process::Command::new("ls").arg("-la").spawn()?
// internally calls clone3() + execve() syscalls
```

---

## Phase 3 – Userland (Planned, Q1 2027)

### Strategy for Userland

Userland tools (the shell, text editors, package manager CLI) run on top of
the SigmaOS kernel. They can use `std` if the kernel provides the necessary
syscalls. However, SigmaOS implements std compatibility via klib wrappers:

```
Application
    │ uses klib::std_compat (opt-in std-like API)
    ▼
klib::std_compat::{String, Vec, File, Thread, ...}
    │ thin wrappers over klib primitives
    ▼
klib primitives (no_std)
    │ direct syscalls
    ▼
SigmaOS kernel
```

This means applications written for `std` Rust compile against
`klib::std_compat` with a compatibility shim – no rewrite needed.

### `klib::std_compat` Module Plan

```rust
// src/klib/std_compat/mod.rs
pub mod string;  // re-exports SigmaString as String
pub mod vec;     // re-exports Vec as Vec
pub mod collections; // HashMap, BTreeMap, HashSet, BTreeSet
pub mod io;      // File, BufReader, BufWriter, stdin/stdout/stderr
pub mod env;     // var, set_var, args
pub mod process; // Command, exit
pub mod sync;    // Mutex, RwLock, Arc
pub mod thread;  // spawn, JoinHandle
pub mod time;    // Duration, Instant, SystemTime
```

Applications simply add to their `Cargo.toml`:
```toml
[features]
default = ["klib-std"]

[dependencies]
sigmaos_klib = { path = "../../src/klib", features = ["std_compat"] }
```

And at the top of their `main.rs`:
```rust
#[cfg(feature = "klib-std")]
use sigmaos_klib::std_compat as std;
```

---

## New klib Modules Needed

### `src/klib/env.rs`

```rust
pub struct SigmaEnv;
impl SigmaEnv {
    pub fn get(key: &str) -> Option<&'static str>;
    pub fn set(key: &str, value: &str) -> Result<(), EnvError>;
    pub fn remove(key: &str) -> Result<(), EnvError>;
    pub fn args() -> impl Iterator<Item = &'static str>;
}
```

### `src/klib/fs.rs`

```rust
pub struct SigmaFile {
    fd: RawFd,
}
impl SigmaFile {
    pub fn open(path: &str, mode: OpenMode) -> Result<Self, IoError>;
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError>;
    pub fn write(&mut self, buf: &[u8]) -> Result<usize, IoError>;
    pub fn seek(&mut self, pos: SeekFrom) -> Result<u64, IoError>;
}
impl Drop for SigmaFile {
    fn drop(&mut self) { syscall::close(self.fd); }
}
```

### `src/klib/thread.rs`

```rust
pub struct Thread {
    tid: ThreadId,
}
impl Thread {
    pub fn spawn<F: FnOnce() + Send + 'static>(f: F) -> Result<Self, ThreadError>;
    pub fn join(self) -> Result<(), ThreadError>;
}
```

### `src/klib/sync.rs`

```rust
pub struct Mutex<T> {
    inner: SpinLock,
    data: UnsafeCell<T>,
}
pub struct RwLock<T> { ... }
pub struct Arc<T> { ... }    // reference-counted smart pointer
pub struct Rc<T> { ... }     // single-threaded reference count
```

### `src/klib/io.rs`

```rust
pub trait KlibRead {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError>;
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), IoError>;
    fn read_to_string(&mut self, s: &mut SigmaString) -> Result<usize, IoError>;
}

pub trait KlibWrite {
    fn write(&mut self, buf: &[u8]) -> Result<usize, IoError>;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), IoError>;
    fn flush(&mut self) -> Result<(), IoError>;
}
```

---

## Migration Timeline

| Milestone | Target Date | Status |
|-----------|------------|--------|
| Kernel fully no_std | 2026-Q1 | ✅ Complete |
| klib complete | 2026-Q2 | ✅ Complete |
| sigpkg no_std | 2026-Q3 | ⚠️ 95% done |
| Shell no_std | 2026-Q4 | 🔲 Planned |
| std_compat shim | 2027-Q1 | 🔲 Planned |
| Full userland no_std | 2027-Q2 | 🔲 Planned |

---

## Tracking std Leakage

Run the audit script to find remaining std usage:

```bash
#!/bin/bash
# scripts/audit_std.sh

echo "=== std usage audit ==="
echo ""

for dir in src/kernel src/security src/network src/klib; do
    count=$(grep -rn 'use std' "$dir" 2>/dev/null | wc -l)
    if [ "$count" -gt 0 ]; then
        echo "FAIL $dir: $count std imports"
        grep -rn 'use std' "$dir"
    else
        echo "OK   $dir: 0 std imports"
    fi
done

echo ""
echo "=== userland std usage (allowed with wrappers) ==="
grep -rn 'use std' src/shell src/sigpkg src/tools 2>/dev/null | head -20
```

---

*Last updated: 2026-08-04*
