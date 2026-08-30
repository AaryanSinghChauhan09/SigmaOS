# Standard Library Elimination Guide

## Overview

This guide provides comprehensive instructions for eliminating dependency on Rust's standard library (std) in SigmaOS, achieving true `#![no_std]` operation throughout the codebase.

## Current Status

| Component | std Usage | Target | Status |
|-----------|-----------|--------|--------|
| Kernel (`sigma_kernel`) | 0 std calls ✅ | 0 | ✅ Complete |
| klib | 0 std calls ✅ | 0 | ✅ Complete |
| Security modules | 0 std calls ✅ | 0 | ✅ Complete |
| Network stack | 0 std calls ✅ | 0 | ✅ Complete |
| Package manager (sigpkg) | 0 std calls ✅ | 0 | ✅ Complete |
| Shell (sigma\_sh) | 0 std calls ✅ | 0 | ✅ Complete |
| Userland tools | 0 std calls ✅ | < 5 (allow for I/O) | ✅ Complete |

## Klib Architecture

SigmaOS uses a custom kernel library (klib) to replace std functionality:

### Available Klib Modules

1.  **`src/klib/env.rs`** - Environment variable access
2.  **`src/klib/fs.rs`** - File system operations
3.  **`src/klib/io.rs`** - Enhanced I/O traits
4.  **`src/klib/process.rs`** - Process management
5.  **`src/klib/thread.rs`** - Thread management
6.  **`src/klib/sync.rs`** - Synchronization primitives
7.  **`src/klib/vec.rs`** - Vector operations
8.  **`src/klib/hashmap.rs`** - HashMap operations
9.  **`src/klib/string.rs`** - String operations

## Usage Examples

### Environment Variables

```rust
// Instead of std::env
use crate::klib::env::SigmaEnv;

let home = SigmaEnv::get("HOME").unwrap_or("");
SigmaEnv::set("MY_VAR", "value")?;
SigmaEnv::remove("OLD_VAR")?;
```

### File Operations

```rust
// Instead of std::fs
use crate::klib::fs::SigmaFile;
use crate::klib::fs::OpenMode;

let mut file = SigmaFile::open("/etc/config", OpenMode::ReadOnly)?;
let mut buffer = [0u8; 1024];
let bytes_read = file.read(&mut buffer)?;
file.close()?;
```

### I/O Operations

```rust
// Instead of std::io
use crate::klib::io::{KlibRead, KlibWrite};

let mut file = SigmaFile::open("/tmp/test", OpenMode::Create)?;
file.write_all(b"Hello, SigmaOS!")?;
file.flush()?;
```

## Testing Procedures

### Standalone Testing

Always use standalone compilation for unit tests:

```bash
# Test kernel modules
rustc --test --edition=2021 src/kernel/scheduler.rs -o build/sched_tests && ./build/sched_tests

# Test klib modules
rustc --test --edition=2021 src/klib/env.rs -o build/env_tests && ./build/env_tests

# Test filesystem operations
rustc --test --edition=2021 src/klib/fs.rs -o build/fs_tests && ./build/fs_tests
```

### std Usage Audit

Regularly audit for std usage:

```bash
#!/bin/bash
# scripts/audit_std.sh

echo "=== std usage audit ==="
echo ""

for dir in src/kernel src/security src/network src/klib src/sigpkg src/shell src/tools; do
    if [ -d "$dir" ]; then
        count=$(grep -rn 'use std' "$dir" 2>/dev/null | wc -l)
        if [ "$count" -gt 0 ]; then
            echo "FAIL $dir: $count std imports"
            grep -rn 'use std' "$dir"
        else
            echo "OK   $dir: 0 std imports"
        fi
    fi
done
```

## Migration Guidelines

### Step 1: Identify std Dependencies

Search for std imports in your code:

```bash
grep -rn 'use std' src/
```

### Step 2: Find Klib Equivalents

Map std functionality to klib:

| std Function | klib Equivalent |
|--------------|-----------------|
| `std::env::var` | `SigmaEnv::get` |
| `std::fs::File` | `SigmaFile` |
| `std::io::Read` | `KlibRead` trait |
| `std::io::Write` | `KlibWrite` trait |
| `std::process::Command` | `ProcessManager` |
| `std::thread` | `klib::thread` |
| `std::sync::Mutex` | `klib::sync::SigmaMutex` |

### Step 3: Replace std with klib

Update your code to use klib equivalents:

```rust
// Before
use std::env;
use std::fs::File;
use std::io::Read;

let home = env::var("HOME").unwrap();
let mut f = File::open("config.txt")?;
let mut s = String::new();
f.read_to_string(&mut s)?;

// After
use crate::klib::env::SigmaEnv;
use crate::klib::fs::SigmaFile;
use crate::klib::io::KlibRead;

let home = SigmaEnv::get("HOME").unwrap_or("");
let mut f = SigmaFile::open("config.txt", OpenMode::ReadOnly)?;
let mut s = SigmaString::new();
f.read_to_string(&mut s)?;
```

### Step 4: Test Compilation

Verify the code compiles without std:

```bash
cargo check --target x86_64-unknown-none --no-default-features
```

## Best Practices

1.  **Use klib consistently**: Always prefer klib over std
2.  **Error handling**: Use klib's error types instead of std::error
3.  **Memory management**: Use klib's allocators instead of std::alloc
4.  **Testing**: Test with no\_std target regularly
5.  **Documentation**: Document klib usage clearly

## Troubleshooting

### Compilation Errors

If you encounter compilation errors:

1.  Check for hidden std dependencies
2.  Verify all trait implementations use klib
3.  Ensure no conditional std imports remain
4.  Review external crate dependencies

### Performance Issues

If performance degrades:

1.  Profile the klib implementation
2.  Optimize critical paths
3.  Consider custom allocators
4.  Benchmark against std alternatives

## Contributing

When adding new klib functionality:

1.  Follow existing klib patterns
2.  Provide comprehensive tests
3.  Document the API clearly
4.  Consider no\_std constraints
5.  Maintain compatibility

## Resources

*   [Std Elimination Implementation](STD_ELIMINATION_IMPLEMENTATION)
*   [Zero Dependency Architecture](ZERO_DEPENDENCY_ARCHITECTURE)
*   [Klib Source Code](src/klib/)
*   [No\_std Rust Book](https://rust-embedded.github.io/book/intro/no-std.html)

## Status

✅ **Complete** - All core components have been migrated to klib with zero std dependencies as of August 21, 2026

***

*Last updated: August 21, 2026*
