# POSIX Compatibility in SigmaOS

SigmaOS includes a selective POSIX compatibility layer that enables porting of existing Unix/Linux software while preserving its AI-first, microkernel identity. This "Minimal POSIX Capsule" provides essential POSIX primitives mapped to SigmaOS's object-oriented kernel abstractions.

## Overview

The POSIX compatibility layer is designed as a bridge to the existing Unix/Linux ecosystem without drowning in legacy. It implements only the essential POSIX primitives needed for common applications, avoiding the heavy baggage of full POSIX compliance.

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

## What's Included

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
- `access()` - Check file accessibility

### Process & Thread Model
- `spawn()` - Modern process spawning (alternative to fork/exec)
- `wait()` / `waitpid()` - Wait for child processes
- `exit()` - Process termination
- `getpid()` - Get process ID
- `getppid()` - Get parent process ID
- `kill()` - Send signals to processes

### Signals
- `sigaction()` - Signal handling
- `signal()` - Simplified signal handling
- `sigprocmask()` - Signal mask manipulation
- `sigemptyset()` / `sigfillset()` - Initialize signal sets
- `sigaddset()` / `sigdelset()` - Modify signal sets
- `sigismember()` - Check signal membership
- Supported signals: SIGINT, SIGTERM, SIGKILL, SIGCHLD, SIGSTOP, SIGCONT, SIGHUP, SIGQUIT, SIGILL, SIGTRAP, SIGABRT, SIGBUS, SIGFPE, SIGUSR1, SIGSEGV, SIGUSR2, SIGPIPE, SIGALRM

### IPC (Inter-Process Communication)
- `pipe()` / `pipe2()` - Create pipes
- Socket-based IPC (see below)

### Networking Sockets
- `socket()` - Create socket
- `bind()` - Bind socket to address
- `connect()` - Connect socket to address
- `listen()` - Listen for connections
- `accept()` - Accept connections
- `send()` / `recv()` - Send/receive data
- `sendto()` / `recvfrom()` - Send/receive with address
- `shutdown()` - Shutdown socket
- `getsockopt()` / `setsockopt()` - Socket options
- Address families: AF_INET, AF_INET6, AF_UNIX
- Socket types: SOCK_STREAM, SOCK_DGRAM, SOCK_RAW, SOCK_SEQPACKET

### Minimal libc Subset
- **String functions**: `strlen`, `strcpy`, `strncpy`, `strcmp`, `strncmp`, `strchr`, `strstr`, `strcat`, `strncat`, `strdup`
- **Memory functions**: `malloc`, `free`, `realloc`, `calloc`, `memcpy`, `memmove`, `memset`, `memcmp`
- **I/O functions**: `printf`, `fprintf`, `sprintf`, `snprintf`, `puts`, `putchar`, `getchar`
- **Math functions**: `atoi`, `atol`, `strtol`, `strtoul`, `abs`, `labs`, `itoa`
- **Error handling**: `strerror`, `errno`
- **Exit functions**: `exit`, `_exit`, `abort`

## What's Excluded

### Legacy Shell Utilities
No reimplementation of grep, awk, sed, etc. SigmaOS provides AI-native equivalents.

### Full POSIX Compliance
- No strict signal semantics
- No job control (bg, fg, jobs)
- No terminal control (termios)
- No obscure POSIX APIs

### Heavy Compatibility Layers
- No full POSIX threads (pthreads) - use SigmaOS native threading
- No full POSIX IPC (System V IPC) - use SigmaOS native IPC
- No full POSIX real-time extensions

## Key Differences from Full POSIX

### No fork()
Use `spawn()` instead for process creation. This is a modern alternative that avoids the overhead of forking and is more suitable for microkernel architectures.

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

### Simplified Signals
Only essential signals are supported. Complex signal semantics are not implemented.

### No Job Control
No background/foreground job management.

### No Terminal Control
No termios, no terminal I/O control.

### Limited IPC
No System V IPC (msgget, semget, shmget), only pipes and sockets.

