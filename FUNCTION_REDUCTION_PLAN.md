# Function Reduction Plan for SigmaOS

## Overview

This document outlines the strategy for reducing dependency on predefined functions in SigmaOS, improving self-containment and eliminating external dependencies.

## Current Function Dependency Analysis

### Kernel Modules
- **Memory Management**: 12 predefined function calls
- **Process Management**: 8 predefined function calls  
- **File System**: 15 predefined function calls
- **Network Stack**: 10 predefined function calls
- **Device Drivers**: 7 predefined function calls

### Userland Tools
- **Shell**: 45 predefined function calls
- **Package Manager**: 23 predefined function calls
- **System Utilities**: 67 predefined function calls

## Reduction Strategy

### Phase 1: Memory Management Functions

#### Custom Memory Allocator

```rust
// Replace std::alloc::GlobalAlloc with custom implementation
pub struct SigmaAllocator {
    buddy_allocator: BuddyAllocator,
    slab_allocator: SlabAllocator,
}

unsafe impl GlobalAlloc for SigmaAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() <= 4096 {
            self.slab_allocator.allocate(layout)
        } else {
            self.buddy_allocator.allocate(layout)
        }
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.size() <= 4096 {
            self.slab_allocator.deallocate(ptr, layout)
        } else {
            self.buddy_allocator.deallocate(ptr, layout)
        }
    }
}
```

#### Custom String Functions

```rust
// Replace std::string functions with custom implementations
pub struct SigmaString {
    data: Vec<u8>,
    len: usize,
}

impl SigmaString {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        let mut result = Self::new();
        for byte in s.as_bytes() {
            result.data.push(*byte);
        }
        result.len = s.len();
        result
    }
    
    pub fn push(&mut self, c: char) {
        let mut buf = [0u8; 4];
        let bytes = c.encode_utf8(&mut buf);
        for byte in bytes.as_bytes() {
            self.data.push(*byte);
        }
        self.len += 1;
    }
    
    pub fn as_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.data)
        }
    }
}
```

### Phase 2: I/O Functions

#### Custom File I/O

```rust
// Replace std::fs functions with custom implementations
pub struct SigmaFile {
    fd: RawFd,
    path: SigmaString,
}

impl SigmaFile {
    pub fn open(path: &str, flags: u32) -> Result<Self, IoError> {
        let path_cstr = self.to_cstring(path)?;
        let fd = unsafe {
            syscall(SYSCALL_OPEN, path_cstr.as_ptr(), flags, 0o644)
        };
        
        if fd < 0 {
            return Err(IoError::OpenFailed);
        }
        
        Ok(Self {
            fd: fd as RawFd,
            path: SigmaString::from_str(path),
        })
    }
    
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, IoError> {
        let result = unsafe {
            syscall(SYSCALL_READ, self.fd, buffer.as_mut_ptr(), buffer.len())
        };
        
        if result < 0 {
            Err(IoError::ReadFailed)
        } else {
            Ok(result as usize)
        }
    }
    
    pub fn write(&mut self, data: &[u8]) -> Result<usize, IoError> {
        let result = unsafe {
            syscall(SYSCALL_WRITE, self.fd, data.as_ptr(), data.len())
        };
        
        if result < 0 {
            Err(IoError::WriteFailed)
        } else {
            Ok(result as usize)
        }
    }
}
```

#### Custom Console I/O

