# SigmaOS Master Documentation Index

Welcome to the central documentation index for SigmaOS architecture, system development plans, and engineering guidelines.

## Userspace & CLI Plans

- [SigmaOS CLI Development Plan](27-SigmaOS-CLI-Development-Plan.md) — Two distinct CLI layers (system CLI utilities vs shell engine), POSIX compatibility baseline, 18-PR roadmap, maintenance cadence, command family rules, and rollout gates.
- [System Utilities Development Plan](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md) — Master architecture for POSIX system utilities, utility registry framework, and file/stream engines.

## Kernel & Core Subsystem Plans

- [Physical Memory Manager Plan](PHYSICAL_MEMORY_MANAGER_PLAN.md) — Two-stage early-boot to runtime frame allocator, 11 invariants, 20-PR sequence roadmap, and memory protection.
- [Process Manager Architecture, Support Matrix, and PR Plan](PROCESS_MANAGER_PLAN.md) — Process manager architecture, lifecycle rules, 18-PR sequence roadmap, periodic maintenance cadences, and completion gates.
## AI Agent Roadmaps & Distro Parity Specifications

- [Omarchy Linux & Sovereign Desktop Parity AI Agent Roadmap](40-Omarchy-Linux-And-Sovereign-Desktop-Parity-AI-Agent-Roadmap.md) — Desktop tiling compositing, Waybar status HUD, Ghostty terminal, Walker launcher, Wallust color sync, and PAM security roadmap.
- [Linux & BSD Kernel, Security, and Virtualization AI Agent Roadmap](41-Linux-BSD-Kernel-Security-And-Virtualization-AI-Agent-Roadmap.md) — Linux sched_ext BPF, FreeBSD Capsicum, OpenBSD Pledge/Unveil, Landlock V6, eBPF zero-trust, and MicroVM roadmap.
## Kernel & Core Subsystem Plans

- [Process Synchronization Plan](PROCESS_SYNCHRONIZATION_PLAN.md) — Master Architecture & Support Matrix Plan for SigmaOS process synchronization: 5-stage architecture progression, 10 core design rules, 17-PR roadmap, maintenance cadences, component rules, PR review template, and completion gates.
- [IPC Handler Master Architecture & Specification Plan](ipc-handler.md) — Formally specified, testable IPC handler subsystem covering pipes, FIFOs, UNIX sockets, shared memory, message queues, event wakeups, signal control plane, FD passing, and access control policies.
- [CPU Scheduler Development Plan](SCHEDULER_DEVELOPMENT_PLAN.md) — CPU Scheduler development roadmap, 18-PR sequence, recommended initial milestone, periodic maintenance cadences, and merge gates.
- [CPU Scheduler Status & Invariants](SCHEDULER_STATUS_AND_INVARIANTS.md) — Subsystem architecture status matrix, task state transitions, and 9 core policy and concurrency invariants.
