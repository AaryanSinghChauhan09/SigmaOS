# System Call Interface

## Overview

SigmaOS implements a comprehensive system call interface inspired by Linux syscalls, FreeBSD syscalls, and POSIX standards. The system provides a secure, efficient, and extensible mechanism for user-space applications to request kernel services.

## System Call Architecture

### System Call Mechanism

- **Syscall entry**: Controlled transition from user to kernel mode
- **Argument passing**: Register-based argument passing
- **Return values**: Return values and error codes
- **Context switching**: Fast context switch on syscall entry/exit

### System Call Table

- **Syscall table**: Linear array of syscall handler functions
- **Syscall numbers**: Unique identifiers for each syscall
- **Fast path optimization**: Inline syscalls for common operations
- **Multi-architecture support**: ABI-specific syscall entry points

## System Call Categories

### Process Management

- **fork()**: Create child process
- **execve()**: Execute new program
- **exit()**: Terminate process
- **waitpid()**: Wait for child process
- **getpid()**: Get process ID
- **getppid()**: Get parent process ID

### File Management

- **open()**: Open file
- **close()**: Close file descriptor
- **read()**: Read from file
- **write()**: Write to file
- **lseek()**: Seek in file
- **stat()**: Get file status
- **fstat()**: Get file descriptor status

### Memory Management

- **mmap()**: Map files into memory
- **munmap()**: Unmap memory region
- **mprotect()**: Change memory protection
- **brk()**: Change data segment size
- **madvise()**: Give advice about memory use

### Process Communication

- **pipe()**: Create pipe
- **socket()**: Create socket
- **bind()**: Bind socket to address
- **listen()**: Listen for connections
- **accept()**: Accept connection
- **connect()**: Connect to socket

### Information

- **uname()**: Get system information
- **sysinfo()**: Get system statistics
- **getrlimit()**: Get resource limits
- **setrlimit()**: Set resource limits
- **gettimeofday()**: Get time of day

## Security Features

### Syscall Filtering

- **seccomp**: Secure computing mode
- **seccomp-bpf**: BPF-based syscall filtering
- **Landlock**: Filesystem access control
- **Capsicum**: Capability-based security

### Privilege Separation

- **Capabilities**: Fine-grained privileges
- **Namespaces**: Process isolation
- **Cgroups**: Resource control
- **UID/GID**: User/group identification

## Performance Optimization

### Fast System Calls

- **sysenter/sysexit**: Fast syscall instructions (x86)
- **syscall/sysret**: Alternative fast syscall (x86-64)
- **vDSO**: Virtual dynamic shared object
- **Batch syscalls**: Group multiple syscalls

### Zero-Copy Operations

- **sendfile()**: Zero-copy file transfer
- **splice()**: Zero-copy pipe transfer
- **tee()**: Duplicate pipe data
- **mmap()**: Memory-mapped I/O

## New System Calls

### SigmaOS-Specific Syscalls

- **sigma_process_isolate()**: Enhanced process isolation
- **sigma_mem_protect()**: Advanced memory protection
- **sigma_secure_exec()**: Secure execution context
- **sigma_resource_reserve()**: Resource reservation

### Extended POSIX Syscalls

- **openat2()**: Extended open with better security
- **process_mprotect()**: Memory protection control
- **mount_setattr()**: Mount attribute control
- **pidfd_open()**: PID file descriptor

## System Call Tracing

### Tracing Mechanisms

- **ptrace**: Process tracing and debugging
- **ftrace**: Function tracer
- **perf_events**: Performance monitoring
- **audit**: System call auditing

### Debugging

```bash
# Trace system calls
strace -p <pid>

# Count system calls
strace -c <command>

# Filter specific syscalls
strace -e trace=open,read,write <command>
```

## Compatibility

### Linux Compatibility

- **Linux syscall emulation**: Linux syscall ABI support
- **Linux binary compatibility**: Run Linux binaries
- **ioctl() compatibility**: Device control compatibility
- **/proc filesystem**: Linux procfs compatibility

### BSD Compatibility

- **BSD syscall emulation**: BSD syscall ABI support
- **BSD binary compatibility**: Run BSD binaries
- **kqueue/kevent**: BSD event notification
- **pledge/unveil**: OpenBSD security features

## Configuration

### System Call Limits

```bash
# Set maximum number of file descriptors
ulimit -n 65536

# Set maximum number of processes
ulimit -u 4096

# Set memory limit
ulimit -v 8388608
```

### Security Configuration

```bash
# Enable seccomp
echo 1 > /proc/sys/kernel/seccomp

# Enable syscall auditing
auditctl -a exit,always -F arch=b64 -S all

# Configure syscall filter
seccomp-tools dump <binary>
```

## Performance Monitoring

### System Call Statistics

```bash
# Get system call statistics
cat /proc/syscalls

# Monitor system call rate
perf stat -e syscalls:sys_enter <command>

# Analyze system call latency
perf record -e syscalls:sys_enter -e syscalls:sys_exit <command>
```

## Troubleshooting

### Common Issues

1. **EACCES errors**: Check file permissions and capabilities
2. **ENOSPC errors**: Check disk space and quotas
3. **EMFILE errors**: Check file descriptor limits
4. **EAGAIN errors**: Check resource limits and non-blocking I/O

### Debugging

```bash
# Check system call table
cat /proc/kallsyms | grep sys_call_table

# Check syscall latency
cyclictest -t1 -p 80 -n 1000000

# Monitor system calls
strace -f -p <pid>
```

## References

- [Linux System Call Table](https://chromium.googlesource.com/chromiumos/docs/+/master/syscall_internals.md)
- [POSIX System Calls](https://pubs.opengroup.org/onlinepubs/9699919799/)
- [FreeBSD System Calls](https://www.freebsd.org/doc/en/books/developers-handbook/x86-system-calls.html)
- [seccomp Documentation](https://www.kernel.org/doc/Documentation/prctl/seccomp_filter.txt)
