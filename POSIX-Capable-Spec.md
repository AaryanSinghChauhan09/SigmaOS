# SigmaOS Minimal POSIX Capsule Specification

## Overview

The SigmaOS Minimal POSIX Capsule is a selective compatibility layer that provides essential POSIX primitives to enable porting of existing Unix/Linux software while preserving SigmaOS's AI-first, microkernel identity. This specification defines the minimal set of POSIX APIs that will be implemented.

## Design Principles

1. **Minimalism**: Only implement essential POSIX primitives needed for common applications
2. **OOP Integration**: Map POSIX APIs to SigmaOS's object-oriented kernel abstractions
3. **Modularity**: The compatibility layer can be swapped or bypassed for native SigmaOS APIs
4. **Safety**: All implementations use Rust's type system for memory safety
5. **Performance**: Avoid unnecessary overhead in the compatibility layer

## Scope: What's Included

### File I/O Primitives
- `open()` - Open/create files with standard flags
- `read()` - Read from file descriptors
- `write()` - Write to file descriptors
- `close()` - Close file descriptors
- `lseek()` - Seek within files
- `stat()` / `fstat()` - Get file status
- `mkdir()` - Create directories
- `rmdir()` - Remove directories
- `unlink()` - Remove files

### Process & Thread Model
- `spawn()` - Modern process spawning (alternative to fork/exec)
- `wait()` - Wait for child processes
- `exit()` - Process termination
- `getpid()` - Get process ID
- `getppid()` - Get parent process ID
- Thread creation and management primitives

### Signals
- `sigaction()` - Signal handling
- `kill()` - Send signals to processes
- `sigprocmask()` - Signal mask manipulation
- Supported signals: SIGINT, SIGTERM, SIGKILL, SIGCHLD, SIGSTOP, SIGCONT

### IPC (Inter-Process Communication)
- `pipe()` - Create pipe
- `socket()` - Create socket
- `bind()` - Bind socket to address
- `connect()` - Connect socket
- `listen()` - Listen for connections
- `accept()` - Accept connections
- `send()` / `recv()` - Send/receive data
- `shutdown()` - Shutdown socket

### Networking Sockets
- POSIX-like socket API for TCP/UDP
- Address family support: AF_INET, AF_INET6
- Socket types: SOCK_STREAM, SOCK_DGRAM
- Protocol support: IPPROTO_TCP, IPPROTO_UDP

### Minimal libc Subset
- String functions: `strlen`, `strcpy`, `strncpy`, `strcmp`, `strncmp`, `strchr`, `strstr`
- Memory functions: `malloc`, `free`, `memcpy`, `memset`, `memcmp`
- I/O functions: `printf`, `fprintf`, `sprintf`, `puts`, `putchar`
- Math functions: `atoi`, `atol`, `strtol`, `strtoul`
- Error handling: `errno`, `strerror`

## Scope: What's Excluded

### Legacy Shell Utilities
- No reimplementation of grep, awk, sed, etc.
- SigmaOS will provide AI-native equivalents

### Full POSIX Compliance
- No strict signal semantics
- No job control (bg, fg, jobs)
- No terminal control (termios)
- No obscure POSIX APIs

### Heavy Compatibility Layers
- No full POSIX threads (pthreads) - use SigmaOS native threading
- No full POSIX IPC (System V IPC) - use SigmaOS native IPC
- No full POSIX real-time extensions

## Architecture

### Layer Structure

```
┌─────────────────────────────────────┐
│   POSIX Application Code            │
├─────────────────────────────────────┤
│   Minimal libc (Sigma libc)         │
├─────────────────────────────────────┤
│   POSIX Compatibility Layer         │
│   (OOP abstractions)                 │
├─────────────────────────────────────┤
│   SigmaOS Microkernel               │
│   (Native OOP APIs)                  │
└─────────────────────────────────────┘
```

### File Descriptor Management

