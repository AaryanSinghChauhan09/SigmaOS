# SigmaOS Branch Guide

Full details of every active branch — purpose, current state, and how to work with it.

---

## Branch Map

| Branch | Purpose | Target Version | Priority |
|--------|---------|----------------|----------|
| `main` | Stable integration target | v15.x current | 🔴 Critical |
| `master` | Legacy mirror (deprecated) | — | — |
| `kernel-exp` | Real kernel implementation lab | v16.0 Apex | 🔴 Critical |
| `drivers-dev` | SDF hardware driver development | v16.0 Apex | 🔴 Critical |
| `fs-dev` | VFS, SigmaFS, Ext4 | v16.0 Apex | 🟠 High |
| `tools-dev` | CLI tools, docs, automation | v15.x ongoing | 🟠 High |
| `performance-optimized` | Scheduler tuning, SIMD, PGO | v16.0 Apex | 🟠 High |
| `docs-update` | Wiki, API docs, man pages | v15.x ongoing | 🟡 Medium |
| `prepare-sigmaos-launch` | v15.1 launch checklist | v15.1 | 🟡 Medium |
| `gh-pages` | GitHub Pages public website | live | 🟡 Medium |
| `release/standalone` | Full desktop profile | v15.1 | 🟠 High |
| `release/microkernel` | Minimal microkernel | v16.0 Apex | 🔴 Critical |
| `release/cloud` | Cloud/container headless | v17.0 Sovereign | 🟠 High |
| `release/distributed` | Multi-node cluster | v17.0 Sovereign | 🟡 Medium |
| `release/dual-boot` | Dual-boot coexistence | v16.0 Apex | 🟡 Medium |
| `release/rtos` | Hard real-time | v17.0 Sovereign | 🟡 Medium |
| `release/mobile` | ARM64/RISC-V mobile | v17.0 Sovereign | 🟡 Medium |
| `release/browser` | WASM browser demo | v15.1 | 🟢 Low |
| `release/app` | App store demo | v15.1 | 🟢 Low |

---

## Critical Path

```
kernel-exp (bootable kernel — EVERYTHING blocks on this)
    ├── drivers-dev (SDF driver launch mechanism)
    │       ├── release/standalone (GPU + Wi-Fi)
    │       └── release/mobile (ARM64 BSP)
    ├── fs-dev (VFS layer)
    │       ├── release/standalone (profile VFS load)
    │       └── release/cloud (dm-verity)
    └── release/microkernel (minimal kernel subset)

performance-optimized → kernel-exp scheduler merged first
release/cloud → kernel-exp cgroup + namespace
release/distributed → release/cloud
release/rtos → performance-optimized EDF + kernel-exp IRQ
release/dual-boot → kernel-exp sigma-boot.efi + fs-dev
```

---

## Merge Order

```
1.  kernel-exp → main          (Phase 0: QEMU CI passing)
2.  drivers-dev → main         (VESA + e1000 + VirtIO-GPU)
3.  fs-dev → main              (VFS + tmpfs + SigmaFS)
4.  tools-dev → main           (ongoing — every green CI run)
5.  performance-optimized → main (after kernel-exp)
6.  docs-update → main         (ongoing)
7.  release/microkernel ← main (branch from stable main)
8.  release/standalone ← main  (after GPU drivers)
9.  release/cloud ← main       (after cgroup enforcement)
10. release/mobile ← main      (after ARM64 BSP)
11. release/rtos ← main        (after EDF scheduler)
12. release/dual-boot ← main   (after sigma-boot.efi)
13. release/distributed ← release/cloud
14. release/browser / release/app ← main (after bootable ISO)
```

---

## Branch Uniformity (S-BUSE Pipeline)

All release branches are kept in sync with `main` via:

```bash
node tools/sync_all_branches.js
```

This checks out each `release/*` branch, merges from `main`, and pushes — guaranteeing structural parity across all profiles.

---

## Working with Branches

```bash
# Checkout a branch
git checkout tools-dev

# Sync with upstream main
git checkout tools-dev
git merge main

# Create a new feature branch
git checkout -b feat/my-feature main

# Sync all release branches with main
node tools/sync_all_branches.js
```

---

## PR Requirements (before merging to main)

1. CI green (`sigma_ci.yml`)
2. `CURRENT_PROBLEMS_MANIFEST.md` updated if fixing a bug
3. Kernel changes: QEMU smoke test log in PR description
4. New subsystems: wiki page in `wiki_repo/`
5. Reviewed by at least one maintainer

---

*See also: [Branch-Development-Roadmap](Branch-Development-Roadmap) · [Development-Roadmap](Development-Roadmap) · [Contributing](Contributing)*


---
## Merged from BRANCH_GUIDE.md
# SigmaOS Branch Guide

This document explains every active branch, its purpose, and how to work with it.

## Branch Overview

| Branch | Target Archetype | Scheduler | Key Feature Flag |
| -------- | ----------------- | ----------- | ----------------- |
| `main` | Stable baseline | MLFQ balanced | all default |
| `kernel-exp` | Kernel lab | in-progress | `SIGMA_KERNEL_EXP=1` |
| `drivers-dev` | Driver dev | standard | `SIGMA_SDF_DRIVERS=1` |
| `fs-dev` | FS dev | standard | `SIGMA_FS_DEV=1` |
| `tools-dev` | CLI / automation | standard | `SIGMA_TOOLS=1` |
| `performance-optimized` | SIMD + PGO | lock-free CAS | `SIGMA_PERF_OPT=1` |
| `docs-update` | Docs / wiki | standard | docs only |
| `prepare-sigmaos-launch` | Launch prep | standard | release gate |
| `gh-pages` | Static website | n/a | web only |
| `release/standalone` | Desktop | MLFQ | `PROFILE=standalone` |
| `release/microkernel` | Minimal | round-robin | `PROFILE=microkernel` |
| `release/cloud` | Headless cloud | MLFQ | `PROFILE=cloud` |
| `release/distributed` | Cluster | distributed | `PROFILE=distributed` |
| `release/dual-boot` | Dual-boot | MLFQ | `PROFILE=dualboot` |
| `release/rtos` | Hard RT | EDF | `PROFILE=rtos` |
| `release/mobile` | ARM64 mobile | P/C-state aware | `PROFILE=mobile` |
| `release/browser` | WASM demo | n/a | `PROFILE=browser` |
| `release/app` | App store demo | n/a | `PROFILE=app` |

## Working with Branches

```bash

# Check out a branch

git checkout tools-dev

# Sync a branch from main

git checkout release/standalone
git merge main

# Create a new feature branch

git checkout -b feat/my-feature main

# Submit work (never push directly to main)

git push origin feat/my-feature

# then open a PR

```

## Branch Uniformity

All release branches maintain structural parity via the S-BUSE pipeline:

```bash
node tools/sync_all_branches.js
```

This merges `main` into every `release/*` branch and pushes to origin.

## PR Requirements

Before opening a PR to `main`:

1. CI must be green (`sigma_ci.yml`)

2. Update `CURRENT_PROBLEMS_MANIFEST.md` if a bug is fixed

3. New subsystems need a corresponding wiki page in `wiki_repo/`

4. Kernel changes need a QEMU smoke test result in the PR description

## Deprecated Branches

- `master` — legacy mirror of `main`. Do not use. Will be removed after v16.0.

---

*See also: [DEVELOPMENT_ROADMAP.md](DEVELOPMENT_ROADMAP.md) · [CONTRIBUTING.md](CONTRIBUTING.md)*
