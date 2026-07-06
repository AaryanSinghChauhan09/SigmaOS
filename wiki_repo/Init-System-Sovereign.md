# Sovereign Init System

SigmaOS uses a custom `no_std`, `no_alloc` init system designed to be fast, reliable, and entirely independent of external libraries like systemd or sysvinit.

## Architecture

The init system is orchestrated as PID 1 and is responsible for:
- Bootstrapping the initial runlevel.
- Parsing simple, non-allocating INI-style service configurations.
- Spawning and monitoring daemons.
- Handling auto-restarts for failed critical services via `SIGCHLD`-like state transitions.

## Runlevels

Runlevels in SigmaOS map to boot targets:
* **0 (Halt):** System shutdown.
* **1 (Single):** Single-user mode (rescue/safe mode), no networking.
* **3 (Multi):** Multi-user mode with networking active. This is the standard default for servers.
* **5 (GUI):** Multi-user mode, networking, and the Zenith Compositor (desktop environment).
* **6 (Reboot):** System reboot.

## Implementation Details
The init system relies on static buffers:
- **`sigma_init.rs`**: Core service state machine (`Down`, `Starting`, `Up`, `Stopping`, `Failed`).
- **`sigma_service.rs`**: Fast text parser extracting `Name`, `Exec`, `Restart`, and `Runlevel` directives.
- **`sigma_runlevel.rs`**: Target state orchestrator.
