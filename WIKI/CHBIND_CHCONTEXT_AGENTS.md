# AI Agent Development Instructions for Context Binding & Chroot/Sandbox Containment (`src/container/` & `src/kernel/proc/`)

This document details guidelines for `chbind` (process CPU/NUMA node binding), `chcontext` (security context & jail namespace switching), FreeBSD `jail_attach`, OpenBSD `pledge`/`unveil`, and clean `chroot` build sandbox management in SigmaOS.

## Subsystem Architecture & Directives

1. **CPU & NUMA Affinity Binding (`chbind` parity)**
   - Process CPU core and NUMA node affinity masks (`cpu_set_t`) must be set via `sched_setaffinity` syscall handlers or `chbind` CLI utilities.
   - Enforce task migration restrictions when process affinity is pinned to specific isolated CPU sets (`cpuset` cgroup controller).

2. **Security Context & Jail Containment (`chcontext` / `jail_attach` parity)**
   - `chcontext` updates the security context label (SELinux/MAC security label, FreeBSD Jail JID, or user namespace mapping) of a running process tree.
   - Switching security contexts requires `CAP_SYS_ADMIN` / `CAP_MAC_ADMIN` capabilities and must atomically refresh thread capability sets (`LinuxCapabilitySet`).

3. **Chroot Sandbox Isolation (`src/container/distro_sandbox.rs` & `src/distro/developer.rs`)**
   - Clean build sandboxes (such as Arch `makechrootpkg` and Debian `sbuild`/`pbuilder` engines) mount isolated root file systems (`/var/lib/sigma_chroot`).
   - Escaping chroot boundaries must be prevented by executing `fchdir` / `pivot_root` and dropping path traversal access to parent inodes outside the chroot root directory (`/`).

4. **Verification**
   - Verify context binding changes using `cargo check --lib`.
