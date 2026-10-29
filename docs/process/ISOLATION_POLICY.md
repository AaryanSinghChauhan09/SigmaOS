# SigmaOS Process Isolation Policy

## Isolation Boundaries & Namespaces

SigmaOS process isolation relies on unified namespace boundaries and capability-based access controls:

1. **PID Namespace:** Processes in child PID namespaces cannot view or signal processes in parent or sibling namespaces outside their tree.
2. **Mount Namespace:** Provides isolated file system mount tables, restricting visible directories and mount points.
3. **Network Namespace:** Isolates network devices, IP routing tables, port bindings, and firewall rules per container or process sandbox.
4. **UTS Namespace:** Isolates hostname and NIS domain name identification.
5. **User / Group Identity & Capabilities:** Restricts privileged system operations to tasks possessing explicit capabilities (`CAP_SYS_ADMIN`, `CAP_NET_ADMIN`, `CAP_KILL`). Unprivileged cross-user operations fail closed.
6. **Filesystem Restrictions (`Pledge` / `Unveil` / `Landlock`):** Restricts file paths and system call categories available to user processes.
