# Service Management Contract

> **Shared SigmaOS platform contract.** Define the service state machine and lifecycle shared across Linux (systemd) and BSD (rc.d) backends.

---

## Interface

ServiceManager { start(name) -> result; stop(name) -> result; restart(name) -> result; status(name) -> ServiceState; list() -> [ServiceInfo]; }

## States

Idle -> Starting -> Started -> Stopping -> Stopped | Failed -> Restarting -> Started | Degraded

## Policies

- Restart policy: on-failure (default), always, or never. Configured per-service in service manifest.
- Dependency graph: services declare `requires = [...]`; started in topological order.
- Health checks: services may declare a health-check command; failed checks trigger Degraded state.
- Quiesce: a service can be asked to drain (quiesce) before stop, allowing in-flight requests to complete.

## Linux Adapter

systemd unit files; systemctl start/stop/status; journalctl for logs.

## BSD Adapter

rc.d scripts; service(8); /var/log for logs. rc.conf enables services at boot.

## Failure Modes and Recovery

Service fails to start (config error, missing dep, port conflict). Recovery: log error, enter Failed state, retry per restart policy, alert if still failing after N retries.

## Open Questions

User-level services (per-session) vs system services — how are they scoped? Current: separate registries; user services start at login.
