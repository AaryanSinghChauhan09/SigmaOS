# SigmaOS Kernel Namespaces

## Overview

SigmaOS implements Linux-compatible kernel namespaces for process isolation. Used by container runtimes (like Docker/podman equivalents on SigmaOS).

**Location:** `src/kernel/sigma_namespaces.rs`

---

## Supported Namespace Types

| Type | Flag | Isolates |
|------|------|---------|
| PID | `CLONE_NEWPID` | Process ID space |
| Network | `CLONE_NEWNET` | Network stack |
| Mount | `CLONE_NEWNS` | Filesystem mounts |
| UTS | `CLONE_NEWUTS` | Hostname + domainname |
| IPC | `CLONE_NEWIPC` | SysV IPC, POSIX mq |
| User | `CLONE_NEWUSER` | UID/GID mapping |
| Cgroup | `CLONE_NEWCGROUP` | cgroup root |
| Time | `CLONE_NEWTIME` | Clock offsets |

---

## API Reference

```rust
let mut mgr = SigmaNamespaceManager::new();

// Register process (uses initial/host namespaces)
mgr.register_process(1000);

// Unshare UTS namespace (like: unshare --uts)
mgr.unshare(1000, NamespaceType::Uts.clone_flag()).unwrap();

// Set isolated hostname
mgr.sethostname(1000, "my-container").unwrap();

// Unshare multiple namespaces at once
let flags = NamespaceType::Uts.clone_flag()
    | NamespaceType::Pid.clone_flag()
    | NamespaceType::Ipc.clone_flag();
mgr.unshare(1000, flags).unwrap();

// UID mapping (rootless containers)
let mut user_ns = UserNamespace::new_child(1000, &parent_ns);
user_ns.add_uid_map(0, 1000, 65536).unwrap(); // container root → host uid 1000
user_ns.ns_uid_to_host(0) // → Some(1000)
```

---

## Container Isolation Example

```rust
// Create container process with full isolation
mgr.register_process(container_pid);
mgr.unshare(container_pid,
    NamespaceType::Pid.clone_flag() |
    NamespaceType::Net.clone_flag() |
    NamespaceType::Mnt.clone_flag() |
    NamespaceType::Uts.clone_flag() |
    NamespaceType::User.clone_flag()
).unwrap();
mgr.sethostname(container_pid, "container-1").unwrap();
```

---

## Comparison

| Feature | Linux | FreeBSD Jails | SigmaOS |
|---------|-------|--------------|---------|
| PID ns | Yes | Partial | Yes |
| Net ns | Yes | Yes (VNET) | Yes |
| UTS ns | Yes | Yes | Yes |
| User ns | Yes | No | Yes |
| UID mapping | Yes | No | Yes |
| no_std | No | No | **Yes** |
