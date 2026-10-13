# SigmaOS Security Threat Model

## Asset Inventory
- **Kernel Space Integrity:** Kernel code pages, IDT/GDT descriptor tables, page tables.
- **User Credentials & Secrets:** Password hashes in `/etc/shadow`, PQC private keys, IPC capabilities.
- **System Storage & Rollback Records:** Root filesystem blocks, Timeshift snapshot records, package manifests.

## Threat Actors & Vectors
- **Malicious Userspace Binary:** Attempts privilege escalation via invalid syscall arguments, ring 3 memory injection, or unauthorized mount calls.
- **Compromised Package / Update Payload:** Tries to install unsigned or tampered binary binaries.
- **Untrusted Network Host:** Attempts TCP packet malformation or unauthorized port binding.

## Mitigations
- **Syscall Capability Validation:** Every syscall dispatcher entry point verifies `CapabilityToken` and clamps user pointers.
- **PQC Signature Verification:** Package updates require valid Dilithium-5 signatures before execution.
- **Address Sanitizer & Memory Guard:** Redzone overflow detection and use-after-free tracking (`AddressSanitizer`).
