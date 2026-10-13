# SigmaOS Capability Enforcement Policy

## Overview

SigmaOS replaces traditional monolithic administrative superuser privilege with granular capability tokens.

## Capability Token Architecture

Every process PCB carries a set of active `CapabilityToken` values:

- `CAP_SYS_ADMIN`: Allows system administrative operations (e.g. module loading, debug tracing).
- `CAP_NET_BIND_SERVICE`: Permits binding to privileged network ports (< 1024).
- `CAP_DAC_OVERRIDE`: Overrides VFS file permission checks.
- `CAP_SYS_PTRACE`: Permits process inspection and debugging.

## Verification Workflow

On syscall entry dispatch (`src/kernel/tier1_kernel_execution_suite.rs`), the dispatcher evaluates:

```rust
if !current_process().has_capability(required_cap) {
    return Err(SyscallError::PermissionDenied);
}
```
