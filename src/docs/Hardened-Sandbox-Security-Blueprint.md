# SigmaOS: Hardened Sandbox & Security Blueprint

This document specifies the security controls, container namespaces, and system-call isolation frameworks implemented within the SigmaOS kernel.

---

## 🔒 Multi-Layered Sandbox Architecture

SigmaOS establishes isolation at multiple logical layers to defend against kernel privilege escalations:

```
  +-------------------------------------------------------------+
  |                   USERSPACE APPLICATION                     |
  +-------------------------------------------------------------+
                                 |
                                 |  1. BSD Pledge / Unveil
                                 v
  +-------------------------------------------------------------+
  |              SYS-CALL CAPABILITY GATE (S-SEC)              |
  +-------------------------------------------------------------+
                                 |
                                 |  2. Seccomp Syscall Filter
                                 v
  +-------------------------------------------------------------+
  |            ROOTLESS NAMESPACE CONTAINER ISOLATION           |
  |             (OverlayFS Layer Stack + UID Mapping)           |
  +-------------------------------------------------------------+
                                 |
                                 |  3. Microkernel Ring-0 Core
                                 v
  +-------------------------------------------------------------+
  |                      HARDWARE SECURITY                      |
  +-------------------------------------------------------------+
```

---

## 🛡️ Subsystem Controls

### 1. BSD-style Pledge and Unveil
* **Pledge:** Allows applications to voluntarily shed kernel capabilities over time. For example, a web scraper calls `sigma_pledge("stdio rpath")` to permanently forfeit network access once data collection begins.
* **Unveil:** Restricts the directory subtree visibility of the process to explicitly declared paths, preventing unauthorized file traversals.

### 2. Seccomp System-Call Filtering
A Seccomp filter checks system calls against a configurable profile before they reach the core microkernel execution layers. Attempts to call blacklisted or un-pledged instructions result in immediate task termination.

### 3. Rootless User Namespaces
* **Podman-style Translation:** Translates unprivileged user UID/GID mapping boundaries inside the container directly.
* **OverlayFS Stacking:** Dynamically mounts stacked read-only system and read-write ephemeral layer folders to form a clean, isolated root directory.
