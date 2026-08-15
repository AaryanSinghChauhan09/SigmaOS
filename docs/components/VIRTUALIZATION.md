# SigmaOS Virtualization and Containerization

SigmaOS implements highly secure, multi-layered isolation inspired by leading BSD and Linux distros.

## Jail & Container Subsystem

Located in [`src/virtualization/container.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/virtualization/container.rs):
- **BSD Jails Parity**: Emulates chroot-style network and system namespace boundaries to isolate environments without hypervisor overhead.
- **Rootless Podman Runtime**: Creates unprivileged containers, mapping local namespace configurations securely.
- **OCI Container Parity**: Direct parsing and execution of OCI-compliant runtime configurations and namespaces.
