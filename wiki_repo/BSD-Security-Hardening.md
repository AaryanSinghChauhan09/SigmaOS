# 🔐 BSD Security Hardening & Isolation Mechanics

SigmaOS incorporates premier security and sandboxing primitives from OpenBSD, FreeBSD, and DragonFly BSD.

---

## 1. OpenBSD Sandbox Primitives (`src/security/unveil.rs` & `src/distro/mod.rs`)

* **Pledge Syscall Restriction (`OpenBsdFdPledgeGate`):** Restricts active process syscalls to explicit promise sets (`stdio`, `rpath`, `wpath`, `cpath`, `inet`, `unix`, `exec`).
* **Unveil Path Masking (`UnveilManager`):** Restricts process filesystem visibility to explicitly unveiled path prefixes with permission constraints (`r`, `w`, `c`, `x`). Supports glob and regex path patterns.

---

## 2. FreeBSD Jails & Capsicum Capability Mode (`src/compatibility/freebsd_jails.rs` & `src/ui/gtk.rs`)

* **FreeBSD Jail Manager (`FreeBsdJailManager`):** Isolated lightweight container environments with independent IP bindings, root directories (`/jails/web1`), and process namespaces (`JID`).
* **Capsicum Sandbox Guard (`FreeBsdCapsicumGtkGuard`):** Places GTK and UI processes in capability mode, revoking global namespace lookup syscalls and enforcing descriptor-only operations.

---

## 3. DragonFly BSD HAMMER2 & FreeBSD GEOM Topology (`src/storage/geom.rs` & `src/filesystem/sigma_fs.rs`)

* **GEOM Storage Topology (`GeomTopology`):** Modular storage provider chains with GELI disk encryption (`ada0p1.eli`) and BIO request dispatching.
* **HAMMER2 PFS & Varsyms (`DragonFlyVarsymsPfsResolver`):** Pseudo-filesystem snapshotting with dynamic variable symlinks (`$MACHINE`, `$SYS`).
