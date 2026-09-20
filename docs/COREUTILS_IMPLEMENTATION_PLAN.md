# SigmaOS Coreutils Implementation Plan

**Version:** 1.0  
**Status:** Draft  
**Last Updated:** 2025-01-22  
**Purpose:** Define the essential coreutils subset for SigmaOS

---

## 1. Overview

SigmaOS requires a native coreutils suite to provide essential command-line functionality. This plan prioritizes POSIX-compliant implementations of the most critical commands, following SigmaOS's zero-dependency and hybrid `std`/`no_std` architecture.

---

## 2. Priority Phases

### Phase 1: File Operations (Highest Priority)
- `cat` - Concatenate and print files
- `cp` - Copy files
- `mv` - Move/rename files
- `rm` - Remove files
- `mkdir` - Create directories
- `ls` - List directory contents
- `pwd` - Print working directory

### Phase 2: Text Processing
- `echo` - Display a line of text
- `printf` - Format and print data
- `head` - Output the first part of files
- `tail` - Output the last part of files
- `grep` - Print lines matching a pattern
- `sort` - Sort lines of text
- `cut` - Remove sections from each line
- `tr` - Translate or delete characters

### Phase 3: Process & System
- `env` - Run a program in a modified environment
- `id` - Print user and group IDs
- `uname` - Print system information
- `true` - Return true
- `false` - Return false

### Phase 4: Advanced Operations
- `find` - Search for files in a directory hierarchy
- `xargs` - Build and execute command lines
- `test` - Check file types and compare values

---

## 3. Implementation Guidelines

### 3.1 Architecture

All coreutils should:

1. **Use `std` for user-space tools** - These are user-space utilities that can safely use `std`
2. **Follow Rust best practices** - Proper error handling, no unwraps, clear error messages
3. **POSIX compatibility** - Match POSIX.1-2017 where feasible
4. **GNU/BusyBox compatibility** - Support common GNU and BusyBox extensions where useful
5. **Zero external dependencies** - Use only `std` library

### 3.2 Command Structure

Each coreutil should follow this structure:

```rust
use std::env;
use std::fs;
use std::io::{self, Write, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match run(&args) {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("{}: error: {}", args[0], e);
            std::process::exit(1);
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    // Implementation
    Ok(())
}
```

### 3.3 Error Handling

- Use `Result<T, String>` for operation errors
- Provide clear, user-friendly error messages
- Exit with status 1 on error, 0 on success
- Handle interrupted operations gracefully

### 3.4 Testing

Each coreutil should have:
- Unit tests for core logic
- Integration tests for command-line behavior
- Error case tests
- Edge case tests (empty files, large files, special characters)

---

## 4. Implementation Status

| Command | Status | Implementation | Tests |
|---------|--------|----------------|-------|
| cat | Not started | None | None |
| cp | Not started | None | None |
| mv | Not started | None | None |
| rm | Not started | None | None |
| mkdir | Not started | None | None |
| ls | Not started | None | None |
| pwd | Not started | None | None |
| echo | Not started | None | None |
| printf | Not started | None | None |
| head | Not started | None | None |
| tail | Not started | None | None |
| grep | Not started | None | None |
| sort | Not started | None | None |
| cut | Not started | None | None |
| tr | Not started | None | None |
| find | Not started | None | None |
| xargs | Not started | None | None |
| env | Not started | None | None |
| id | Not started | None | None |
| uname | Not started | None | None |
| true | Not started | None | None |
| false | Not started | None | None |
| test | Not started | None | None |

---

## 5. Next Steps

1. Implement Phase 1 commands (file operations)
2. Add comprehensive tests
3. Integrate with SigmaOS build system
4. Add man pages
5. Benchmark against GNU coreutils

---

## 6. References

- [POSIX.1-2017 Specification](https://pubs.opengroup.org/onlinepubs/9699919799/)
- [GNU Coreutils Manual](https://www.gnu.org/software/coreutils/manual/)
- [BusyBox Coreutils](https://busybox.net/)
- [PROJECT_STATUS.md](PROJECT_STATUS.md)
- [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md)
