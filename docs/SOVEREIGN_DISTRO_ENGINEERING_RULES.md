# Sovereign SigmaOS Distribution Engineering Rules & Architectural Standards

This document establishes the official engineering rules and architectural standards for **SigmaOS**, inspired by the world's most prominent Linux and BSD distribution paradigms.

---

## Pillar 1: Arch Linux Packaging Purity & ALPM Invariants

* **Rule 1.1 (PKGBUILD Purity):** Package manifests (`.sigpkg` / `PKGBUILD`) MUST explicitly define all runtime dependencies (`depends`) and build-time dependencies (`makedepends`). No implicit host environment leakage is permitted.
* **Rule 1.2 (Topological Dependency Sorting):** The package dependency graph MUST be resolved via Kahn's algorithm or Tarjan's strongly connected components algorithm (`AlpmDatabase::resolve_dependencies`). Cyclic dependencies are strictly prohibited and MUST fail builds during CI/CD.
* **Rule 1.3 (Cleanroom Chroot Building):** Packages MUST be compiled inside isolated chroot namespaces (`ArchCdevtoolsEngine` / `arch-nspawn`) with clean VFS mounts.

---

## Pillar 2: Debian FHS 3.0 & Reproducible Build Rules

* **Rule 2.1 (FHS Compliance):** System binaries MUST reside in canonical path locations (`/usr/bin`, `/usr/sbin`, `/usr/lib`). No arbitrary root filesystem pollutions are permitted.
* **Rule 2.2 (Hermetic Reproducibility):** Build recipes MUST bind `SOURCE_DATE_EPOCH` to fixed timestamps to ensure bit-for-bit reproducible binary outputs.
* **Rule 2.3 (Debian Free Software Guidelines - DFSG):** All code under `src/` MUST be licensed under OSI-approved open-source licenses (MIT, Apache 2.0, BSD-2-Clause/3-Clause). Unfree blobs MUST be isolated into sandboxed firmware shards.

---

## Pillar 3: Fedora SELinux Security & Systemd Presets

* **Rule 3.1 (SELinux Security Contexts):** Every VFS inode, process domain, and network socket MUST carry a valid SELinux security context label (`user:role:type:sensitivity`).
* **Rule 3.2 (Domain Transitions):** Privileged domain transitions (e.g., `user_t` -> `passwd_exec_t` -> `passwd_t`) MUST be explicitly authorized in `SeLinuxEngine` policies. Unlabeled or unauthorized transitions MUST trigger AVC denial alerts.
* **Rule 3.3 (Systemd Service Presets):** Newly registered services MUST be gated by preset activation policies (`SystemdPresetConfigurator`) matching standard `/usr/lib/systemd/system-preset/*.preset` rules.

---

## Pillar 4: FreeBSD Ports QA, Poudriere & Capsicum Sandboxing

* **Rule 4.1 (Poudriere Cleanroom QA):** Package builds MUST undergo QA staging in isolated Jail sandboxes before promotion to stable mirrors.
* **Rule 4.2 (Capsicum Capability Rights):** File descriptors passed to untrusted helper processes MUST have Capability Rights (`cap_rights_limit(2)`) restricted to exact required operations (`CAP_READ`, `cap_rights_contains`).

---

## Pillar 5: OpenBSD Process Restriction & Hardened Execution

* **Rule 5.1 (Pledge & Unveil Constraints):** All userland processes MUST invoke `pledge(2)` restrictions (e.g., `"stdio rpath inet"`) and `unveil(2)` VFS path masks upon completion of initialization.
* **Rule 5.2 (Immutable W^X Memory Rules):** Memory page allocations MUST NOT be simultaneously writable and executable (`W^X`). Dynamic JIT pages MUST use Retguard stack canaries and KaslrWx page allocators.

---

## Pillar 6: NixOS CAS Hermetic Store & Atomic Rollbacks

* **Rule 6.1 (Content-Addressed Storage):** System configurations, package objects, and kernel modules MUST be stored as Merkle DAG trees in a content-addressed storage (CAS) layout.
* **Rule 6.2 (Sub-1ms Atomic Rollbacks):** Upgrades and configuration changes MUST be executed via atomic Merkle root pointer updates (`CasPackageStore::atomic_repoint_boot_root`), guaranteeing sub-1ms rollback recovery.

---

## Pillar 7: Gentoo EAPI 8 USE Flags & Microarchitecture Tuning

* **Rule 7.1 (USE Flag Constraint Solving):** Package features MUST be conditionally gated using Gentoo EAPI 8 USE flag constraint solving (`GentooPortageSlotOperatorEngine`).
* **Rule 7.2 (CFLAGS Microarchitecture Optimization):** Toolchain builds MUST auto-tune CFLAGS/CXXFLAGS (`-march=x86-64-v4`, `-O3`, `-flto`) based on ISA detection (`V4OptimizedPackageManager`).

---

## Pillar 8: CachyOS BORE Scheduler & Low-Latency Tuning

* **Rule 8.1 (BORE Scheduler Latency Bounds):** Desktop and interactive threads MUST be scheduled via BORE (Burst-Oriented Response Enhancer) or SchedExt BPF engines to guarantee < 1ms frame rendering bounds under 100% load.
* **Rule 8.2 (Kernel Parameter Sysctl Tuning):** Kernel sysctl parameters (`vm.max_map_count`, `vm.swappiness`, `vm.dirty_ratio`) MUST be continuously adjusted by `CachyosSysctlTuningEngine` for peak gaming and developer workstation performance.

---

## Pillar 9: DragonFly BSD HAMMER2 Emergency CoW & Deduplication

* **Rule 9.1 (Emergency Read-Only Snapshots):** Storage subsystems MUST automatically trigger read-only CoW snapshots when underlying disk pressure exceeds 95% capacity.
* **Rule 9.2 (FNV Bulk Deduplication):** Storage blocks MUST be deduplicated via lock-free parallel FNV-1a hashing algorithms to maximize storage efficiency.

---

## Compliance Verification

All code merged into `main` MUST comply with these rules. Automated compliance evaluation is enforced via `LinuxBsdDistroGuidelineRules` in `src/distro/compliance.rs` and executed via `./run_sigma_tests.sh`.
