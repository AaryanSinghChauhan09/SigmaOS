# Sovereign Init System (`sigma-init`)

`sigma-init` is the boot orchestrator and service manager for SigmaOS, absorbing the best ideas from systemd, OpenRC, and runit while remaining zero-dependency and minimal.

## Boot Stages

Booting is divided into 5 distinct, sequential stages:
1. `FIRMWARE` (UEFI/BIOS handoff)
2. `KERNEL` (VMM, Process Manager)
3. `DRIVERS` (Device Manager, Filesystem)
4. `SERVICES` (Network, IPC, Recovery)
5. `USERLAND` (Shell, Zenith Desktop)

## Dependency Resolution

Services declare explicit dependencies using a topological sort algorithm to ensure correct startup order. If service B depends on service A, `sigma-init` guarantees A is fully `RUNNING` before attempting to start B.

## Service Management

Services can be registered with one of three restart policies:
- `RESTART_NEVER`
- `RESTART_ON_FAILURE`
- `RESTART_ALWAYS`

Critical services (like `sigma-sandbox`) will halt the boot process or trigger a kernel panic if they fail to start.
