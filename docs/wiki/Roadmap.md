# SigmaOS Roadmap
**Single source of truth — phases, milestones, and 12-month sprint plan**

SigmaOS leverages proven design patterns from leading Linux distributions (Debian for stability, Fedora for innovation, Arch for modularity, Ubuntu for UX, Bottlerocket for cloud minimal) to guide our sovereign ecosystem strategy.

Status key: ✅ Done · 🔄 In Progress · 🎯 Planned · ❌ Blocked

---

## 🏗️ Phase Plan & Timeline

| Phase | Distro Model | Timeline | Primary Focus | Key Deliverables |
|---|---|---|---|---|
| **0. Foundation** | Debian (LTS) | 0–3 months | Stabilize core and documentation | ABI spec; `main` build reproducible; role-based Wiki; CI baseline |
| **1. Hardware** | Fedora (Fast) | 3–9 months | Drivers and device model | USB, NVMe, basic GPU, audio drivers; driver API docs; CI tests |
| **2. Security & FS** | Arch (Modular) | 6–12 months | Robust FS and hardening | Journaling FS; encryption; sandboxing; CVE process; formal verification plan |
| **3. Ecosystem** | Ubuntu (UX) | 9–18 months | `sigpkg` and built-in apps | `sigpkg` MVP; coreutils/shell in Rust/Zig; SigmaVCS prototype; SDK |
| **4. Profiles** | Bottlerocket | 12–24 months | Desktop, Cloud, RTOS, Mobile | `sigma-desktop` alpha; signed cloud images; RTOS profile; mobile UI |
| **5. Research** | Theseus/Redox | 18–36 months | Verification and research features | Verified FS module; microkernel variant; WASM runtime; full app ecosystem |

---

## 🏃 Detailed 12-Month Sprint Plan

### Months 0–3: Foundation Sprint
- **Issue:** Publish ABI spec and LTS policy.
- **Issue:** Create reproducible build CI job for `main`.
- **Issue:** Add role-based Wiki pages and contribution templates.
- **Issue:** Implement `sigma-sh` minimal shell and 3 core utilities (Rust/Zig).
- **Issue:** Define Security Model and CVE reporting workflow.

### Months 3–6: Drivers & Packaging Sprint
- **Issue:** Publish Driver API spec and sample USB driver in `drivers-dev`.
- **Issue:** Create `sigpkg` repo skeleton and package signing spec.
- **Issue:** Publish FS design doc for journaling and encryption.

### Months 6–12: Ecosystem & Images Sprint
- **Issue:** Implement NVMe driver and add QEMU tests.
- **Issue:** Define `sigma-core` meta-manifest and alpha desktop image.
- **Issue:** `sigpkg` MVP: build, sign, and install a core package.

---

## 📦 Detailed Task Tracking (v0.1 → v1.0)

### v0.1 — Foundation ("Genesis")
> **DoD:** Boots in QEMU x86_64 → framebuffer login → 3 userspace apps run

| Task | Status | Location |
|---|---|---|
| Multiboot2 boot on x86_64 | ✅ Done | `S01_Genesis/` |
| Physical/Virtual memory | ✅ Done | `memory/` |
| Basic EEVDF scheduler | ✅ Done | `scheduling/` |
| Serial (COM1) + VGA output | ✅ Done | `drivers/` |
| VFS + ramfs | ✅ Done | `fs/` |
| ELF userland loader | ✅ Done | `userland/` |

### v0.2 — Stability & Contributor Funnel (Months 0-3)
> **Goal:** Green CI, reproducible builds, top-level docs, one canonical build entrypoint

| Task | Status | Location |
|---|---|---|
| `sigma.toml` declarative config | ✅ Done | root |
| **sigma-sh v0.2 (full Rust)** | 🔄 In Progress | `sigma-sh/` |
| **sigpkg v0.2 (resolver + crypto)** | 🔄 In Progress | `userland/sigpkg/` |
| **Absorption Matrix** | ✅ Done | `docs/Absorption-Matrix.md` |
| **Security Model doc** | ✅ Done | `docs/Security-Model.md` |
| Publish ABI spec & LTS policy | 🎯 Planned | `docs/ABI.md` |
| Reproducible build CI (`main`) | 🎯 Planned | `.github/workflows/` |
| Role-based Wiki pages | 🎯 Planned | `docs/wiki/` |

### v0.3 — Networking & Security Hardening (Months 6-12)
> **Goal:** Functional TCP/IP + DoH + WireGuard; SPARK proofs gate on merge

| Task | Status | Location |
|---|---|---|
| TCP/IP & QUIC stack | ✅ Done | `net/` |
| DoH resolver & TLS 1.3 | ✅ Done | `net/` |
| WireGuard VPN (`sigma-vpn`) | 🎯 Planned | `net/vpn/` |
| **Syscall audit framework** | 🎯 Planned | `kernel/security/audit/` |
| **Capability sandbox v1** | 🎯 Planned | `kernel/security/caps/` |

### v0.4 — Desktop & GPU (Months 12-24)
> **Goal:** Zenith compositor boots, GPU acceleration, basic app model

| Task | Status | Location |
|---|---|---|
| Zenith compositor | ✅ Done | `desktop/`, `zenith_desktop.js` |
| GPU HAL (Vulkan-like) | ✅ Done | `graphics/` |
| **sigpkg v0.2** (real resolver) | 🔄 In Progress | `userland/sigpkg/` |
| **USB/HID stack** | 🎯 Planned | `drivers/usb/` |
| **NVMe driver** | 🎯 Planned | `drivers/storage/nvme/` |
| `sigma-core` meta manifest | 🎯 Planned | `config/profiles/` |

---

## 🗂️ Branch Ownership & Architecture

To scale development, we use strict branch mapping with specific maintainers and CI gates.

👉 **Read the full guide:** [Branch Ownership & CI Gates](Branch-Ownership.md)

---

## 📈 Success Metrics

- **Short term (0–6 months):** `main` builds reproducibly across 3 targets; ABI/Driver docs published; 3 `good-first-issue` PRs merged.
- **Mid term (6–18 months):** USB/NVMe drivers functional in CI; `sigpkg` can install signed packages; `sigma-desktop` alpha boots.
- **Long term (18–36 months):** Signed cloud images deployed on providers; verified FS integrated; active maintainers for all subsystems.

---

*[Back to Wiki Home](Home.md) | [Absorption Matrix](Absorption-Matrix.md) | [Contributing](Contributing.md)*
