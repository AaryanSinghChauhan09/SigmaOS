# Universal BSD Subsystems Synthesis & Parity Matrix

SigmaOS synthesizes the greatest engineering breakthroughs across **DragonFly BSD**, **FreeBSD**, and **OpenBSD** into a single cohesive, high-performance operating system architecture.

---

## 🏛️ Tri-BSD Architecture Synthesis

```
+-----------------------------------------------------------------------------------+
|                            SigmaOS Universal BSD Layer                            |
+-------------------------+-------------------------------+-------------------------+
|      DragonFly BSD      |            FreeBSD            |         OpenBSD         |
|   Concurrency & VFS     |      Sandboxing & Storage     |    Hardening & Network  |
+-------------------------+-------------------------------+-------------------------+
| - HAMMER2 CoW Storage   | - Capsicum Capabilities       | - Pledge Syscall Filter |
| - LWKT Lock-Free Rings  | - Jails & VNET Namespaces     | - Unveil Path Isolation |
| - VKERNEL Micro-Inst    | - kqueue / kevent Dispatch    | - W^X Memory Guarantees |
| - Per-CPU Runqueues     | - GEOM Layered Storage        | - PF Stateful Firewall  |
+-------------------------+-------------------------------+-------------------------+
```

---

## 📊 Comprehensive Subsystem Parity Matrix
| Subsystem Domain | FreeBSD 14 | OpenBSD 7.5 | DragonFly 6.4 | SigmaOS Native Parity |
| :--- | :---: | :---: | :---: | :---: |
| **Process Sandboxing** | Capsicum | Pledge / Unveil | Capsicum Shim | **Native Pledge + Unveil + Capsicum** |
| **Containerization** | Jails | None | Chroot | **Lightweight Jails + MicroVMs** |
| **High-Perf Filesystem**| OpenZFS | FFS2 + Softdep | HAMMER2 | **SigmaFS + HAMMER2 + Ext4/ZFS** |
| **Event Polling** | kqueue | kqueue | kqueue | **kqueue + epoll + io_uring Native** |
| **Kernel Concurrency** | Mutex/SMP | Big Lock / SMP | LWKT Per-CPU | **Lock-Free LWKT + Read-Copy-Update** |
| **Network Firewall** | pf / ipfw | pf | pf | **Hardware-Accelerated PF Engine** |

---

## 🧭 Full BSD Family Coverage

Category taxonomy kept uniform with the repository
(**UNIFORM INFORMATION IN GITHUB REPO & GITHUB WIKI**):

- **Primary tri-BSD:** `DragonFly; Free; Open` — full subsystem parity in the
  matrices above.
- **Debian / niche BSD family:** `Ghost; Nomad; Midnight; Hardened; Liberty;
  Fury; Desktop; Karma; Star`

| Family Member | Kind | Reference |
| :--- | :--- | :--- |
| GhostBSD | FreeBSD-based desktop BSD | community parity notes |
| NomadBSD | FreeBSD-based live/portable BSD | live-session parity notes |
| MidnightBSD | FreeBSD-derived desktop BSD | ports & GNU-toolchain parity notes |
| HardenedBSD | Hardened FreeBSD (ASLR, Capsicum+) | security parity notes |
| LibertyBSD | OpenBSD-derived hardened BSD | pledge/unveil parity notes |
| FuryBSD | FreeBSD-based live desktop BSD | live-session parity notes |
| DesktopBSD | FreeBSD-based desktop BSD | desktop parity notes |
| KarmaBSD | OpenBSD-derived community BSD | security parity notes |
| StarBSD | FreeBSD-family community BSD | community parity notes |

Consolidated category index: [[SOFTWARE-CATEGORY-MATRIX]]
