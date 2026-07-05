# SigmaOS Init System (`sigma-init`)

This module implements **PID 1** — the first process spawned by the kernel after
boot, responsible for orchestrating every subsequent subsystem start-up.

## Runlevels & Initialization Flow

The boot sequence is structured in 5 key phases:

1. **Core Kernel Bootstrap**: Sets up process tables, memory pagers, and the
   Round-Robin / EDF Scheduler.
2. **HAL & Vitals**: Detects bare-metal hardware (PCIe, USB, ACPI) and spins up
   telemetry metrics.
3. **VFS & Storage Mounting**: Mounts the Virtual File System (VFS) and loads the
   `ext4` / `SovereignFS` filesystem drivers.
4. **Network Stack**: Spins up the network interface loopback and prepares
   standard socket connections.
5. **Userland Handoff**: Spawns the CLI shell (`/usr/bin/sh`) to transition
   execution to user space.

## Services Lifecycle

Services are declared in a global registry table and started dynamically based
on designated runlevels.

```c
typedef enum {
    SERVICE_STOPPED,
    SERVICE_STARTING,
    SERVICE_RUNNING,
    SERVICE_FAILED
} service_state_t;

typedef struct {
    const char    *name;
    service_state_t state;
    int (*start)(void);
    int (*stop)(void);
} sigma_service_t;
```

## Key Entry Points

| Symbol | File | Purpose |
|---|---|---|
| `sigma_init_main()` | `init/sigma_init.c` | PID 1 entry point |
| `sigma_run_level(n)` | `init/runlevel.c` | Transition to runlevel `n` |
| `sigma_service_start(svc)` | `init/service.c` | Start an individual service |
| `sigma_service_stop(svc)` | `init/service.c` | Cleanly stop a service |

## Service Registry

Services register themselves at compile time via a linker section macro:

```c
SIGMA_SERVICE_REGISTER(my_service, .start = my_start, .stop = my_stop);
```

## Roadmap

- [x] Basic 5-phase boot sequence
- [x] Service registry table (compile-time)
- [ ] Dynamic service dependencies (DAG-based ordering)
- [ ] Parallel service startup (topological sort)
- [ ] Service restart policies (always / on-failure / never)
- [ ] Watchdog integration (`modules/core/kernel/watchdog.rs`)
- [ ] Journal-based service logs (`modules/tools/diag/logger.rs`)
