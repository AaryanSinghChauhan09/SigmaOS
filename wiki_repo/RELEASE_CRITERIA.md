# SigmaOS Desktop Edition: Milestone & Release Criteria

## 1. Milestones Overview

| Milestone | Target Scope | Verification Method | Status |
|---|---|---|---|
| **M0: Engineering Baseline** | Unified toolchain, 0 compilation errors, clean cargo check | `cargo check --lib`, `cargo check --all-targets` | ✅ Met |
| **M1: QEMU Desktop Preview** | UEFI boot into Zenith Wayland compositor, launcher, terminal, keyboard shortcuts | QEMU x86_64 VirtIO smoke test | 🔄 Active Focus |
| **M2: Native Package MVP** | Signed SigmaPkg with Merkle store, Boolean SAT resolver, delta updates, atomic rollback | `src/sigpkg/` verification test suite | ✅ Engine Ready |
| **M3: Declarative Profiles** | `/system/profile.toml` and `/user/theme.toml` atomic reconciliation | Theme live switcher & system generation tests | ✅ Engine Ready |
| **M4: Hardware Alpha** | Real Intel/AMD x86_64 hardware boot with display, audio, USB HID, and network | Bare-metal test matrix & hardware report | 📅 Scheduled |
| **M5: Stable Desktop Release** | End-to-end install, use, update, and rollback with complete documentation | Public release ISO validation | 📅 Scheduled |

---

## 2. Quantitative Performance Targets

| Metric | Target Budget | Verification Gate |
|---|---|---|
| **Cold Boot to Login** | < 10.0 seconds (QEMU), < 2.0 seconds (Bare-metal NVMe) | Automated QEMU serial timestamp log |
| **Login to Zenith Desktop** | < 2.0 seconds | Compositor frame 1 emission timestamp |
| **Application Launcher Latency** | < 50 milliseconds | Keypress-to-render event delta |
| **Workspace Context Switch** | < 30 milliseconds | Wayland surface reparenting benchmark |
| **Package Transaction Rollback** | < 500 milliseconds | Merkle store generation pointer swap |
| **Base Idle Memory Footprint** | < 250 Megabytes | VFS + Compositor + Init combined RSS |

---

## 3. Definition of Release Readiness (Gate Checklist)
An image is certified for release only when an independent user can:
1. Boot the installation ISO in QEMU or supported UEFI hardware without kernel panic.
2. Complete setup with under 3 prompts.
3. Arrive directly at a fluid Zenith Wayland desktop.
4. Execute `sigma-pkg install <pkg>` and verify signature authenticity.
5. Apply a system update, trigger an artificial power failure/revert, and cleanly roll back to the prior generation in one reboot cycle.
