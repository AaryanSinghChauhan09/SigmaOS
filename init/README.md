# SigmaOS Init System

This module implements the Boot/Init process (PID 1) of the SigmaOS microkernel architecture.

## Runlevels & Initialization Flow

The boot sequence is structured in 5 key phases:
1. **Core Kernel Bootstrap**: Sets up process tables, memory pagers, and the Round-Robin Scheduler.
2. **HAL & Vitals**: Detects bare-metal hardware and spins up telemetry metrics.
3. **VFS & Storage Mounting**: Mounts the Virtual File System (VFS) and loads the mock `ext4` filesystem drivers.
4. **Network Stack**: Spins up the network interface loopback and prepares standard socket connections.
5. **Userland Handoff**: Spawns the CLI shell (`/usr/bin/sh`) to transition execution to User space.

## Services Lifecycle

Services are declared in a global registry table and started dynamically based on designated runlevels.

```c
typedef enum {
    SERVICE_STOPPED,
    SERVICE_STARTING,
    SERVICE_RUNNING,
    SERVICE_FAILED
} service_state_t;
```text
