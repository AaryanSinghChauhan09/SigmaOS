# Init System & Service Management — AI Agent Guidelines

**Component Status**: Critical — Currently Missing
**Inspiration**: systemd (Linux), runit (Void Linux), OpenRC (Gentoo), launchd (macOS), rc.d (BSD)

## Overview

The init system is the first userspace process (PID 1) responsible for bootstrapping the operating system, managing services, handling dependencies, and coordinating system shutdown. SigmaOS needs a modern, fast, and secure init system inspired by proven designs.

---

## Linux & BSD Inspiration

### systemd (Linux Standard)
- **Socket activation**: Services start on-demand when clients connect
- **Dependency management**: Units specify Wants=, Requires=, After=, Before=
- **Cgroup integration**: Every service in isolated cgroup for resource control
- **Journal logging**: Structured binary logs with fast indexing
- **Timer units**: Cron replacement with calendar and monotonic triggers

### runit (Void Linux)
- **Simplicity**: Stage 1 (early boot), Stage 2 (services), Stage 3 (shutdown)
- **Per-service supervision**: Each service has dedicated supervisor process
- **Fast parallel startup**: Independent services start simultaneously
- **Automatic restart**: Services respawn on failure with exponential backoff
- **No complex dependencies**: Simple "run" scripts in /etc/sv/

### OpenRC (Gentoo)
- **Parallel service startup**: Uses dependency information for optimal ordering
- **Pluggable init scripts**: Support for different executors
- **BSD-style simplicity**: Easy to understand shell scripts
- **Cgroup support**: Optional cgroup management

### BSD rc.d System
- **rcorder dependency resolution**: Graph-based startup ordering
- **Simple shell scripts**: Each service is a /etc/rc.d/ script
- **Variables in rc.conf**: Central configuration file
- **Clean separation**: Boot (rc) vs service management (rc.d)

---

## Required Features for SigmaOS

### Phase 1: Basic Init System
```rust
// src/init/mod.rs

pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

pub struct Service {
    pub name: String,
    pub exec_start: String,
    pub dependencies: Vec<String>,
    pub restart_policy: RestartPolicy,
    pub state: ServiceState,
}

pub struct InitSystem {
    services: HashMap<String, Service>,
    dependency_graph: DependencyGraph,
}

impl InitSystem {
    pub fn new() -> Self { /* ... */ }
    pub fn register_service(&mut self, service: Service) { /* ... */ }
    pub fn start_service(&mut self, name: &str) -> Result<(), InitError> { /* ... */ }
    pub fn stop_service(&mut self, name: &str) -> Result<(), InitError> { /* ... */ }
    pub fn get_boot_order(&self) -> Vec<&str> { /* ... */ }
}
```

### Phase 2: Supervision & Restart
- Automatic service respawn on crash
- Exponential backoff for failing services
- Maximum restart limit to prevent boot loops
- Health check integration

### Phase 3: Socket Activation
- Services start lazily when socket receives connection
- Reduces boot time and memory usage
- Inspired by systemd socket units

### Phase 4: Logging & Monitoring
- Structured logging (journal-style)
- Service status queries
- Real-time service output streaming

---

## Implementation Priority

| Feature | Priority | Inspiration | Module |
|---------|----------|-------------|--------|
| Basic service manager | P0 (Critical) | runit | `src/init/service_manager.rs` |
| Dependency resolver | P0 (Critical) | rcorder | `src/init/dependencies.rs` |
| Parallel startup | P1 (High) | systemd/OpenRC | `src/init/parallel.rs` |
| Socket activation | P1 (High) | systemd | `src/init/socket_activation.rs` |
| Supervision/restart | P0 (Critical) | runit | `src/init/supervisor.rs` |
| Structured logging | P2 (Medium) | journald | `src/init/journal.rs` |
| Timer units | P2 (Medium) | systemd | `src/init/timers.rs` |
| Cgroup integration | P1 (High) | systemd | `src/init/cgroups.rs` |

---

## Development Guidelines

### Bolt ⚡ (Performance Agent)
- Optimize dependency graph traversal (topological sort in O(V+E))
- Use async I/O for service output capture
- Minimize fork overhead in service spawning
- Parallel service startup for independent services

### Sentinel 🛡️ (Security Agent)
- Services run in separate cgroups with resource limits
- Capability dropping before service exec
- Seccomp filters per service type
- No services run as root by default

### Palette 🎨 (UX Agent)
- Clear service status output (`sigmactl status servicename`)
- Boot progress visualization
- Easy service enable/disable commands
- Human-readable log output

---

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_resolution() {
        let mut init = InitSystem::new();
        
        let network = Service::new("network", vec![]);
        let dns = Service::new("dns", vec!["network"]);
        let web = Service::new("web", vec!["network", "dns"]);
        
        init.register_service(network);
        init.register_service(dns);
        init.register_service(web);
        
        let order = init.get_boot_order();
        assert_eq!(order, vec!["network", "dns", "web"]);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut init = InitSystem::new();
        
        let a = Service::new("a", vec!["b"]);
        let b = Service::new("b", vec!["a"]);
        
        init.register_service(a);
        let result = init.register_service(b);
        
        assert!(result.is_err()); // Should detect cycle
    }
}
```

---

## Integration Points

### With Kernel
- Process spawning via `fork()` + `exec()`
- Signal handling for service termination
- Cgroup creation and management
- Resource limit enforcement

### With Security
- Capability management
- Seccomp filter application
- Namespace isolation
- Pledge/unveil restrictions

### With Networking
- Socket activation for network services
- Firewall rule application on service start
- DNS resolution before dependent services

---

## Service Definition Format

Inspired by systemd units but simpler:

```ini
[Service]
Name=sshd
Description=OpenSSH Daemon
ExecStart=/usr/bin/sshd -D
Restart=on-failure
RestartSec=5
Dependencies=network.service

[Resources]
MemoryMax=512M
CPUQuota=50%

[Security]
Capabilities=CAP_NET_BIND_SERVICE
ReadOnlyPaths=/usr /etc
ReadWritePaths=/var/run/sshd
```

---

## File Locations

| Path | Purpose |
|------|---------|
| `src/init/mod.rs` | Main init system module |
| `src/init/service_manager.rs` | Service lifecycle management |
| `src/init/dependencies.rs` | Dependency graph resolution |
| `src/init/supervisor.rs` | Process supervision and restart |
| `src/init/socket_activation.rs` | Socket-based service activation |
| `/etc/sigma/services/` | Service definition files |
| `/var/run/sigma/` | Runtime state and PID files |

---

## References

- [systemd Documentation](https://www.freedesktop.org/wiki/Software/systemd/)
- [runit Design](http://smarden.org/runit/benefits.html)
- [OpenRC User Guide](https://wiki.gentoo.org/wiki/OpenRC)
- [BSD rc.d System](https://man.openbsd.org/rc.d)
- [Unix Process Supervision](https://cr.yp.to/daemontools.html)

---

## Current Status

- ❌ No init system implemented
- ❌ Services started manually
- ❌ No dependency management
- ❌ No automatic restart on failure
- ❌ No structured logging

**Next Steps**: Implement basic service manager with dependency resolution (Phase 1)

---

*Last Updated: October 2026*
*Agent Guidelines for Init System Development*
