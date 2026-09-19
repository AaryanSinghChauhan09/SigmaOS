# SigmaOS Sovereign BSD Jails — OS-Level Virtualization

## Overview

SigmaOS implements **FreeBSD Jails-style lightweight OS virtualization** in 100% safe Rust (`src/kernel/bsd_jails_sovereign.rs`). FreeBSD Jails (since FreeBSD 4.0, year 2000) pioneered the concept that later inspired Docker/containers.

Each jail provides:
- **Isolated filesystem root** (like `chroot` but stronger)
- **Independent hostname** and IP address assignment
- **Separate process namespace** — processes cannot see outside their jail
- **Syscall enforcement** — per-jail allowlist of privileged operations
- **OpenBSD securelevel** integration (−1 to +2)

## Why BSD Jails?

| Feature | Docker/OCI | BSD Jails (SigmaOS) |
|---------|-----------|---------------------|
| Kernel overhead | ~10-100MB container runtime | Near-zero (pure kernel) |
| Dependencies | containerd, runc, libcontainer | None — pure Rust |
| Isolation model | Linux namespaces + cgroups | Native kernel jail table |
| Network | veth pairs + iptables | Direct IP assignment |
| Nested jails | Partial (rootless) | Native hierarchy |

## Architecture

```
SovereignBsdJailManager
├── jail[1] web    /jails/web    10.0.0.1  [httpd, php-fpm]
├── jail[2] db     /jails/db     10.0.0.2  [postgres]
├── jail[3] mail   /jails/mail   10.0.0.3  [postfix, dovecot]
└── jail[4] cache  /jails/cache  10.0.0.4  [redis]
```

## Key Types

### `SovereignBsdJailManager`

```rust
let mut mgr = SovereignBsdJailManager::new(64); // max 64 jails
let jid = mgr.create_jail("web", "/jails/web", "web.sigmaos.local").unwrap();
mgr.start_jail(jid);

let jail = mgr.get_mut(jid).unwrap();
jail.network.add_ipv4([10, 0, 0, 1]);
jail.permissions = JailPermissions::relaxed();

// Attach process
let proc = JailProcess { pid: 1001, ppid: 1, name: "nginx".to_string(), uid: 80, gid: 80 };
jail.attach_process(proc);

// Syscall enforcement
assert!(!jail.check_syscall("mount")); // Blocked by secure defaults
```

### `JailPermissions`

Mirrors FreeBSD `jail(8)` parameters:

```rust
pub struct JailPermissions {
    pub allow_mount: bool,        // jail.allow.mount
    pub allow_sysvipc: bool,      // jail.allow.sysvipc
    pub allow_raw_sockets: bool,  // jail.allow.raw_sockets
    pub allow_set_hostname: bool, // jail.allow.set_hostname
    pub securelevel: i8,          // OpenBSD securelevel (-1..2)
    pub devfs_ruleset: u32,       // devfs ruleset number
    // ... more
}
```

### `JailNetworkConfig`

```rust
let mut net = JailNetworkConfig::new("web.sigmaos.local");
net.add_ipv4([192, 168, 1, 100]);
net.allow_raw_sockets = false; // default secure
```

## Jail Lifecycle

```
Creating → start() → Running → teardown() → Dead
```

## FreeBSD Parity

| FreeBSD jail(8) | SigmaOS Equivalent |
|----------------|-------------------|
| `jail -c path=/jails/web` | `mgr.create_jail("web", "/jails/web", host)` |
| `jail.allow.mount` | `JailPermissions::allow_mount` |
| `jail.allow.sysvipc` | `JailPermissions::allow_sysvipc` |
| `jls` | `mgr.jls()` |
| `jail -r jid` | `mgr.teardown_jail(jid)` |
| OpenBSD `kern.securelevel` | `JailPermissions::securelevel` |

## Tests

6 unit tests, all passing:

- `test_jail_create_start` — create and start jail lifecycle
- `test_jail_process_attach` — attach process to running jail
- `test_jail_syscall_enforcement` — mount/sysvipc blocked, violation counting
- `test_jail_max_limit` — cannot exceed max jails
- `test_jail_teardown` — teardown clears processes, sets Dead state
- `test_network_config` — per-jail IPv4 address assignment

## Source

[`src/kernel/bsd_jails_sovereign.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/kernel/bsd_jails_sovereign.rs)
