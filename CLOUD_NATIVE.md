# SigmaOS Cloud Native — sigma-nebula Architecture

> **Status**: ✅ Implemented — prototype in `userland/nebula/sigma_nebula.nim`  
> **Language**: Nim (freestanding, OOP)  
> **Branch**: `feature/multi-lang-impl-batch1`

---

## Overview

`sigma-nebula` is SigmaOS's sovereign cloud orchestration daemon. It implements a lightweight, OCI-compatible container runtime without relying on external runtimes (no Docker, no containerd). All container lifecycle management is implemented from first principles.

## Architecture

```
sigma-nebula
├── ContainerRuntime (abstract base)
│   └── SovereignContainer (concrete — lifecycle state machine)
└── ContainerScheduler (composition — multi-container placement)
```

## Container Lifecycle

```
Created → Running → Paused → Running → Stopped → Exited
                                     → Failed
```

## Resource Management

Each container has explicit resource caps enforced at creation:

| Field | Type | Description |
| :--- | :--- | :--- |
| `maxMemoryMb` | `u32` | Memory cap in MB |
| `maxCpuMillis` | `u32` | CPU cap in millicores |
| `maxPids` | `u16` | Max process count |
| `maxFdCount` | `u16` | Max open file descriptors |

## Implementation Files

| File | Language | Description |
| :--- | :--- | :--- |
| `userland/nebula/sigma_nebula.nim` | Nim | Core container orchestration daemon |

## OOP Design

- **Base**: `ContainerRuntime ref object of RootObj` — abstract init/shutdown
- **Derived**: `SovereignContainer` — state machine + PID tracking
- **Composition**: `ContainerScheduler` — array of up to 64 containers, round-robin scheduling

## API (inter-daemon IPC)

```
sigma-nebula run   <image-hash>   [--mem=<MB>] [--cpu=<millicores>]
sigma-nebula stop  <container-id>
sigma-nebula pause <container-id>
sigma-nebula list
```

## Test Coverage

```nim
proc testContainerLifecycle*(): bool
# Created → Running → Paused → Resumed → Stopped
```

## Future Work

- [ ] Namespace isolation (via `sigma_clone()` syscall)
- [ ] cgroup v2 resource enforcement
- [ ] OCI image layer mounting via SigmaFS
- [ ] Multi-node scheduling via sigma-nexus cluster bus
