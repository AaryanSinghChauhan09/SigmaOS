# Future Development: Init and Services

**Status:** Proposal. Init and service-manager source code does not by itself establish a bootable, supervised service environment.

## Scope

Plan the transition from kernel handoff through userspace startup, service supervision, login, shutdown, and recovery. Current source areas include [`src/init`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/init), [`src/userspace/init.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/userspace/init.rs), and [`src/system/service_manager.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/system/service_manager.rs).

## Linux and BSD ideas to evaluate

| Project | Design idea | SigmaOS question |
|---|---|---|
| systemd (Linux) | Dependency-based units, readiness, and ordered shutdown | Which dependency semantics are essential, and how can unit parsing remain bounded? |
| OpenRC (Alpine/Gentoo) | Explicit runlevels and service scripts | Can startup policy stay readable and deterministic? |
| runit (Void Linux) and s6 | Small supervision loops and explicit service states | Can crash restart, readiness, and logging be implemented with a narrow supervisor? |
| FreeBSD | rc scripts and jails | How should startup integrate with isolated service environments? |

These projects provide design references, not compatibility or support claims.

## Proposed work sequence

1. Specify the boot handoff contract, required initial resources, and behavior if the configured init program is missing or invalid.
2. Define service states, dependencies, readiness, restart limits, timeouts, signal delivery, and shutdown ordering.
3. Keep service definitions declarative and validate them before starting processes; reject cycles and unsupported directives clearly.
4. Route logs and exit status to bounded, inspectable interfaces, with backpressure and storage failure behavior defined.
5. Add a recovery path that remains available when normal service configuration is corrupt.

## Completion criteria

- A boot test demonstrates the actual kernel-to-init handoff on a documented target.
- A service can start, report readiness, fail, restart within configured limits, and stop cleanly through runtime process APIs.
- Invalid dependency graphs and configurations fail before partially starting the service set.
- Shutdown timeouts and forced termination are observable and do not hang indefinitely.
- Recovery behavior is documented and tested independently of normal service startup.

## Maintenance

Link the real boot and process call sites when documenting behavior. Keep implemented, prototype, and proposed service features distinct; remove any claim that is not backed by an integrated runtime path.
