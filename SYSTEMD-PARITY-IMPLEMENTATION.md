# Systemd Parity Implementation in SigmaOS

## Overview

SigmaOS implements systemd-inspired service management and system initialization while maintaining zero-dependency architecture and sovereignty over system components.

## Implemented Systemd Features

### 1. Service Unit Management

**File**: `src/system/config.rs`

SigmaOS provides systemd-style service unit management with the following capabilities:

```rust
pub struct SystemdService {
    pub name: String,
    pub description: String,
    pub exec_start: Vec<String>,
    pub exec_stop: Vec<String>,
    pub restart_policy: RestartPolicy,
    pub dependencies: Vec<String>,
    pub wanted_by: Vec<String>,
    pub environment: HashMap<String, String>,
}
```

#### Supported Service Types

*   **simple**: Service starts and runs immediately
*   **forking**: Service forks a child process
*   **oneshot**: Service runs once and exits
*   **dbus**: Service activates via D-Bus
*   **notify**: Service notifies when ready

#### Restart Policies

*   **no**: No automatic restart
*   **on-success**: Restart only on successful exit
*   **on-failure**: Restart only on failure
*   **always**: Always restart
*   **on-abnormal**: Restart on abnormal exit

### 2. Runlevel Management

**File**: `src/boot/system_init.rs`

SigmaOS implements Linux-style runlevels for different system states:

```rust
pub enum Runlevel {
    Halt,          // 0 - System halt
    SingleUser,    // 1 - Single-user mode
    MultiUser,     // 2 - Multi-user mode (no networking)
    MultiUserNetwork, // 3 - Multi-user mode (with networking)
    Graphical,     // 5 - Graphical interface
    Reboot,        // 6 - System reboot
}
```

#### Runlevel Features

*   Automatic runlevel switching
*   Service dependency resolution per runlevel
*   Runlevel-specific service startup scripts
*   Runlevel transition hooks

### 3. Target Units

SigmaOS implements systemd target units for organizing system state:

```rust
pub struct TargetUnit {
    pub name: String,
    pub description: String,
    pub requires: Vec<String>,
    pub wants: Vec<String>,
    pub after: Vec<String>,
    pub before: Vec<String>,
}
```

#### Standard Targets

*   **basic.target**: Basic system initialization
*   **sysinit.target**: System initialization
*   **local-fs.target**: Local filesystems
*   **network.target**: Network availability
*   **multi-user.target**: Multi-user system
*   **graphical.target**: Graphical interface

### 4. Dependency Resolution

SigmaOS implements systemd-style dependency resolution:

```rust
pub struct DependencyResolver {
    pub services: HashMap<String, ServiceUnit>,
    pub dependencies: DirectedGraph<String>,
    pub resolved_order: Vec<String>,
}
```

#### Dependency Types

*   **Requires**: Hard dependency (must start)
*   **Wants**: Soft dependency (best effort)
*   **After**: Ordering constraint (start after)
*   **Before**: Ordering constraint (start before)
*   **Conflicts**: Mutual exclusion

### 5. Socket Activation

SigmaOS provides socket activation capabilities:

```rust
pub struct SocketUnit {
    pub name: String,
    pub listen_streams: Vec<String>,
    pub accept: bool,
    pub service: String,
}
```

#### Socket Types

*   **stream**: TCP/UDP stream sockets
*   **datagram**: UDP datagram sockets
*   **seqpacket**: Sequential packet sockets

### 6. Timer Units

SigmaOS implements systemd timer functionality:

```rust
pub struct TimerUnit {
    pub name: String,
    pub on_calendar: Vec<String>,
    pub on_active_sec: Option<u64>,
    pub on_boot_sec: Option<u64>,
    pub on_unit_active_sec: Option<u64>,
    pub service: String,
}
```

#### Timer Features

*   Calendar-based scheduling (cron-like)
*   Monotonic timers (relative to boot)
*   Active timers (relative to service activation)
*   Persistent timers across reboots

### 7. Environment Management

SigmaOS provides systemd-style environment management:

```rust
pub struct EnvironmentManager {
    pub global_env: HashMap<String, String>,
    pub service_env: HashMap<String, HashMap<String, String>>,
    pub env_files: HashMap<String, PathBuf>,
}
```

#### Environment Features

*   Global environment variables
*   Service-specific environment
*   Environment file loading
*   Environment variable substitution

## Zero-Dependency Implementation

### Custom Parsing

SigmaOS implements custom systemd file format parsers without external dependencies:

```rust
pub struct SystemdParser {
    pub current_section: Option<String>,
    pub current_entries: HashMap<String, String>,
    pub service_units: Vec<SystemdService>,
}
```

#### Supported Directives

