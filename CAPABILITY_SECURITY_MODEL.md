# SigmaOS Capability-Based Security Model

## Overview
SigmaOS implements a seL4/QNX-inspired capability-based security model as its primary access control mechanism, replacing traditional ACL tables with **unforgeable kernel-issued tokens**.

## Core Concepts

| Concept | Description |
| :--- | :--- |
| **Capability Token** | An opaque handle issued by the kernel, granting rights to a specific resource (memory page, IPC port, device I/O). |
| **Unforgeability** | Capabilities are XOR-masked with a kernel secret at mint time, making them impossible for user-space to forge. |
| **Delegation** | A process holding `CAP_GRANT` rights may delegate its capability to another via the IPC subsystem. |
| **Revocation** | The kernel can instantly nullify any capability by clearing its rights bitmask (e.g. when the resource is freed). |
| **Isolation** | A process cannot address memory it holds no capability for — preventing buffer overflows at the architectural level. |

## Rights Bitmask

```c
#define CAP_READ    0x01  // Read memory/resource
#define CAP_WRITE   0x02  // Write memory/resource
#define CAP_EXECUTE 0x04  // Execute code in this region
#define CAP_GRANT   0x08  // Delegate capability to another process
```

## Integration Points
- **Memory Paging**: Every `map_virtual_page()` call produces a capability token. No page is accessible without it.
- **Scheduler Hook**: The `scheduler_tick()` verifies `CAP_EXECUTE` on the process's active code segment before dispatching it.
- **Sandbox**: Combined with sandboxing, limits the total number of capabilities a process can hold.

## Source Files
- `modules/security/capabilities/caps.c` — Capability minting, checking, and revocation.
- `modules/core/kernel/scheduler.c` — Scheduler with integrated capability verification.
