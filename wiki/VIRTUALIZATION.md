# Virtualization & Containers

SigmaOS provides sandboxing models inspired by Qubes OS and FreeBSD Jails.

## Isolation Levels
1. **Unprivileged Sandbox**: Restricts syscall footprint to absolute minimum.
2. **OCI Container Runtime**: Matches OCI specifications to run Linux/BSD container images natively.
3. **Hardware Isolation**: GPU and network isolation with direct PCI pass-through.\n