# SigmaOS Architecture

SigmaOS follows a modern, modular monolithic kernel design augmented with microkernel-like capability models.

## Architecture Overview

```mermaid
graph TD
    A[User Space Applications] --> B(Compatibility Layers: POSIX, Win32)
    B --> C[Syscall Interface]
    C --> D[Capability Checker]
    D --> E[Kernel Subsystems]
    E --> F[VFS / SigmaFS]
    E --> G[Network Stack]
    E --> H[Memory Manager]
    E --> I[Process Scheduler]
    F --> J[Storage Drivers]
    G --> K[NIC Drivers]
```

## Security Layer
Every transition from User Space to Kernel Space requires a verified Capability Token.

```mermaid
sequenceDiagram
    User->>SyscallDispatcher: Request Open()
    SyscallDispatcher->>CapabilityManager: Verify Token
    CapabilityManager-->>SyscallDispatcher: Valid/Invalid
    SyscallDispatcher->>VFS: Execute Open
    VFS-->>User: File Descriptor
```

## Package Management
SigmaPkg operates immutably:
- Transactions are atomic.
- Configurations are declarative.
