# Future Development: Virtualization and Containers

**Status:** Proposal. VM, namespace, or OCI data models do not establish guest execution or container isolation.

## Scope

Plan hardware-assisted virtual machines, process containers, resource control, and image execution as separate features with explicit threat boundaries. Current source areas include [`src/virtualization`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/virtualization), [`src/container`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/container), and [`src/kernel/cgroup_v2_controller.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/kernel/cgroup_v2_controller.rs).

## Linux and BSD ideas to evaluate

| Project | Design idea | SigmaOS question |
|---|---|---|
| Linux | KVM, namespaces, cgroups, and seccomp | Which CPU, memory, device, and syscall boundaries are actually enforced? |
| FreeBSD | bhyve and jails | Can the VM monitor and jail-style isolation share safe device and lifecycle management? |
| OpenBSD | vmm and pledge/unveil | What is the smallest host interface a guest manager requires? |
| NetBSD | rump kernels | Can selected kernel components run with less privilege for isolation and independent validation? |

## Proposed work sequence

1. Separate VM execution from containers in documentation, APIs, resource ownership, and acceptance criteria.
2. For VMs, define CPU feature detection, guest memory ownership, interrupt/device model, exit handling, and teardown before exposing a run interface.
3. For containers, enumerate namespaces and resources, then connect every isolation policy to the syscall, filesystem, network, and device enforcement path.
4. Make resource limits and cleanup apply under normal exit, crashes, and partial startup; reject unsupported isolation requests.
5. Treat OCI compatibility as a format and behavior commitment only after image parsing, execution, and security semantics are implemented end-to-end.

## Completion criteria

- VM execution runs a documented guest workload on a named hardware or emulator configuration, with host/guest isolation boundaries reviewed.
- Container tests show that processes cannot access resources outside granted namespaces and capabilities.
- Resource limits are enforced in runtime paths, including under concurrent workloads.
- Startup failure and forced teardown release memory, devices, and namespace resources.
- Unsupported CPU features, isolation policies, or image directives fail clearly without claiming successful execution.

## Maintenance

Track VM, container, OCI, and resource-control status separately. Link each security claim to its enforcement path and record the tested machine or emulator configuration.
