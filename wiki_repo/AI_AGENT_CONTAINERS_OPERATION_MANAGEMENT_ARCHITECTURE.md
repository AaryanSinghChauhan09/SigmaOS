# AI Agent Containers Operation Management Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                         AI Containers Operation Manager                         |
|   (ContainerOperationManager, FreeBsdJailManager, QubesIsolationGovernor)       |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                    Isolation & Resource Governance Layer                        |
|       (cgroups v2, Linux Namespaces, RACCT Throttling, CapabilityTokens)        |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
| FreeBSD Jails (VNET)  |   | Qubes KVM Micro-VMs   |   | Flatpak / Snap / APEX |
| (Isolated chroot)     |   | (VirtIO Ring Isolation|   | (SquashFS + OverlayFS)|
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                     Storage & Atomic A/B Layer Manager                          |
|             (OverlayFS CoW, Immutable SquashFS, Dilithium-5 Attest)             |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **Namespace & cgroups v2 Control Engine**:
   - Creates isolated container environments with PID, Mount, Network (`vNET`/`veth`), IPC, UTS, and User namespaces.
   - Sets strict memory, CPU, and block I/O limits using `ContainerResourceGovernor`.

2. **FreeBSD Jails & Qubes VM Isolator**:
   - `FreeBsdJailManager` instantiates lightweight jail sandboxes with RACCT resource governors.
   - `QubesIsolationGovernor` runs untrusted tasks inside KVM micro-VMs with VirtIO split ring backends.

3. **Flatpak, Snap & APEX Container Engine**:
   - Mounts read-only SquashFS application layers merged with writable OverlayFS directories.
   - Handles A/B slot updates and verifies post-quantum signatures (`sigma attest`).

4. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