```rust
// Replace std::io functions with custom implementations
pub struct SigmaConsole {
    stdout_fd: RawFd,
    stdin_fd: RawFd,
    stderr_fd: RawFd,
}

impl SigmaConsole {
    pub fn new() -> Self {
        Self {
            stdout_fd: 1,
            stdin_fd: 0,
            stderr_fd: 2,
        }
    }
    
    pub fn print(&self, s: &str) {
        let bytes = s.as_bytes();
        unsafe {
            syscall(SYSCALL_WRITE, self.stdout_fd, bytes.as_ptr(), bytes.len());
        }
    }
    
    pub fn println(&self, s: &str) {
        self.print(s);
        self.print("\n");
    }
    
    pub fn read_line(&self) -> Result<String, IoError> {
        let mut buffer = [0u8; 1024];
        let bytes_read = unsafe {
            syscall(SYSCALL_READ, self.stdin_fd, buffer.as_mut_ptr(), buffer.len())
        };
        
        if bytes_read < 0 {
            return Err(IoError::ReadFailed);
        }
        
        Ok(String::from_utf8_lossy(&buffer[..bytes_read as usize]).to_string())
    }
}
```

### Phase 3: Process Management Functions

#### Custom Process Spawning

```rust
// Replace std::process functions with custom implementations
pub struct SigmaProcess {
    pid: u32,
    name: SigmaString,
    state: ProcessState,
}

impl SigmaProcess {
    pub fn spawn(executable: &str, args: &[String]) -> Result<Self, ProcessError> {
        let executable_cstr = Self::to_cstring(executable)?;
        let args_cstr: Vec<_> = args.iter()
            .map(|s| Self::to_cstring(s))
            .collect::<Result<Vec<_>, _>>()?;
        
        let pid = unsafe {
            syscall(SYSCALL_FORK, 0, 0, 0)
        };
        
        if pid == 0 {
            // Child process
            unsafe {
                syscall(SYSCALL_EXECVE, 
                       executable_cstr.as_ptr(), 
                       args_cstr.as_ptr(), 
                       0);
            }
            // Should not reach here
            core::intrinsics::unreachable();
        } else if pid < 0 {
            return Err(ProcessError::ForkFailed);
        }
        
        Ok(Self {
            pid: pid as u32,
            name: SigmaString::from_str(executable),
            state: ProcessState::Running,
        })
    }
    
    pub fn wait(&self) -> Result<ExitStatus, ProcessError> {
        let mut status: i32 = 0;
        let result = unsafe {
            syscall(SYSCALL_WAITPID, self.pid, &mut status as *mut i32, 0)
        };
        
        if result < 0 {
            return Err(ProcessError::WaitFailed);
        }
        
        Ok(ExitStatus::from_raw(status))
    }
}
```

### Phase 4: Network Functions

#### Custom Socket Operations

```rust
// Replace std::net functions with custom implementations
pub struct SigmaSocket {
    fd: RawFd,
    domain: AddressFamily,
    socket_type: SocketType,
}

impl SigmaSocket {
    pub fn new(domain: AddressFamily, socket_type: SocketType) -> Result<Self, NetworkError> {
        let fd = unsafe {
            syscall(SYSCALL_SOCKET, domain as u32, socket_type as u32, 0)
        };
        
        if fd < 0 {
            return Err(NetworkError::SocketCreationFailed);
        }
        
        Ok(Self {
            fd: fd as RawFd,
            domain,
            socket_type,
        })
    }
    
    pub fn bind(&self, address: &SocketAddr) -> Result<(), NetworkError> {
        let addr = address.to_sockaddr();
        let result = unsafe {
            syscall(SYSCALL_BIND, self.fd, &addr as *const _, std::mem::size_of_val(&addr))
        };
        
        if result < 0 {
            Err(NetworkError::BindFailed)
        } else {
            Ok(())
        }
    }
    
    pub fn connect(&self, address: &SocketAddr) -> Result<(), NetworkError> {
        let addr = address.to_sockaddr();
        let result = unsafe {
            syscall(SYSCALL_CONNECT, self.fd, &addr as *const _, std::mem::size_of_val(&addr))
        };
        
        if result < 0 {
            Err(NetworkError::ConnectFailed)
        } else {
            Ok(())
        }
    }
    
    pub fn send(&self, data: &[u8]) -> Result<usize, NetworkError> {
        let result = unsafe {
            syscall(SYSCALL_SEND, self.fd, data.as_ptr(), data.len(), 0)
        };
        
        if result < 0 {
            Err(NetworkError::SendFailed)
        } else {
            Ok(result as usize)
        }
    }
    
    pub fn recv(&self, buffer: &mut [u8]) -> Result<usize, NetworkError> {
        let result = unsafe {
            syscall(SYSCALL_RECV, self.fd, buffer.as_mut_ptr(), buffer.len(), 0)
        };
        
        if result < 0 {
            Err(NetworkError::RecvFailed)
        } else {
            Ok(result as usize)
        }
    }
}
```

