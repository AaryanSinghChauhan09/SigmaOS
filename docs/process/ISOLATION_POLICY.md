# SigmaOS Process Isolation Policy

## Isolation Namespaces & Boundaries

1. **PID Namespace**:
   - Isolates process ID space. Processes inside a PID namespace only see processes within the same or child namespaces.
   - PID 1 inside a PID namespace acts as the local init process.

2. **Mount Namespace**:
   - Isolates file system mount points. Operations (`mount`/`umount`) inside the namespace do not affect the host mount hierarchy unless explicitly shared.

3. **Network Namespace**:
   - Isolates network devices, IPv4/IPv6 stacks, IP routing tables, firewall rules, and socket port bindings.

4. **UTS Namespace**:
   - Isolates domain name and hostname configurations (`sethostname`/`setdomainname`).

5. **User & Group Identity**:
   - Maps UID/GID ranges inside the namespace to distinct host UID/GID ranges.
   - Allows unprivileged root user inside container namespace without host root access.

6. **Capabilities Boundary**:
   - Cross-namespace or privileged process operations require explicit capability tokens (`CAP_SYS_ADMIN`, `CAP_NET_ADMIN`, `CAP_KILL`, etc.).

7. **Filesystem Access Restrictions**:
   - Restricts filesystem access using Sandboxing primitives (`Landlock`, `Unveil`) to enforce fine-grained path-level read/write/exec permissions.
