# Namespace Syscalls API Reference

## Overview

This document provides a complete API reference for the namespace syscalls implemented in Phase 8.1.4.

## Syscall Definitions

### sys_clone

Create a child process with optional namespace isolation.

**Signature**:
```rust
pub fn sys_clone(
    flags: u32,
    child_stack: *mut u8,
    parent_tidptr: *mut i32,
    child_tidptr: *mut i32,
    tls_val: u64,
) -> i64
```

**Parameters**:

| Parameter | Type | Description |
|-----------|------|-------------|
| `flags` | u32 | Clone flags including namespace flags |
| `child_stack` | *mut u8 | Stack pointer for child process |
| `parent_tidptr` | *mut i32 | Optional parent TID pointer |
| `child_tidptr` | *mut i32 | Optional child TID pointer |
| `tls_val` | u64 | TLS value for child |

**Return Value**:

| Value | Meaning |
|-------|---------|
| > 0 | Child process PID (success in parent) |
| = 0 | Child process (success in child) |
| < 0 | Error code (negative Linux errno) |

**Flags**:

```c
#define CLONE_NEWPID    0x20000000  /* New PID namespace */
#define CLONE_NEWIPC    0x08000000  /* New IPC namespace */
#define CLONE_NEWNS     0x00020000  /* New mount namespace */
#define CLONE_NEWNET    0x40000000  /* New network namespace (not supported) */
#define CLONE_NEWUSER   0x10000000  /* New user namespace (not supported) */
#define CLONE_NEWUTS    0x04000000  /* New UTS namespace (not supported) */
#define CLONE_NEWCGROUP 0x02000000  /* New cgroup namespace (not supported) */
```

**Example**:
```c
// Create child with new PID namespace
void *stack = malloc(4096);
pid_t child = sys_clone(CLONE_NEWPID, (char*)stack + 4096, NULL, NULL, 0);

if (child > 0) {
    // Parent process
    printf("Child PID: %d\n", child);
} else if (child == 0) {
    // Child process
    printf("I am in a new namespace\n");
} else {
    // Error
    perror("clone");
}
```

**Error Handling**:
- **-22 (EINVAL)**: Invalid argument (null child_stack without CLONE_VM)
- **-95 (ENOTSUP)**: Unsupported namespace type (NEWNET, NEWUSER, etc.)
- **-12 (ENOMEM)**: Out of memory
- **-1 (EPERM)**: Permission denied

---

### sys_unshare

Unshare namespaces from the parent process.

**Signature**:
```rust
pub fn sys_unshare(flags: u32) -> i64
```

**Parameters**:

| Parameter | Type | Description |
|-----------|------|-------------|
| `flags` | u32 | Unshare flags indicating which namespaces to unshare |

**Return Value**:

| Value | Meaning |
|-------|---------|
| = 0 | Success |
| < 0 | Error code (negative Linux errno) |

**Flags**:

```c
#define UNSHARE_NEWPID    0x20000000  /* Unshare PID namespace */
#define UNSHARE_NEWIPC    0x08000000  /* Unshare IPC namespace */
#define UNSHARE_NEWNS     0x00020000  /* Unshare mount namespace */
```

**Example**:
```c
// Create new PID namespace for current process
if (sys_unshare(UNSHARE_NEWPID) == 0) {
    printf("Successfully created new PID namespace\n");
} else {
    perror("unshare");
}

// Combine multiple namespaces
int result = sys_unshare(UNSHARE_NEWPID | UNSHARE_NEWIPC | UNSHARE_NEWNS);
if (result < 0) {
    perror("unshare");
}
```

**Error Handling**:
- **-22 (EINVAL)**: Invalid flags
- **-12 (ENOMEM)**: Out of memory
- **-1 (EPERM)**: Permission denied

**Behavior**:
- Creates new namespaces for specified types
- Subsequent child processes inherit new namespaces
- Current process starts with PID 2 in new PID namespace
- IPC and mount objects are isolated

---

### sys_setns

Join an existing namespace.

**Signature**:
```rust
pub fn sys_setns(nsfd: u64, nstype: i32) -> i64
```

**Parameters**:

| Parameter | Type | Description |
|-----------|------|-------------|
| `nsfd` | u64 | Namespace file descriptor (or namespace ID) |
| `nstype` | i32 | Type of namespace (0=auto, 1=PID, 2=IPC, 3=Mount) |

**Return Value**:

| Value | Meaning |
|-------|---------|
| = 0 | Success |
| < 0 | Error code (negative Linux errno) |

**Namespace Type Constants**:

