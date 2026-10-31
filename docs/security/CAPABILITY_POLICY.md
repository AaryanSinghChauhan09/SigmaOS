# SigmaOS Capability Policy

## Capability System

SigmaOS utilizes fine-grained process capabilities to restrict root/privileged system operations.

## Enforceable Capabilities

- `CAP_SYS_ADMIN`: Overall system administration and hardware configuration.
- `CAP_NET_ADMIN`: Network interface and routing configuration.
- `CAP_DAC_OVERRIDE`: Bypass file read/write permission checks.
- `CAP_SYS_PTRACE`: Process tracing and debugging inspection.
- `CAP_SYS_RAWIO`: Raw port/device I/O access.

## Enforcement

Capabilities are attached to process descriptors and checked explicitly at syscall boundaries.
