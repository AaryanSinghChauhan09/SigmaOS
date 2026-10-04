# Container Runtime

SigmaOS ships a native OCI-compliant container runtime written entirely in Rust. It implements the full OCI Runtime Specification, supports rootless containers, integrates with the kernel's cgroup v2 and namespaces, and provides a Podman-compatible CLI — with lower overhead than runc/crun.

---

## Architecture Overview

```
 ┌─────────────────────────────────────────────────────┐
 │              sigma-ctr  (CLI / Podman compat)        │
 └──────────────────────┬──────────────────────────────┘
                        │ OCI runtime API
 ┌──────────────────────▼──────────────────────────────┐
 │          SigmaContainer Runtime (src/container/)     │
 │  OCI Bundle Parser │ Namespace Setup │ Cgroup Setup  │
 │  Pivot Root        │ Seccomp Filter  │ Capabilities  │
 │  Network Veth      │ Overlay FS      │ Lifecycle Mgr │
 └──────────────────────┬──────────────────────────────┘
                        │ Kernel syscalls
 ┌──────────────────────▼──────────────────────────────┐
 │              Linux Namespaces + Cgroup v2            │
 │  pid │ mnt │ net │ ipc │ uts │ user │ cgroup         │
 └─────────────────────────────────────────────────────┘
```

---

## OCI Runtime (`src/container/oci_runtime.rs`)

Implements the OCI Runtime Specification 1.1:

- **Bundle parsing**: reads `config.json`, validates spec version
- **State machine**: `creating → created → running → stopped`
- **Hooks**: `prestart`, `createRuntime`, `createContainer`, `startContainer`, `poststop`
- **Annotations**: arbitrary key-value metadata on containers

---

## Namespace Isolation

| Namespace | Syscall | Purpose |
|-----------|---------|---------|
| PID | `CLONE_NEWPID` | Isolated process tree |
| Mount | `CLONE_NEWNS` | Independent filesystem view |
| Network | `CLONE_NEWNET` | Private network stack |
| IPC | `CLONE_NEWIPC` | Isolated SysV/POSIX IPC |
| UTS | `CLONE_NEWUTS` | Independent hostname |
| User | `CLONE_NEWUSER` | UID/GID remapping (rootless) |
| Cgroup | `CLONE_NEWCGROUP` | Isolated cgroup hierarchy |

---

## Rootless Containers

SigmaOS supports rootless containers (user namespaces) without any SUID helpers:
- UID mapping: `0 → 100000` in the host namespace
- Newuidmap/newgidmap via `/proc/self/uid_map`
- Rootless networking via `slirp4netns` equivalent in Rust

---

## Filesystem

### Overlay FS
```
Upper layer:  container writes (tmpfs)
Lower layers: image layers (read-only)
─────────────────────────────────────
Merged view:  container's / 
```

### Image Layers
- OCI image spec: manifests, configs, layers (tar+gzip)
- Content-addressed storage: SHA256 digest keyed
- Lazy pulling: layers fetched on demand

---

## Security

### Seccomp
- Default deny-list: 300+ dangerous syscalls blocked
- Custom per-container profiles via `config.json`
- Audit mode: log but allow denied syscalls

### Capabilities
- Default set: 14 standard capabilities (no `CAP_SYS_ADMIN`)
- Drop all then add-back model
- Ambient capabilities for non-root process escalation

### AppArmor / SELinux
- Integration hooks for MAC policy enforcement
- SigmaOS-specific `sigma_container` profile

---

## Networking

| Mode | Description |
|------|-------------|
| `bridge` | veth pair + Linux bridge (default) |
| `host` | Share host network namespace |
| `none` | No network |
| `overlay` | Multi-host networking (Swarm/K8s) |

---

## Resource Limits (Cgroup v2)

```toml
[resources]
cpu_quota_percent = 50
memory_max_mb = 512
memory_swap_max_mb = 0      # disable swap
pids_max = 200
io_weight = 100
```

---

## Comparison vs runc / crun / Docker

| Feature | runc | crun | Docker | **SigmaContainer** |
|---------|------|------|--------|---------------------|
| Language | Go | C | Go | **Rust** |
| OCI 1.1 | ✅ | ✅ | ✅ | ✅ |
| Rootless | ✅ | ✅ | ✅ | ✅ |
| Cgroup v2 | ✅ | ✅ | ✅ | ✅ |
| Seccomp | ✅ | ✅ | ✅ | ✅ |
| Memory safety | ❌ GC | ❌ C | ❌ GC | ✅ Rust |
| AI integration | ❌ | ❌ | ❌ | ✅ |

---

## Source Files

| File | Description |
|------|-------------|
| `src/container/oci_runtime.rs` | OCI runtime spec implementation |
| `src/container/` | Container lifecycle, image store |
| `src/kernel/cgroup_v2_controller.rs` | Cgroup v2 resource controller |
| `src/virt/` | Virtualization primitives |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/container/`, `src/kernel/cgroup_v2_controller.rs`
> - Update OCI spec version when the spec releases new versions
> - Add new namespace types when Linux kernel adds them
> - Keep seccomp syscall count accurate