File descriptors are mapped to SigmaOS's object-oriented file handles:

```rust
pub struct PosixFileDescriptor {
    pub fd: i32,
    pub sigma_handle: SigmaFileHandle,
    pub flags: u32,
    pub mode: u32,
}
```

### Process Management

Processes are mapped to SigmaOS's process objects:

```rust
pub struct PosixProcess {
    pub pid: i32,
    pub sigma_process: SigmaProcess,
    pub parent_pid: i32,
    pub state: ProcessState,
}
```

### Socket Management

Sockets are mapped to SigmaOS's networking objects:

```rust
pub struct PosixSocket {
    pub fd: i32,
    pub sigma_socket: SigmaSocket,
    pub domain: AddressFamily,
    pub type_: SocketType,
    pub protocol: i32,
}
```

## Error Handling

All POSIX functions use the standard `errno` mechanism:

```rust
pub static mut ERRNO: i32 = 0;

pub const EPERM: i32 = 1;
pub const ENOENT: i32 = 2;
pub const ESRCH: i32 = 3;
pub const EINTR: i32 = 4;
pub const EIO: i32 = 5;
pub const ENXIO: i32 = 6;
pub const E2BIG: i32 = 7;
pub const ENOEXEC: i32 = 8;
pub const EBADF: i32 = 9;
pub const ECHILD: i32 = 10;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const EFAULT: i32 = 14;
pub const ENOTBLK: i32 = 15;
pub const EBUSY: i32 = 16;
pub const EEXIST: i32 = 17;
pub const EXDEV: i32 = 18;
pub const ENODEV: i32 = 19;
pub const ENOTDIR: i32 = 20;
pub const EISDIR: i32 = 21;
pub const EINVAL: i32 = 22;
pub const ENFILE: i32 = 23;
pub const EMFILE: i32 = 24;
pub const ENOTTY: i32 = 25;
pub const ETXTBSY: i32 = 26;
pub const EFBIG: i32 = 27;
pub const ENOSPC: i32 = 28;
pub const ESPIPE: i32 = 29;
pub const EROFS: i32 = 30;
pub const EMLINK: i32 = 31;
pub const EPIPE: i32 = 32;
pub const EDOM: i32 = 33;
pub const ERANGE: i32 = 34;
pub const EWOULDBLOCK: i32 = EAGAIN;
pub const EINPROGRESS: i32 = 115;
pub const EALREADY: i32 = 114;
pub const ENOTSOCK: i32 = 88;
pub const EDESTADDRREQ: i32 = 89;
pub const EMSGSIZE: i32 = 90;
pub const EPROTOTYPE: i32 = 91;
pub const ENOPROTOOPT: i32 = 92;
pub const EPROTONOSUPPORT: i32 = 93;
pub const ESOCKTNOSUPPORT: i32 = 94;
pub const EOPNOTSUPP: i32 = 95;
pub const EPFNOSUPPORT: i32 = 96;
pub const EAFNOSUPPORT: i32 = 97;
pub const EADDRINUSE: i32 = 98;
pub const EADDRNOTAVAIL: i32 = 99;
pub const ENETDOWN: i32 = 100;
pub const ENETUNREACH: i32 = 101;
pub const ENETRESET: i32 = 102;
pub const ECONNABORTED: i32 = 103;
pub const ECONNRESET: i32 = 104;
pub const ENOBUFS: i32 = 105;
pub const EISCONN: i32 = 106;
pub const ENOTCONN: i32 = 107;
pub const ESHUTDOWN: i32 = 108;
pub const ETOOMANYREFS: i32 = 109;
pub const ETIMEDOUT: i32 = 110;
pub const ECONNREFUSED: i32 = 111;
pub const EHOSTDOWN: i32 = 112;
pub const EHOSTUNREACH: i32 = 113;
```

## File Flags

