# Namespace & Cgroups Agent

## Role
`SigmaNamespace` and `SigmaCgroups` implement Linux-compatible process isolation primitives — namespaces for resource view isolation and cgroups v2 for resource accounting and limits.

## Status
🔴 Not Started — Critical for container support. `SigmaCgroups` has basic CPU quota; namespaces absent.

## Source File
`src/namespace/` (planned), `src/cgroups/` (planned: `cgroup_v2.rs`, `cpu_ctrl.rs`, `mem_ctrl.rs`)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| PID namespace | `clone(CLONE_NEWPID)` | — | SigmaPidNS |
| Mount namespace | `clone(CLONE_NEWNS)` | Jail vfs | SigmaMntNS |
| Net namespace | `clone(CLONE_NEWNET)` | VIMAGE | SigmaNetNS |
| UTS namespace | `clone(CLONE_NEWUTS)` | Jail hostname | SigmaUtsNS |
| User namespace | `clone(CLONE_NEWUSER)` | — | SigmaUserNS |
| Cgroup v2 | `cgroupfs` / `libcgroup` | — | SigmaCgroupV2 |
| CPU controller | `cpu.weight` | — | SigmaCpuCtrl |
| Memory controller | `memory.max` | `rctl` | SigmaMemCtrl |

## Responsibilities
- Create/join PID, mount, network, UTS, IPC, user namespaces
- Cgroup v2 hierarchy creation and management
- CPU controller: weight-based scheduling, quota/period
- Memory controller: hard/soft limits, OOM group kill
- I/O controller: read/write BPS and IOPS limits
- Freezer: pause/resume entire cgroup subtrees
- `/sys/fs/cgroup/` virtual filesystem
- Namespace file descriptor (`/proc/PID/ns/`) for `setns()`

## Implementation Notes
- `#![no_std]` — cgroup tree as intrusive linked list
- `SigmaCgroup { name: [u8;64], parent: *mut SigmaCgroup, cpu_weight: u32, mem_max: u64 }`
- Namespace created via `SigmaFork` flags
- Mount namespace: copy-on-write mount table per namespace

## Testing
```bash
rustc --test src/cgroups/cpu_ctrl.rs --edition 2021 -o /tmp/test_cg && /tmp/test_cg --test-threads=1
```

## Future Development
1. **Cgroup v2 BPF programs** — Attach BPF to cgroup for network/device control
2. **Systemd cgroup delegation** — Auto-delegate subtree to user services
3. **PSI** — Pressure Stall Information metrics per cgroup
4. **Cgroup aware OOM** — Kill entire cgroup on memory pressure
5. **Network namespace with veth pairs** — Container networking primitive
6. **User namespace UID mapping** — Rootless containers
