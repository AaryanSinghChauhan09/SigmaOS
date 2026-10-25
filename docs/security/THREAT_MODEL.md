# SigmaOS Threat Model & Adversary Profiles

## Adversary Profiles

1. **Malicious Untrusted Userspace Application**:
   - *Goal*: Escalate privileges, access arbitrary files, or corrupt kernel memory via syscalls.
   - *Mitigation*: Syscall argument bounds checks, ring 3/0 privilege separation, W^X page protection, and capability token enforcement.

2. **Compromised Network Service**:
   - *Goal*: Execute remote code via malformed network packets.
   - *Mitigation*: Landlock-like process sandboxing, unprivileged network sockets, and non-executable stack/heap boundaries.

3. **Malicious Peripheral / USB Device**:
   - *Goal*: Perform DMA attack or trigger kernel buffer overflow via rogue device descriptors.
   - *Mitigation*: VT-d/IOMMU bus isolation, strict descriptor length validation in xHCI/NVMe drivers.

4. **Tampered Package Repository**:
   - *Goal*: Inject rogue binaries during package updates.
   - *Mitigation*: Dilithium-5 PQC digital signature verifier and atomic transaction rollback.
