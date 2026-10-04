# Init and Services

**Capability state: Proposed.** There is no verified boot-to-init or service supervision path. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

The repository contains service-management models, but no demonstrated PID 1, process spawning, dependency ordering, restart policy, or shutdown path. In-memory service state is not a running service manager.

## Design references

Study runit and s6 for small supervision contracts, OpenRC for dependency-based startup, and systemd for service isolation and operational tooling. Keep the first design small and observable.

## Roadmap

1. Establish process creation, exit reporting, and reliable reaping.
2. Implement a minimal init that starts a console shell and reports startup failures.
3. Add declarative service dependencies and bounded restart behavior with logs.
4. Test dependency failure, crash loops, shutdown, and recovery on QEMU.

**Completion evidence:** boot logs show init starting real processes; failure injection proves dependent services fail clearly and do not leave orphaned processes.
