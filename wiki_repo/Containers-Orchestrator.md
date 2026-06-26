# Containers & Orchestrator

Lightweight sovereign pods — no Docker/containerd.

## CLI

```bash
sigma-pod run <package.spkg>
sigma-pod run-native <package.spkg> --all-ns --cpu=250 --mem=128
sigma-pod list
sigma-pod stop <id>
```

Source: `userland/tools/sigma_pod_cli.cpp`

## Kernel

- Orchestrator: `kernel/core/orchestrator/sigma_orchestrator.cpp`
- Cgroups: `kernel/core/process/sigma_cgroup.c`
- Spec: `include/sigma_pod_spec.h`

## Logging

Pod events ring-buffer → `/var/log/sigma_pod.log` (VFS persistence when available).

## vs RancherOS / CoreOS

SigmaOS pods use IPC + namespace flags + cgroup limits without pulling a container runtime stack.
