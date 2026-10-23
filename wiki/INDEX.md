# SigmaOS Master Documentation Index

Welcome to the central documentation index for SigmaOS architecture, system development plans, and engineering guidelines.

## Userspace & CLI Plans

- [SigmaOS CLI Development Plan](27-SigmaOS-CLI-Development-Plan.md) — Two distinct CLI layers (system CLI utilities vs shell engine), POSIX compatibility baseline, 18-PR roadmap, maintenance cadence, command family rules, and rollout gates.
- [System Utilities Development Plan](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md) — Master architecture for POSIX system utilities, utility registry framework, and file/stream engines.

## Kernel & Core Subsystem Plans

- [IPC Handler Master Architecture & Specification Plan](ipc-handler.md) — Formally specified, testable IPC handler subsystem covering pipes, FIFOs, UNIX sockets, shared memory, message queues, event wakeups, signal control plane, FD passing, and access control policies.
