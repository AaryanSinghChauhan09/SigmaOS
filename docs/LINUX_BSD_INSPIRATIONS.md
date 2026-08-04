# Linux and BSD Distro Inspirations in SigmaOS

SigmaOS draws heavy inspiration from various Linux and BSD distributions to combine absolute security, flexibility, and architectural efficiency.

---

## 1. OpenBSD: Pledge, Unveil, and Securelevels

SigmaOS absorbs OpenBSD's security philosophy by introducing:
- **Pledge**: Capability restrictions (`src/security/sigma_pledge.rs`) which allow processes to declare what system calls they need, killing them if they attempt a forbidden action.
- **Unveil**: Path-based filesystem restriction (`src/security/sigma_unveil.rs`), hiding parts of the VFS hierarchy from processes.
- **Securelevels**: System-wide protection states (`src/security/securelevels.rs`) which limit kernel modifications and raw disk writing as the secure level escalates.

---

## 2. FreeBSD: Jails and Capabilities (Capsicum)

From FreeBSD, SigmaOS absorbs:
- **Isolated Domains**: Similar to FreeBSD Jails, these allow compartmentalizing processes and environments.
- **Capability Tokens**: An object-based capability system (`src/security/capability.rs`) which enforces granular access to devices and streams.

---

## 3. Qubes OS: VM-Based Compartmentalization

Rather than relying purely on soft containers, SigmaOS integrates Qubes-style isolation:
- **`qubes_isolation`**: Segregates processes into distinct domain classes (e.g., NetVM, AppVM, AdminVM) with safe inter-domain communication (similar to vchan).

---

## 4. Arch Linux & Gentoo: Package Management and Compilation

SigmaOS mirrors:
- **Rolling Release**: Built-in support for rolling system configurations.
- **`sigpkg`**: Custom package format adapters (`src/sigpkg/`) inspired by the flexibility of Portage and pacman.
