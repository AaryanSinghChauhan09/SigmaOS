# IPC and Syscalls

**Capability state: Prototype.** API models do not establish a kernel-user boundary or working process communication. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

No verified user-mode process, syscall entry/return path, or IPC mechanism has been demonstrated through a booted kernel. Syscall names and POSIX/Linux compatibility stubs are not evidence of ABI compatibility. Do not treat service models as separate processes or claim signals, sockets, message queues, or shared memory as runtime features without integration tests.

## Design references

Linux syscall and fd semantics, FreeBSD Capsicum, Redox's message-based interfaces, and xv6's small auditable trap path are useful references. Keep the ABI narrow and document resource ownership and failure behavior.

## Roadmap

1. Define one architecture syscall ABI and validate privilege transitions.
2. Implement safe user-memory validation and a minimal read/write/exit path.
3. Add process lifecycle and descriptor ownership before IPC extensions.
4. Add one IPC primitive with malformed-message, permission, and resource-exhaustion tests.

**Completion evidence:** QEMU tests execute a real userspace program, exercise syscalls and IPC, and verify invalid pointers and unauthorized operations fail safely.