```c
#define NS_TYPE_AUTO   0  /* Auto-detect from fd (not in this impl) */
#define NS_TYPE_PID    1  /* PID namespace */
#define NS_TYPE_IPC    2  /* IPC namespace */
#define NS_TYPE_MOUNT  3  /* Mount namespace */
```

**Example**:
```c
// Join PID namespace with ID 42
if (sys_setns(42, NS_TYPE_PID) == 0) {
    printf("Successfully joined namespace\n");
} else {
    perror("setns");
}

// Open namespace and join
int nsfd = open("/proc/pid/ns/pid", O_RDONLY);
if (sys_setns(nsfd, NS_TYPE_PID) < 0) {
    perror("setns");
}
```

**Error Handling**:
- **-22 (EINVAL)**: Invalid namespace ID or type
- **-9 (EBADF)**: Bad file descriptor (or invalid namespace ID)
- **-3 (ESRCH)**: Namespace not found
- **-1 (EPERM)**: Permission denied

**Behavior**:
- Process joins the specified namespace
- Reference count on namespace is incremented
- Process can now access namespace-isolated resources
- Namespace persists until reference count reaches zero

---

## Data Structures

### CloneFlags

```rust
pub struct CloneFlags(u32);

impl CloneFlags {
    pub fn new(flags: u32) -> Self;
    pub fn raw(&self) -> u32;
    pub fn clone_newpid(&self) -> bool;
    pub fn clone_newipc(&self) -> bool;
    pub fn clone_newns(&self) -> bool;
    pub fn clone_newnet(&self) -> bool;
    pub fn clone_newuser(&self) -> bool;
    pub fn clone_newuts(&self) -> bool;
    pub fn clone_newcgroup(&self) -> bool;
    pub fn namespace_flags(&self) -> u32;
}
```

**Methods**:
- `new(flags)`: Create from raw flags
- `raw()`: Get raw flag value
- `clone_new*()`: Check if specific namespace flag is set
- `namespace_flags()`: Extract all namespace flags

---

### UnshareFlags

```rust
pub struct UnshareFlags(u32);

impl UnshareFlags {
    pub fn new(flags: u32) -> Self;
    pub fn raw(&self) -> u32;
    pub fn unshare_newpid(&self) -> bool;
    pub fn unshare_newipc(&self) -> bool;
    pub fn unshare_newns(&self) -> bool;
    pub fn namespace_flags(&self) -> u32;
}
```

---

### NamespaceSyscallError

```rust
pub enum NamespaceSyscallError {
    InvalidArgument = -22,
    PermissionDenied = -1,
    NoMemory = -12,
    BadFileDescriptor = -9,
    NotSupported = -95,
    NoSuchProcess = -3,
    AlreadyInNamespace = -17,
}

impl NamespaceSyscallError {
    pub fn code(&self) -> i32;
}
```

---

### NamespaceRegistry

```rust
pub struct NamespaceRegistry {
    // Internal: pid_namespaces, ipc_namespaces, mount_namespaces
}

impl NamespaceRegistry {
    pub fn new() -> Self;
    
    pub fn register_pid_namespace(
        &self,
        ns_id: u64,
        owner_pid: u32,
    ) -> Result<(), NamespaceSyscallError>;
    
    pub fn register_ipc_namespace(
        &self,
        ns_id: u64,
        owner_pid: u32,
    ) -> Result<(), NamespaceSyscallError>;
    
    pub fn register_mount_namespace(
        &self,
        ns_id: u64,
        owner_pid: u32,
    ) -> Result<(), NamespaceSyscallError>;
    
    pub fn increment_ref(
        &self,
        ns_id: u64,
        ns_type: &str,
    ) -> Result<(), NamespaceSyscallError>;
    
    pub fn decrement_ref(
        &self,
        ns_id: u64,
        ns_type: &str,
    ) -> Result<(), NamespaceSyscallError>;
    
    pub fn namespace_exists(&self, ns_id: u64, ns_type: &str) -> bool;
}
```

**Thread Safety**: All methods are thread-safe via internal Mutex

---

### ProcessNamespaceContext

```rust
pub struct ProcessNamespaceContext {
    pub pid_namespace_id: Option<u64>,
    pub ipc_namespace_id: Option<u64>,
    pub mount_namespace_id: Option<u64>,
}

impl ProcessNamespaceContext {
    pub fn new() -> Self;
    
    pub fn from_clone_flags(
        flags: CloneFlags,
        base_context: &ProcessNamespaceContext,
    ) -> Result<Self, NamespaceSyscallError>;
    
    pub fn in_namespace(&self, ns_id: u64, ns_type: &str) -> bool;
}
```

---

