# OSS Absorption: systemd

## Overview

systemd is the dominant init system and service manager in the Linux ecosystem. It introduced powerful concepts like dependency-based parallel booting, socket activation, and cgroup integration, replacing the legacy SysVinit scripts.

## Key Principles Absorbed

### Dependency-Based Service Ordering

- systemd models services and targets as a Directed Acyclic Graph (DAG).
- SigmaOS absorbs this through `sigma_init::DependencyGraph`, a native Rust topological sort engine.
- Instead of text-based `.service` files, SigmaOS defines services as strongly-typed `Service` structs, preventing syntax errors at compile time.

### Socket Activation

- systemd allows services to be launched lazily on-demand when a connection to a specific socket is received.
- `sigma_init` absorbs this natively via `Service::socket_activation(port)`.
- The `sigma_init` daemon listens on the port and transitions the service from `Stopped` to `Running` when the first bytes arrive.

### Target States

- Instead of runlevels, systemd uses targets.
- SigmaOS translates these into native declarative profiles within the `sigma_init::SigmaInit` boot configuration.

## Displaced Technologies

| Technology | SigmaOS Replacement |
| --- | --- |
| systemd (PID 1) | `sigma_init::SigmaInit` |
| `.service` files | `sigma_init::Service` (Native Rust) |
| `systemctl` | `sigma_init::ServiceManager` API |
| `journald` | `sigma_log` (Planned structured binary logging) |

## Status

**Core Absorbed** — The dependency DAG and socket activation primitives have been successfully implemented natively in `userland/sigma_init/src/`.
