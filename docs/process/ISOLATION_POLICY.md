# SigmaOS Process Isolation Policy

## Overview

Process isolation in SigmaOS relies on explicit namespace boundaries, capability verification, and filesystem restriction hooks.

## Isolation Boundaries

1. **PID Namespace:** Controls visibility of process IDs. Processes in child PID namespaces cannot view or signal processes outside their boundary.
2. **Mount Namespace:** Provides isolated filesystem mount tables. Operations within an isolated mount namespace do not affect external mount namespaces.
3. **Network Namespace:** Isolates network interfaces, routing tables, firewall rules, and socket binding spaces.
4. **UTS Namespace:** Isolates hostname and NIS domain name identifiers.
5. **User Credentials & Capabilities:** Enforces POSIX UID/GID checks alongside capability tokens (`CAP_SYS_ADMIN`, `CAP_NET_ADMIN`, `CAP_KILL`). Privileged cross-namespace operations require explicit capability validation.
6. **Landlock & Unveil Restrictions:** Restricts file access hierarchies per process subtree to enforce least privilege.
