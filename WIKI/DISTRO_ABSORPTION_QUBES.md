# SigmaOS Distro Absorption: Qubes OS (Hardware VM Isolation Engine)

## 1. Overview
SigmaOS incorporates Qubes OS's hardware-enforced Xen/KVM hypervisor compartmentalization, running application domains in isolated virtual machine containers (`sigcompartment`).

## 2. Security Domains
- **Domain Vault**: Air-gapped secret keys, password vaults, and crypto identity keys.
- **Domain Work**: Development environment, document processing, internal network access.
- **Domain Personal**: Web browser, multimedia, unauthenticated web access.
- **Domain Untrusted**: Downloaded binaries, unknown attachments, sandboxed testing.
- **Domain Net/USB**: Isolated hardware drivers for USB devices and physical NICs.
