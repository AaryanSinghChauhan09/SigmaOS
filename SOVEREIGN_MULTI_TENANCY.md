
# 👥 Sovereign Multi-Tenancy Engine


SigmaOS supports industrial-grade session isolation through the Sovereign Multi-Tenancy Engine. This allows multiple operators to maintain isolated lattices, settings, and VFS mounts on a single machine.


## 🏛️ Session Model

- **Lattice-Level Isolation**: Each session is sharded within the S01 Scheduler, ensuring that Guest background processes cannot influence Root execution.
- **Dynamic VFS Mounting**: Upon switching users, the VFS dynamically remounts the corresponding `/home/user` path using Sovereign HAL hooks.
- **Shared Memory Pools**: Global read-only assets are shared via memory redirection to minimize footprint.


## 🚀 Native Switching

Sessions can be managed directly via the Terminal or the Zenith Dashboard Account panel.


## 📅 Roadmap

- **Phase 1**: Profile Switching (v33.0.4) - **[ACTIVE]**
- **Phase 2**: Concurrent Headless Sessions (multi-seat).
- **Phase 3**: Cross-Network Handoff of Active User State.

---
*Computing is personal. Sovereignty is shared.*
