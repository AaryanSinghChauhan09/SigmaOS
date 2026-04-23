# System Call Interface

SigmaOS fundamentally redesigns the System Call boundary, abandoning the legacy POSIX synchronous interrupt model in favor of an **Asynchronous, Capability-Verified Ring Buffer**.

Located in `modules/core/kernel/syscalls.c`.

## Competitive Advantages (USPs) over Linux/Windows

### 1. Asynchronous Batching (io_uring for everything)
- **Standard OS**: When a program makes a syscall (like `read()` or `malloc()`), the CPU must execute a costly context switch into Ring 0, do the work, and switch back.
- **SigmaOS USP**: Processes communicate with the kernel via a shared memory `syscall_queue_t` ring buffer. A process can queue up 50 different system calls (memory leasing, IPC, network sending) and submit them all at once. The kernel processes them asynchronously, drastically reducing context-switch overhead and maximizing CPU cache hits.

### 2. Zero-Trust Capability Verification
- **Standard OS**: Syscall security relies on UID/GID checks (e.g., "Is this user root?").
- **SigmaOS USP**: There is no concept of "root". **Every single system call** requires the caller to pass a `cap_token`. If you want to lease memory, your token must mathematically prove you have the right to request memory. If the capability is missing or revoked, the syscall fails and the Sovereign Watchdog is notified.

### 3. Native Anomaly Tracing
- **Standard OS**: Tracing requires loading external eBPF programs or running strace, which adds massive overhead.
- **SigmaOS USP**: Every syscall automatically flows through `syscall_tracer.c`. If a process attempts a "syscall storm" (e.g., trying to brute-force capabilities or spamming IPC), the kernel's anomaly detection catches it instantly and logs it to the Tamper-Proof Audit Chain.
