# SigmaOS Process Isolation Policy

## Namespace Boundaries

SigmaOS provides process isolation through namespaces and capability models:

1. **PID Namespace**:
   - Isolates process ID numbers. Processes in a child PID namespace see local PIDs starting from 1.
2. **Mount Namespace**:
   - Isolates filesystem mount points, providing isolated root filesystems and chroot environments.
3. **Network Namespace**:
   - Isolates network interfaces, IP routing tables, socket lookup tables, and firewall rules.
4. **UTS Namespace**:
   - Isolates hostname and NIS domain name setting.
5. **User & Group Credentials**:
   - Standard POSIX UID/GID, EUID/EGID, and SUID/SGID evaluation for access control.
6. **Capability Boundaries**:
   - Replaces monolithic superuser privilege with fine-grained POSIX capability bitmasks (`CAP_SYS_ADMIN`, `CAP_NET_ADMIN`, `CAP_SYS_RESOURCE`).
7. **Filesystem Sandbox Controls**:
   - Integrates `landlock` and `unveil` restrictions to constrain path traversal and file operations.
