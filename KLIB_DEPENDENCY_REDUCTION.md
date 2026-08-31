# Klib Dependency Reduction Documentation

## Overview

This document describes the custom library (klib) implementation for SigmaOS that reduces dependency on predefined functions and standard library (std) components.

## Klib Modules

### 1. Environment Access (`src/klib/env.rs`)

**Purpose:** Provides no_std alternatives to `std::env` for environment variable and command line argument access.

**Key Components:**
- `SigmaEnv` - Main environment variable access structure
- `EnvIterator` - Iterator for environment variables
- `ArgsIterator` - Iterator for command line arguments
- `EnvError` - Error types for environment operations

**Features:**
- Direct syscall integration for environment operations
- Custom C string handling without std
- Iterator support for environment enumeration
- Error handling with custom types

**Usage Example:**
```rust
use crate::klib::env::SigmaEnv;

// Get environment variable
let home = SigmaEnv::get("HOME").unwrap_or("/home/user");

// Set environment variable
SigmaEnv::set("MY_VAR", "value").unwrap();

// Get command line arguments
let args = SigmaEnv::args();
```

### 2. String Implementation (`src/klib/string.rs`)

**Purpose:** Provides no_std alternatives to `std::string` with reduced dependency on predefined functions.

**Key Components:**
- `SigmaString` - Custom string type
- `Pattern` trait - Pattern matching for string operations
- `Split` iterator - String splitting functionality

**Features:**
- Custom memory management using SigmaVec
- Pattern matching without std functions
- Reduced function dependencies
- Full string manipulation capabilities

**Usage Example:**
```rust
use crate::klib::string::SigmaString;

let mut s = SigmaString::from_str("hello");
s.push_str(" world");
s.push('!');
assert_eq!(s.as_str(), "hello world!");
```

### 3. File System (`src/klib/fs.rs`)

**Purpose:** Provides no_std alternatives to `std::fs` for file system operations.

**Key Components:**
- `SigmaFile` - Custom file operations
- `SigmaDir` - Directory operations
- `FsError` - File system error types
- `Metadata` - File metadata structure

**Features:**
- Direct syscall integration for file operations
- Custom error handling
- File metadata retrieval
- Directory enumeration

**Usage Example:**
```rust
use crate::klib::fs::{SigmaFile, OpenMode};

let mut file = SigmaFile::open("/tmp/test.txt", OpenMode::Create).unwrap();
file.write(b"Hello, SigmaOS!").unwrap();
file.flush().unwrap();
```

### 4. Vector Implementation (`src/klib/vec.rs`)

**Purpose:** Provides no_std alternatives to `std::vec` with reduced dependency on predefined functions.

**Key Components:**
- `SigmaVec<T>` - Custom vector type
- `IntoIter<T>` - Iterator for vector
- Memory management without std

**Features:**
- Custom memory allocation using core::alloc
- Manual memory management
- Iterator support
- Reduced function dependencies

**Usage Example:**
```rust
use crate::klib::vec::SigmaVec;

let mut vec = SigmaVec::new();
vec.push(1);
vec.push(2);
vec.push(3);
assert_eq!(vec.len(), 3);
```

## Dependency Reduction Impact

### Before Implementation
- Heavy reliance on `std::env`, `std::string`, `std::fs`, `std::vec`
- Limited no_std compatibility
- Less control over memory management
- Dependency on predefined functions

### After Implementation
- ✅ Core components use custom klib modules
- ✅ Zero std dependencies in kernel, klib, security, network
- ✅ Direct syscall integration for low-level operations
- ✅ Custom memory management with full control
- ✅ Reduced dependency on predefined functions

## Current Status

### klib Module Status
| Module | Status | std Usage | Test Coverage |
|--------|--------|-----------|---------------|
| env.rs | ✅ Complete | 0 | ✅ |
| string.rs | ✅ Complete | 0 | ✅ |
| fs.rs | ✅ Complete | 0 | ✅ |
| vec.rs | ✅ Complete | 0 | ✅ |

### Overall Project std Usage
| Component | std Usage | Target | Status |
|-----------|-----------|--------|--------|
| Kernel | 0 | 0 | ✅ Complete |
| klib | 0 | 0 | ✅ Complete |
| Security | 0 | 0 | ✅ Complete |
| Network | 0 | 0 | ✅ Complete |
| Userland | 47 | < 5 | ⚠️ In Progress |
| Compatibility | 308 | < 10 | ⚠️ High |

## Integration Guidelines

### When to Use klib
- **Kernel modules:** Always use klib (no_std)
- **Security modules:** Always use klib (no_std)
- **Network stack:** Always use klib (no_std)
- **Userland tools:** Prefer klib, minimal std allowed for I/O
- **Compatibility:** std allowed for cross-platform compatibility

### Migration Path
1. Replace `std::env` with `klib::env::SigmaEnv`
2. Replace `std::string::String` with `klib::string::SigmaString`
3. Replace `std::fs::File` with `klib::fs::SigmaFile`
4. Replace `std::vec::Vec` with `klib::vec::SigmaVec`

## Testing

### Unit Tests
Each klib module includes comprehensive unit tests:
```bash
# Test klib modules
rustc --test --edition=2021 src/klib/env.rs -o build/env_tests
rustc --test --edition=2021 src/klib/string.rs -o build/string_tests
rustc --test --edition=2021 src/klib/fs.rs -o build/fs_tests
rustc --test --edition=2021 src/klib/vec.rs -o build/vec_tests
```

### Integration Tests
Integration tests verify klib modules work together:
```bash
# Test klib integration
cargo test --lib klib
```

## Performance Considerations

### Memory Management
- Custom allocation strategy
- No std allocator overhead
- Direct memory control
- Efficient memory reuse

### Function Overhead
- Reduced function call overhead
- Direct syscall integration
- Minimal abstraction layers
- Optimized for performance

## Security Considerations

### Memory Safety
- Manual bounds checking
- Safe pointer operations
- Proper memory cleanup
- No undefined behavior

### Error Handling
- Custom error types
- No panic in production code
- Graceful error recovery
- Clear error messages

## Future Enhancements

### Planned Modules
- **hashmap.rs** - Custom hash map implementation
- **sync.rs** - Synchronization primitives
- **thread.rs** - Thread management
- **process.rs** - Process management

### Optimization Goals
- Further reduce function dependencies
- Improve memory efficiency
- Enhance performance
- Increase test coverage

## Troubleshooting

### Common Issues

**Compilation Errors:**
- Ensure klib modules are properly imported
- Check for conflicting std imports
- Verify no_std feature flags

**Memory Issues:**
- Check memory allocation in custom implementations
- Verify proper cleanup in Drop implementations
- Monitor memory usage patterns

**Integration Issues:**
- Ensure compatibility with existing code
- Test thoroughly before deployment
- Monitor performance impact

## References

- [SigmaOS Zero Dependency Architecture](ZERO_DEPENDENCY_ARCHITECTURE.md)
- [Std Reduction Plan](STD_REDUCTION_PLAN.md)
- [Std Elimination Implementation](STD_ELIMINATION_IMPLEMENTATION.md)
- [Security Code Scanning Status](SECURITY_CODE_SCANNING_STATUS.md)

## Contributing

When contributing to klib:
1. Maintain no_std compatibility
2. Follow existing code patterns
3. Include comprehensive tests
4. Document security considerations
5. Performance test changes

---

**Documentation Updated:** August 21, 2026  
**Version:** 1.0  
**Status:** ✅ Core Modules Complete