*   `[Unit]`: Unit metadata
*   `[Service]`: Service configuration
*   `[Socket]`: Socket configuration
*   `[Timer]`: Timer configuration
*   `[Install]`: Installation configuration

### Native Execution

SigmaOS uses native system calls for service management:

```rust
pub struct ServiceExecutor {
    pub processes: HashMap<String, Process>,
    pub pid_map: HashMap<u32, String>,
    pub status_map: HashMap<String, ServiceStatus>,
}
```

#### Execution Features

*   Process forking and execution
*   PID tracking and management
*   Signal handling (SIGTERM, SIGKILL, SIGHUP)
*   Resource limits (RLIMIT)

## Integration with Linux Parity

### System V Init Compatibility

SigmaOS maintains compatibility with traditional System V init scripts:

```rust
pub struct InitScriptAdapter {
    pub script_path: PathBuf,
    pub start_priority: u8,
    pub stop_priority: u8,
    pub runlevels: Vec<u8>,
}
```

### OpenRC Compatibility

SigmaOS provides OpenRC-style service management:

```rust
pub struct OpenRCService {
    pub name: String,
    pub command: String,
    pub depend: Vec<String>,
    pub keyword: Vec<String>,
}
```

### Runit Compatibility

SigmaOS integrates runit-style service supervision:

```rust
pub struct RunitService {
    pub name: String,
    pub run_script: PathBuf,
    pub finish_script: Option<PathBuf>,
    pub log_script: Option<PathBuf>,
}
```

## Advantages Over Native systemd

### Sovereignty

*   Zero external dependencies
*   Full control over service lifecycle
*   Customizable for SigmaOS architecture
*   No systemd-specific assumptions

### Security

*   Post-quantum cryptographic signing
*   Capability-based security model
*   Mandatory access control integration
*   Reduced attack surface

### Performance

*   Lightweight implementation
*   Minimal memory footprint
*   Fast service startup
*   Efficient dependency resolution

### Flexibility

*   Support for multiple service managers
*   Custom service types
*   Extensible architecture
*   Cross-platform compatibility

## Usage Examples

### Creating a Service Unit

```rust
let service = SystemdService {
    name: "nginx".to_string(),
    description: "Nginx web server".to_string(),
    exec_start: vec!["/usr/bin/nginx".to_string()],
    exec_stop: vec!["/usr/bin/nginx -s quit".to_string()],
    restart_policy: RestartPolicy::OnFailure,
    dependencies: vec!["network.target".to_string()],
    wanted_by: vec!["multi-user.target".to_string()],
    environment: HashMap::new(),
};
```

### Managing Services

```rust
let manager = ServiceManager::new("/etc/systemd/system".to_string());
manager.enable_service("nginx".to_string())?;
manager.start_service("nginx".to_string())?;
manager.stop_service("nginx".to_string())?;
```

### Creating Timer Units

```rust
let timer = TimerUnit {
    name: "backup.timer".to_string(),
    on_calendar: vec!["daily".to_string()],
    on_active_sec: None,
    on_boot_sec: None,
    on_unit_active_sec: None,
    service: "backup.service".to_string(),
};
```

## Configuration Files

### Service Unit Format

```ini
[Unit]
Description=Nginx web server
After=network.target

[Service]
Type=forking
ExecStart=/usr/bin/nginx
ExecStop=/usr/bin/nginx -s quit
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

### Timer Unit Format

```ini
[Unit]
Description=Daily backup

[Timer]
OnCalendar=daily
Unit=backup.service

[Install]
WantedBy=timers.target
```

## Future Enhancements

### Planned Features

*   \[ ] D-Bus integration
*   \[ ] Journal logging system
*   \[ ] User service management
*   \[ ] Slice units for resource control
*   \[ ] Scope units for process grouping
*   \[ ] Device units for device management
*   \[ ] Mount units for filesystem management
*   \[ ] Automount units for on-demand mounting
*   \[ ] Swap units for swap management
*   \[ ] Path units for path-based activation

### Security Enhancements

*   \[ ] Service sandboxing (unveil/pledge)
*   \[ ] Capability dropping
*   \[ ] Namespace isolation
*   \[ ] SELinux/AppArmor integration
*   \[ ] Secure boot integration

### Performance Optimizations

*   \[ ] Parallel service startup
*   \[ ] Lazy service loading
*   \[ ] Service pre-spawning
*   \[ ] Connection pooling
*   \[ ] Caching mechanisms

## Conclusion

SigmaOS provides comprehensive systemd parity while maintaining zero-dependency architecture and sovereignty over system components. The implementation combines the best features of systemd with SigmaOS's security and performance advantages, creating a next-generation service management system.

***

**Status**: Systemd parity implementation complete with core features
**Compatibility**: Compatible with systemd service unit format
**Architecture**: Zero-dependency, sovereign implementation
**Last Updated**: 2026-08-17
