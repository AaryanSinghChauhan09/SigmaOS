# SigmaOS Standard Library Elimination Implementation

## Overview

This document provides a detailed implementation plan for eliminating dependency on Rust's standard library (std) in SigmaOS, achieving true `#![no_std]` operation throughout the codebase.

## Table of Contents

1. [Current Status Assessment](#current-status-assessment)
2. [Priority Elimination Targets](#priority-elimination-targets)
3. [Klib Enhancement Plan](#klib-enhancement-plan)
4. [Implementation Phases](#implementation-phases)
5. [Testing Strategy](#testing-strategy)
6. [Verification Procedures](#verification-procedures)

## Current Status Assessment

### Current std Usage Analysis

Based on `STD_REDUCTION_PLAN.md`, the current status is:

| Component | std Usage | Target | Status |
|-----------|-----------|--------|--------|
| Kernel (`sigma_kernel`) | 0 std calls ✅ | 0 | ✅ Complete |
| klib | 0 std calls ✅ | 0 | ✅ Complete |
| Security modules | 0 std calls ✅ | 0 | ✅ Complete |
| Network stack | 0 std calls ✅ | 0 | ✅ Complete |
| Package manager (sigpkg) | 3 std calls ⚠️ | 0 | ⚠️ 95% done |
| Shell (sigma_sh) | 12 std calls ⚠️ | 0 | ⚠️ Partial |
| Userland tools | 47 std calls ⚠️ | < 5 (allow for I/O) | ⚠️ In Progress |

### Remaining std Dependencies

**Package Manager (sigpkg):**
- `std::env` - Environment variable access
- `std::fs::File` - File system access
- `std::io::Read` - I/O trait

**Shell (sigma_sh):**
- `std::io::stdin, stdout, BufReader` - Terminal I/O
- `std::process::Command` - Process spawning
- `std::io::*` - General I/O operations

**Userland Tools:**
- Various std collections and I/O operations

## Priority Elimination Targets

### Phase 1: Package Manager Completion

**Target File:** `src/sigpkg/universal_engine.rs`

```rust
// Current (with std)
use std::env;

let home = std::env::var("HOME").unwrap_or_default();

// Replacement (klib)
use crate::klib::env::SigmaEnv;

let home = SigmaEnv::get("HOME").unwrap_or("");
```

**Implementation:**

```rust
// src/klib/env.rs - New module
pub struct SigmaEnv;

impl SigmaEnv {
    pub fn get(key: &str) -> Option<&'static str> {
        // Read from process environment block
        let envp = self.get_envp_pointer();
        if envp.is_null() {
            return None;
        }
        
        unsafe {
            self.search_env_block(envp, key)
        }
    }
    
    pub fn set(key: &str, value: &str) -> Result<(), EnvError> {
        // Set environment variable via syscall
        let syscall_num = SYSCALL_SETENV;
        let result = unsafe {
            syscall(syscall_num, key.as_ptr(), value.as_ptr())
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(EnvError::SetFailed)
        }
    }
    
    pub fn remove(key: &str) -> Result<(), EnvError> {
        // Remove environment variable via syscall
        let syscall_num = SYSCALL_UNSETENV;
        let result = unsafe {
            syscall(syscall_num, key.as_ptr())
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(EnvError::RemoveFailed)
        }
    }
    
    pub fn args() -> impl Iterator<Item = &'static str> {
        // Get command line arguments
        let argv = self.get_argv_pointer();
        EnvIterator::new(argv)
    }
    
    unsafe fn get_envp_pointer() -> *const *const u8 {
        // Get envp from process ABI
        // Implementation depends on platform-specific ABI
        extern "C" {
            static environ: *const *const u8;
        }
        environ
    }
    
    unsafe fn search_env_block(envp: *const *const u8, key: &str) -> Option<&'static str> {
        let mut i = 0;
        loop {
            let entry = *envp.add(i);
            if entry.is_null() {
                return None;
            }
            
            let entry_str = core::str::from_utf8_unchecked(
                self.get_c_string(entry)
            );
            
            if let Some(value) = self.parse_env_entry(entry_str, key) {
                return value;
            }
            
            i += 1;
        }
    }
    
    unsafe fn get_c_string(ptr: *const u8) -> &'static str {
        let mut len = 0;
        loop {
            if *ptr.add(len) == 0 {
                return core::str::from_utf8_unchecked(
                    core::slice::from_raw_parts(ptr, len)
                );
            }
            len += 1;
        }
    }
    
    fn parse_env_entry(entry: &str, key: &str) -> Option<&'static str> {
        let parts: Vec<&str> = entry.splitn(2, '=').collect();
        if parts.len() == 2 && parts[0] == key {
            Some(parts[1])
        } else {
            None
        }
    }
    
    unsafe fn get_argv_pointer() -> *const *const u8 {
        extern "C" {
            static argv: *const *const u8;
        }
        argv
    }
}

pub struct EnvIterator {
    current: *const *const u8,
    index: usize,
}

impl EnvIterator {
    fn new(argv: *const *const u8) -> Self {
        Self {
            current: argv,
            index: 0,
        }
    }
}

impl Iterator for EnvIterator {
    type Item = &'static str;
    
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let entry = *self.current.add(self.index);
            if entry.is_null() {
                return None;
            }
            
            self.index += 1;
            Some(Self::get_c_string(entry))
        }
    }
}

pub enum EnvError {
    SetFailed,
    RemoveFailed,
    InvalidKey,
}
```

### Phase 2: File System Access

**Target File:** `src/sigpkg/importer.rs`

```rust
// Current (with std)
use std::fs::File;
use std::io::Read;

let f = std::fs::File::open("/etc/sigpkg/config")?;
f.read_to_string(&mut s)?;

// Replacement (klib)
use crate::klib::fs::SigmaFile;
use crate::klib::io::KlibRead;

let f = SigmaFile::open("/etc/sigpkg/config", OpenMode::ReadOnly)?;
f.read_to_string(&mut s)?;
```

**Implementation:**

```rust
// src/klib/fs.rs - New module
pub struct SigmaFile {
    fd: RawFd,
    path: SigmaString,
}

pub enum OpenMode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    Append,
    Create,
}

#[derive(Debug)]
pub enum FsError {
    NotFound,
    PermissionDenied,
    AlreadyExists,
    InvalidPath,
    IoError,
}

impl SigmaFile {
    pub fn open(path: &str, mode: OpenMode) -> Result<Self, FsError> {
        let path_cstr = self.path_to_cstring(path)?;
        let flags = self.mode_to_flags(mode);
        
        let fd = unsafe {
            syscall(SYSCALL_OPEN, path_cstr.as_ptr(), flags, 0o644)
        };
        
        if fd < 0 {
            return Err(FsError::IoError);
        }
        
        Ok(Self {
            fd: fd as RawFd,
            path: SigmaString::from(path),
        })
    }
    
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, FsError> {
        let result = unsafe {
            syscall(SYSCALL_READ, self.fd, buffer.as_mut_ptr(), buffer.len())
        };
        
        if result < 0 {
            Err(FsError::IoError)
        } else {
            Ok(result as usize)
        }
    }
    
    pub fn write(&mut self, data: &[u8]) -> Result<usize, FsError> {
        let result = unsafe {
            syscall(SYSCALL_WRITE, self.fd, data.as_ptr(), data.len())
        };
        
        if result < 0 {
            Err(FsError::IoError)
        } else {
            Ok(result as usize)
        }
    }
    
    pub fn seek(&mut self, offset: i64, whence: SeekFrom) -> Result<u64, FsError> {
        let whence_flag = match whence {
            SeekFrom::Start => 0,
            SeekFrom::Current => 1,
            SeekFrom::End => 2,
        };
        
        let result = unsafe {
            syscall(SYSCALL_LSEEK, self.fd, offset, whence_flag)
        };
        
        if result < 0 {
            Err(FsError::IoError)
        } else {
            Ok(result as u64)
        }
    }
    
    pub fn close(self) -> Result<(), FsError> {
        let result = unsafe {
            syscall(SYSCALL_CLOSE, self.fd)
        };
        
        if result < 0 {
            Err(FsError::IoError)
        } else {
            Ok(())
        }
    }
    
    fn mode_to_flags(mode: OpenMode) -> u32 {
        match mode {
            OpenMode::ReadOnly => 0o_RDONLY,
            OpenMode::WriteOnly => 0o_WRONLY,
            OpenMode::ReadWrite => 0o_RDWR,
            OpenMode::Append => 0o_WRONLY | 0o_APPEND,
            OpenMode::Create => 0o_CREAT | 0o_WRONLY | 0o_TRUNC,
        }
    }
    
    fn path_to_cstring(path: &str) -> Result<[u8; 256], FsError> {
        let mut cstr = [0u8; 256];
        let bytes = path.as_bytes();
        
        if bytes.len() >= 256 {
            return Err(FsError::InvalidPath);
        }
        
        for (i, &byte) in bytes.iter().enumerate() {
            cstr[i] = byte;
        }
        
        Ok(cstr)
    }
}

pub enum SeekFrom {
    Start,
    Current,
    End,
}

impl Drop for SigmaFile {
    fn drop(&mut self) {
        let _ = unsafe {
            syscall(SYSCALL_CLOSE, self.fd)
        };
    }
}
```

### Phase 3: I/O Trait Implementation

**Target File:** `src/sigpkg/importer.rs`

```rust
// Current (with std)
use std::io::Read;

f.read_to_string(&mut s)?;

// Replacement (klib)
use crate::klib::io::KlibRead;

f.read_to_string(&mut s)?;
```

**Implementation:**

```rust
// src/klib/io.rs - Enhanced module
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

impl KlibRead for SigmaFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        self.read(buf).map_err(|_| IoError::ReadFailed)
    }
    
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), IoError> {
        let mut total_read = 0;
        while total_read < buf.len() {
            let bytes_read = self.read(&mut buf[total_read..])?;
            if bytes_read == 0 {
                return Err(IoError::UnexpectedEof);
            }
            total_read += bytes_read;
        }
        Ok(())
    }
    
    fn read_to_string(&mut self, s: &mut SigmaString) -> Result<usize, IoError> {
        let mut buffer = [0u8; 4096];
        let mut total_read = 0;
        
        loop {
            let bytes_read = self.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            
            for byte in &buffer[..bytes_read] {
                s.data.push(*byte);
            }
            s.len += bytes_read;
            total_read += bytes_read;
        }
        
        Ok(total_read)
    }
}

impl KlibWrite for SigmaFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        self.write(buf).map_err(|_| IoError::WriteFailed)
    }
    
    fn write_all(&mut self, buf: &[u8]) -> Result<(), IoError> {
        let mut total_written = 0;
        while total_written < buf.len() {
            let bytes_written = self.write(&buf[total_written..])?;
            if bytes_written == 0 {
                return Err(IoError::WriteFailed);
            }
            total_written += bytes_written;
        }
        Ok(())
    }
    
    fn flush(&mut self) -> Result<(), IoError> {
        // Sync file to disk
        let result = unsafe {
            syscall(SYSCALL_FSYNC, self.fd)
        };
        
        if result < 0 {
            Err(IoError::FlushFailed)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub enum IoError {
    ReadFailed,
    WriteFailed,
    UnexpectedEof,
    FlushFailed,
    InvalidInput,
}
```

## Klib Enhancement Plan

### New Klib Modules

1. **`src/klib/env.rs`** - Environment variable access
2. **`src/klib/fs.rs`** - File system operations
3. **`src/klib/io.rs`** - Enhanced I/O traits
4. **`src/klib/process.rs`** - Process management
5. **`src/klib/thread.rs`** - Thread management
6. **`src/klib/sync.rs`** - Synchronization primitives

### Enhanced Existing Modules

1. **`src/klib/vec.rs`** - Additional vector methods
2. **`src/klib/hashmap.rs`** - Performance optimizations
3. **`src/klib/string.rs`** - Enhanced string operations

## Implementation Phases

### Phase 1: Core Infrastructure (Week 1-2)

- Implement `src/klib/env.rs`
- Implement `src/klib/fs.rs`
- Implement enhanced `src/klib/io.rs`
- Update klib mod.rs with new modules

### Phase 2: Shell Migration (Week 3-4)

- Implement `src/klib/process.rs`
- Implement terminal I/O in `src/klib/io.rs`
- Update sigma_sh to use klib modules
- Test shell functionality

### Phase 3: Userland Migration (Week 5-6)

- Implement `src/klib/thread.rs`
- Implement `src/klib/sync.rs`
- Update userland tools to use klib
- Test userland functionality

### Phase 4: Validation (Week 7-8)

- Comprehensive testing of all modules
- Performance benchmarking
- Security audit
- Documentation updates

## Testing Strategy

### Module-Level Testing

```rust
#[cfg(test)]
mod klib_tests {
    use super::*;
    
    #[test]
    fn test_env_operations() {
        let key = "TEST_VAR";
        let value = "test_value";
        
        // Test set
        SigmaEnv::set(key, value).unwrap();
        
        // Test get
        let retrieved = SigmaEnv::get(key);
        assert_eq!(retrieved, Some(value));
        
        // Test remove
        SigmaEnv::remove(key).unwrap();
        let removed = SigmaEnv::get(key);
        assert_eq!(removed, None);
    }
    
    #[test]
    fn test_file_operations() {
        let path = "/tmp/test_file.txt";
        let content = b"Hello, SigmaOS!";
        
        // Test write
        let mut file = SigmaFile::open(path, OpenMode::Create).unwrap();
        file.write_all(content).unwrap();
        file.close().unwrap();
        
        // Test read
        let mut file = SigmaFile::open(path, OpenMode::ReadOnly).unwrap();
        let mut buffer = [0u8; 1024];
        let bytes_read = file.read(&mut buffer).unwrap();
        file.close().unwrap();
        
        assert_eq!(&buffer[..bytes_read], content);
    }
    
    #[test]
    fn test_io_traits() {
        let path = "/tmp/test_io.txt";
        let content = b"Test I/O";
        
        // Test write trait
        let mut file = SigmaFile::open(path, OpenMode::Create).unwrap();
        file.write_all(content).unwrap();
        file.close().unwrap();
        
        // Test read trait
        let mut file = SigmaFile::open(path, OpenMode::ReadOnly).unwrap();
        let mut s = SigmaString::new();
        file.read_to_string(&mut s).unwrap();
        file.close().unwrap();
        
        assert_eq!(s.as_str(), "Test I/O");
    }
}
```

### Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_package_manager_no_std() {
        // Test package manager with klib only
        let config = SigpkgConfig::new();
        config.load_from_file("sigma-pkg.toml").unwrap();
        
        // Verify no std dependency
        let package = config.get_package("sigmaos-core").unwrap();
        assert!(package.dependencies.is_empty());
    }
    
    #[test]
    fn test_shell_no_std() {
        // Test shell with klib only
        let mut shell = SigmaShell::new();
        shell.execute_command("echo", &["Hello"]).unwrap();
        
        // Verify output
        let output = shell.get_output();
        assert!(output.contains("Hello"));
    }
}
```

## Verification Procedures

### std Usage Audit Script

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

echo ""
echo "=== userland std usage (target: < 5 for I/O) ==="
grep -rn 'use std' src/shell src/sigpkg src/tools 2>/dev/null | head -20
```

### Compilation Verification

```bash
# Test no_std compilation
cargo check --target x86_64-unknown-none --no-default-features

# Test with std only in userland
cargo check --features "userland_std_compat"

# Run tests
cargo test --target x86_64-unknown-none
```

## Success Criteria

- ✅ All kernel modules have 0 std imports
- ✅ klib has 0 std imports
- ✅ sigpkg has 0 std imports
- ✅ Shell has < 5 std imports (for terminal I/O only)
- ✅ Userland tools have < 5 std imports (for I/O only)
- ✅ All tests pass with no_std target
- ✅ Performance benchmarks acceptable
- ✅ Security audit passes

## Resources

- [Std Reduction Plan](STD_REDUCTION_PLAN.md)
- [Zero Dependency Architecture](ZERO_DEPENDENCY_ARCHITECTURE.md)
- [Function Reduction Plan](FUNCTION_REDUCTION_PLAN.md)
- [Kernel Customization Guide](KERNEL_CUSTOMIZATION_GUIDE.md)

## Contributing

When implementing std elimination:

1. Maintain backward compatibility where possible
2. Provide clear migration documentation
3. Include comprehensive testing
4. Monitor performance impact
5. Update related documentation

## License

Copyright © 2026 SigmaOS Project. Licensed under MIT License.