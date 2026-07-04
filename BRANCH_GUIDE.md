# SigmaOS Branch Guide

This document explains every active branch, its purpose, and how to work with it.

## Branch Overview

| Branch | Target Archetype | Scheduler | Key Feature Flag |
|--------|-----------------|-----------|-----------------|
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
