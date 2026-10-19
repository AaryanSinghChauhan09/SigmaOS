# SigmaOS Jails/Container Components

## Overview
SigmaOS jail/container system providing lightweight OS-level virtualization, competitive with FreeBSD Jails, OpenBSD pledge, and Linux containers (Docker/LXC).

## Features
- **Lightweight OS-level virtualization** (like FreeBSD Jails)
- **Process and filesystem isolation**
- **Network isolation** with virtual interfaces
- **Resource limits** (CPU, memory, disk)
- **Secure defaults** with capability-based access
- **Process isolation** (like FreeBSD Jails)
- **Component isolation** (like OpenBSD pledge)

## Structures

| Structure | Description |
|-----------|-------------|
| **JailManager** | Jail manager with create/start/stop/destroy |
| **Jail** | Jail instance with config and state |
| **JailConfig** | Jail configuration (name, hostname, root_path, limits) |
| **JailState** | Creating, Running, Stopped, Destroying, Error |

## Jail States

| State | Description |
|-------|-------------|
| **Creating** | Jail is being created |
| **Running** | Jail is running |
| **Stopped** | Jail is stopped |
| **Destroying** | Jail is being destroyed |
| **Error** | Jail is errored |

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Container/Virtualization (P1)

## Related Files
- `src/jail/manager.rs` - Jail manager implementation
- `wiki/Jails-Containers.md` - Wiki documentation

## Principles Followed
PROCESS_ISOLATION, COMPONENT_ISOLATION, SINGLE_RESPONSIBILITY, MINIMAL_KERNEL_PHILOSOPHY,
MODULARITY, SECURE_DEFAULTS, LEAST_PRIVILEGE, INFORMATION_HIDING, ENCAPSULATION,
DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM