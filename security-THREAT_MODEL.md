# SigmaOS Threat Model

## Threat Vectors & Actor Profiles

1. **Malicious / Compromised Userspace Application**:
   - **Vector**: Attempting privilege escalation, unauthorized file read/write, or kernel memory corruption.
   - **Mitigation**: Capability token validation, ring-3 isolation, bounds-checked memory buffers, seccomp filters.

2. **Supply Chain / Malicious Package Update**:
   - **Vector**: Tampered package tarball or forged release artifact.
   - **Mitigation**: Dilithium-5 PQC signature verification in `sigpkg` and installer recovery.

3. **Network / Packet Injection**:
   - **Vector**: Malformed TCP/IP header, DNS spoofing, or buffer overflow in socket layer.
   - **Mitigation**: Fuzzing targets (`fuzz/`), packet bounds validation, and memory safety invariants (`#![no_std]`).

4. **Installer / Partition Mutation Interruption**:
   - **Vector**: Unexpected power loss or disk failure during installation/update.
   - **Mitigation**: Transactional snapshots, dry-run mode, and recovery runbooks (`docs/operations/RECOVERY_RUNBOOK.md`).