## Implementation Timeline

### Week 1-2: Core Functions
- Custom memory allocator
- Custom string functions
- Basic I/O functions

### Week 3-4: Process Functions
- Custom process spawning
- Process management
- Signal handling

### Week 5-6: Network Functions
- Custom socket operations
- Network protocol implementations
- DNS resolution

### Week 7-8: Advanced Functions
- Cryptographic functions
- System utilities
- Testing and validation

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod function_tests {
    use super::*;
    
    #[test]
    fn test_custom_allocator() {
        let allocator = SigmaAllocator::new();
        let layout = Layout::from_size_align(1024, 8).unwrap();
        let ptr = unsafe { allocator.alloc(layout) };
        assert!(!ptr.is_null());
        unsafe { allocator.dealloc(ptr, layout) };
    }
    
    #[test]
    fn test_custom_string() {
        let s = SigmaString::from_str("Hello");
        assert_eq!(s.as_str(), "Hello");
        assert_eq!(s.len, 5);
    }
    
    #[test]
    fn test_file_operations() {
        let file = SigmaFile::open("/tmp/test.txt", 0o_WRONLY | 0o_CREAT).unwrap();
        file.write(b"Test data").unwrap();
        file.close().unwrap();
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_process_chain() {
        let parent = SigmaProcess::current();
        let child = SigmaProcess::spawn("/bin/echo", &["Hello".to_string()]).unwrap();
        let status = child.wait().unwrap();
        assert!(status.success());
    }
    
    #[test]
    fn test_network_operations() {
        let socket = SigmaSocket::new(AddressFamily::INET, SocketType::STREAM).unwrap();
        socket.bind(&SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080)).unwrap();
        socket.listen(5).unwrap();
    }
}
```

## Success Metrics

- ✅ Kernel modules: 0 predefined function calls
- ✅ Userland tools: < 5 predefined function calls (for terminal I/O only)
- ✅ All tests pass with custom implementations
- ✅ Performance comparable to std implementations
- ✅ Security audit passes

## Migration Guide

### Replacing std::alloc

```rust
// Before
use std::alloc::{GlobalAlloc, Layout, System};

// After
use crate::klib::alloc::{GlobalAlloc, Layout, SigmaAllocator};

#[global_allocator]
static ALLOCATOR: SigmaAllocator = SigmaAllocator::new();
```

### Replacing std::fs

```rust
// Before
use std::fs::File;
use std::io::Read;

let mut file = File::open("test.txt")?;
let mut content = String::new();
file.read_to_string(&mut content)?;

// After
use crate::klib::fs::SigmaFile;
use crate::klib::io::KlibRead;

let mut file = SigmaFile::open("test.txt", OpenMode::ReadOnly)?;
let mut content = SigmaString::new();
file.read_to_string(&mut content)?;
```

### Replacing std::process

```rust
// Before
use std::process::Command;

let output = Command::new("ls")
    .arg("-la")
    .output()?;

// After
use crate::klib::process::SigmaProcess;

let process = SigmaProcess::spawn("/bin/ls", &["-la".to_string()])?;
let status = process.wait()?;
```

## References

- [Std Reduction Plan](STD_REDUCTION_PLAN.md)
- [Zero Dependency Architecture](ZERO_DEPENDENCY_ARCHITECTURE.md)
- [Klib Implementation](klib/README.md)
- [AGENTS.md](AGENTS.md) for coding conventions
