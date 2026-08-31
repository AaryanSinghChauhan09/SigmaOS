# SigmaOS POSIX Compatibility Layer

## Overview

The SigmaOS POSIX Compatibility Layer is a selective compatibility layer that provides essential POSIX primitives to enable porting of existing Unix/Linux software while preserving SigmaOS's AI-first, microkernel identity. This layer implements a minimal set of POSIX APIs mapped to SigmaOS's object-oriented kernel abstractions.

## Architecture

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

## Components

### 1. Base Layer (`posix_base.rs`)
- Error handling (errno)
- File descriptor management
- Process management structures
- Socket management structures
- Signal definitions
- Socket definitions

### 2. File I/O (`posix_file.rs`)
- `open()` - Open/create files with standard flags
- `read()` - Read from file descriptors
- `write()` - Write to file descriptors
- `close()` - Close file descriptors
- `lseek()` - Seek within files
- `stat()` / `fstat()` - Get file status
- `mkdir()` - Create directories
- `rmdir()` - Remove directories
- `unlink()` - Remove files
- `access()` - Check file accessibility

### 3. Process Management (`posix_process.rs`)
- `spawn()` - Modern process spawning (alternative to fork/exec)
- `wait()` / `waitpid()` - Wait for child processes
- `exit()` - Process termination
- `getpid()` - Get process ID
- `getppid()` - Get parent process ID
- `kill()` - Send signals to processes
- `raise()` - Send signal to current process
- `abort()` - Abort current process

### 4. Signals & IPC (`posix_signal.rs`)
- `sigaction()` - Signal handling
- `signal()` - Simplified signal handling
- `sigprocmask()` - Signal mask manipulation
- `sigemptyset()` / `sigfillset()` - Initialize signal sets
- `sigaddset()` / `sigdelset()` - Modify signal sets
- `sigismember()` - Check signal membership
- `sigpending()` - Get pending signals
- `sigsuspend()` - Wait for signal
- `sigwait()` - Wait for signal synchronously
- `pipe()` / `pipe2()` - Create pipes

### 5. Sockets (`posix_socket.rs`)
- `socket()` - Create socket
- `bind()` - Bind socket to address
- `connect()` - Connect socket to address
- `listen()` - Listen for connections
- `accept()` - Accept connections
- `send()` / `recv()` - Send/receive data
- `sendto()` / `recvfrom()` - Send/receive with address
- `shutdown()` - Shutdown socket
- `getsockopt()` / `setsockopt()` - Socket options

### 6. Minimal libc (`sigma_libc.rs`)
- **String functions**: `strlen`, `strcpy`, `strncpy`, `strcmp`, `strncmp`, `strchr`, `strstr`, `strcat`, `strncat`, `strdup`
- **Memory functions**: `malloc`, `free`, `realloc`, `calloc`, `memcpy`, `memmove`, `memset`, `memcmp`
- **I/O functions**: `printf`, `fprintf`, `sprintf`, `snprintf`, `puts`, `putchar`, `getchar`
- **Math functions**: `atoi`, `atol`, `strtol`, `strtoul`, `abs`, `labs`, `itoa`
- **Error handling**: `strerror`, `__errno_location`
- **Exit functions**: `exit`, `_exit`, `abort`

## Usage

### Linking with Sigma libc

To link your POSIX application with Sigma libc:

```rust
// In your Cargo.toml
[dependencies]
sigma_libc = { path = "../posix" }
```

### Using POSIX APIs

```c
#include <sigma_libc.h>

int main() {
    // Open a file
    int fd = open("test.txt", O_RDONLY);
    if (fd < 0) {
        perror("open");
        return 1;
    }

    // Read from file
    char buffer[1024];
    ssize_t bytes_read = read(fd, buffer, sizeof(buffer));

    // Close file
    close(fd);

    return 0;
}
```

### Using spawn() instead of fork/exec

```c
#include <sigma_libc.h>

int main() {
    SpawnOptions opts = {
        .path = "/bin/ls",
        .argv = (const char *[]){"ls", "-la", NULL},
        .envp = NULL,
        .stdin_fd = 0,
        .stdout_fd = 1,
        .stderr_fd = 2,
        .working_dir = NULL,
        .uid = 0,
        .gid = 0,
    };

    pid_t pid = posix_spawn(&opts);
    if (pid < 0) {
        perror("spawn");
        return 1;
    }

    // Wait for child
    int status;
    waitpid(pid, &status, 0);

    return 0;
}
```

