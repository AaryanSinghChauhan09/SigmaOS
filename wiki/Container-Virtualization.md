# Containers and Virtualization

**Capability state: Proposed.** No container runtime, hypervisor, or isolation boundary is verified. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

Names such as namespaces, cgroups, OCI, KVM, bhyve, and containers in the source or plans do not establish enforced isolation. There is no supported container execution or virtual machine path.

## Design references

Linux namespaces/cgroups, FreeBSD jails/bhyve, OpenBSD pledge/unveil, and Redox capabilities offer distinct isolation approaches. Select a threat model and enforceable primitives before exposing a runtime interface.

## Roadmap

1. Establish process, memory, filesystem, and capability boundaries in the kernel.
2. Define an isolation test that demonstrates a process cannot access prohibited resources.
3. Add resource limits with exhaustion tests and deterministic cleanup.
4. Consider OCI compatibility or a hypervisor only after a usable base system and security review exist.

**Completion evidence:** adversarial integration tests demonstrate enforced boundaries, resource accounting, teardown, and failure cleanup on the supported configuration.