```rust
pub const O_RDONLY: i32 = 0o0000;
pub const O_WRONLY: i32 = 0o0001;
pub const O_RDWR: i32 = 0o0002;
pub const O_CREAT: i32 = 0o0100;
pub const O_EXCL: i32 = 0o0200;
pub const O_NOCTTY: i32 = 0o0400;
pub const O_TRUNC: i32 = 0o1000;
pub const O_APPEND: i32 = 0o2000;
pub const O_NONBLOCK: i32 = 0o4000;
pub const O_SYNC: i32 = 0o10000;
```

## Signal Definitions

```rust
pub const SIGHUP: i32 = 1;
pub const SIGINT: i32 = 2;
pub const SIGQUIT: i32 = 3;
pub const SIGILL: i32 = 4;
pub const SIGTRAP: i32 = 5;
pub const SIGABRT: i32 = 6;
pub const SIGBUS: i32 = 7;
pub const SIGFPE: i32 = 8;
pub const SIGKILL: i32 = 9;
pub const SIGUSR1: i32 = 10;
pub const SIGSEGV: i32 = 11;
pub const SIGUSR2: i32 = 12;
pub const SIGPIPE: i32 = 13;
pub const SIGALRM: i32 = 14;
pub const SIGTERM: i32 = 15;
pub const SIGCHLD: i32 = 17;
pub const SIGCONT: i32 = 18;
pub const SIGSTOP: i32 = 19;
pub const SIGTSTP: i32 = 20;
pub const SIGTTIN: i32 = 21;
pub const SIGTTOU: i32 = 22;
```

## Socket Definitions

```rust
pub const AF_INET: i32 = 2;
pub const AF_INET6: i32 = 10;
pub const AF_UNIX: i32 = 1;
pub const AF_UNSPEC: i32 = 0;

pub const SOCK_STREAM: i32 = 1;
pub const SOCK_DGRAM: i32 = 2;
pub const SOCK_RAW: i32 = 3;
pub const SOCK_SEQPACKET: i32 = 5;

pub const IPPROTO_TCP: i32 = 6;
pub const IPPROTO_UDP: i32 = 17;
pub const IPPROTO_IP: i32 = 0;
pub const IPPROTO_IPV6: i32 = 41;

pub const SOL_SOCKET: i32 = 1;
pub const SO_REUSEADDR: i32 = 2;
pub const SO_KEEPALIVE: i32 = 9;
pub const SO_ERROR: i32 = 4;
```

## Implementation Phases

### Phase 1: Core Infrastructure
- Error handling (errno)
- File descriptor management
- Process management structures

### Phase 2: File I/O
- open, read, write, close
- lseek, stat, fstat
- mkdir, rmdir, unlink

### Phase 3: Process Management
- spawn, wait, exit
- getpid, getppid
- Basic signal handling

### Phase 4: IPC & Networking
- pipe, socket
- bind, connect, listen, accept
- send, recv, shutdown

### Phase 5: Minimal libc
- String functions
- Memory functions
- I/O functions
- Math functions

### Phase 6: Testing
- Port simple utilities (BusyBox)
- Validate compatibility
- Performance testing

## Compatibility Notes

### Differences from Full POSIX

1. **No fork()**: Use `spawn()` instead for process creation
2. **Simplified signals**: Only essential signals are supported
3. **No job control**: No background/foreground job management
4. **No terminal control**: No termios, no terminal I/O control
5. **Limited IPC**: No System V IPC, only pipes and sockets

### Migration Guide for Developers

1. Replace `fork()` + `exec()` with `spawn()`
2. Use SigmaOS native APIs for advanced features
3. Avoid relying on obscure POSIX features
4. Test thoroughly with the compatibility layer

## Future Extensions

The following may be added based on demand:

- Additional signal types
- More socket options
- Extended file operations (mmap, etc.)
- Additional libc functions
- POSIX threads subset (if needed)

## References

- POSIX.1-2017 Specification
- Linux System Programming
- Rust Standard Library