### Using sockets

```c
#include <sigma_libc.h>

int main() {
    // Create socket
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) {
        perror("socket");
        return 1;
    }

    // Connect to server
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_port = htons(80);
    addr.sin_addr.s_addr = inet_addr("127.0.0.1");

    if (connect(sockfd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        perror("connect");
        close(sockfd);
        return 1;
    }

    // Send data
    const char *msg = "Hello, World!";
    send(sockfd, msg, strlen(msg), 0);

    // Close socket
    close(sockfd);

    return 0;
}
```

## Differences from Full POSIX

### No fork()
Use `spawn()` instead for process creation. This is a modern alternative that avoids the overhead of forking and is more suitable for microkernel architectures.

### Simplified Signals
Only essential signals are supported: SIGINT, SIGTERM, SIGKILL, SIGCHLD, SIGSTOP, SIGCONT, SIGHUP, SIGQUIT, SIGILL, SIGTRAP, SIGABRT, SIGBUS, SIGFPE, SIGUSR1, SIGSEGV, SIGUSR2, SIGPIPE, SIGALRM.

### No Job Control
No background/foreground job management (bg, fg, jobs).

### No Terminal Control
No termios, no terminal I/O control.

### Limited IPC
No System V IPC (msgget, semget, shmget), only pipes and sockets.

### No Real-time Extensions
No POSIX real-time scheduling or timers.

## Migration Guide

### Replace fork() + exec() with spawn()

**Before (POSIX):**
```c
pid_t pid = fork();
if (pid == 0) {
    execvp("ls", argv);
    exit(1);
}
```

**After (SigmaOS):**
```c
SpawnOptions opts = {
    .path = "/bin/ls",
    .argv = argv,
    .envp = environ,
    .stdin_fd = 0,
    .stdout_fd = 1,
    .stderr_fd = 2,
};
pid_t pid = posix_spawn(&opts);
```

### Use SigmaOS Native APIs for Advanced Features

For advanced features not available in the POSIX capsule, use SigmaOS's native OOP APIs directly. These provide better performance and integration with SigmaOS's AI-orchestration system.

### Avoid Obscure POSIX Features

Stick to the core POSIX primitives provided. Avoid relying on obscure POSIX features that are not implemented.

### Test Thoroughly

Test your application thoroughly with the compatibility layer. Report any issues or missing features.

## Building

### Building the POSIX Layer

```bash
cd posix
cargo build --release
```

### Building Applications with Sigma libc

```bash
# Link with sigma_libc
gcc -o myapp myapp.c -L../posix -lsigma_libc
```

## Testing

### Testing with Simple Utilities

Port simple POSIX utilities (BusyBox) to validate compatibility:

```bash
# Build BusyBox with Sigma libc
cd busybox
make CROSS_COMPILE=sigma-
```

### Test Coverage

Current test coverage includes:
- File I/O operations
- Process spawning and waiting
- Signal handling
- Pipe creation and usage
- Socket operations (TCP/UDP)

## Performance Considerations

The POSIX compatibility layer adds minimal overhead:
- Direct mapping to SigmaOS kernel calls
- No unnecessary abstraction layers
- Efficient file descriptor management
- Optimized memory allocation

## Security

The POSIX compatibility layer maintains SigmaOS's security model:
- All operations go through SigmaOS's security checks
- No bypass of mandatory access control
- Proper error handling and validation
- Memory-safe Rust implementation

## Future Extensions

The following may be added based on demand:
- Additional signal types
- More socket options
- Extended file operations (mmap, etc.)
- Additional libc functions
- POSIX threads subset (if needed)

## Contributing

When contributing to the POSIX compatibility layer:

1. Follow SigmaOS coding standards
2. Implement only essential POSIX features
3. Map to SigmaOS OOP abstractions
4. Add comprehensive documentation
5. Test on real hardware when possible
6. Update this README with new features

## References

- [POSIX_CAPABLE_SPEC.md](POSIX_CAPABLE_SPEC.md) - Detailed specification
- [SigmaOS Architecture](../Architecture.md) - Overall system architecture
- [Driver Development Guide](../drivers/DRIVER_DEVELOPMENT_GUIDE.md) - Driver development information

## License

All POSIX compatibility layer code is licensed under GPL-2.0-or-later, consistent with the Linux kernel.
