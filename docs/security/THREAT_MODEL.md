# SigmaOS Threat Model

## Threat Vectors

1. **Kernel Syscall Argument Manipulation**: Passing null, invalid, or out-of-bounds pointers to syscall handlers.
2. **Buffer / Resource Exhaustion**: Exhausting kernel heap frames, file descriptors, process table entries, or swap metadata.
3. **Privilege Escalation**: Attempting unprivileged execution of restricted syscalls or hardware access.
4. **Side-Channel & Concurrency Vulnerabilities**: Race conditions in shared data structures or lock-free queues.

## Mitigations

- Strict copy-in / copy-out buffer length validation for all syscall pointers.
- Bounded queues and resource quotas across all kernel modules.
- Capability-based access control and process sandbox constraints.
