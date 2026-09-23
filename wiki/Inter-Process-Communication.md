# Inter-Process Communication (IPC)

## Overview

SigmaOS implements comprehensive inter-process communication mechanisms inspired by Linux IPC, FreeBSD IPC, and POSIX standards. The system provides various IPC mechanisms for processes to communicate and synchronize with each other.

## IPC Mechanisms

### Pipes

- **Anonymous pipes**: One-way communication between related processes
- **Named pipes (FIFOs)**: Communication between unrelated processes
- **Pipe capacity**: Configurable pipe buffer size
- **Atomic writes**: Guaranteed atomicity for small writes

### Message Queues

- **System V message queues**: POSIX-compliant message queues
- **Priority queues**: Priority-based message delivery
- **Message types**: Typed messages for multiplexing
- **Blocking/non-blocking**: Configurable blocking behavior

### Shared Memory

- **POSIX shared memory**: Memory-mapped shared segments
- **System V shared memory**: Legacy shared memory API
- **Memory protection**: Access control via permissions
- **Synchronization**: Integrated with mutexes and semaphores

### Semaphores

- **POSIX semaphores**: Named and unnamed semaphores
- **System V semaphores**: Semaphore sets with operations
- **Binary semaphores**: Mutual exclusion
- **Counting semaphores**: Resource counting

### Sockets

- **Unix domain sockets**: Local IPC via filesystem
- **TCP/IP sockets**: Network-based IPC
- **UDP sockets**: Connectionless datagram communication
- **Raw sockets**: Low-level protocol access

## Advanced IPC

### File Descriptors

- **File descriptor passing**: Send/receive file descriptors
- **SCM_RIGHTS**: Ancillary data for fd passing
- **Sealing**: Prevent fd modification
- **Duplication**: Efficient fd duplication

### Memory-Mapped Files

- **mmap()**: Map files into process address space
- **MAP_SHARED**: Shared mapping for IPC
- **MAP_PRIVATE**: Private copy-on-write mapping
- **msync()**: Synchronize mapped memory

### Event Notification

- **signalfd**: Signal delivery via file descriptor
- **eventfd**: Event notification
- **timerfd**: Timer notifications via fd
- **inotify**: Filesystem event monitoring

## Synchronization Primitives

### Mutexes

- **POSIX mutexes**: Mutual exclusion locks
- **Recursive mutexes**: Allow recursive locking
- **Priority inheritance**: Prevent priority inversion
- **Error checking**: Robust mutex checking

### Condition Variables

- **POSIX condition variables**: Wait/signal synchronization
- **Spurious wakeups**: Handle spurious wakeups correctly
- **Broadcast vs signal**: Efficient wake-up strategies
- **Timeouts**: Configurable wait timeouts

### Read-Write Locks

- **Shared-exclusive locking**: Multiple readers, single writer
- **Write preference**: Prioritize writers
- **Read preference**: Prioritize readers
- **Fair scheduling**: Balanced reader/writer access

## Performance Optimization

### Zero-Copy IPC

- **splice()**: Zero-copy data transfer
- **tee()**: Duplicate pipe data
- **vmsplice()**: Zero-copy from user memory
- **sendfile()**: Zero-copy file transfer

### Shared Memory Optimization

- **Huge pages**: Reduce TLB misses
- **NUMA awareness**: Optimize for NUMA architectures
- **Cache alignment**: Align data for cache efficiency
- **Prefetching**: Pre-fetch data for better performance

### Lock-Free IPC

- **Atomic operations**: Lock-free data structures
- **RCU (Read-Copy-Update)**: Lock-free read access
- **Seqlocks**: Optimistic locking
- **Ring buffers**: Lock-free circular buffers

## Security

### IPC Security

- **Permissions**: File-based access control
- **Capabilities**: Fine-grained privilege control
- **Namespaces**: IPC namespace isolation
- **Seccomp**: System call filtering

### Secure IPC

- **Authenticated IPC**: Verify peer identity
- **Encrypted IPC**: Protect data in transit
- **Integrity checking**: Detect tampering
- **Access control**: Restrict IPC access

## Configuration

### IPC Limits

```bash
# Set message queue size
echo 65536 > /proc/sys/kernel/msgmnb

# Set semaphore limits
echo 250 > /proc/sys/kernel/sem

# Set shared memory size
echo 4194304 > /proc/sys/kernel/shmmax

# Set pipe capacity
echo 1048576 > /proc/sys/fs/pipe-max-size
```

### IPC Namespace

```bash
# Create IPC namespace
unshare --ipc

# List IPC objects
ipcs

# Remove IPC objects
ipcrm -m <shmid>
ipcrm -q <msgid>
ipcrm -s <semid>
```

## Usage Examples

### Pipes

```c
// Create pipe
int pipefd[2];
pipe(pipefd);

// Fork process
pid_t pid = fork();
if (pid == 0) {
    // Child: read from pipe
    close(pipefd[1]);
    char buf[1024];
    read(pipefd[0], buf, sizeof(buf));
} else {
    // Parent: write to pipe
    close(pipefd[0]);
    write(pipefd[1], "Hello", 5);
}
```

### Shared Memory

```c
// Create shared memory
int shmid = shmget(IPC_PRIVATE, 4096, IPC_CREAT | 0666);

// Attach shared memory
void *shmaddr = shmat(shmid, NULL, 0);

// Use shared memory
strcpy((char *)shmaddr, "Hello, World!");

// Detach shared memory
shmdt(shmaddr);

// Remove shared memory
shmctl(shmid, IPC_RMID, NULL);
```

### Message Queues

```c
// Create message queue
int msqid = msgget(IPC_PRIVATE, IPC_CREAT | 0666);

// Send message
struct msgbuf msg;
msg.mtype = 1;
strcpy(msg.mtext, "Hello");
msgsnd(msqid, &msg, sizeof(msg.mtext), 0);

// Receive message
msgrcv(msqid, &msg, sizeof(msg.mtext), 1, 0);

// Remove message queue
msgctl(msqid, IPC_RMID, NULL);
```

## Monitoring

### IPC Statistics

```bash
# Get IPC statistics
ipcs -u

# Monitor IPC usage
watch -n 1 'ipcs -u'

# Trace IPC operations
strace -e trace=msgsnd,msgrcv,shmget,shmat <command>
```

## Troubleshooting

### Common Issues

1. **EAGAIN errors**: Resource limits exceeded
2. **EIDRM errors**: IPC object removed
3. **EACCES errors**: Permission denied
4. **Deadlocks**: Incorrect synchronization

### Debugging

```bash
# Check IPC limits
cat /proc/sys/kernel/msgmnb
cat /proc/sys/kernel/shmmax
cat /proc/sys/kernel/sem

# Check IPC objects
ipcs -a

# Trace IPC operations
strace -f -e trace=ipc <command>
```

## References

- [Linux IPC Documentation](https://man7.org/linux/man-pages/man7/ipc.7.html)
- [POSIX IPC](https://pubs.opengroup.org/onlinepubs/9699919799/)
- [FreeBSD IPC](https://www.freebsd.org/doc/en/books/arch-handbook/ips.html)
- [System V IPC](https://man7.org/linux/man-pages/man7/svipc.7.html)
