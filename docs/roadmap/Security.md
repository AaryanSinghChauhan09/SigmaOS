# Security Roadmap & Spec

## 1. MicroVM Sandboxing (Firecracker & KVM Primitives)
Rather than relying on kernel namespaces and cgroups (which share a single kernel surface), SigmaOS enforces process isolation via hardware virtualization.
- **VMM core**: Interacts directly with KVM/VMM primitives (`sigma_sandbox.rs`).
- **Memory isolation**: Userspace applications execute inside MicroVM structures with isolated page directories (`root_cr3`).
- **System Call Filters**: Allowlist filtering traps unknown or raw syscalls instantly at the hypervisor level.

## 2. TPM Attestation & MAC Policies
- **Measured Boot**: TPM2 registers record cryptographic signatures throughout the boot cycle, locking root encryption keys against configuration tampering.
- **Mandatory Access Control (MAC)**: Custom security rules configure fine-grained permissions for all storage mounts and network namespaces.
- **Privilege management**: The traditional `root` user is disabled. Elevation (`sudo`) requires secure attestation.

## 3. Network Policies & IDS
- Network namespaces are air-gapped by default.
- Apps declare network profiles in `sigpkg` manifests.
- Built-in packet analyzers (Suricata, Snort) run inside network interfaces to block unauthorized connections.

## 4. Roadmap Phases
- **Phase 1 (0–3m)**: Basic KVM integration and memory layout isolation.
- **Phase 2 (3–6m)**: Syscall allowlist checking and security auditing APIs.
- **Phase 3 (6–9m)**: Declarative MAC engine integration and virtual network policy enforcement.
- **Phase 4 (9–12m)**: Full TPM attestation verification and host intrusion detection module.

## 5. Contributor Guidelines
- Any kernel subsystem modification must be vetted against the central Security Policy.
- Never write unprotected `unsafe` pointers without documenting validation checks.