## Usage

### Linking with Sigma libc

To link your POSIX application with Sigma libc:

```rust
// In your Cargo.toml
[dependencies]
sigma_libc = { path = "../posix" }
```

Or with C:

```bash
gcc -o myapp myapp.c -L../posix -lsigma_libc
```

### Example: File I/O

```c
#include <sigma_libc.h>

int main() {
    int fd = open("test.txt", O_RDONLY);
    if (fd < 0) {
        perror("open");
        return 1;
    }

    char buffer[1024];
    ssize_t bytes_read = read(fd, buffer, sizeof(buffer));
    
    close(fd);
    return 0;
}
```

### Example: Process Spawning

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

    int status;
    waitpid(pid, &status, 0);

    return 0;
}
```

### Example: Sockets

```c
#include <sigma_libc.h>

int main() {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) {
        perror("socket");
        return 1;
    }

    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_port = htons(80);
    addr.sin_addr.s_addr = inet_addr("127.0.0.1");

    if (connect(sockfd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        perror("connect");
        close(sockfd);
        return 1;
    }

    const char *msg = "Hello, World!";
    send(sockfd, msg, strlen(msg), 0);

    close(sockfd);
    return 0;
}
```

## Migration Guide

### For Developers

1. **Replace fork/exec with spawn()**: Use the modern `spawn()` abstraction instead of fork/exec
2. **Use SigmaOS native APIs for advanced features**: For features not in the POSIX capsule, use SigmaOS's native OOP APIs
3. **Avoid obscure POSIX features**: Stick to core POSIX primitives
4. **Test thoroughly**: Test your application with the compatibility layer

### For Porting Applications

1. **Identify POSIX dependencies**: Check which POSIX APIs your application uses
2. **Verify compatibility**: Ensure all used APIs are in the POSIX capsule
3. **Adapt to spawn()**: Replace fork/exec with spawn()
4. **Test on SigmaOS**: Run tests on actual SigmaOS hardware or emulator

## Benefits

### Compatibility Boost
Developers can port software more easily from Linux/Unix to SigmaOS.

### Lightweight
Avoids full POSIX baggage, keeping the OS lean and fast.

### Innovation Preserved
SigmaOS keeps its AI-native orchestration and unique design.

### Gradual Adoption
Developers can start with familiar APIs, then migrate to SigmaOS's new paradigms.

## Performance

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

## Implementation Details

### File Descriptor Management
File descriptors are mapped to SigmaOS's object-oriented file handles using a global file descriptor table.

### Process Management
Processes are mapped to SigmaOS's process objects with a global process table. The `spawn()` function provides a modern alternative to fork/exec.

### Socket Management
Sockets are mapped to SigmaOS's networking objects with a global socket table.

### Signal Handling
Signals are managed through a global signal action table with mask support.

## Testing

### Test Coverage
Current test coverage includes:
- File I/O operations
- Process spawning and waiting
- Signal handling
- Pipe creation and usage
- Socket operations (TCP/UDP)

### Future Testing
- Port simple POSIX utilities (BusyBox)
- Validate compatibility with common tools (curl, ssh)
- Performance benchmarking

## Future Extensions

The following may be added based on demand:
- Additional signal types
- More socket options
- Extended file operations (mmap, etc.)
- Additional libc functions
- POSIX threads subset (if needed)

## Documentation

- [POSIX_CAPABLE_SPEC.md](../posix/POSIX_CAPABLE_SPEC.md) - Detailed specification
- [README.md](../posix/README.md) - Usage guide and examples
- [SigmaOS Architecture](Architecture.md) - Overall system architecture

## Contributing

When contributing to the POSIX compatibility layer:

1. Follow SigmaOS coding standards
2. Implement only essential POSIX features
3. Map to SigmaOS OOP abstractions
4. Add comprehensive documentation
5. Test on real hardware when possible
6. Update this wiki page with new features

## License

All POSIX compatibility layer code is licensed under GPL-2.0-or-later, consistent with the Linux kernel.