## Usage Examples

### Example 1: Simple Process Isolation

```c
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>

int main() {
    // Create new PID namespace
    if (sys_unshare(CLONE_NEWPID) < 0) {
        perror("unshare");
        return 1;
    }
    
    pid_t pid = fork();
    if (pid == 0) {
        // Child process
        printf("Child PID: %d\n", getpid());  // Will be 1
        exit(0);
    } else {
        // Parent process
        printf("Parent PID: %d\n", getpid());
        waitpid(pid, NULL, 0);
    }
    
    return 0;
}
```

### Example 2: Multiple Namespace Isolation

```c
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>

int main() {
    // Create new PID, IPC, and mount namespaces
    int flags = CLONE_NEWPID | CLONE_NEWIPC | CLONE_NEWNS;
    
    if (sys_unshare(flags) < 0) {
        perror("unshare");
        return 1;
    }
    
    printf("Process isolated in new namespaces\n");
    
    // Subsequent operations are namespace-isolated
    return 0;
}
```

### Example 3: Namespace Joining

```c
#include <unistd.h>
#include <stdio.h>

int main() {
    // Join PID namespace 100
    if (sys_setns(100, NS_TYPE_PID) < 0) {
        perror("setns");
        return 1;
    }
    
    printf("Successfully joined namespace 100\n");
    
    // Now part of namespace 100
    return 0;
}
```

---

## Error Handling Guide

### Common Error Codes

| Code | Name | Cause | Recovery |
|------|------|-------|----------|
| -1 | EPERM | Permission denied | May need elevated privileges |
| -3 | ESRCH | No such process | Namespace may have been destroyed |
| -9 | EBADF | Bad file descriptor | Invalid namespace ID |
| -12 | ENOMEM | Out of memory | Free system memory and retry |
| -17 | EEXIST | Already in namespace | Namespace already active |
| -22 | EINVAL | Invalid argument | Check arguments for valid values |
| -95 | ENOTSUP | Operation not supported | Feature not yet implemented |

### Error Handling Pattern

```c
int result = sys_clone(flags, stack, NULL, NULL, 0);

if (result < 0) {
    switch (-result) {
        case 22:  // EINVAL
            fprintf(stderr, "Invalid argument\n");
            break;
        case 12:  // ENOMEM
            fprintf(stderr, "Out of memory\n");
            break;
        case 95:  // ENOTSUP
            fprintf(stderr, "Namespace type not supported\n");
            break;
        default:
            perror("sys_clone");
    }
    return 1;
}
```

---

## Constants and Flags

### Clone Flags (sys_clone)

```c
#define CLONE_NEWPID    0x20000000
#define CLONE_NEWIPC    0x08000000
#define CLONE_NEWNS     0x00020000
#define CLONE_NEWNET    0x40000000  /* Not supported */
#define CLONE_NEWUSER   0x10000000  /* Not supported */
#define CLONE_NEWUTS    0x04000000  /* Not supported */
#define CLONE_NEWCGROUP 0x02000000  /* Not supported */
```

### Unshare Flags (sys_unshare)

```c
#define UNSHARE_NEWPID  0x20000000
#define UNSHARE_NEWIPC  0x08000000
#define UNSHARE_NEWNS   0x00020000
```

### Namespace Types (sys_setns)

```c
#define NS_TYPE_AUTO   0
#define NS_TYPE_PID    1
#define NS_TYPE_IPC    2
#define NS_TYPE_MOUNT  3
```

---

## Performance Notes

- **sys_clone**: O(1) namespace allocation + O(log n) registry lookup
- **sys_unshare**: O(1) per namespace type + O(log n) registry operations
- **sys_setns**: O(log n) namespace lookup + O(1) reference increment

All operations are thread-safe with minimal lock contention.

---

## Thread Safety

All syscalls and data structures are thread-safe:

- NamespaceRegistry uses Mutex for concurrent access
- AtomicU64 used for ID allocation
- Reference counting is atomic

Multiple threads can safely:
- Call sys_clone, sys_unshare, sys_setns concurrently
- Access and modify ProcessNamespaceContext
- Query namespace existence and membership

---

## Version Information

- **Implementation Version**: 1.0
- **Linux ABI Compatibility**: x86_64 (syscalls 56, 272, 308)
- **Supported Namespaces**: PID, IPC, Mount
- **Status**: Production Ready

---

## Additional Resources

- Linux namespaces man pages: man 7 namespaces
- Clone syscall: man 2 clone
- Unshare syscall: man 2 unshare
- Setns syscall: man 2 setns
- SigmaOS namespace documentation: src/kernel/namespaces.rs
