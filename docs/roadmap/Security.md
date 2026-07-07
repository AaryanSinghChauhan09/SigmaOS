# Security Framework Roadmap

## MicroVM Sandboxing
SigmaOS replaces traditional cgroups/namespaces with hardware-accelerated MicroVMs (similar to Firecracker/gVisor). Untrusted applications and developer sandboxes run in isolated address spaces.

## TPM Attestation & Cryptography
- Administrative escalation requires hardware token or TPM attestation.
- The `root` user is disabled entirely.

## Mandatory Access Control (MAC)
A lightweight, declarative MAC engine is embedded in the kernel, governing file access and syscall permissions without the configuration overhead of SELinux.

## IDS Integration
Native hooks into the network stack allow direct integration with tools like Suricata and Snort for real-time threat analysis and packet dropping at the hypervisor level.
